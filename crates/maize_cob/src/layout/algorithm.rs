use crate::layout::{Layout, shape::Shape, stride::Stride, tile::Tile, tuple::TupleWalkError};

// [a] ; (b) / [c] ; (d) == [c, [a/cd, d]] ; (db, (dbc, b))
pub fn tiled_divide(num: Layout, div: Tile) -> Result<Layout, TupleWalkError> {
    let (shape, stride) = num.decompose();
    let _: () = shape.get().same_topology(div.get())?;
    let tile_size = div.get().apply(|t| t.tile_size);
    let num_tiles = div.get().apply(|t| t.num_tiles);
    let mul = |a: &_, b: &_| *a * *b;
    let tile_stride = stride.get().try_apply2(&tile_size, mul)?;
    let local_tile_stride = tile_stride.try_apply2(&num_tiles, mul)?;
    let full_tile_size = tile_size.try_apply2(&num_tiles, mul)?;
    let num_local_tiles = shape.get().apply2_fallible(&full_tile_size, |&a, &cd| {
        a.is_multiple_of(cd)
            .then_some(a / cd)
            .ok_or(super::tuple::TupleWalkErrorReason::Divisibility)
    })?;
    let full_shape = Shape::new((num_tiles, Shape::new((num_local_tiles, tile_size))));
    let full_stride = Stride::new((tile_stride, Stride::new((local_tile_stride, stride))));
    Ok(Layout::try_new(full_shape, full_stride)?)
}

#[test]
fn test_divide_1d() {
    let lay = Layout::new(Shape::new((30,)), Stride::new((2,)));
    let tile = Tile::new((3,), (5,));
    let mut div = tiled_divide(lay, tile)
        .expect("Should be divisible...")
        .canonicalize();
    let prop = Layout::new(
        Shape::new((3, Shape::new((2, 5)))),
        Stride::new((10, Stride::new((30, 2)))),
    );
    println!("div={}", div.to_string());
    println!("prop={}", prop.to_string());
    assert_eq!(div, prop);
}

#[test]
fn test_divide_2d() {
    let shape = Shape::new((64, 32));
    let stride = Stride::new((32, 1));
    let lay = Layout::new(shape, stride);
    let tile = Tile::new((2, 2), (4, 4));
    let div = tiled_divide(lay, tile).expect("Should be divisible");
    let prop_sha = Shape::new((
        Shape::new((2, 2)),
        Shape::new((Shape::new((8, 4)), Shape::new((4, 4)))),
    ));
    let prop_str = Stride::new((
        Stride::new((4 * 32, 4 * 1)),
        Stride::new((Stride::new((8 * 32, 8 * 1)), Stride::new((32, 1)))),
    ));
    let prop = Layout::new(prop_sha, prop_str);
    assert_eq!(div, prop);
}
