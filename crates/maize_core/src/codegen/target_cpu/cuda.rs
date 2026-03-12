use crate::codegen::typed_func::ToCPU;

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

impl ToCPU for SM {
    fn cpu(&self) -> &str {
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
    fn triple(&self) -> &str {
        "nvptx64-nvidia-cuda"
    }
}
