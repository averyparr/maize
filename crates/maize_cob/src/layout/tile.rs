use crate::layout::tuple::{Tuple, TupleArg, TupleWalkError};

#[derive(Clone, Copy, PartialEq)]
pub struct TileInner {
    pub num_tiles: u64,
    pub tile_size: u64,
}
pub struct Tile(Tuple<TileInner>);

impl Tile {
    pub fn get(&self) -> &Tuple<TileInner> {
        &self.0
    }
    pub fn try_new(
        num_tiles: impl TupleArg<u64>,
        tile_size: impl TupleArg<u64>,
    ) -> Result<Self, TupleWalkError> {
        let num_tiles = Tuple::new(num_tiles);
        let tile_size = Tuple::new(tile_size);

        Ok(Self(num_tiles.try_apply2(
            &tile_size,
            |&num_tiles, &tile_size| TileInner {
                num_tiles,
                tile_size,
            },
        )?))
    }
    pub fn new(num_tiles: impl TupleArg<u64>, tile_size: impl TupleArg<u64>) -> Self {
        Self::try_new(num_tiles, tile_size)
            .expect("Mismatch found in topology of `num_tiles` and `tile_size`")
    }
}
