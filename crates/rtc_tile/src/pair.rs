use rtc_types::struct_reflect;

struct_reflect!(
    pub struct TilePair<TA, TB> {
        pub a: TA,
        pub b: TB,
    } => tile_pair
);
