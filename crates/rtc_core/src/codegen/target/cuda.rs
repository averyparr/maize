pub struct PTXOptions {
    pub sm: SM,
}

impl PTXOptions {
    pub fn cpu(&self) -> &'static str {
        self.sm.sm()
    }
    pub fn features(&self) -> &'static str {
        self.sm.ptx()
    }
}

pub enum SM {
    // Volta
    SM70,
    SM72,

    // Turing
    SM75,

    // Ampere
    SM80,
    SM86,
    SM87,

    // Ada Lovelace
    SM89,

    // Hopper
    SM90,
    SM90a,

    // Blackwell
    SM100,  // B100/B200
    SM100a, // Architecture-specific features
    SM101,  // Thor
    SM101a, // DIGITS
    SM120,  // RTX 50 series
}

impl SM {
    pub fn sm(&self) -> &'static str {
        match self {
            SM::SM70 => "sm_70",
            SM::SM72 => "sm_72",
            SM::SM75 => "sm_75",
            SM::SM80 => "sm_80",
            SM::SM86 => "sm_86",
            SM::SM87 => "sm_87",
            SM::SM89 => "sm_89",
            SM::SM90 => "sm_90",
            SM::SM90a => "sm_90a",
            SM::SM100 => "sm_100",
            SM::SM100a => "sm_100a",
            SM::SM101 => "sm_101",
            SM::SM101a => "sm_101a",
            SM::SM120 => "sm_120",
        }
    }

    pub fn compute(&self) -> &'static str {
        match self {
            SM::SM70 => "compute_70",
            SM::SM72 => "compute_72",
            SM::SM75 => "compute_75",
            SM::SM80 => "compute_80",
            SM::SM86 => "compute_86",
            SM::SM87 => "compute_87",
            SM::SM89 => "compute_89",
            SM::SM90 => "compute_90",
            SM::SM90a => "compute_90a",
            SM::SM100 => "compute_100",
            SM::SM100a => "compute_100a",
            SM::SM101 => "compute_101",
            SM::SM101a => "compute_101a",
            SM::SM120 => "compute_120",
        }
    }

    pub fn ptx(&self) -> &'static str {
        match self {
            SM::SM70 => "+ptx70",
            SM::SM72 => "+ptx72",
            SM::SM75 => "+ptx75",
            SM::SM80 => "+ptx80",
            SM::SM86 => "+ptx86",
            SM::SM87 => "+ptx87",
            SM::SM89 => "+ptx89",
            SM::SM90 => "+ptx90",
            SM::SM90a => "+ptx90a",
            SM::SM100 => "+ptx100",
            SM::SM100a => "+ptx100a",
            SM::SM101 => "+ptx101",
            SM::SM101a => "+ptx101a",
            SM::SM120 => "+ptx120",
        }
    }
}
