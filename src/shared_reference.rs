#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - REF Shared Reference control register 0"]
    pub refctl0: crate::Reg<refctl0::REFCTL0_SPEC>,
}
#[doc = "REFCTL0 register accessor: an alias for `Reg<REFCTL0_SPEC>`"]
pub type REFCTL0 = crate::Reg<refctl0::REFCTL0_SPEC>;
#[doc = "REF Shared Reference control register 0"]
pub mod refctl0;
