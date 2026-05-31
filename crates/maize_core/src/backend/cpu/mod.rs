pub mod cuda;

pub trait ToCPU {
    fn cpu(&self) -> &str;
    fn triple(&self) -> &str;
    fn features(&self) -> &str {
        ""
    }
}
