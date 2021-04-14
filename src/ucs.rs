#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UCS Control Register 0"]
    pub ucsctl0: crate::Reg<ucsctl0::UCSCTL0_SPEC>,
    #[doc = "0x02 - UCS Control Register 1"]
    pub ucsctl1: crate::Reg<ucsctl1::UCSCTL1_SPEC>,
    #[doc = "0x04 - UCS Control Register 2"]
    pub ucsctl2: crate::Reg<ucsctl2::UCSCTL2_SPEC>,
    #[doc = "0x06 - UCS Control Register 3"]
    pub ucsctl3: crate::Reg<ucsctl3::UCSCTL3_SPEC>,
    #[doc = "0x08 - UCS Control Register 4"]
    pub ucsctl4: crate::Reg<ucsctl4::UCSCTL4_SPEC>,
    #[doc = "0x0a - UCS Control Register 5"]
    pub ucsctl5: crate::Reg<ucsctl5::UCSCTL5_SPEC>,
    #[doc = "0x0c - UCS Control Register 6"]
    pub ucsctl6: crate::Reg<ucsctl6::UCSCTL6_SPEC>,
    #[doc = "0x0e - UCS Control Register 7"]
    pub ucsctl7: crate::Reg<ucsctl7::UCSCTL7_SPEC>,
    #[doc = "0x10 - UCS Control Register 8"]
    pub ucsctl8: crate::Reg<ucsctl8::UCSCTL8_SPEC>,
}
#[doc = "UCSCTL0 register accessor: an alias for `Reg<UCSCTL0_SPEC>`"]
pub type UCSCTL0 = crate::Reg<ucsctl0::UCSCTL0_SPEC>;
#[doc = "UCS Control Register 0"]
pub mod ucsctl0;
#[doc = "UCSCTL1 register accessor: an alias for `Reg<UCSCTL1_SPEC>`"]
pub type UCSCTL1 = crate::Reg<ucsctl1::UCSCTL1_SPEC>;
#[doc = "UCS Control Register 1"]
pub mod ucsctl1;
#[doc = "UCSCTL2 register accessor: an alias for `Reg<UCSCTL2_SPEC>`"]
pub type UCSCTL2 = crate::Reg<ucsctl2::UCSCTL2_SPEC>;
#[doc = "UCS Control Register 2"]
pub mod ucsctl2;
#[doc = "UCSCTL3 register accessor: an alias for `Reg<UCSCTL3_SPEC>`"]
pub type UCSCTL3 = crate::Reg<ucsctl3::UCSCTL3_SPEC>;
#[doc = "UCS Control Register 3"]
pub mod ucsctl3;
#[doc = "UCSCTL4 register accessor: an alias for `Reg<UCSCTL4_SPEC>`"]
pub type UCSCTL4 = crate::Reg<ucsctl4::UCSCTL4_SPEC>;
#[doc = "UCS Control Register 4"]
pub mod ucsctl4;
#[doc = "UCSCTL5 register accessor: an alias for `Reg<UCSCTL5_SPEC>`"]
pub type UCSCTL5 = crate::Reg<ucsctl5::UCSCTL5_SPEC>;
#[doc = "UCS Control Register 5"]
pub mod ucsctl5;
#[doc = "UCSCTL6 register accessor: an alias for `Reg<UCSCTL6_SPEC>`"]
pub type UCSCTL6 = crate::Reg<ucsctl6::UCSCTL6_SPEC>;
#[doc = "UCS Control Register 6"]
pub mod ucsctl6;
#[doc = "UCSCTL7 register accessor: an alias for `Reg<UCSCTL7_SPEC>`"]
pub type UCSCTL7 = crate::Reg<ucsctl7::UCSCTL7_SPEC>;
#[doc = "UCS Control Register 7"]
pub mod ucsctl7;
#[doc = "UCSCTL8 register accessor: an alias for `Reg<UCSCTL8_SPEC>`"]
pub type UCSCTL8 = crate::Reg<ucsctl8::UCSCTL8_SPEC>;
#[doc = "UCS Control Register 8"]
pub mod ucsctl8;
