#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable 1"]
    pub sfrie1: crate::Reg<sfrie1::SFRIE1_SPEC>,
    #[doc = "0x02 - Interrupt Flag 1"]
    pub sfrifg1: crate::Reg<sfrifg1::SFRIFG1_SPEC>,
    #[doc = "0x04 - RESET Pin Control Register"]
    pub sfrrpcr: crate::Reg<sfrrpcr::SFRRPCR_SPEC>,
}
#[doc = "SFRIE1 register accessor: an alias for `Reg<SFRIE1_SPEC>`"]
pub type SFRIE1 = crate::Reg<sfrie1::SFRIE1_SPEC>;
#[doc = "Interrupt Enable 1"]
pub mod sfrie1;
#[doc = "SFRIFG1 register accessor: an alias for `Reg<SFRIFG1_SPEC>`"]
pub type SFRIFG1 = crate::Reg<sfrifg1::SFRIFG1_SPEC>;
#[doc = "Interrupt Flag 1"]
pub mod sfrifg1;
#[doc = "SFRRPCR register accessor: an alias for `Reg<SFRRPCR_SPEC>`"]
pub type SFRRPCR = crate::Reg<sfrrpcr::SFRRPCR_SPEC>;
#[doc = "RESET Pin Control Register"]
pub mod sfrrpcr;
