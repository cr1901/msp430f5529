#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    rcctl0: RCCTL0,
}
impl RegisterBlock {
    #[doc = "0x00 - Ram Controller Control Register"]
    #[inline(always)]
    pub const fn rcctl0(&self) -> &RCCTL0 {
        &self.rcctl0
    }
}
#[doc = "RCCTL0 (rw) register accessor: Ram Controller Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcctl0`]
module"]
pub type RCCTL0 = crate::Reg<rcctl0::RCCTL0_SPEC>;
#[doc = "Ram Controller Control Register"]
pub mod rcctl0;
