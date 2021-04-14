#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - 32-bit operand 1 - multiply - low word"]
    pub mpy32l: crate::Reg<mpy32l::MPY32L_SPEC>,
    #[doc = "0x02 - 32-bit operand 1 - multiply - high word"]
    pub mpy32h: crate::Reg<mpy32h::MPY32H_SPEC>,
    #[doc = "0x04 - 32-bit operand 1 - signed multiply - low word"]
    pub mpys32l: crate::Reg<mpys32l::MPYS32L_SPEC>,
    #[doc = "0x06 - 32-bit operand 1 - signed multiply - high word"]
    pub mpys32h: crate::Reg<mpys32h::MPYS32H_SPEC>,
    #[doc = "0x08 - 32-bit operand 1 - multiply accumulate - low word"]
    pub mac32l: crate::Reg<mac32l::MAC32L_SPEC>,
    #[doc = "0x0a - 32-bit operand 1 - multiply accumulate - high word"]
    pub mac32h: crate::Reg<mac32h::MAC32H_SPEC>,
    #[doc = "0x0c - 32-bit operand 1 - signed multiply accumulate - low word"]
    pub macs32l: crate::Reg<macs32l::MACS32L_SPEC>,
    #[doc = "0x0e - 32-bit operand 1 - signed multiply accumulate - high word"]
    pub macs32h: crate::Reg<macs32h::MACS32H_SPEC>,
    #[doc = "0x10 - 32-bit operand 2 - low word"]
    pub op2l: crate::Reg<op2l::OP2L_SPEC>,
    #[doc = "0x12 - 32-bit operand 2 - high word"]
    pub op2h: crate::Reg<op2h::OP2H_SPEC>,
    #[doc = "0x14 - 32x32-bit result 0 - least significant word"]
    pub res0: crate::Reg<res0::RES0_SPEC>,
    #[doc = "0x16 - 32x32-bit result 1"]
    pub res1: crate::Reg<res1::RES1_SPEC>,
    #[doc = "0x18 - 32x32-bit result 2"]
    pub res2: crate::Reg<res2::RES2_SPEC>,
    #[doc = "0x1a - 32x32-bit result 3 - most significant word"]
    pub res3: crate::Reg<res3::RES3_SPEC>,
}
#[doc = "MPY32L register accessor: an alias for `Reg<MPY32L_SPEC>`"]
pub type MPY32L = crate::Reg<mpy32l::MPY32L_SPEC>;
#[doc = "32-bit operand 1 - multiply - low word"]
pub mod mpy32l;
#[doc = "MPY32H register accessor: an alias for `Reg<MPY32H_SPEC>`"]
pub type MPY32H = crate::Reg<mpy32h::MPY32H_SPEC>;
#[doc = "32-bit operand 1 - multiply - high word"]
pub mod mpy32h;
#[doc = "MPYS32L register accessor: an alias for `Reg<MPYS32L_SPEC>`"]
pub type MPYS32L = crate::Reg<mpys32l::MPYS32L_SPEC>;
#[doc = "32-bit operand 1 - signed multiply - low word"]
pub mod mpys32l;
#[doc = "MPYS32H register accessor: an alias for `Reg<MPYS32H_SPEC>`"]
pub type MPYS32H = crate::Reg<mpys32h::MPYS32H_SPEC>;
#[doc = "32-bit operand 1 - signed multiply - high word"]
pub mod mpys32h;
#[doc = "MAC32L register accessor: an alias for `Reg<MAC32L_SPEC>`"]
pub type MAC32L = crate::Reg<mac32l::MAC32L_SPEC>;
#[doc = "32-bit operand 1 - multiply accumulate - low word"]
pub mod mac32l;
#[doc = "MAC32H register accessor: an alias for `Reg<MAC32H_SPEC>`"]
pub type MAC32H = crate::Reg<mac32h::MAC32H_SPEC>;
#[doc = "32-bit operand 1 - multiply accumulate - high word"]
pub mod mac32h;
#[doc = "MACS32L register accessor: an alias for `Reg<MACS32L_SPEC>`"]
pub type MACS32L = crate::Reg<macs32l::MACS32L_SPEC>;
#[doc = "32-bit operand 1 - signed multiply accumulate - low word"]
pub mod macs32l;
#[doc = "MACS32H register accessor: an alias for `Reg<MACS32H_SPEC>`"]
pub type MACS32H = crate::Reg<macs32h::MACS32H_SPEC>;
#[doc = "32-bit operand 1 - signed multiply accumulate - high word"]
pub mod macs32h;
#[doc = "OP2L register accessor: an alias for `Reg<OP2L_SPEC>`"]
pub type OP2L = crate::Reg<op2l::OP2L_SPEC>;
#[doc = "32-bit operand 2 - low word"]
pub mod op2l;
#[doc = "OP2H register accessor: an alias for `Reg<OP2H_SPEC>`"]
pub type OP2H = crate::Reg<op2h::OP2H_SPEC>;
#[doc = "32-bit operand 2 - high word"]
pub mod op2h;
#[doc = "RES0 register accessor: an alias for `Reg<RES0_SPEC>`"]
pub type RES0 = crate::Reg<res0::RES0_SPEC>;
#[doc = "32x32-bit result 0 - least significant word"]
pub mod res0;
#[doc = "RES1 register accessor: an alias for `Reg<RES1_SPEC>`"]
pub type RES1 = crate::Reg<res1::RES1_SPEC>;
#[doc = "32x32-bit result 1"]
pub mod res1;
#[doc = "RES2 register accessor: an alias for `Reg<RES2_SPEC>`"]
pub type RES2 = crate::Reg<res2::RES2_SPEC>;
#[doc = "32x32-bit result 2"]
pub mod res2;
#[doc = "RES3 register accessor: an alias for `Reg<RES3_SPEC>`"]
pub type RES3 = crate::Reg<res3::RES3_SPEC>;
#[doc = "32x32-bit result 3 - most significant word"]
pub mod res3;
