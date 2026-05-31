mod fconv;

pub use fconv::*;

#[derive(Default, Clone, Copy)]
enum FloatRoundMode {
    #[default]
    Rn,
    Rz,
}
