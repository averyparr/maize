use maize_core::{
    backend::{LLVM, Opt, cpu::cuda::SM::SM100a},
    control_flow::{If, Looper},
    func::implement_ptx_kernel,
    intrinsics::cuda::tma::cp_async_bulk::{cp_async_bulk_g2s, cp_async_bulk_s2g},
    jit_assert,
    tipe::indexing::len,
};
use maize_pipe::new_producer_consumer;
use maize_types::cuda::{GlobalSlice, GlobalSliceMut};

fn main() {
    let ker = implement_ptx_kernel::<(GlobalSlice<[u8; 128]>, GlobalSliceMut<[u8; 128]>, u64)>(
        LLVM::new(&SM100a),
        "kernel",
    );
    {
        let (from, to, _count) = ker.args();
        {
            let mbars = ker.cuda().alloc_shared();
            let smem_buff = ker.cuda().alloc_shared();

            let (mut prod, mut cons) =
                new_producer_consumer::<[u8; 128], 6>(mbars, smem_buff, 128, 1);
            let warp_id = ker.cuda().sreg().warp_id();
            jit_assert!(len(&from).eq(len(&to)));
            let range = from.constant(0)..len(&to);
            If(warp_id.eq_const(0))
                .then(|| {
                    range.clone().for_each(|i| {
                        prod.step(|mbar, data| {
                            cp_async_bulk_g2s(data, mbar, from.index(i.copy()), None);
                        });
                    });
                })
                .or_else(|| {
                    range.clone().for_each(|i| {
                        cons.step(|data| {
                            cp_async_bulk_s2g(to.index(i.copy()), data, None);
                        });
                    });
                });
        }
    }
    ker.return_void();
    // println!("{}", ker.module_string());
    let b = ker.compile(Opt::O3);
    println!("{}", str::from_utf8(&b).expect("Should be valid"));
}
