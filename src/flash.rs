#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    fctl1: FCTL1,
    _reserved1: [u8; 0x02],
    fctl3: FCTL3,
    fctl4: FCTL4,
}
impl RegisterBlock {
    #[doc = "0x00 - FLASH Control 1"]
    #[inline(always)]
    pub const fn fctl1(&self) -> &FCTL1 {
        &self.fctl1
    }
    #[doc = "0x04 - FLASH Control 3"]
    #[inline(always)]
    pub const fn fctl3(&self) -> &FCTL3 {
        &self.fctl3
    }
    #[doc = "0x06 - FLASH Control 4"]
    #[inline(always)]
    pub const fn fctl4(&self) -> &FCTL4 {
        &self.fctl4
    }
}
#[doc = "FCTL1 (rw) register accessor: FLASH Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctl1`]
module"]
pub type FCTL1 = crate::Reg<fctl1::FCTL1_SPEC>;
#[doc = "FLASH Control 1"]
pub mod fctl1;
#[doc = "FCTL3 (rw) register accessor: FLASH Control 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctl3`]
module"]
pub type FCTL3 = crate::Reg<fctl3::FCTL3_SPEC>;
#[doc = "FLASH Control 3"]
pub mod fctl3;
#[doc = "FCTL4 (rw) register accessor: FLASH Control 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctl4`]
module"]
pub type FCTL4 = crate::Reg<fctl4::FCTL4_SPEC>;
#[doc = "FLASH Control 4"]
pub mod fctl4;
