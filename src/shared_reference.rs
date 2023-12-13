#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    refctl0: REFCTL0,
}
impl RegisterBlock {
    #[doc = "0x00 - REF Shared Reference control register 0"]
    #[inline(always)]
    pub const fn refctl0(&self) -> &REFCTL0 {
        &self.refctl0
    }
}
#[doc = "REFCTL0 (rw) register accessor: REF Shared Reference control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`refctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`refctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refctl0`]
module"]
pub type REFCTL0 = crate::Reg<refctl0::REFCTL0_SPEC>;
#[doc = "REF Shared Reference control register 0"]
pub mod refctl0;
