use maize_types::{
    codegen::loops::Looper,
    intrinsics::cuda::cp_async::CpAsyncPipeline,
    ty::{
        M, R, SizedTy,
        cuda::{Global, Shared},
    },
    val::Val,
};

use crate::{
    Tile, TilePair, WarpCollectiveTileTy, Window,
    gmem::{ColPanel, RowPanel},
    group::{ConstSizeGroup, Group, warp::Warp},
    mma::SyncMMAOp,
};

fn sync_mma_from_smem<'a, MMA, TileA, TileB>(
    _: &MMA,
    smem_a: Val<'a, R<&Tile<TileA>, Shared>>,
    smem_b: Val<'a, R<&Tile<TileB>, Shared>>,
    mut rmem_c: Val<'a, M<&mut MMA::FragC>>,
) where
    MMA: SyncMMAOp<FragA = TileA::FragT, FragB = TileB::FragT>,
    TileA: WarpCollectiveTileTy,
    TileB: WarpCollectiveTileTy,
{
    let cx = smem_a.cx();
    let lane = Warp::new(cx).index_size().0;
    let ra = TileA::collective_load(&smem_a, lane);
    let rb = TileB::collective_load(&smem_b, lane);
    rmem_c.store(MMA::call(ra, rb, rmem_c.load()));
}

pub struct KTile<const K_TILE: u32>;

pub fn sm80_gemm_smem_multibuffer<
    'a,
    MMA,
    RowWindow,
    ColWindow,
    TileA,
    TileB,
    Data,
    const K_TILE: u32,
    const PIPE: usize,
>(
    mma: &MMA,
    _: KTile<{ K_TILE }>,
    a: RowPanel<'a, RowWindow, R<&Data, Global>>,
    b: ColPanel<'a, ColWindow, R<&Data, Global>>,
    smem: Val<'a, M<&mut [TilePair<Tile<TileA>, Tile<TileB>>; PIPE], Shared>>,
    cp_group: impl ConstSizeGroup,
    cp_size: u8,
) -> Val<'a, MMA::FragC>
where
    MMA: SyncMMAOp<FragA = TileA::FragT, FragB = TileB::FragT>,
    Data: SizedTy,
    RowWindow: Window<ElemT = Data>,
    ColWindow: Window<ElemT = Data>,
    TileA: WarpCollectiveTileTy<ElemT = Data>,
    TileB: WarpCollectiveTileTy<ElemT = Data>,
{
    let depth: u32 = PIPE.try_into().expect("usize -> u32 overflow");
    let cx = smem.cx();

    let pipe = CpAsyncPipeline::new(depth, smem);

    let a_tiles = a.into_gmem_tiles::<{ K_TILE }>();
    let b_tiles = b.into_gmem_tiles::<{ K_TILE }>();
    let tiles = a_tiles.zip(b_tiles);

    let mut accum = MMA::zero_accum(cx).with_storage();
    let mut accum = accum.as_mut();
    let mut do_mma = |smem: Val<'_, R<&TilePair<Tile<TileA>, Tile<TileB>>, Shared>>| {
        let smem_a = smem.accessor().a;
        let smem_b = smem.accessor().b;
        sync_mma_from_smem(mma, smem_a, smem_b, accum.reborrow_mut());
    };
    pipe.prime_with(tiles, |token, (gmem_a, gmem_b), mut smem| {
        let smem_a = smem.accessor_mut().a;
        gmem_a.collective_cp_async(&token, smem_a, cp_group, cp_size, true);
        let smem_b = smem.accessor_mut().b;
        gmem_b.collective_cp_async(&token, smem_b, cp_group, cp_size, true);
    })
    .at_steady_state(&mut do_mma)
    .finalize(&mut do_mma);

    accum.load()
}
