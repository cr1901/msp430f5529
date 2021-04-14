#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port P4.0 mapping register"]
    pub p4map0: crate::Reg<p4map0::P4MAP0_SPEC>,
    #[doc = "0x01 - Port P4.1 mapping register"]
    pub p4map1: crate::Reg<p4map1::P4MAP1_SPEC>,
    #[doc = "0x02 - Port P4.2 mapping register"]
    pub p4map2: crate::Reg<p4map2::P4MAP2_SPEC>,
    #[doc = "0x03 - Port P4.3 mapping register"]
    pub p4map3: crate::Reg<p4map3::P4MAP3_SPEC>,
    #[doc = "0x04 - Port P4.4 mapping register"]
    pub p4map4: crate::Reg<p4map4::P4MAP4_SPEC>,
    #[doc = "0x05 - Port P4.5 mapping register"]
    pub p4map5: crate::Reg<p4map5::P4MAP5_SPEC>,
    #[doc = "0x06 - Port P4.6 mapping register"]
    pub p4map6: crate::Reg<p4map6::P4MAP6_SPEC>,
    #[doc = "0x07 - Port P4.7 mapping register"]
    pub p4map7: crate::Reg<p4map7::P4MAP7_SPEC>,
}
#[doc = "P4MAP0 register accessor: an alias for `Reg<P4MAP0_SPEC>`"]
pub type P4MAP0 = crate::Reg<p4map0::P4MAP0_SPEC>;
#[doc = "Port P4.0 mapping register"]
pub mod p4map0;
#[doc = "P4MAP1 register accessor: an alias for `Reg<P4MAP1_SPEC>`"]
pub type P4MAP1 = crate::Reg<p4map1::P4MAP1_SPEC>;
#[doc = "Port P4.1 mapping register"]
pub mod p4map1;
#[doc = "P4MAP2 register accessor: an alias for `Reg<P4MAP2_SPEC>`"]
pub type P4MAP2 = crate::Reg<p4map2::P4MAP2_SPEC>;
#[doc = "Port P4.2 mapping register"]
pub mod p4map2;
#[doc = "P4MAP3 register accessor: an alias for `Reg<P4MAP3_SPEC>`"]
pub type P4MAP3 = crate::Reg<p4map3::P4MAP3_SPEC>;
#[doc = "Port P4.3 mapping register"]
pub mod p4map3;
#[doc = "P4MAP4 register accessor: an alias for `Reg<P4MAP4_SPEC>`"]
pub type P4MAP4 = crate::Reg<p4map4::P4MAP4_SPEC>;
#[doc = "Port P4.4 mapping register"]
pub mod p4map4;
#[doc = "P4MAP5 register accessor: an alias for `Reg<P4MAP5_SPEC>`"]
pub type P4MAP5 = crate::Reg<p4map5::P4MAP5_SPEC>;
#[doc = "Port P4.5 mapping register"]
pub mod p4map5;
#[doc = "P4MAP6 register accessor: an alias for `Reg<P4MAP6_SPEC>`"]
pub type P4MAP6 = crate::Reg<p4map6::P4MAP6_SPEC>;
#[doc = "Port P4.6 mapping register"]
pub mod p4map6;
#[doc = "P4MAP7 register accessor: an alias for `Reg<P4MAP7_SPEC>`"]
pub type P4MAP7 = crate::Reg<p4map7::P4MAP7_SPEC>;
#[doc = "Port P4.7 mapping register"]
pub mod p4map7;
