#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    sfrie1: SFRIE1,
    sfrifg1: SFRIFG1,
    sfrrpcr: SFRRPCR,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Enable 1"]
    #[inline(always)]
    pub const fn sfrie1(&self) -> &SFRIE1 {
        &self.sfrie1
    }
    #[doc = "0x02 - Interrupt Flag 1"]
    #[inline(always)]
    pub const fn sfrifg1(&self) -> &SFRIFG1 {
        &self.sfrifg1
    }
    #[doc = "0x04 - RESET Pin Control Register"]
    #[inline(always)]
    pub const fn sfrrpcr(&self) -> &SFRRPCR {
        &self.sfrrpcr
    }
}
#[doc = "SFRIE1 (rw) register accessor: Interrupt Enable 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfrie1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfrie1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfrie1`]
module"]
pub type SFRIE1 = crate::Reg<sfrie1::SFRIE1_SPEC>;
#[doc = "Interrupt Enable 1"]
pub mod sfrie1;
#[doc = "SFRIFG1 (rw) register accessor: Interrupt Flag 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfrifg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfrifg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfrifg1`]
module"]
pub type SFRIFG1 = crate::Reg<sfrifg1::SFRIFG1_SPEC>;
#[doc = "Interrupt Flag 1"]
pub mod sfrifg1;
#[doc = "SFRRPCR (rw) register accessor: RESET Pin Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfrrpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfrrpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfrrpcr`]
module"]
pub type SFRRPCR = crate::Reg<sfrrpcr::SFRRPCR_SPEC>;
#[doc = "RESET Pin Control Register"]
pub mod sfrrpcr;
