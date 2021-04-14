#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PMM Control 0"]
    pub pmmctl0: crate::Reg<pmmctl0::PMMCTL0_SPEC>,
    #[doc = "0x02 - PMM Control 1"]
    pub pmmctl1: crate::Reg<pmmctl1::PMMCTL1_SPEC>,
    #[doc = "0x04 - SVS and SVM high side control register"]
    pub svsmhctl: crate::Reg<svsmhctl::SVSMHCTL_SPEC>,
    #[doc = "0x06 - SVS and SVM low side control register"]
    pub svsmlctl: crate::Reg<svsmlctl::SVSMLCTL_SPEC>,
    #[doc = "0x08 - SVSIN and SVSOUT control register"]
    pub svsmio: crate::Reg<svsmio::SVSMIO_SPEC>,
    _reserved5: [u8; 0x02],
    #[doc = "0x0c - PMM Interrupt Flag"]
    pub pmmifg: crate::Reg<pmmifg::PMMIFG_SPEC>,
    #[doc = "0x0e - PMM and RESET Interrupt Enable"]
    pub pmmrie: crate::Reg<pmmrie::PMMRIE_SPEC>,
    #[doc = "0x10 - PMM Power Mode 5 Control Register 0"]
    pub pm5ctl0: crate::Reg<pm5ctl0::PM5CTL0_SPEC>,
}
#[doc = "PMMCTL0 register accessor: an alias for `Reg<PMMCTL0_SPEC>`"]
pub type PMMCTL0 = crate::Reg<pmmctl0::PMMCTL0_SPEC>;
#[doc = "PMM Control 0"]
pub mod pmmctl0;
#[doc = "PMMCTL1 register accessor: an alias for `Reg<PMMCTL1_SPEC>`"]
pub type PMMCTL1 = crate::Reg<pmmctl1::PMMCTL1_SPEC>;
#[doc = "PMM Control 1"]
pub mod pmmctl1;
#[doc = "SVSMHCTL register accessor: an alias for `Reg<SVSMHCTL_SPEC>`"]
pub type SVSMHCTL = crate::Reg<svsmhctl::SVSMHCTL_SPEC>;
#[doc = "SVS and SVM high side control register"]
pub mod svsmhctl;
#[doc = "SVSMLCTL register accessor: an alias for `Reg<SVSMLCTL_SPEC>`"]
pub type SVSMLCTL = crate::Reg<svsmlctl::SVSMLCTL_SPEC>;
#[doc = "SVS and SVM low side control register"]
pub mod svsmlctl;
#[doc = "SVSMIO register accessor: an alias for `Reg<SVSMIO_SPEC>`"]
pub type SVSMIO = crate::Reg<svsmio::SVSMIO_SPEC>;
#[doc = "SVSIN and SVSOUT control register"]
pub mod svsmio;
#[doc = "PMMIFG register accessor: an alias for `Reg<PMMIFG_SPEC>`"]
pub type PMMIFG = crate::Reg<pmmifg::PMMIFG_SPEC>;
#[doc = "PMM Interrupt Flag"]
pub mod pmmifg;
#[doc = "PMMRIE register accessor: an alias for `Reg<PMMRIE_SPEC>`"]
pub type PMMRIE = crate::Reg<pmmrie::PMMRIE_SPEC>;
#[doc = "PMM and RESET Interrupt Enable"]
pub mod pmmrie;
#[doc = "PM5CTL0 register accessor: an alias for `Reg<PM5CTL0_SPEC>`"]
pub type PM5CTL0 = crate::Reg<pm5ctl0::PM5CTL0_SPEC>;
#[doc = "PMM Power Mode 5 Control Register 0"]
pub mod pm5ctl0;
