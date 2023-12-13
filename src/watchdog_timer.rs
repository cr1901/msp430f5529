#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    wdtctl: WDTCTL,
}
impl RegisterBlock {
    #[doc = "0x00 - Watchdog Timer Control"]
    #[inline(always)]
    pub const fn wdtctl(&self) -> &WDTCTL {
        &self.wdtctl
    }
}
#[doc = "WDTCTL (rw) register accessor: Watchdog Timer Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtctl`]
module"]
pub type WDTCTL = crate::Reg<wdtctl::WDTCTL_SPEC>;
#[doc = "Watchdog Timer Control"]
pub mod wdtctl;
