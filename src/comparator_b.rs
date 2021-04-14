#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator B Control Register 0"]
    pub cbctl0: crate::Reg<cbctl0::CBCTL0_SPEC>,
    #[doc = "0x02 - Comparator B Control Register 1"]
    pub cbctl1: crate::Reg<cbctl1::CBCTL1_SPEC>,
    #[doc = "0x04 - Comparator B Control Register 2"]
    pub cbctl2: crate::Reg<cbctl2::CBCTL2_SPEC>,
    #[doc = "0x06 - Comparator B Control Register 3"]
    pub cbctl3: crate::Reg<cbctl3::CBCTL3_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x0c - Comparator B Interrupt Register"]
    pub cbint: crate::Reg<cbint::CBINT_SPEC>,
    #[doc = "0x0e - Comparator B Interrupt Vector Word"]
    pub cbiv: crate::Reg<cbiv::CBIV_SPEC>,
}
#[doc = "CBCTL0 register accessor: an alias for `Reg<CBCTL0_SPEC>`"]
pub type CBCTL0 = crate::Reg<cbctl0::CBCTL0_SPEC>;
#[doc = "Comparator B Control Register 0"]
pub mod cbctl0;
#[doc = "CBCTL1 register accessor: an alias for `Reg<CBCTL1_SPEC>`"]
pub type CBCTL1 = crate::Reg<cbctl1::CBCTL1_SPEC>;
#[doc = "Comparator B Control Register 1"]
pub mod cbctl1;
#[doc = "CBCTL2 register accessor: an alias for `Reg<CBCTL2_SPEC>`"]
pub type CBCTL2 = crate::Reg<cbctl2::CBCTL2_SPEC>;
#[doc = "Comparator B Control Register 2"]
pub mod cbctl2;
#[doc = "CBCTL3 register accessor: an alias for `Reg<CBCTL3_SPEC>`"]
pub type CBCTL3 = crate::Reg<cbctl3::CBCTL3_SPEC>;
#[doc = "Comparator B Control Register 3"]
pub mod cbctl3;
#[doc = "CBINT register accessor: an alias for `Reg<CBINT_SPEC>`"]
pub type CBINT = crate::Reg<cbint::CBINT_SPEC>;
#[doc = "Comparator B Interrupt Register"]
pub mod cbint;
#[doc = "CBIV register accessor: an alias for `Reg<CBIV_SPEC>`"]
pub type CBIV = crate::Reg<cbiv::CBIV_SPEC>;
#[doc = "Comparator B Interrupt Vector Word"]
pub mod cbiv;
