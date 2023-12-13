#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cbctl0: CBCTL0,
    cbctl1: CBCTL1,
    cbctl2: CBCTL2,
    cbctl3: CBCTL3,
    _reserved4: [u8; 0x04],
    cbint: CBINT,
    cbiv: CBIV,
}
impl RegisterBlock {
    #[doc = "0x00 - Comparator B Control Register 0"]
    #[inline(always)]
    pub const fn cbctl0(&self) -> &CBCTL0 {
        &self.cbctl0
    }
    #[doc = "0x02 - Comparator B Control Register 1"]
    #[inline(always)]
    pub const fn cbctl1(&self) -> &CBCTL1 {
        &self.cbctl1
    }
    #[doc = "0x04 - Comparator B Control Register 2"]
    #[inline(always)]
    pub const fn cbctl2(&self) -> &CBCTL2 {
        &self.cbctl2
    }
    #[doc = "0x06 - Comparator B Control Register 3"]
    #[inline(always)]
    pub const fn cbctl3(&self) -> &CBCTL3 {
        &self.cbctl3
    }
    #[doc = "0x0c - Comparator B Interrupt Register"]
    #[inline(always)]
    pub const fn cbint(&self) -> &CBINT {
        &self.cbint
    }
    #[doc = "0x0e - Comparator B Interrupt Vector Word"]
    #[inline(always)]
    pub const fn cbiv(&self) -> &CBIV {
        &self.cbiv
    }
}
#[doc = "CBCTL0 (rw) register accessor: Comparator B Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cbctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cbctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cbctl0`]
module"]
pub type CBCTL0 = crate::Reg<cbctl0::CBCTL0_SPEC>;
#[doc = "Comparator B Control Register 0"]
pub mod cbctl0;
#[doc = "CBCTL1 (rw) register accessor: Comparator B Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cbctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cbctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cbctl1`]
module"]
pub type CBCTL1 = crate::Reg<cbctl1::CBCTL1_SPEC>;
#[doc = "Comparator B Control Register 1"]
pub mod cbctl1;
#[doc = "CBCTL2 (rw) register accessor: Comparator B Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cbctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cbctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cbctl2`]
module"]
pub type CBCTL2 = crate::Reg<cbctl2::CBCTL2_SPEC>;
#[doc = "Comparator B Control Register 2"]
pub mod cbctl2;
#[doc = "CBCTL3 (rw) register accessor: Comparator B Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cbctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cbctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cbctl3`]
module"]
pub type CBCTL3 = crate::Reg<cbctl3::CBCTL3_SPEC>;
#[doc = "Comparator B Control Register 3"]
pub mod cbctl3;
#[doc = "CBINT (rw) register accessor: Comparator B Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cbint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cbint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cbint`]
module"]
pub type CBINT = crate::Reg<cbint::CBINT_SPEC>;
#[doc = "Comparator B Interrupt Register"]
pub mod cbint;
#[doc = "CBIV (rw) register accessor: Comparator B Interrupt Vector Word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cbiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cbiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cbiv`]
module"]
pub type CBIV = crate::Reg<cbiv::CBIV_SPEC>;
#[doc = "Comparator B Interrupt Vector Word"]
pub mod cbiv;
