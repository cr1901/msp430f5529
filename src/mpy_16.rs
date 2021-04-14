#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Multiply Unsigned/Operand 1"]
    pub mpy: crate::Reg<mpy::MPY_SPEC>,
    #[doc = "0x02 - Multiply Signed/Operand 1"]
    pub mpys: crate::Reg<mpys::MPYS_SPEC>,
    #[doc = "0x04 - Multiply Unsigned and Accumulate/Operand 1"]
    pub mac: crate::Reg<mac::MAC_SPEC>,
    #[doc = "0x06 - Multiply Signed and Accumulate/Operand 1"]
    pub macs: crate::Reg<macs::MACS_SPEC>,
    #[doc = "0x08 - Operand 2"]
    pub op2: crate::Reg<op2::OP2_SPEC>,
    #[doc = "0x0a - Result Low Word"]
    pub reslo: crate::Reg<reslo::RESLO_SPEC>,
    #[doc = "0x0c - Result High Word"]
    pub reshi: crate::Reg<reshi::RESHI_SPEC>,
    #[doc = "0x0e - Sum Extend"]
    pub sumext: crate::Reg<sumext::SUMEXT_SPEC>,
    _reserved8: [u8; 0x1c],
    #[doc = "0x2c - MPY32 Control Register 0"]
    pub mpy32ctl0: crate::Reg<mpy32ctl0::MPY32CTL0_SPEC>,
}
#[doc = "MPY register accessor: an alias for `Reg<MPY_SPEC>`"]
pub type MPY = crate::Reg<mpy::MPY_SPEC>;
#[doc = "Multiply Unsigned/Operand 1"]
pub mod mpy;
#[doc = "MPYS register accessor: an alias for `Reg<MPYS_SPEC>`"]
pub type MPYS = crate::Reg<mpys::MPYS_SPEC>;
#[doc = "Multiply Signed/Operand 1"]
pub mod mpys;
#[doc = "MAC register accessor: an alias for `Reg<MAC_SPEC>`"]
pub type MAC = crate::Reg<mac::MAC_SPEC>;
#[doc = "Multiply Unsigned and Accumulate/Operand 1"]
pub mod mac;
#[doc = "MACS register accessor: an alias for `Reg<MACS_SPEC>`"]
pub type MACS = crate::Reg<macs::MACS_SPEC>;
#[doc = "Multiply Signed and Accumulate/Operand 1"]
pub mod macs;
#[doc = "OP2 register accessor: an alias for `Reg<OP2_SPEC>`"]
pub type OP2 = crate::Reg<op2::OP2_SPEC>;
#[doc = "Operand 2"]
pub mod op2;
#[doc = "RESLO register accessor: an alias for `Reg<RESLO_SPEC>`"]
pub type RESLO = crate::Reg<reslo::RESLO_SPEC>;
#[doc = "Result Low Word"]
pub mod reslo;
#[doc = "RESHI register accessor: an alias for `Reg<RESHI_SPEC>`"]
pub type RESHI = crate::Reg<reshi::RESHI_SPEC>;
#[doc = "Result High Word"]
pub mod reshi;
#[doc = "SUMEXT register accessor: an alias for `Reg<SUMEXT_SPEC>`"]
pub type SUMEXT = crate::Reg<sumext::SUMEXT_SPEC>;
#[doc = "Sum Extend"]
pub mod sumext;
#[doc = "MPY32CTL0 register accessor: an alias for `Reg<MPY32CTL0_SPEC>`"]
pub type MPY32CTL0 = crate::Reg<mpy32ctl0::MPY32CTL0_SPEC>;
#[doc = "MPY32 Control Register 0"]
pub mod mpy32ctl0;
