#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ucsctl0: UCSCTL0,
    ucsctl1: UCSCTL1,
    ucsctl2: UCSCTL2,
    ucsctl3: UCSCTL3,
    ucsctl4: UCSCTL4,
    ucsctl5: UCSCTL5,
    ucsctl6: UCSCTL6,
    ucsctl7: UCSCTL7,
    ucsctl8: UCSCTL8,
}
impl RegisterBlock {
    #[doc = "0x00 - UCS Control Register 0"]
    #[inline(always)]
    pub const fn ucsctl0(&self) -> &UCSCTL0 {
        &self.ucsctl0
    }
    #[doc = "0x02 - UCS Control Register 1"]
    #[inline(always)]
    pub const fn ucsctl1(&self) -> &UCSCTL1 {
        &self.ucsctl1
    }
    #[doc = "0x04 - UCS Control Register 2"]
    #[inline(always)]
    pub const fn ucsctl2(&self) -> &UCSCTL2 {
        &self.ucsctl2
    }
    #[doc = "0x06 - UCS Control Register 3"]
    #[inline(always)]
    pub const fn ucsctl3(&self) -> &UCSCTL3 {
        &self.ucsctl3
    }
    #[doc = "0x08 - UCS Control Register 4"]
    #[inline(always)]
    pub const fn ucsctl4(&self) -> &UCSCTL4 {
        &self.ucsctl4
    }
    #[doc = "0x0a - UCS Control Register 5"]
    #[inline(always)]
    pub const fn ucsctl5(&self) -> &UCSCTL5 {
        &self.ucsctl5
    }
    #[doc = "0x0c - UCS Control Register 6"]
    #[inline(always)]
    pub const fn ucsctl6(&self) -> &UCSCTL6 {
        &self.ucsctl6
    }
    #[doc = "0x0e - UCS Control Register 7"]
    #[inline(always)]
    pub const fn ucsctl7(&self) -> &UCSCTL7 {
        &self.ucsctl7
    }
    #[doc = "0x10 - UCS Control Register 8"]
    #[inline(always)]
    pub const fn ucsctl8(&self) -> &UCSCTL8 {
        &self.ucsctl8
    }
}
#[doc = "UCSCTL0 (rw) register accessor: UCS Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsctl0`]
module"]
pub type UCSCTL0 = crate::Reg<ucsctl0::UCSCTL0_SPEC>;
#[doc = "UCS Control Register 0"]
pub mod ucsctl0;
#[doc = "UCSCTL1 (rw) register accessor: UCS Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsctl1`]
module"]
pub type UCSCTL1 = crate::Reg<ucsctl1::UCSCTL1_SPEC>;
#[doc = "UCS Control Register 1"]
pub mod ucsctl1;
#[doc = "UCSCTL2 (rw) register accessor: UCS Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsctl2`]
module"]
pub type UCSCTL2 = crate::Reg<ucsctl2::UCSCTL2_SPEC>;
#[doc = "UCS Control Register 2"]
pub mod ucsctl2;
#[doc = "UCSCTL3 (rw) register accessor: UCS Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsctl3`]
module"]
pub type UCSCTL3 = crate::Reg<ucsctl3::UCSCTL3_SPEC>;
#[doc = "UCS Control Register 3"]
pub mod ucsctl3;
#[doc = "UCSCTL4 (rw) register accessor: UCS Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsctl4`]
module"]
pub type UCSCTL4 = crate::Reg<ucsctl4::UCSCTL4_SPEC>;
#[doc = "UCS Control Register 4"]
pub mod ucsctl4;
#[doc = "UCSCTL5 (rw) register accessor: UCS Control Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsctl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsctl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsctl5`]
module"]
pub type UCSCTL5 = crate::Reg<ucsctl5::UCSCTL5_SPEC>;
#[doc = "UCS Control Register 5"]
pub mod ucsctl5;
#[doc = "UCSCTL6 (rw) register accessor: UCS Control Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsctl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsctl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsctl6`]
module"]
pub type UCSCTL6 = crate::Reg<ucsctl6::UCSCTL6_SPEC>;
#[doc = "UCS Control Register 6"]
pub mod ucsctl6;
#[doc = "UCSCTL7 (rw) register accessor: UCS Control Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsctl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsctl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsctl7`]
module"]
pub type UCSCTL7 = crate::Reg<ucsctl7::UCSCTL7_SPEC>;
#[doc = "UCS Control Register 7"]
pub mod ucsctl7;
#[doc = "UCSCTL8 (rw) register accessor: UCS Control Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsctl8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsctl8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucsctl8`]
module"]
pub type UCSCTL8 = crate::Reg<ucsctl8::UCSCTL8_SPEC>;
#[doc = "UCS Control Register 8"]
pub mod ucsctl8;
