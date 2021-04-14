#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start of buffer space"]
    pub usbstabuff: crate::Reg<usbstabuff::USBSTABUFF_SPEC>,
    _reserved1: [u8; 0x076e],
    #[doc = "0x76f - Top of buffer space"]
    pub usbtopbuff: crate::Reg<usbtopbuff::USBTOPBUFF_SPEC>,
    #[doc = "0x770 - Output endpoint_0 buffer"]
    pub usboep0buf: crate::Reg<usboep0buf::USBOEP0BUF_SPEC>,
    _reserved3: [u8; 0x07],
    #[doc = "0x778 - Input endpoint_0 buffer"]
    pub usbiep0buf: crate::Reg<usbiep0buf::USBIEP0BUF_SPEC>,
    _reserved4: [u8; 0x07],
    #[doc = "0x780 - Setup Packet Block"]
    pub usbsublk: crate::Reg<usbsublk::USBSUBLK_SPEC>,
    _reserved5: [u8; 0x07],
    #[doc = "0x788 - Output Endpoint_1: Configuration"]
    pub usboepcnf_1: crate::Reg<usboepcnf_1::USBOEPCNF_1_SPEC>,
    #[doc = "0x789 - Output Endpoint_1: X-buffer base addr."]
    pub usboepbbax_1: crate::Reg<usboepbbax_1::USBOEPBBAX_1_SPEC>,
    #[doc = "0x78a - Output Endpoint_1: X-byte count"]
    pub usboepbctx_1: crate::Reg<usboepbctx_1::USBOEPBCTX_1_SPEC>,
    _reserved8: [u8; 0x02],
    #[doc = "0x78d - Output Endpoint_1: Y-buffer base addr."]
    pub usboepbbay_1: crate::Reg<usboepbbay_1::USBOEPBBAY_1_SPEC>,
    #[doc = "0x78e - Output Endpoint_1: Y-byte count"]
    pub usboepbcty_1: crate::Reg<usboepbcty_1::USBOEPBCTY_1_SPEC>,
    #[doc = "0x78f - Output Endpoint_1: X/Y-buffer size"]
    pub usboepsizxy_1: crate::Reg<usboepsizxy_1::USBOEPSIZXY_1_SPEC>,
    #[doc = "0x790 - Output Endpoint_2: Configuration"]
    pub usboepcnf_2: crate::Reg<usboepcnf_2::USBOEPCNF_2_SPEC>,
    #[doc = "0x791 - Output Endpoint_2: X-buffer base addr."]
    pub usboepbbax_2: crate::Reg<usboepbbax_2::USBOEPBBAX_2_SPEC>,
    #[doc = "0x792 - Output Endpoint_2: X-byte count"]
    pub usboepbctx_2: crate::Reg<usboepbctx_2::USBOEPBCTX_2_SPEC>,
    _reserved14: [u8; 0x02],
    #[doc = "0x795 - Output Endpoint_2: Y-buffer base addr."]
    pub usboepbbay_2: crate::Reg<usboepbbay_2::USBOEPBBAY_2_SPEC>,
    #[doc = "0x796 - Output Endpoint_2: Y-byte count"]
    pub usboepbcty_2: crate::Reg<usboepbcty_2::USBOEPBCTY_2_SPEC>,
    #[doc = "0x797 - Output Endpoint_2: X/Y-buffer size"]
    pub usboepsizxy_2: crate::Reg<usboepsizxy_2::USBOEPSIZXY_2_SPEC>,
    #[doc = "0x798 - Output Endpoint_3: Configuration"]
    pub usboepcnf_3: crate::Reg<usboepcnf_3::USBOEPCNF_3_SPEC>,
    #[doc = "0x799 - Output Endpoint_3: X-buffer base addr."]
    pub usboepbbax_3: crate::Reg<usboepbbax_3::USBOEPBBAX_3_SPEC>,
    #[doc = "0x79a - Output Endpoint_3: X-byte count"]
    pub usboepbctx_3: crate::Reg<usboepbctx_3::USBOEPBCTX_3_SPEC>,
    _reserved20: [u8; 0x02],
    #[doc = "0x79d - Output Endpoint_3: Y-buffer base addr."]
    pub usboepbbay_3: crate::Reg<usboepbbay_3::USBOEPBBAY_3_SPEC>,
    #[doc = "0x79e - Output Endpoint_3: Y-byte count"]
    pub usboepbcty_3: crate::Reg<usboepbcty_3::USBOEPBCTY_3_SPEC>,
    #[doc = "0x79f - Output Endpoint_3: X/Y-buffer size"]
    pub usboepsizxy_3: crate::Reg<usboepsizxy_3::USBOEPSIZXY_3_SPEC>,
    #[doc = "0x7a0 - Output Endpoint_4: Configuration"]
    pub usboepcnf_4: crate::Reg<usboepcnf_4::USBOEPCNF_4_SPEC>,
    #[doc = "0x7a1 - Output Endpoint_4: X-buffer base addr."]
    pub usboepbbax_4: crate::Reg<usboepbbax_4::USBOEPBBAX_4_SPEC>,
    #[doc = "0x7a2 - Output Endpoint_4: X-byte count"]
    pub usboepbctx_4: crate::Reg<usboepbctx_4::USBOEPBCTX_4_SPEC>,
    _reserved26: [u8; 0x02],
    #[doc = "0x7a5 - Output Endpoint_4: Y-buffer base addr."]
    pub usboepbbay_4: crate::Reg<usboepbbay_4::USBOEPBBAY_4_SPEC>,
    #[doc = "0x7a6 - Output Endpoint_4: Y-byte count"]
    pub usboepbcty_4: crate::Reg<usboepbcty_4::USBOEPBCTY_4_SPEC>,
    #[doc = "0x7a7 - Output Endpoint_4: X/Y-buffer size"]
    pub usboepsizxy_4: crate::Reg<usboepsizxy_4::USBOEPSIZXY_4_SPEC>,
    #[doc = "0x7a8 - Output Endpoint_5: Configuration"]
    pub usboepcnf_5: crate::Reg<usboepcnf_5::USBOEPCNF_5_SPEC>,
    #[doc = "0x7a9 - Output Endpoint_5: X-buffer base addr."]
    pub usboepbbax_5: crate::Reg<usboepbbax_5::USBOEPBBAX_5_SPEC>,
    #[doc = "0x7aa - Output Endpoint_5: X-byte count"]
    pub usboepbctx_5: crate::Reg<usboepbctx_5::USBOEPBCTX_5_SPEC>,
    _reserved32: [u8; 0x02],
    #[doc = "0x7ad - Output Endpoint_5: Y-buffer base addr."]
    pub usboepbbay_5: crate::Reg<usboepbbay_5::USBOEPBBAY_5_SPEC>,
    #[doc = "0x7ae - Output Endpoint_5: Y-byte count"]
    pub usboepbcty_5: crate::Reg<usboepbcty_5::USBOEPBCTY_5_SPEC>,
    #[doc = "0x7af - Output Endpoint_5: X/Y-buffer size"]
    pub usboepsizxy_5: crate::Reg<usboepsizxy_5::USBOEPSIZXY_5_SPEC>,
    #[doc = "0x7b0 - Output Endpoint_6: Configuration"]
    pub usboepcnf_6: crate::Reg<usboepcnf_6::USBOEPCNF_6_SPEC>,
    #[doc = "0x7b1 - Output Endpoint_6: X-buffer base addr."]
    pub usboepbbax_6: crate::Reg<usboepbbax_6::USBOEPBBAX_6_SPEC>,
    #[doc = "0x7b2 - Output Endpoint_6: X-byte count"]
    pub usboepbctx_6: crate::Reg<usboepbctx_6::USBOEPBCTX_6_SPEC>,
    _reserved38: [u8; 0x02],
    #[doc = "0x7b5 - Output Endpoint_6: Y-buffer base addr."]
    pub usboepbbay_6: crate::Reg<usboepbbay_6::USBOEPBBAY_6_SPEC>,
    #[doc = "0x7b6 - Output Endpoint_6: Y-byte count"]
    pub usboepbcty_6: crate::Reg<usboepbcty_6::USBOEPBCTY_6_SPEC>,
    #[doc = "0x7b7 - Output Endpoint_6: X/Y-buffer size"]
    pub usboepsizxy_6: crate::Reg<usboepsizxy_6::USBOEPSIZXY_6_SPEC>,
    #[doc = "0x7b8 - Output Endpoint_7: Configuration"]
    pub usboepcnf_7: crate::Reg<usboepcnf_7::USBOEPCNF_7_SPEC>,
    #[doc = "0x7b9 - Output Endpoint_7: X-buffer base addr."]
    pub usboepbbax_7: crate::Reg<usboepbbax_7::USBOEPBBAX_7_SPEC>,
    #[doc = "0x7ba - Output Endpoint_7: X-byte count"]
    pub usboepbctx_7: crate::Reg<usboepbctx_7::USBOEPBCTX_7_SPEC>,
    _reserved44: [u8; 0x02],
    #[doc = "0x7bd - Output Endpoint_7: Y-buffer base addr."]
    pub usboepbbay_7: crate::Reg<usboepbbay_7::USBOEPBBAY_7_SPEC>,
    #[doc = "0x7be - Output Endpoint_7: Y-byte count"]
    pub usboepbcty_7: crate::Reg<usboepbcty_7::USBOEPBCTY_7_SPEC>,
    #[doc = "0x7bf - Output Endpoint_7: X/Y-buffer size"]
    pub usboepsizxy_7: crate::Reg<usboepsizxy_7::USBOEPSIZXY_7_SPEC>,
    _reserved47: [u8; 0x08],
    #[doc = "0x7c8 - Input Endpoint_1: Configuration"]
    pub usbiepcnf_1: crate::Reg<usbiepcnf_1::USBIEPCNF_1_SPEC>,
    #[doc = "0x7c9 - Input Endpoint_1: X-buffer base addr."]
    pub usbiepbbax_1: crate::Reg<usbiepbbax_1::USBIEPBBAX_1_SPEC>,
    #[doc = "0x7ca - Input Endpoint_1: X-byte count"]
    pub usbiepbctx_1: crate::Reg<usbiepbctx_1::USBIEPBCTX_1_SPEC>,
    _reserved50: [u8; 0x02],
    #[doc = "0x7cd - Input Endpoint_1: Y-buffer base addr."]
    pub usbiepbbay_1: crate::Reg<usbiepbbay_1::USBIEPBBAY_1_SPEC>,
    #[doc = "0x7ce - Input Endpoint_1: Y-byte count"]
    pub usbiepbcty_1: crate::Reg<usbiepbcty_1::USBIEPBCTY_1_SPEC>,
    #[doc = "0x7cf - Input Endpoint_1: X/Y-buffer size"]
    pub usbiepsizxy_1: crate::Reg<usbiepsizxy_1::USBIEPSIZXY_1_SPEC>,
    #[doc = "0x7d0 - Input Endpoint_2: Configuration"]
    pub usbiepcnf_2: crate::Reg<usbiepcnf_2::USBIEPCNF_2_SPEC>,
    #[doc = "0x7d1 - Input Endpoint_2: X-buffer base addr."]
    pub usbiepbbax_2: crate::Reg<usbiepbbax_2::USBIEPBBAX_2_SPEC>,
    #[doc = "0x7d2 - Input Endpoint_2: X-byte count"]
    pub usbiepbctx_2: crate::Reg<usbiepbctx_2::USBIEPBCTX_2_SPEC>,
    _reserved56: [u8; 0x02],
    #[doc = "0x7d5 - Input Endpoint_2: Y-buffer base addr."]
    pub usbiepbbay_2: crate::Reg<usbiepbbay_2::USBIEPBBAY_2_SPEC>,
    #[doc = "0x7d6 - Input Endpoint_2: Y-byte count"]
    pub usbiepbcty_2: crate::Reg<usbiepbcty_2::USBIEPBCTY_2_SPEC>,
    #[doc = "0x7d7 - Input Endpoint_2: X/Y-buffer size"]
    pub usbiepsizxy_2: crate::Reg<usbiepsizxy_2::USBIEPSIZXY_2_SPEC>,
    #[doc = "0x7d8 - Input Endpoint_3: Configuration"]
    pub usbiepcnf_3: crate::Reg<usbiepcnf_3::USBIEPCNF_3_SPEC>,
    #[doc = "0x7d9 - Input Endpoint_3: X-buffer base addr."]
    pub usbiepbbax_3: crate::Reg<usbiepbbax_3::USBIEPBBAX_3_SPEC>,
    #[doc = "0x7da - Input Endpoint_3: X-byte count"]
    pub usbiepbctx_3: crate::Reg<usbiepbctx_3::USBIEPBCTX_3_SPEC>,
    _reserved62: [u8; 0x02],
    #[doc = "0x7dd - Input Endpoint_3: Y-buffer base addr."]
    pub usbiepbbay_3: crate::Reg<usbiepbbay_3::USBIEPBBAY_3_SPEC>,
    #[doc = "0x7de - Input Endpoint_3: Y-byte count"]
    pub usbiepbcty_3: crate::Reg<usbiepbcty_3::USBIEPBCTY_3_SPEC>,
    #[doc = "0x7df - Input Endpoint_3: X/Y-buffer size"]
    pub usbiepsizxy_3: crate::Reg<usbiepsizxy_3::USBIEPSIZXY_3_SPEC>,
    #[doc = "0x7e0 - Input Endpoint_4: Configuration"]
    pub usbiepcnf_4: crate::Reg<usbiepcnf_4::USBIEPCNF_4_SPEC>,
    #[doc = "0x7e1 - Input Endpoint_4: X-buffer base addr."]
    pub usbiepbbax_4: crate::Reg<usbiepbbax_4::USBIEPBBAX_4_SPEC>,
    #[doc = "0x7e2 - Input Endpoint_4: X-byte count"]
    pub usbiepbctx_4: crate::Reg<usbiepbctx_4::USBIEPBCTX_4_SPEC>,
    _reserved68: [u8; 0x02],
    #[doc = "0x7e5 - Input Endpoint_4: Y-buffer base addr."]
    pub usbiepbbay_4: crate::Reg<usbiepbbay_4::USBIEPBBAY_4_SPEC>,
    #[doc = "0x7e6 - Input Endpoint_4: Y-byte count"]
    pub usbiepbcty_4: crate::Reg<usbiepbcty_4::USBIEPBCTY_4_SPEC>,
    #[doc = "0x7e7 - Input Endpoint_4: X/Y-buffer size"]
    pub usbiepsizxy_4: crate::Reg<usbiepsizxy_4::USBIEPSIZXY_4_SPEC>,
    #[doc = "0x7e8 - Input Endpoint_5: Configuration"]
    pub usbiepcnf_5: crate::Reg<usbiepcnf_5::USBIEPCNF_5_SPEC>,
    #[doc = "0x7e9 - Input Endpoint_5: X-buffer base addr."]
    pub usbiepbbax_5: crate::Reg<usbiepbbax_5::USBIEPBBAX_5_SPEC>,
    #[doc = "0x7ea - Input Endpoint_5: X-byte count"]
    pub usbiepbctx_5: crate::Reg<usbiepbctx_5::USBIEPBCTX_5_SPEC>,
    _reserved74: [u8; 0x02],
    #[doc = "0x7ed - Input Endpoint_5: Y-buffer base addr."]
    pub usbiepbbay_5: crate::Reg<usbiepbbay_5::USBIEPBBAY_5_SPEC>,
    #[doc = "0x7ee - Input Endpoint_5: Y-byte count"]
    pub usbiepbcty_5: crate::Reg<usbiepbcty_5::USBIEPBCTY_5_SPEC>,
    #[doc = "0x7ef - Input Endpoint_5: X/Y-buffer size"]
    pub usbiepsizxy_5: crate::Reg<usbiepsizxy_5::USBIEPSIZXY_5_SPEC>,
    #[doc = "0x7f0 - Input Endpoint_6: Configuration"]
    pub usbiepcnf_6: crate::Reg<usbiepcnf_6::USBIEPCNF_6_SPEC>,
    #[doc = "0x7f1 - Input Endpoint_6: X-buffer base addr."]
    pub usbiepbbax_6: crate::Reg<usbiepbbax_6::USBIEPBBAX_6_SPEC>,
    #[doc = "0x7f2 - Input Endpoint_6: X-byte count"]
    pub usbiepbctx_6: crate::Reg<usbiepbctx_6::USBIEPBCTX_6_SPEC>,
    _reserved80: [u8; 0x02],
    #[doc = "0x7f5 - Input Endpoint_6: Y-buffer base addr."]
    pub usbiepbbay_6: crate::Reg<usbiepbbay_6::USBIEPBBAY_6_SPEC>,
    #[doc = "0x7f6 - Input Endpoint_6: Y-byte count"]
    pub usbiepbcty_6: crate::Reg<usbiepbcty_6::USBIEPBCTY_6_SPEC>,
    #[doc = "0x7f7 - Input Endpoint_6: X/Y-buffer size"]
    pub usbiepsizxy_6: crate::Reg<usbiepsizxy_6::USBIEPSIZXY_6_SPEC>,
    #[doc = "0x7f8 - Input Endpoint_7: Configuration"]
    pub usbiepcnf_7: crate::Reg<usbiepcnf_7::USBIEPCNF_7_SPEC>,
    #[doc = "0x7f9 - Input Endpoint_7: X-buffer base addr."]
    pub usbiepbbax_7: crate::Reg<usbiepbbax_7::USBIEPBBAX_7_SPEC>,
    #[doc = "0x7fa - Input Endpoint_7: X-byte count"]
    pub usbiepbctx_7: crate::Reg<usbiepbctx_7::USBIEPBCTX_7_SPEC>,
    _reserved86: [u8; 0x02],
    #[doc = "0x7fd - Input Endpoint_7: Y-buffer base addr."]
    pub usbiepbbay_7: crate::Reg<usbiepbbay_7::USBIEPBBAY_7_SPEC>,
    #[doc = "0x7fe - Input Endpoint_7: Y-byte count"]
    pub usbiepbcty_7: crate::Reg<usbiepbcty_7::USBIEPBCTY_7_SPEC>,
    #[doc = "0x7ff - Input Endpoint_7: X/Y-buffer size"]
    pub usbiepsizxy_7: crate::Reg<usbiepsizxy_7::USBIEPSIZXY_7_SPEC>,
}
#[doc = "USBSTABUFF register accessor: an alias for `Reg<USBSTABUFF_SPEC>`"]
pub type USBSTABUFF = crate::Reg<usbstabuff::USBSTABUFF_SPEC>;
#[doc = "Start of buffer space"]
pub mod usbstabuff;
#[doc = "USBTOPBUFF register accessor: an alias for `Reg<USBTOPBUFF_SPEC>`"]
pub type USBTOPBUFF = crate::Reg<usbtopbuff::USBTOPBUFF_SPEC>;
#[doc = "Top of buffer space"]
pub mod usbtopbuff;
#[doc = "USBOEP0BUF register accessor: an alias for `Reg<USBOEP0BUF_SPEC>`"]
pub type USBOEP0BUF = crate::Reg<usboep0buf::USBOEP0BUF_SPEC>;
#[doc = "Output endpoint_0 buffer"]
pub mod usboep0buf;
#[doc = "USBIEP0BUF register accessor: an alias for `Reg<USBIEP0BUF_SPEC>`"]
pub type USBIEP0BUF = crate::Reg<usbiep0buf::USBIEP0BUF_SPEC>;
#[doc = "Input endpoint_0 buffer"]
pub mod usbiep0buf;
#[doc = "USBSUBLK register accessor: an alias for `Reg<USBSUBLK_SPEC>`"]
pub type USBSUBLK = crate::Reg<usbsublk::USBSUBLK_SPEC>;
#[doc = "Setup Packet Block"]
pub mod usbsublk;
#[doc = "USBOEPCNF_1 register accessor: an alias for `Reg<USBOEPCNF_1_SPEC>`"]
pub type USBOEPCNF_1 = crate::Reg<usboepcnf_1::USBOEPCNF_1_SPEC>;
#[doc = "Output Endpoint_1: Configuration"]
pub mod usboepcnf_1;
#[doc = "USBOEPBBAX_1 register accessor: an alias for `Reg<USBOEPBBAX_1_SPEC>`"]
pub type USBOEPBBAX_1 = crate::Reg<usboepbbax_1::USBOEPBBAX_1_SPEC>;
#[doc = "Output Endpoint_1: X-buffer base addr."]
pub mod usboepbbax_1;
#[doc = "USBOEPBCTX_1 register accessor: an alias for `Reg<USBOEPBCTX_1_SPEC>`"]
pub type USBOEPBCTX_1 = crate::Reg<usboepbctx_1::USBOEPBCTX_1_SPEC>;
#[doc = "Output Endpoint_1: X-byte count"]
pub mod usboepbctx_1;
#[doc = "USBOEPBBAY_1 register accessor: an alias for `Reg<USBOEPBBAY_1_SPEC>`"]
pub type USBOEPBBAY_1 = crate::Reg<usboepbbay_1::USBOEPBBAY_1_SPEC>;
#[doc = "Output Endpoint_1: Y-buffer base addr."]
pub mod usboepbbay_1;
#[doc = "USBOEPBCTY_1 register accessor: an alias for `Reg<USBOEPBCTY_1_SPEC>`"]
pub type USBOEPBCTY_1 = crate::Reg<usboepbcty_1::USBOEPBCTY_1_SPEC>;
#[doc = "Output Endpoint_1: Y-byte count"]
pub mod usboepbcty_1;
#[doc = "USBOEPSIZXY_1 register accessor: an alias for `Reg<USBOEPSIZXY_1_SPEC>`"]
pub type USBOEPSIZXY_1 = crate::Reg<usboepsizxy_1::USBOEPSIZXY_1_SPEC>;
#[doc = "Output Endpoint_1: X/Y-buffer size"]
pub mod usboepsizxy_1;
#[doc = "USBOEPCNF_2 register accessor: an alias for `Reg<USBOEPCNF_2_SPEC>`"]
pub type USBOEPCNF_2 = crate::Reg<usboepcnf_2::USBOEPCNF_2_SPEC>;
#[doc = "Output Endpoint_2: Configuration"]
pub mod usboepcnf_2;
#[doc = "USBOEPBBAX_2 register accessor: an alias for `Reg<USBOEPBBAX_2_SPEC>`"]
pub type USBOEPBBAX_2 = crate::Reg<usboepbbax_2::USBOEPBBAX_2_SPEC>;
#[doc = "Output Endpoint_2: X-buffer base addr."]
pub mod usboepbbax_2;
#[doc = "USBOEPBCTX_2 register accessor: an alias for `Reg<USBOEPBCTX_2_SPEC>`"]
pub type USBOEPBCTX_2 = crate::Reg<usboepbctx_2::USBOEPBCTX_2_SPEC>;
#[doc = "Output Endpoint_2: X-byte count"]
pub mod usboepbctx_2;
#[doc = "USBOEPBBAY_2 register accessor: an alias for `Reg<USBOEPBBAY_2_SPEC>`"]
pub type USBOEPBBAY_2 = crate::Reg<usboepbbay_2::USBOEPBBAY_2_SPEC>;
#[doc = "Output Endpoint_2: Y-buffer base addr."]
pub mod usboepbbay_2;
#[doc = "USBOEPBCTY_2 register accessor: an alias for `Reg<USBOEPBCTY_2_SPEC>`"]
pub type USBOEPBCTY_2 = crate::Reg<usboepbcty_2::USBOEPBCTY_2_SPEC>;
#[doc = "Output Endpoint_2: Y-byte count"]
pub mod usboepbcty_2;
#[doc = "USBOEPSIZXY_2 register accessor: an alias for `Reg<USBOEPSIZXY_2_SPEC>`"]
pub type USBOEPSIZXY_2 = crate::Reg<usboepsizxy_2::USBOEPSIZXY_2_SPEC>;
#[doc = "Output Endpoint_2: X/Y-buffer size"]
pub mod usboepsizxy_2;
#[doc = "USBOEPCNF_3 register accessor: an alias for `Reg<USBOEPCNF_3_SPEC>`"]
pub type USBOEPCNF_3 = crate::Reg<usboepcnf_3::USBOEPCNF_3_SPEC>;
#[doc = "Output Endpoint_3: Configuration"]
pub mod usboepcnf_3;
#[doc = "USBOEPBBAX_3 register accessor: an alias for `Reg<USBOEPBBAX_3_SPEC>`"]
pub type USBOEPBBAX_3 = crate::Reg<usboepbbax_3::USBOEPBBAX_3_SPEC>;
#[doc = "Output Endpoint_3: X-buffer base addr."]
pub mod usboepbbax_3;
#[doc = "USBOEPBCTX_3 register accessor: an alias for `Reg<USBOEPBCTX_3_SPEC>`"]
pub type USBOEPBCTX_3 = crate::Reg<usboepbctx_3::USBOEPBCTX_3_SPEC>;
#[doc = "Output Endpoint_3: X-byte count"]
pub mod usboepbctx_3;
#[doc = "USBOEPBBAY_3 register accessor: an alias for `Reg<USBOEPBBAY_3_SPEC>`"]
pub type USBOEPBBAY_3 = crate::Reg<usboepbbay_3::USBOEPBBAY_3_SPEC>;
#[doc = "Output Endpoint_3: Y-buffer base addr."]
pub mod usboepbbay_3;
#[doc = "USBOEPBCTY_3 register accessor: an alias for `Reg<USBOEPBCTY_3_SPEC>`"]
pub type USBOEPBCTY_3 = crate::Reg<usboepbcty_3::USBOEPBCTY_3_SPEC>;
#[doc = "Output Endpoint_3: Y-byte count"]
pub mod usboepbcty_3;
#[doc = "USBOEPSIZXY_3 register accessor: an alias for `Reg<USBOEPSIZXY_3_SPEC>`"]
pub type USBOEPSIZXY_3 = crate::Reg<usboepsizxy_3::USBOEPSIZXY_3_SPEC>;
#[doc = "Output Endpoint_3: X/Y-buffer size"]
pub mod usboepsizxy_3;
#[doc = "USBOEPCNF_4 register accessor: an alias for `Reg<USBOEPCNF_4_SPEC>`"]
pub type USBOEPCNF_4 = crate::Reg<usboepcnf_4::USBOEPCNF_4_SPEC>;
#[doc = "Output Endpoint_4: Configuration"]
pub mod usboepcnf_4;
#[doc = "USBOEPBBAX_4 register accessor: an alias for `Reg<USBOEPBBAX_4_SPEC>`"]
pub type USBOEPBBAX_4 = crate::Reg<usboepbbax_4::USBOEPBBAX_4_SPEC>;
#[doc = "Output Endpoint_4: X-buffer base addr."]
pub mod usboepbbax_4;
#[doc = "USBOEPBCTX_4 register accessor: an alias for `Reg<USBOEPBCTX_4_SPEC>`"]
pub type USBOEPBCTX_4 = crate::Reg<usboepbctx_4::USBOEPBCTX_4_SPEC>;
#[doc = "Output Endpoint_4: X-byte count"]
pub mod usboepbctx_4;
#[doc = "USBOEPBBAY_4 register accessor: an alias for `Reg<USBOEPBBAY_4_SPEC>`"]
pub type USBOEPBBAY_4 = crate::Reg<usboepbbay_4::USBOEPBBAY_4_SPEC>;
#[doc = "Output Endpoint_4: Y-buffer base addr."]
pub mod usboepbbay_4;
#[doc = "USBOEPBCTY_4 register accessor: an alias for `Reg<USBOEPBCTY_4_SPEC>`"]
pub type USBOEPBCTY_4 = crate::Reg<usboepbcty_4::USBOEPBCTY_4_SPEC>;
#[doc = "Output Endpoint_4: Y-byte count"]
pub mod usboepbcty_4;
#[doc = "USBOEPSIZXY_4 register accessor: an alias for `Reg<USBOEPSIZXY_4_SPEC>`"]
pub type USBOEPSIZXY_4 = crate::Reg<usboepsizxy_4::USBOEPSIZXY_4_SPEC>;
#[doc = "Output Endpoint_4: X/Y-buffer size"]
pub mod usboepsizxy_4;
#[doc = "USBOEPCNF_5 register accessor: an alias for `Reg<USBOEPCNF_5_SPEC>`"]
pub type USBOEPCNF_5 = crate::Reg<usboepcnf_5::USBOEPCNF_5_SPEC>;
#[doc = "Output Endpoint_5: Configuration"]
pub mod usboepcnf_5;
#[doc = "USBOEPBBAX_5 register accessor: an alias for `Reg<USBOEPBBAX_5_SPEC>`"]
pub type USBOEPBBAX_5 = crate::Reg<usboepbbax_5::USBOEPBBAX_5_SPEC>;
#[doc = "Output Endpoint_5: X-buffer base addr."]
pub mod usboepbbax_5;
#[doc = "USBOEPBCTX_5 register accessor: an alias for `Reg<USBOEPBCTX_5_SPEC>`"]
pub type USBOEPBCTX_5 = crate::Reg<usboepbctx_5::USBOEPBCTX_5_SPEC>;
#[doc = "Output Endpoint_5: X-byte count"]
pub mod usboepbctx_5;
#[doc = "USBOEPBBAY_5 register accessor: an alias for `Reg<USBOEPBBAY_5_SPEC>`"]
pub type USBOEPBBAY_5 = crate::Reg<usboepbbay_5::USBOEPBBAY_5_SPEC>;
#[doc = "Output Endpoint_5: Y-buffer base addr."]
pub mod usboepbbay_5;
#[doc = "USBOEPBCTY_5 register accessor: an alias for `Reg<USBOEPBCTY_5_SPEC>`"]
pub type USBOEPBCTY_5 = crate::Reg<usboepbcty_5::USBOEPBCTY_5_SPEC>;
#[doc = "Output Endpoint_5: Y-byte count"]
pub mod usboepbcty_5;
#[doc = "USBOEPSIZXY_5 register accessor: an alias for `Reg<USBOEPSIZXY_5_SPEC>`"]
pub type USBOEPSIZXY_5 = crate::Reg<usboepsizxy_5::USBOEPSIZXY_5_SPEC>;
#[doc = "Output Endpoint_5: X/Y-buffer size"]
pub mod usboepsizxy_5;
#[doc = "USBOEPCNF_6 register accessor: an alias for `Reg<USBOEPCNF_6_SPEC>`"]
pub type USBOEPCNF_6 = crate::Reg<usboepcnf_6::USBOEPCNF_6_SPEC>;
#[doc = "Output Endpoint_6: Configuration"]
pub mod usboepcnf_6;
#[doc = "USBOEPBBAX_6 register accessor: an alias for `Reg<USBOEPBBAX_6_SPEC>`"]
pub type USBOEPBBAX_6 = crate::Reg<usboepbbax_6::USBOEPBBAX_6_SPEC>;
#[doc = "Output Endpoint_6: X-buffer base addr."]
pub mod usboepbbax_6;
#[doc = "USBOEPBCTX_6 register accessor: an alias for `Reg<USBOEPBCTX_6_SPEC>`"]
pub type USBOEPBCTX_6 = crate::Reg<usboepbctx_6::USBOEPBCTX_6_SPEC>;
#[doc = "Output Endpoint_6: X-byte count"]
pub mod usboepbctx_6;
#[doc = "USBOEPBBAY_6 register accessor: an alias for `Reg<USBOEPBBAY_6_SPEC>`"]
pub type USBOEPBBAY_6 = crate::Reg<usboepbbay_6::USBOEPBBAY_6_SPEC>;
#[doc = "Output Endpoint_6: Y-buffer base addr."]
pub mod usboepbbay_6;
#[doc = "USBOEPBCTY_6 register accessor: an alias for `Reg<USBOEPBCTY_6_SPEC>`"]
pub type USBOEPBCTY_6 = crate::Reg<usboepbcty_6::USBOEPBCTY_6_SPEC>;
#[doc = "Output Endpoint_6: Y-byte count"]
pub mod usboepbcty_6;
#[doc = "USBOEPSIZXY_6 register accessor: an alias for `Reg<USBOEPSIZXY_6_SPEC>`"]
pub type USBOEPSIZXY_6 = crate::Reg<usboepsizxy_6::USBOEPSIZXY_6_SPEC>;
#[doc = "Output Endpoint_6: X/Y-buffer size"]
pub mod usboepsizxy_6;
#[doc = "USBOEPCNF_7 register accessor: an alias for `Reg<USBOEPCNF_7_SPEC>`"]
pub type USBOEPCNF_7 = crate::Reg<usboepcnf_7::USBOEPCNF_7_SPEC>;
#[doc = "Output Endpoint_7: Configuration"]
pub mod usboepcnf_7;
#[doc = "USBOEPBBAX_7 register accessor: an alias for `Reg<USBOEPBBAX_7_SPEC>`"]
pub type USBOEPBBAX_7 = crate::Reg<usboepbbax_7::USBOEPBBAX_7_SPEC>;
#[doc = "Output Endpoint_7: X-buffer base addr."]
pub mod usboepbbax_7;
#[doc = "USBOEPBCTX_7 register accessor: an alias for `Reg<USBOEPBCTX_7_SPEC>`"]
pub type USBOEPBCTX_7 = crate::Reg<usboepbctx_7::USBOEPBCTX_7_SPEC>;
#[doc = "Output Endpoint_7: X-byte count"]
pub mod usboepbctx_7;
#[doc = "USBOEPBBAY_7 register accessor: an alias for `Reg<USBOEPBBAY_7_SPEC>`"]
pub type USBOEPBBAY_7 = crate::Reg<usboepbbay_7::USBOEPBBAY_7_SPEC>;
#[doc = "Output Endpoint_7: Y-buffer base addr."]
pub mod usboepbbay_7;
#[doc = "USBOEPBCTY_7 register accessor: an alias for `Reg<USBOEPBCTY_7_SPEC>`"]
pub type USBOEPBCTY_7 = crate::Reg<usboepbcty_7::USBOEPBCTY_7_SPEC>;
#[doc = "Output Endpoint_7: Y-byte count"]
pub mod usboepbcty_7;
#[doc = "USBOEPSIZXY_7 register accessor: an alias for `Reg<USBOEPSIZXY_7_SPEC>`"]
pub type USBOEPSIZXY_7 = crate::Reg<usboepsizxy_7::USBOEPSIZXY_7_SPEC>;
#[doc = "Output Endpoint_7: X/Y-buffer size"]
pub mod usboepsizxy_7;
#[doc = "USBIEPCNF_1 register accessor: an alias for `Reg<USBIEPCNF_1_SPEC>`"]
pub type USBIEPCNF_1 = crate::Reg<usbiepcnf_1::USBIEPCNF_1_SPEC>;
#[doc = "Input Endpoint_1: Configuration"]
pub mod usbiepcnf_1;
#[doc = "USBIEPBBAX_1 register accessor: an alias for `Reg<USBIEPBBAX_1_SPEC>`"]
pub type USBIEPBBAX_1 = crate::Reg<usbiepbbax_1::USBIEPBBAX_1_SPEC>;
#[doc = "Input Endpoint_1: X-buffer base addr."]
pub mod usbiepbbax_1;
#[doc = "USBIEPBCTX_1 register accessor: an alias for `Reg<USBIEPBCTX_1_SPEC>`"]
pub type USBIEPBCTX_1 = crate::Reg<usbiepbctx_1::USBIEPBCTX_1_SPEC>;
#[doc = "Input Endpoint_1: X-byte count"]
pub mod usbiepbctx_1;
#[doc = "USBIEPBBAY_1 register accessor: an alias for `Reg<USBIEPBBAY_1_SPEC>`"]
pub type USBIEPBBAY_1 = crate::Reg<usbiepbbay_1::USBIEPBBAY_1_SPEC>;
#[doc = "Input Endpoint_1: Y-buffer base addr."]
pub mod usbiepbbay_1;
#[doc = "USBIEPBCTY_1 register accessor: an alias for `Reg<USBIEPBCTY_1_SPEC>`"]
pub type USBIEPBCTY_1 = crate::Reg<usbiepbcty_1::USBIEPBCTY_1_SPEC>;
#[doc = "Input Endpoint_1: Y-byte count"]
pub mod usbiepbcty_1;
#[doc = "USBIEPSIZXY_1 register accessor: an alias for `Reg<USBIEPSIZXY_1_SPEC>`"]
pub type USBIEPSIZXY_1 = crate::Reg<usbiepsizxy_1::USBIEPSIZXY_1_SPEC>;
#[doc = "Input Endpoint_1: X/Y-buffer size"]
pub mod usbiepsizxy_1;
#[doc = "USBIEPCNF_2 register accessor: an alias for `Reg<USBIEPCNF_2_SPEC>`"]
pub type USBIEPCNF_2 = crate::Reg<usbiepcnf_2::USBIEPCNF_2_SPEC>;
#[doc = "Input Endpoint_2: Configuration"]
pub mod usbiepcnf_2;
#[doc = "USBIEPBBAX_2 register accessor: an alias for `Reg<USBIEPBBAX_2_SPEC>`"]
pub type USBIEPBBAX_2 = crate::Reg<usbiepbbax_2::USBIEPBBAX_2_SPEC>;
#[doc = "Input Endpoint_2: X-buffer base addr."]
pub mod usbiepbbax_2;
#[doc = "USBIEPBCTX_2 register accessor: an alias for `Reg<USBIEPBCTX_2_SPEC>`"]
pub type USBIEPBCTX_2 = crate::Reg<usbiepbctx_2::USBIEPBCTX_2_SPEC>;
#[doc = "Input Endpoint_2: X-byte count"]
pub mod usbiepbctx_2;
#[doc = "USBIEPBBAY_2 register accessor: an alias for `Reg<USBIEPBBAY_2_SPEC>`"]
pub type USBIEPBBAY_2 = crate::Reg<usbiepbbay_2::USBIEPBBAY_2_SPEC>;
#[doc = "Input Endpoint_2: Y-buffer base addr."]
pub mod usbiepbbay_2;
#[doc = "USBIEPBCTY_2 register accessor: an alias for `Reg<USBIEPBCTY_2_SPEC>`"]
pub type USBIEPBCTY_2 = crate::Reg<usbiepbcty_2::USBIEPBCTY_2_SPEC>;
#[doc = "Input Endpoint_2: Y-byte count"]
pub mod usbiepbcty_2;
#[doc = "USBIEPSIZXY_2 register accessor: an alias for `Reg<USBIEPSIZXY_2_SPEC>`"]
pub type USBIEPSIZXY_2 = crate::Reg<usbiepsizxy_2::USBIEPSIZXY_2_SPEC>;
#[doc = "Input Endpoint_2: X/Y-buffer size"]
pub mod usbiepsizxy_2;
#[doc = "USBIEPCNF_3 register accessor: an alias for `Reg<USBIEPCNF_3_SPEC>`"]
pub type USBIEPCNF_3 = crate::Reg<usbiepcnf_3::USBIEPCNF_3_SPEC>;
#[doc = "Input Endpoint_3: Configuration"]
pub mod usbiepcnf_3;
#[doc = "USBIEPBBAX_3 register accessor: an alias for `Reg<USBIEPBBAX_3_SPEC>`"]
pub type USBIEPBBAX_3 = crate::Reg<usbiepbbax_3::USBIEPBBAX_3_SPEC>;
#[doc = "Input Endpoint_3: X-buffer base addr."]
pub mod usbiepbbax_3;
#[doc = "USBIEPBCTX_3 register accessor: an alias for `Reg<USBIEPBCTX_3_SPEC>`"]
pub type USBIEPBCTX_3 = crate::Reg<usbiepbctx_3::USBIEPBCTX_3_SPEC>;
#[doc = "Input Endpoint_3: X-byte count"]
pub mod usbiepbctx_3;
#[doc = "USBIEPBBAY_3 register accessor: an alias for `Reg<USBIEPBBAY_3_SPEC>`"]
pub type USBIEPBBAY_3 = crate::Reg<usbiepbbay_3::USBIEPBBAY_3_SPEC>;
#[doc = "Input Endpoint_3: Y-buffer base addr."]
pub mod usbiepbbay_3;
#[doc = "USBIEPBCTY_3 register accessor: an alias for `Reg<USBIEPBCTY_3_SPEC>`"]
pub type USBIEPBCTY_3 = crate::Reg<usbiepbcty_3::USBIEPBCTY_3_SPEC>;
#[doc = "Input Endpoint_3: Y-byte count"]
pub mod usbiepbcty_3;
#[doc = "USBIEPSIZXY_3 register accessor: an alias for `Reg<USBIEPSIZXY_3_SPEC>`"]
pub type USBIEPSIZXY_3 = crate::Reg<usbiepsizxy_3::USBIEPSIZXY_3_SPEC>;
#[doc = "Input Endpoint_3: X/Y-buffer size"]
pub mod usbiepsizxy_3;
#[doc = "USBIEPCNF_4 register accessor: an alias for `Reg<USBIEPCNF_4_SPEC>`"]
pub type USBIEPCNF_4 = crate::Reg<usbiepcnf_4::USBIEPCNF_4_SPEC>;
#[doc = "Input Endpoint_4: Configuration"]
pub mod usbiepcnf_4;
#[doc = "USBIEPBBAX_4 register accessor: an alias for `Reg<USBIEPBBAX_4_SPEC>`"]
pub type USBIEPBBAX_4 = crate::Reg<usbiepbbax_4::USBIEPBBAX_4_SPEC>;
#[doc = "Input Endpoint_4: X-buffer base addr."]
pub mod usbiepbbax_4;
#[doc = "USBIEPBCTX_4 register accessor: an alias for `Reg<USBIEPBCTX_4_SPEC>`"]
pub type USBIEPBCTX_4 = crate::Reg<usbiepbctx_4::USBIEPBCTX_4_SPEC>;
#[doc = "Input Endpoint_4: X-byte count"]
pub mod usbiepbctx_4;
#[doc = "USBIEPBBAY_4 register accessor: an alias for `Reg<USBIEPBBAY_4_SPEC>`"]
pub type USBIEPBBAY_4 = crate::Reg<usbiepbbay_4::USBIEPBBAY_4_SPEC>;
#[doc = "Input Endpoint_4: Y-buffer base addr."]
pub mod usbiepbbay_4;
#[doc = "USBIEPBCTY_4 register accessor: an alias for `Reg<USBIEPBCTY_4_SPEC>`"]
pub type USBIEPBCTY_4 = crate::Reg<usbiepbcty_4::USBIEPBCTY_4_SPEC>;
#[doc = "Input Endpoint_4: Y-byte count"]
pub mod usbiepbcty_4;
#[doc = "USBIEPSIZXY_4 register accessor: an alias for `Reg<USBIEPSIZXY_4_SPEC>`"]
pub type USBIEPSIZXY_4 = crate::Reg<usbiepsizxy_4::USBIEPSIZXY_4_SPEC>;
#[doc = "Input Endpoint_4: X/Y-buffer size"]
pub mod usbiepsizxy_4;
#[doc = "USBIEPCNF_5 register accessor: an alias for `Reg<USBIEPCNF_5_SPEC>`"]
pub type USBIEPCNF_5 = crate::Reg<usbiepcnf_5::USBIEPCNF_5_SPEC>;
#[doc = "Input Endpoint_5: Configuration"]
pub mod usbiepcnf_5;
#[doc = "USBIEPBBAX_5 register accessor: an alias for `Reg<USBIEPBBAX_5_SPEC>`"]
pub type USBIEPBBAX_5 = crate::Reg<usbiepbbax_5::USBIEPBBAX_5_SPEC>;
#[doc = "Input Endpoint_5: X-buffer base addr."]
pub mod usbiepbbax_5;
#[doc = "USBIEPBCTX_5 register accessor: an alias for `Reg<USBIEPBCTX_5_SPEC>`"]
pub type USBIEPBCTX_5 = crate::Reg<usbiepbctx_5::USBIEPBCTX_5_SPEC>;
#[doc = "Input Endpoint_5: X-byte count"]
pub mod usbiepbctx_5;
#[doc = "USBIEPBBAY_5 register accessor: an alias for `Reg<USBIEPBBAY_5_SPEC>`"]
pub type USBIEPBBAY_5 = crate::Reg<usbiepbbay_5::USBIEPBBAY_5_SPEC>;
#[doc = "Input Endpoint_5: Y-buffer base addr."]
pub mod usbiepbbay_5;
#[doc = "USBIEPBCTY_5 register accessor: an alias for `Reg<USBIEPBCTY_5_SPEC>`"]
pub type USBIEPBCTY_5 = crate::Reg<usbiepbcty_5::USBIEPBCTY_5_SPEC>;
#[doc = "Input Endpoint_5: Y-byte count"]
pub mod usbiepbcty_5;
#[doc = "USBIEPSIZXY_5 register accessor: an alias for `Reg<USBIEPSIZXY_5_SPEC>`"]
pub type USBIEPSIZXY_5 = crate::Reg<usbiepsizxy_5::USBIEPSIZXY_5_SPEC>;
#[doc = "Input Endpoint_5: X/Y-buffer size"]
pub mod usbiepsizxy_5;
#[doc = "USBIEPCNF_6 register accessor: an alias for `Reg<USBIEPCNF_6_SPEC>`"]
pub type USBIEPCNF_6 = crate::Reg<usbiepcnf_6::USBIEPCNF_6_SPEC>;
#[doc = "Input Endpoint_6: Configuration"]
pub mod usbiepcnf_6;
#[doc = "USBIEPBBAX_6 register accessor: an alias for `Reg<USBIEPBBAX_6_SPEC>`"]
pub type USBIEPBBAX_6 = crate::Reg<usbiepbbax_6::USBIEPBBAX_6_SPEC>;
#[doc = "Input Endpoint_6: X-buffer base addr."]
pub mod usbiepbbax_6;
#[doc = "USBIEPBCTX_6 register accessor: an alias for `Reg<USBIEPBCTX_6_SPEC>`"]
pub type USBIEPBCTX_6 = crate::Reg<usbiepbctx_6::USBIEPBCTX_6_SPEC>;
#[doc = "Input Endpoint_6: X-byte count"]
pub mod usbiepbctx_6;
#[doc = "USBIEPBBAY_6 register accessor: an alias for `Reg<USBIEPBBAY_6_SPEC>`"]
pub type USBIEPBBAY_6 = crate::Reg<usbiepbbay_6::USBIEPBBAY_6_SPEC>;
#[doc = "Input Endpoint_6: Y-buffer base addr."]
pub mod usbiepbbay_6;
#[doc = "USBIEPBCTY_6 register accessor: an alias for `Reg<USBIEPBCTY_6_SPEC>`"]
pub type USBIEPBCTY_6 = crate::Reg<usbiepbcty_6::USBIEPBCTY_6_SPEC>;
#[doc = "Input Endpoint_6: Y-byte count"]
pub mod usbiepbcty_6;
#[doc = "USBIEPSIZXY_6 register accessor: an alias for `Reg<USBIEPSIZXY_6_SPEC>`"]
pub type USBIEPSIZXY_6 = crate::Reg<usbiepsizxy_6::USBIEPSIZXY_6_SPEC>;
#[doc = "Input Endpoint_6: X/Y-buffer size"]
pub mod usbiepsizxy_6;
#[doc = "USBIEPCNF_7 register accessor: an alias for `Reg<USBIEPCNF_7_SPEC>`"]
pub type USBIEPCNF_7 = crate::Reg<usbiepcnf_7::USBIEPCNF_7_SPEC>;
#[doc = "Input Endpoint_7: Configuration"]
pub mod usbiepcnf_7;
#[doc = "USBIEPBBAX_7 register accessor: an alias for `Reg<USBIEPBBAX_7_SPEC>`"]
pub type USBIEPBBAX_7 = crate::Reg<usbiepbbax_7::USBIEPBBAX_7_SPEC>;
#[doc = "Input Endpoint_7: X-buffer base addr."]
pub mod usbiepbbax_7;
#[doc = "USBIEPBCTX_7 register accessor: an alias for `Reg<USBIEPBCTX_7_SPEC>`"]
pub type USBIEPBCTX_7 = crate::Reg<usbiepbctx_7::USBIEPBCTX_7_SPEC>;
#[doc = "Input Endpoint_7: X-byte count"]
pub mod usbiepbctx_7;
#[doc = "USBIEPBBAY_7 register accessor: an alias for `Reg<USBIEPBBAY_7_SPEC>`"]
pub type USBIEPBBAY_7 = crate::Reg<usbiepbbay_7::USBIEPBBAY_7_SPEC>;
#[doc = "Input Endpoint_7: Y-buffer base addr."]
pub mod usbiepbbay_7;
#[doc = "USBIEPBCTY_7 register accessor: an alias for `Reg<USBIEPBCTY_7_SPEC>`"]
pub type USBIEPBCTY_7 = crate::Reg<usbiepbcty_7::USBIEPBCTY_7_SPEC>;
#[doc = "Input Endpoint_7: Y-byte count"]
pub mod usbiepbcty_7;
#[doc = "USBIEPSIZXY_7 register accessor: an alias for `Reg<USBIEPSIZXY_7_SPEC>`"]
pub type USBIEPSIZXY_7 = crate::Reg<usbiepsizxy_7::USBIEPSIZXY_7_SPEC>;
#[doc = "Input Endpoint_7: X/Y-buffer size"]
pub mod usbiepsizxy_7;
