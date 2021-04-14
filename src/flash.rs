#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FLASH Control 1"]
    pub fctl1: crate::Reg<fctl1::FCTL1_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - FLASH Control 3"]
    pub fctl3: crate::Reg<fctl3::FCTL3_SPEC>,
    #[doc = "0x06 - FLASH Control 4"]
    pub fctl4: crate::Reg<fctl4::FCTL4_SPEC>,
}
#[doc = "FCTL1 register accessor: an alias for `Reg<FCTL1_SPEC>`"]
pub type FCTL1 = crate::Reg<fctl1::FCTL1_SPEC>;
#[doc = "FLASH Control 1"]
pub mod fctl1;
#[doc = "FCTL3 register accessor: an alias for `Reg<FCTL3_SPEC>`"]
pub type FCTL3 = crate::Reg<fctl3::FCTL3_SPEC>;
#[doc = "FLASH Control 3"]
pub mod fctl3;
#[doc = "FCTL4 register accessor: an alias for `Reg<FCTL4_SPEC>`"]
pub type FCTL4 = crate::Reg<fctl4::FCTL4_SPEC>;
#[doc = "FLASH Control 4"]
pub mod fctl4;
