mod compute_capability;

pub use compute_capability::SM;

pub struct PTXOptions {
    pub sm: SM,
}

impl PTXOptions {
    pub fn cpu(&self) -> &'static str {
        self.sm.compute()
    }
    pub fn features(&self) -> &'static str {
        self.sm.ptx()
    }
}
