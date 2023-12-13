#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pmmctl0: PMMCTL0,
    pmmctl1: PMMCTL1,
    svsmhctl: SVSMHCTL,
    svsmlctl: SVSMLCTL,
    svsmio: SVSMIO,
    _reserved5: [u8; 0x02],
    pmmifg: PMMIFG,
    pmmrie: PMMRIE,
    pm5ctl0: PM5CTL0,
}
impl RegisterBlock {
    #[doc = "0x00 - PMM Control 0"]
    #[inline(always)]
    pub const fn pmmctl0(&self) -> &PMMCTL0 {
        &self.pmmctl0
    }
    #[doc = "0x02 - PMM Control 1"]
    #[inline(always)]
    pub const fn pmmctl1(&self) -> &PMMCTL1 {
        &self.pmmctl1
    }
    #[doc = "0x04 - SVS and SVM high side control register"]
    #[inline(always)]
    pub const fn svsmhctl(&self) -> &SVSMHCTL {
        &self.svsmhctl
    }
    #[doc = "0x06 - SVS and SVM low side control register"]
    #[inline(always)]
    pub const fn svsmlctl(&self) -> &SVSMLCTL {
        &self.svsmlctl
    }
    #[doc = "0x08 - SVSIN and SVSOUT control register"]
    #[inline(always)]
    pub const fn svsmio(&self) -> &SVSMIO {
        &self.svsmio
    }
    #[doc = "0x0c - PMM Interrupt Flag"]
    #[inline(always)]
    pub const fn pmmifg(&self) -> &PMMIFG {
        &self.pmmifg
    }
    #[doc = "0x0e - PMM and RESET Interrupt Enable"]
    #[inline(always)]
    pub const fn pmmrie(&self) -> &PMMRIE {
        &self.pmmrie
    }
    #[doc = "0x10 - PMM Power Mode 5 Control Register 0"]
    #[inline(always)]
    pub const fn pm5ctl0(&self) -> &PM5CTL0 {
        &self.pm5ctl0
    }
}
#[doc = "PMMCTL0 (rw) register accessor: PMM Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmmctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmmctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmmctl0`]
module"]
pub type PMMCTL0 = crate::Reg<pmmctl0::PMMCTL0_SPEC>;
#[doc = "PMM Control 0"]
pub mod pmmctl0;
#[doc = "PMMCTL1 (rw) register accessor: PMM Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmmctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmmctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmmctl1`]
module"]
pub type PMMCTL1 = crate::Reg<pmmctl1::PMMCTL1_SPEC>;
#[doc = "PMM Control 1"]
pub mod pmmctl1;
#[doc = "SVSMHCTL (rw) register accessor: SVS and SVM high side control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`svsmhctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`svsmhctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@svsmhctl`]
module"]
pub type SVSMHCTL = crate::Reg<svsmhctl::SVSMHCTL_SPEC>;
#[doc = "SVS and SVM high side control register"]
pub mod svsmhctl;
#[doc = "SVSMLCTL (rw) register accessor: SVS and SVM low side control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`svsmlctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`svsmlctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@svsmlctl`]
module"]
pub type SVSMLCTL = crate::Reg<svsmlctl::SVSMLCTL_SPEC>;
#[doc = "SVS and SVM low side control register"]
pub mod svsmlctl;
#[doc = "SVSMIO (rw) register accessor: SVSIN and SVSOUT control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`svsmio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`svsmio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@svsmio`]
module"]
pub type SVSMIO = crate::Reg<svsmio::SVSMIO_SPEC>;
#[doc = "SVSIN and SVSOUT control register"]
pub mod svsmio;
#[doc = "PMMIFG (rw) register accessor: PMM Interrupt Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmmifg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmmifg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmmifg`]
module"]
pub type PMMIFG = crate::Reg<pmmifg::PMMIFG_SPEC>;
#[doc = "PMM Interrupt Flag"]
pub mod pmmifg;
#[doc = "PMMRIE (rw) register accessor: PMM and RESET Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmmrie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmmrie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmmrie`]
module"]
pub type PMMRIE = crate::Reg<pmmrie::PMMRIE_SPEC>;
#[doc = "PMM and RESET Interrupt Enable"]
pub mod pmmrie;
#[doc = "PM5CTL0 (rw) register accessor: PMM Power Mode 5 Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pm5ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pm5ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pm5ctl0`]
module"]
pub type PM5CTL0 = crate::Reg<pm5ctl0::PM5CTL0_SPEC>;
#[doc = "PMM Power Mode 5 Control Register 0"]
pub mod pm5ctl0;
