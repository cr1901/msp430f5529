#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    p4map0: P4MAP0,
    p4map1: P4MAP1,
    p4map2: P4MAP2,
    p4map3: P4MAP3,
    p4map4: P4MAP4,
    p4map5: P4MAP5,
    p4map6: P4MAP6,
    p4map7: P4MAP7,
}
impl RegisterBlock {
    #[doc = "0x00 - Port P4.0 mapping register"]
    #[inline(always)]
    pub const fn p4map0(&self) -> &P4MAP0 {
        &self.p4map0
    }
    #[doc = "0x01 - Port P4.1 mapping register"]
    #[inline(always)]
    pub const fn p4map1(&self) -> &P4MAP1 {
        &self.p4map1
    }
    #[doc = "0x02 - Port P4.2 mapping register"]
    #[inline(always)]
    pub const fn p4map2(&self) -> &P4MAP2 {
        &self.p4map2
    }
    #[doc = "0x03 - Port P4.3 mapping register"]
    #[inline(always)]
    pub const fn p4map3(&self) -> &P4MAP3 {
        &self.p4map3
    }
    #[doc = "0x04 - Port P4.4 mapping register"]
    #[inline(always)]
    pub const fn p4map4(&self) -> &P4MAP4 {
        &self.p4map4
    }
    #[doc = "0x05 - Port P4.5 mapping register"]
    #[inline(always)]
    pub const fn p4map5(&self) -> &P4MAP5 {
        &self.p4map5
    }
    #[doc = "0x06 - Port P4.6 mapping register"]
    #[inline(always)]
    pub const fn p4map6(&self) -> &P4MAP6 {
        &self.p4map6
    }
    #[doc = "0x07 - Port P4.7 mapping register"]
    #[inline(always)]
    pub const fn p4map7(&self) -> &P4MAP7 {
        &self.p4map7
    }
}
#[doc = "P4MAP0 (rw) register accessor: Port P4.0 mapping register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p4map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p4map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4map0`]
module"]
pub type P4MAP0 = crate::Reg<p4map0::P4MAP0_SPEC>;
#[doc = "Port P4.0 mapping register"]
pub mod p4map0;
#[doc = "P4MAP1 (rw) register accessor: Port P4.1 mapping register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p4map1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p4map1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4map1`]
module"]
pub type P4MAP1 = crate::Reg<p4map1::P4MAP1_SPEC>;
#[doc = "Port P4.1 mapping register"]
pub mod p4map1;
#[doc = "P4MAP2 (rw) register accessor: Port P4.2 mapping register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p4map2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p4map2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4map2`]
module"]
pub type P4MAP2 = crate::Reg<p4map2::P4MAP2_SPEC>;
#[doc = "Port P4.2 mapping register"]
pub mod p4map2;
#[doc = "P4MAP3 (rw) register accessor: Port P4.3 mapping register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p4map3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p4map3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4map3`]
module"]
pub type P4MAP3 = crate::Reg<p4map3::P4MAP3_SPEC>;
#[doc = "Port P4.3 mapping register"]
pub mod p4map3;
#[doc = "P4MAP4 (rw) register accessor: Port P4.4 mapping register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p4map4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p4map4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4map4`]
module"]
pub type P4MAP4 = crate::Reg<p4map4::P4MAP4_SPEC>;
#[doc = "Port P4.4 mapping register"]
pub mod p4map4;
#[doc = "P4MAP5 (rw) register accessor: Port P4.5 mapping register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p4map5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p4map5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4map5`]
module"]
pub type P4MAP5 = crate::Reg<p4map5::P4MAP5_SPEC>;
#[doc = "Port P4.5 mapping register"]
pub mod p4map5;
#[doc = "P4MAP6 (rw) register accessor: Port P4.6 mapping register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p4map6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p4map6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4map6`]
module"]
pub type P4MAP6 = crate::Reg<p4map6::P4MAP6_SPEC>;
#[doc = "Port P4.6 mapping register"]
pub mod p4map6;
#[doc = "P4MAP7 (rw) register accessor: Port P4.7 mapping register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p4map7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p4map7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4map7`]
module"]
pub type P4MAP7 = crate::Reg<p4map7::P4MAP7_SPEC>;
#[doc = "Port P4.7 mapping register"]
pub mod p4map7;
