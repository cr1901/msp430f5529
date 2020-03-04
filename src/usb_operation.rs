#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start of buffer space"]
    pub usbstabuff: USBSTABUFF,
    _reserved1: [u8; 1902usize],
    #[doc = "0x76f - Top of buffer space"]
    pub usbtopbuff: USBTOPBUFF,
    #[doc = "0x770 - Output endpoint_0 buffer"]
    pub usboep0buf: USBOEP0BUF,
    _reserved3: [u8; 7usize],
    #[doc = "0x778 - Input endpoint_0 buffer"]
    pub usbiep0buf: USBIEP0BUF,
    _reserved4: [u8; 7usize],
    #[doc = "0x780 - Setup Packet Block"]
    pub usbsublk: USBSUBLK,
    _reserved5: [u8; 7usize],
    #[doc = "0x788 - Output Endpoint_1: Configuration"]
    pub usboepcnf_1: USBOEPCNF_1,
    #[doc = "0x789 - Output Endpoint_1: X-buffer base addr."]
    pub usboepbbax_1: USBOEPBBAX_1,
    #[doc = "0x78a - Output Endpoint_1: X-byte count"]
    pub usboepbctx_1: USBOEPBCTX_1,
    _reserved8: [u8; 2usize],
    #[doc = "0x78d - Output Endpoint_1: Y-buffer base addr."]
    pub usboepbbay_1: USBOEPBBAY_1,
    #[doc = "0x78e - Output Endpoint_1: Y-byte count"]
    pub usboepbcty_1: USBOEPBCTY_1,
    #[doc = "0x78f - Output Endpoint_1: X/Y-buffer size"]
    pub usboepsizxy_1: USBOEPSIZXY_1,
    #[doc = "0x790 - Output Endpoint_2: Configuration"]
    pub usboepcnf_2: USBOEPCNF_2,
    #[doc = "0x791 - Output Endpoint_2: X-buffer base addr."]
    pub usboepbbax_2: USBOEPBBAX_2,
    #[doc = "0x792 - Output Endpoint_2: X-byte count"]
    pub usboepbctx_2: USBOEPBCTX_2,
    _reserved14: [u8; 2usize],
    #[doc = "0x795 - Output Endpoint_2: Y-buffer base addr."]
    pub usboepbbay_2: USBOEPBBAY_2,
    #[doc = "0x796 - Output Endpoint_2: Y-byte count"]
    pub usboepbcty_2: USBOEPBCTY_2,
    #[doc = "0x797 - Output Endpoint_2: X/Y-buffer size"]
    pub usboepsizxy_2: USBOEPSIZXY_2,
    #[doc = "0x798 - Output Endpoint_3: Configuration"]
    pub usboepcnf_3: USBOEPCNF_3,
    #[doc = "0x799 - Output Endpoint_3: X-buffer base addr."]
    pub usboepbbax_3: USBOEPBBAX_3,
    #[doc = "0x79a - Output Endpoint_3: X-byte count"]
    pub usboepbctx_3: USBOEPBCTX_3,
    _reserved20: [u8; 2usize],
    #[doc = "0x79d - Output Endpoint_3: Y-buffer base addr."]
    pub usboepbbay_3: USBOEPBBAY_3,
    #[doc = "0x79e - Output Endpoint_3: Y-byte count"]
    pub usboepbcty_3: USBOEPBCTY_3,
    #[doc = "0x79f - Output Endpoint_3: X/Y-buffer size"]
    pub usboepsizxy_3: USBOEPSIZXY_3,
    #[doc = "0x7a0 - Output Endpoint_4: Configuration"]
    pub usboepcnf_4: USBOEPCNF_4,
    #[doc = "0x7a1 - Output Endpoint_4: X-buffer base addr."]
    pub usboepbbax_4: USBOEPBBAX_4,
    #[doc = "0x7a2 - Output Endpoint_4: X-byte count"]
    pub usboepbctx_4: USBOEPBCTX_4,
    _reserved26: [u8; 2usize],
    #[doc = "0x7a5 - Output Endpoint_4: Y-buffer base addr."]
    pub usboepbbay_4: USBOEPBBAY_4,
    #[doc = "0x7a6 - Output Endpoint_4: Y-byte count"]
    pub usboepbcty_4: USBOEPBCTY_4,
    #[doc = "0x7a7 - Output Endpoint_4: X/Y-buffer size"]
    pub usboepsizxy_4: USBOEPSIZXY_4,
    #[doc = "0x7a8 - Output Endpoint_5: Configuration"]
    pub usboepcnf_5: USBOEPCNF_5,
    #[doc = "0x7a9 - Output Endpoint_5: X-buffer base addr."]
    pub usboepbbax_5: USBOEPBBAX_5,
    #[doc = "0x7aa - Output Endpoint_5: X-byte count"]
    pub usboepbctx_5: USBOEPBCTX_5,
    _reserved32: [u8; 2usize],
    #[doc = "0x7ad - Output Endpoint_5: Y-buffer base addr."]
    pub usboepbbay_5: USBOEPBBAY_5,
    #[doc = "0x7ae - Output Endpoint_5: Y-byte count"]
    pub usboepbcty_5: USBOEPBCTY_5,
    #[doc = "0x7af - Output Endpoint_5: X/Y-buffer size"]
    pub usboepsizxy_5: USBOEPSIZXY_5,
    #[doc = "0x7b0 - Output Endpoint_6: Configuration"]
    pub usboepcnf_6: USBOEPCNF_6,
    #[doc = "0x7b1 - Output Endpoint_6: X-buffer base addr."]
    pub usboepbbax_6: USBOEPBBAX_6,
    #[doc = "0x7b2 - Output Endpoint_6: X-byte count"]
    pub usboepbctx_6: USBOEPBCTX_6,
    _reserved38: [u8; 2usize],
    #[doc = "0x7b5 - Output Endpoint_6: Y-buffer base addr."]
    pub usboepbbay_6: USBOEPBBAY_6,
    #[doc = "0x7b6 - Output Endpoint_6: Y-byte count"]
    pub usboepbcty_6: USBOEPBCTY_6,
    #[doc = "0x7b7 - Output Endpoint_6: X/Y-buffer size"]
    pub usboepsizxy_6: USBOEPSIZXY_6,
    #[doc = "0x7b8 - Output Endpoint_7: Configuration"]
    pub usboepcnf_7: USBOEPCNF_7,
    #[doc = "0x7b9 - Output Endpoint_7: X-buffer base addr."]
    pub usboepbbax_7: USBOEPBBAX_7,
    #[doc = "0x7ba - Output Endpoint_7: X-byte count"]
    pub usboepbctx_7: USBOEPBCTX_7,
    _reserved44: [u8; 2usize],
    #[doc = "0x7bd - Output Endpoint_7: Y-buffer base addr."]
    pub usboepbbay_7: USBOEPBBAY_7,
    #[doc = "0x7be - Output Endpoint_7: Y-byte count"]
    pub usboepbcty_7: USBOEPBCTY_7,
    #[doc = "0x7bf - Output Endpoint_7: X/Y-buffer size"]
    pub usboepsizxy_7: USBOEPSIZXY_7,
    _reserved47: [u8; 8usize],
    #[doc = "0x7c8 - Input Endpoint_1: Configuration"]
    pub usbiepcnf_1: USBIEPCNF_1,
    #[doc = "0x7c9 - Input Endpoint_1: X-buffer base addr."]
    pub usbiepbbax_1: USBIEPBBAX_1,
    #[doc = "0x7ca - Input Endpoint_1: X-byte count"]
    pub usbiepbctx_1: USBIEPBCTX_1,
    _reserved50: [u8; 2usize],
    #[doc = "0x7cd - Input Endpoint_1: Y-buffer base addr."]
    pub usbiepbbay_1: USBIEPBBAY_1,
    #[doc = "0x7ce - Input Endpoint_1: Y-byte count"]
    pub usbiepbcty_1: USBIEPBCTY_1,
    #[doc = "0x7cf - Input Endpoint_1: X/Y-buffer size"]
    pub usbiepsizxy_1: USBIEPSIZXY_1,
    #[doc = "0x7d0 - Input Endpoint_2: Configuration"]
    pub usbiepcnf_2: USBIEPCNF_2,
    #[doc = "0x7d1 - Input Endpoint_2: X-buffer base addr."]
    pub usbiepbbax_2: USBIEPBBAX_2,
    #[doc = "0x7d2 - Input Endpoint_2: X-byte count"]
    pub usbiepbctx_2: USBIEPBCTX_2,
    _reserved56: [u8; 2usize],
    #[doc = "0x7d5 - Input Endpoint_2: Y-buffer base addr."]
    pub usbiepbbay_2: USBIEPBBAY_2,
    #[doc = "0x7d6 - Input Endpoint_2: Y-byte count"]
    pub usbiepbcty_2: USBIEPBCTY_2,
    #[doc = "0x7d7 - Input Endpoint_2: X/Y-buffer size"]
    pub usbiepsizxy_2: USBIEPSIZXY_2,
    #[doc = "0x7d8 - Input Endpoint_3: Configuration"]
    pub usbiepcnf_3: USBIEPCNF_3,
    #[doc = "0x7d9 - Input Endpoint_3: X-buffer base addr."]
    pub usbiepbbax_3: USBIEPBBAX_3,
    #[doc = "0x7da - Input Endpoint_3: X-byte count"]
    pub usbiepbctx_3: USBIEPBCTX_3,
    _reserved62: [u8; 2usize],
    #[doc = "0x7dd - Input Endpoint_3: Y-buffer base addr."]
    pub usbiepbbay_3: USBIEPBBAY_3,
    #[doc = "0x7de - Input Endpoint_3: Y-byte count"]
    pub usbiepbcty_3: USBIEPBCTY_3,
    #[doc = "0x7df - Input Endpoint_3: X/Y-buffer size"]
    pub usbiepsizxy_3: USBIEPSIZXY_3,
    #[doc = "0x7e0 - Input Endpoint_4: Configuration"]
    pub usbiepcnf_4: USBIEPCNF_4,
    #[doc = "0x7e1 - Input Endpoint_4: X-buffer base addr."]
    pub usbiepbbax_4: USBIEPBBAX_4,
    #[doc = "0x7e2 - Input Endpoint_4: X-byte count"]
    pub usbiepbctx_4: USBIEPBCTX_4,
    _reserved68: [u8; 2usize],
    #[doc = "0x7e5 - Input Endpoint_4: Y-buffer base addr."]
    pub usbiepbbay_4: USBIEPBBAY_4,
    #[doc = "0x7e6 - Input Endpoint_4: Y-byte count"]
    pub usbiepbcty_4: USBIEPBCTY_4,
    #[doc = "0x7e7 - Input Endpoint_4: X/Y-buffer size"]
    pub usbiepsizxy_4: USBIEPSIZXY_4,
    #[doc = "0x7e8 - Input Endpoint_5: Configuration"]
    pub usbiepcnf_5: USBIEPCNF_5,
    #[doc = "0x7e9 - Input Endpoint_5: X-buffer base addr."]
    pub usbiepbbax_5: USBIEPBBAX_5,
    #[doc = "0x7ea - Input Endpoint_5: X-byte count"]
    pub usbiepbctx_5: USBIEPBCTX_5,
    _reserved74: [u8; 2usize],
    #[doc = "0x7ed - Input Endpoint_5: Y-buffer base addr."]
    pub usbiepbbay_5: USBIEPBBAY_5,
    #[doc = "0x7ee - Input Endpoint_5: Y-byte count"]
    pub usbiepbcty_5: USBIEPBCTY_5,
    #[doc = "0x7ef - Input Endpoint_5: X/Y-buffer size"]
    pub usbiepsizxy_5: USBIEPSIZXY_5,
    #[doc = "0x7f0 - Input Endpoint_6: Configuration"]
    pub usbiepcnf_6: USBIEPCNF_6,
    #[doc = "0x7f1 - Input Endpoint_6: X-buffer base addr."]
    pub usbiepbbax_6: USBIEPBBAX_6,
    #[doc = "0x7f2 - Input Endpoint_6: X-byte count"]
    pub usbiepbctx_6: USBIEPBCTX_6,
    _reserved80: [u8; 2usize],
    #[doc = "0x7f5 - Input Endpoint_6: Y-buffer base addr."]
    pub usbiepbbay_6: USBIEPBBAY_6,
    #[doc = "0x7f6 - Input Endpoint_6: Y-byte count"]
    pub usbiepbcty_6: USBIEPBCTY_6,
    #[doc = "0x7f7 - Input Endpoint_6: X/Y-buffer size"]
    pub usbiepsizxy_6: USBIEPSIZXY_6,
    #[doc = "0x7f8 - Input Endpoint_7: Configuration"]
    pub usbiepcnf_7: USBIEPCNF_7,
    #[doc = "0x7f9 - Input Endpoint_7: X-buffer base addr."]
    pub usbiepbbax_7: USBIEPBBAX_7,
    #[doc = "0x7fa - Input Endpoint_7: X-byte count"]
    pub usbiepbctx_7: USBIEPBCTX_7,
    _reserved86: [u8; 2usize],
    #[doc = "0x7fd - Input Endpoint_7: Y-buffer base addr."]
    pub usbiepbbay_7: USBIEPBBAY_7,
    #[doc = "0x7fe - Input Endpoint_7: Y-byte count"]
    pub usbiepbcty_7: USBIEPBCTY_7,
    #[doc = "0x7ff - Input Endpoint_7: X/Y-buffer size"]
    pub usbiepsizxy_7: USBIEPSIZXY_7,
}
#[doc = "Start of buffer space\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbstabuff](usbstabuff) module"]
pub type USBSTABUFF = crate::Reg<u8, _USBSTABUFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBSTABUFF;
#[doc = "`read()` method returns [usbstabuff::R](usbstabuff::R) reader structure"]
impl crate::Readable for USBSTABUFF {}
#[doc = "`write(|w| ..)` method takes [usbstabuff::W](usbstabuff::W) writer structure"]
impl crate::Writable for USBSTABUFF {}
#[doc = "Start of buffer space"]
pub mod usbstabuff;
#[doc = "Top of buffer space\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbtopbuff](usbtopbuff) module"]
pub type USBTOPBUFF = crate::Reg<u8, _USBTOPBUFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBTOPBUFF;
#[doc = "`read()` method returns [usbtopbuff::R](usbtopbuff::R) reader structure"]
impl crate::Readable for USBTOPBUFF {}
#[doc = "`write(|w| ..)` method takes [usbtopbuff::W](usbtopbuff::W) writer structure"]
impl crate::Writable for USBTOPBUFF {}
#[doc = "Top of buffer space"]
pub mod usbtopbuff;
#[doc = "Output endpoint_0 buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboep0buf](usboep0buf) module"]
pub type USBOEP0BUF = crate::Reg<u8, _USBOEP0BUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEP0BUF;
#[doc = "`read()` method returns [usboep0buf::R](usboep0buf::R) reader structure"]
impl crate::Readable for USBOEP0BUF {}
#[doc = "`write(|w| ..)` method takes [usboep0buf::W](usboep0buf::W) writer structure"]
impl crate::Writable for USBOEP0BUF {}
#[doc = "Output endpoint_0 buffer"]
pub mod usboep0buf;
#[doc = "Input endpoint_0 buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiep0buf](usbiep0buf) module"]
pub type USBIEP0BUF = crate::Reg<u8, _USBIEP0BUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEP0BUF;
#[doc = "`read()` method returns [usbiep0buf::R](usbiep0buf::R) reader structure"]
impl crate::Readable for USBIEP0BUF {}
#[doc = "`write(|w| ..)` method takes [usbiep0buf::W](usbiep0buf::W) writer structure"]
impl crate::Writable for USBIEP0BUF {}
#[doc = "Input endpoint_0 buffer"]
pub mod usbiep0buf;
#[doc = "Setup Packet Block\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbsublk](usbsublk) module"]
pub type USBSUBLK = crate::Reg<u8, _USBSUBLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBSUBLK;
#[doc = "`read()` method returns [usbsublk::R](usbsublk::R) reader structure"]
impl crate::Readable for USBSUBLK {}
#[doc = "`write(|w| ..)` method takes [usbsublk::W](usbsublk::W) writer structure"]
impl crate::Writable for USBSUBLK {}
#[doc = "Setup Packet Block"]
pub mod usbsublk;
#[doc = "Output Endpoint_1: Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepcnf_1](usboepcnf_1) module"]
pub type USBOEPCNF_1 = crate::Reg<u8, _USBOEPCNF_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPCNF_1;
#[doc = "`read()` method returns [usboepcnf_1::R](usboepcnf_1::R) reader structure"]
impl crate::Readable for USBOEPCNF_1 {}
#[doc = "`write(|w| ..)` method takes [usboepcnf_1::W](usboepcnf_1::W) writer structure"]
impl crate::Writable for USBOEPCNF_1 {}
#[doc = "Output Endpoint_1: Configuration"]
pub mod usboepcnf_1;
#[doc = "Output Endpoint_1: X-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbbax_1](usboepbbax_1) module"]
pub type USBOEPBBAX_1 = crate::Reg<u8, _USBOEPBBAX_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBBAX_1;
#[doc = "`read()` method returns [usboepbbax_1::R](usboepbbax_1::R) reader structure"]
impl crate::Readable for USBOEPBBAX_1 {}
#[doc = "`write(|w| ..)` method takes [usboepbbax_1::W](usboepbbax_1::W) writer structure"]
impl crate::Writable for USBOEPBBAX_1 {}
#[doc = "Output Endpoint_1: X-buffer base addr."]
pub mod usboepbbax_1;
#[doc = "Output Endpoint_1: X-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbctx_1](usboepbctx_1) module"]
pub type USBOEPBCTX_1 = crate::Reg<u8, _USBOEPBCTX_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBCTX_1;
#[doc = "`read()` method returns [usboepbctx_1::R](usboepbctx_1::R) reader structure"]
impl crate::Readable for USBOEPBCTX_1 {}
#[doc = "`write(|w| ..)` method takes [usboepbctx_1::W](usboepbctx_1::W) writer structure"]
impl crate::Writable for USBOEPBCTX_1 {}
#[doc = "Output Endpoint_1: X-byte count"]
pub mod usboepbctx_1;
#[doc = "Output Endpoint_1: Y-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbbay_1](usboepbbay_1) module"]
pub type USBOEPBBAY_1 = crate::Reg<u8, _USBOEPBBAY_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBBAY_1;
#[doc = "`read()` method returns [usboepbbay_1::R](usboepbbay_1::R) reader structure"]
impl crate::Readable for USBOEPBBAY_1 {}
#[doc = "`write(|w| ..)` method takes [usboepbbay_1::W](usboepbbay_1::W) writer structure"]
impl crate::Writable for USBOEPBBAY_1 {}
#[doc = "Output Endpoint_1: Y-buffer base addr."]
pub mod usboepbbay_1;
#[doc = "Output Endpoint_1: Y-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbcty_1](usboepbcty_1) module"]
pub type USBOEPBCTY_1 = crate::Reg<u8, _USBOEPBCTY_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBCTY_1;
#[doc = "`read()` method returns [usboepbcty_1::R](usboepbcty_1::R) reader structure"]
impl crate::Readable for USBOEPBCTY_1 {}
#[doc = "`write(|w| ..)` method takes [usboepbcty_1::W](usboepbcty_1::W) writer structure"]
impl crate::Writable for USBOEPBCTY_1 {}
#[doc = "Output Endpoint_1: Y-byte count"]
pub mod usboepbcty_1;
#[doc = "Output Endpoint_1: X/Y-buffer size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepsizxy_1](usboepsizxy_1) module"]
pub type USBOEPSIZXY_1 = crate::Reg<u8, _USBOEPSIZXY_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPSIZXY_1;
#[doc = "`read()` method returns [usboepsizxy_1::R](usboepsizxy_1::R) reader structure"]
impl crate::Readable for USBOEPSIZXY_1 {}
#[doc = "`write(|w| ..)` method takes [usboepsizxy_1::W](usboepsizxy_1::W) writer structure"]
impl crate::Writable for USBOEPSIZXY_1 {}
#[doc = "Output Endpoint_1: X/Y-buffer size"]
pub mod usboepsizxy_1;
#[doc = "Output Endpoint_2: Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepcnf_2](usboepcnf_2) module"]
pub type USBOEPCNF_2 = crate::Reg<u8, _USBOEPCNF_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPCNF_2;
#[doc = "`read()` method returns [usboepcnf_2::R](usboepcnf_2::R) reader structure"]
impl crate::Readable for USBOEPCNF_2 {}
#[doc = "`write(|w| ..)` method takes [usboepcnf_2::W](usboepcnf_2::W) writer structure"]
impl crate::Writable for USBOEPCNF_2 {}
#[doc = "Output Endpoint_2: Configuration"]
pub mod usboepcnf_2;
#[doc = "Output Endpoint_2: X-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbbax_2](usboepbbax_2) module"]
pub type USBOEPBBAX_2 = crate::Reg<u8, _USBOEPBBAX_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBBAX_2;
#[doc = "`read()` method returns [usboepbbax_2::R](usboepbbax_2::R) reader structure"]
impl crate::Readable for USBOEPBBAX_2 {}
#[doc = "`write(|w| ..)` method takes [usboepbbax_2::W](usboepbbax_2::W) writer structure"]
impl crate::Writable for USBOEPBBAX_2 {}
#[doc = "Output Endpoint_2: X-buffer base addr."]
pub mod usboepbbax_2;
#[doc = "Output Endpoint_2: X-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbctx_2](usboepbctx_2) module"]
pub type USBOEPBCTX_2 = crate::Reg<u8, _USBOEPBCTX_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBCTX_2;
#[doc = "`read()` method returns [usboepbctx_2::R](usboepbctx_2::R) reader structure"]
impl crate::Readable for USBOEPBCTX_2 {}
#[doc = "`write(|w| ..)` method takes [usboepbctx_2::W](usboepbctx_2::W) writer structure"]
impl crate::Writable for USBOEPBCTX_2 {}
#[doc = "Output Endpoint_2: X-byte count"]
pub mod usboepbctx_2;
#[doc = "Output Endpoint_2: Y-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbbay_2](usboepbbay_2) module"]
pub type USBOEPBBAY_2 = crate::Reg<u8, _USBOEPBBAY_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBBAY_2;
#[doc = "`read()` method returns [usboepbbay_2::R](usboepbbay_2::R) reader structure"]
impl crate::Readable for USBOEPBBAY_2 {}
#[doc = "`write(|w| ..)` method takes [usboepbbay_2::W](usboepbbay_2::W) writer structure"]
impl crate::Writable for USBOEPBBAY_2 {}
#[doc = "Output Endpoint_2: Y-buffer base addr."]
pub mod usboepbbay_2;
#[doc = "Output Endpoint_2: Y-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbcty_2](usboepbcty_2) module"]
pub type USBOEPBCTY_2 = crate::Reg<u8, _USBOEPBCTY_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBCTY_2;
#[doc = "`read()` method returns [usboepbcty_2::R](usboepbcty_2::R) reader structure"]
impl crate::Readable for USBOEPBCTY_2 {}
#[doc = "`write(|w| ..)` method takes [usboepbcty_2::W](usboepbcty_2::W) writer structure"]
impl crate::Writable for USBOEPBCTY_2 {}
#[doc = "Output Endpoint_2: Y-byte count"]
pub mod usboepbcty_2;
#[doc = "Output Endpoint_2: X/Y-buffer size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepsizxy_2](usboepsizxy_2) module"]
pub type USBOEPSIZXY_2 = crate::Reg<u8, _USBOEPSIZXY_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPSIZXY_2;
#[doc = "`read()` method returns [usboepsizxy_2::R](usboepsizxy_2::R) reader structure"]
impl crate::Readable for USBOEPSIZXY_2 {}
#[doc = "`write(|w| ..)` method takes [usboepsizxy_2::W](usboepsizxy_2::W) writer structure"]
impl crate::Writable for USBOEPSIZXY_2 {}
#[doc = "Output Endpoint_2: X/Y-buffer size"]
pub mod usboepsizxy_2;
#[doc = "Output Endpoint_3: Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepcnf_3](usboepcnf_3) module"]
pub type USBOEPCNF_3 = crate::Reg<u8, _USBOEPCNF_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPCNF_3;
#[doc = "`read()` method returns [usboepcnf_3::R](usboepcnf_3::R) reader structure"]
impl crate::Readable for USBOEPCNF_3 {}
#[doc = "`write(|w| ..)` method takes [usboepcnf_3::W](usboepcnf_3::W) writer structure"]
impl crate::Writable for USBOEPCNF_3 {}
#[doc = "Output Endpoint_3: Configuration"]
pub mod usboepcnf_3;
#[doc = "Output Endpoint_3: X-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbbax_3](usboepbbax_3) module"]
pub type USBOEPBBAX_3 = crate::Reg<u8, _USBOEPBBAX_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBBAX_3;
#[doc = "`read()` method returns [usboepbbax_3::R](usboepbbax_3::R) reader structure"]
impl crate::Readable for USBOEPBBAX_3 {}
#[doc = "`write(|w| ..)` method takes [usboepbbax_3::W](usboepbbax_3::W) writer structure"]
impl crate::Writable for USBOEPBBAX_3 {}
#[doc = "Output Endpoint_3: X-buffer base addr."]
pub mod usboepbbax_3;
#[doc = "Output Endpoint_3: X-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbctx_3](usboepbctx_3) module"]
pub type USBOEPBCTX_3 = crate::Reg<u8, _USBOEPBCTX_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBCTX_3;
#[doc = "`read()` method returns [usboepbctx_3::R](usboepbctx_3::R) reader structure"]
impl crate::Readable for USBOEPBCTX_3 {}
#[doc = "`write(|w| ..)` method takes [usboepbctx_3::W](usboepbctx_3::W) writer structure"]
impl crate::Writable for USBOEPBCTX_3 {}
#[doc = "Output Endpoint_3: X-byte count"]
pub mod usboepbctx_3;
#[doc = "Output Endpoint_3: Y-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbbay_3](usboepbbay_3) module"]
pub type USBOEPBBAY_3 = crate::Reg<u8, _USBOEPBBAY_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBBAY_3;
#[doc = "`read()` method returns [usboepbbay_3::R](usboepbbay_3::R) reader structure"]
impl crate::Readable for USBOEPBBAY_3 {}
#[doc = "`write(|w| ..)` method takes [usboepbbay_3::W](usboepbbay_3::W) writer structure"]
impl crate::Writable for USBOEPBBAY_3 {}
#[doc = "Output Endpoint_3: Y-buffer base addr."]
pub mod usboepbbay_3;
#[doc = "Output Endpoint_3: Y-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbcty_3](usboepbcty_3) module"]
pub type USBOEPBCTY_3 = crate::Reg<u8, _USBOEPBCTY_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBCTY_3;
#[doc = "`read()` method returns [usboepbcty_3::R](usboepbcty_3::R) reader structure"]
impl crate::Readable for USBOEPBCTY_3 {}
#[doc = "`write(|w| ..)` method takes [usboepbcty_3::W](usboepbcty_3::W) writer structure"]
impl crate::Writable for USBOEPBCTY_3 {}
#[doc = "Output Endpoint_3: Y-byte count"]
pub mod usboepbcty_3;
#[doc = "Output Endpoint_3: X/Y-buffer size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepsizxy_3](usboepsizxy_3) module"]
pub type USBOEPSIZXY_3 = crate::Reg<u8, _USBOEPSIZXY_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPSIZXY_3;
#[doc = "`read()` method returns [usboepsizxy_3::R](usboepsizxy_3::R) reader structure"]
impl crate::Readable for USBOEPSIZXY_3 {}
#[doc = "`write(|w| ..)` method takes [usboepsizxy_3::W](usboepsizxy_3::W) writer structure"]
impl crate::Writable for USBOEPSIZXY_3 {}
#[doc = "Output Endpoint_3: X/Y-buffer size"]
pub mod usboepsizxy_3;
#[doc = "Output Endpoint_4: Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepcnf_4](usboepcnf_4) module"]
pub type USBOEPCNF_4 = crate::Reg<u8, _USBOEPCNF_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPCNF_4;
#[doc = "`read()` method returns [usboepcnf_4::R](usboepcnf_4::R) reader structure"]
impl crate::Readable for USBOEPCNF_4 {}
#[doc = "`write(|w| ..)` method takes [usboepcnf_4::W](usboepcnf_4::W) writer structure"]
impl crate::Writable for USBOEPCNF_4 {}
#[doc = "Output Endpoint_4: Configuration"]
pub mod usboepcnf_4;
#[doc = "Output Endpoint_4: X-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbbax_4](usboepbbax_4) module"]
pub type USBOEPBBAX_4 = crate::Reg<u8, _USBOEPBBAX_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBBAX_4;
#[doc = "`read()` method returns [usboepbbax_4::R](usboepbbax_4::R) reader structure"]
impl crate::Readable for USBOEPBBAX_4 {}
#[doc = "`write(|w| ..)` method takes [usboepbbax_4::W](usboepbbax_4::W) writer structure"]
impl crate::Writable for USBOEPBBAX_4 {}
#[doc = "Output Endpoint_4: X-buffer base addr."]
pub mod usboepbbax_4;
#[doc = "Output Endpoint_4: X-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbctx_4](usboepbctx_4) module"]
pub type USBOEPBCTX_4 = crate::Reg<u8, _USBOEPBCTX_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBCTX_4;
#[doc = "`read()` method returns [usboepbctx_4::R](usboepbctx_4::R) reader structure"]
impl crate::Readable for USBOEPBCTX_4 {}
#[doc = "`write(|w| ..)` method takes [usboepbctx_4::W](usboepbctx_4::W) writer structure"]
impl crate::Writable for USBOEPBCTX_4 {}
#[doc = "Output Endpoint_4: X-byte count"]
pub mod usboepbctx_4;
#[doc = "Output Endpoint_4: Y-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbbay_4](usboepbbay_4) module"]
pub type USBOEPBBAY_4 = crate::Reg<u8, _USBOEPBBAY_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBBAY_4;
#[doc = "`read()` method returns [usboepbbay_4::R](usboepbbay_4::R) reader structure"]
impl crate::Readable for USBOEPBBAY_4 {}
#[doc = "`write(|w| ..)` method takes [usboepbbay_4::W](usboepbbay_4::W) writer structure"]
impl crate::Writable for USBOEPBBAY_4 {}
#[doc = "Output Endpoint_4: Y-buffer base addr."]
pub mod usboepbbay_4;
#[doc = "Output Endpoint_4: Y-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbcty_4](usboepbcty_4) module"]
pub type USBOEPBCTY_4 = crate::Reg<u8, _USBOEPBCTY_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBCTY_4;
#[doc = "`read()` method returns [usboepbcty_4::R](usboepbcty_4::R) reader structure"]
impl crate::Readable for USBOEPBCTY_4 {}
#[doc = "`write(|w| ..)` method takes [usboepbcty_4::W](usboepbcty_4::W) writer structure"]
impl crate::Writable for USBOEPBCTY_4 {}
#[doc = "Output Endpoint_4: Y-byte count"]
pub mod usboepbcty_4;
#[doc = "Output Endpoint_4: X/Y-buffer size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepsizxy_4](usboepsizxy_4) module"]
pub type USBOEPSIZXY_4 = crate::Reg<u8, _USBOEPSIZXY_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPSIZXY_4;
#[doc = "`read()` method returns [usboepsizxy_4::R](usboepsizxy_4::R) reader structure"]
impl crate::Readable for USBOEPSIZXY_4 {}
#[doc = "`write(|w| ..)` method takes [usboepsizxy_4::W](usboepsizxy_4::W) writer structure"]
impl crate::Writable for USBOEPSIZXY_4 {}
#[doc = "Output Endpoint_4: X/Y-buffer size"]
pub mod usboepsizxy_4;
#[doc = "Output Endpoint_5: Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepcnf_5](usboepcnf_5) module"]
pub type USBOEPCNF_5 = crate::Reg<u8, _USBOEPCNF_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPCNF_5;
#[doc = "`read()` method returns [usboepcnf_5::R](usboepcnf_5::R) reader structure"]
impl crate::Readable for USBOEPCNF_5 {}
#[doc = "`write(|w| ..)` method takes [usboepcnf_5::W](usboepcnf_5::W) writer structure"]
impl crate::Writable for USBOEPCNF_5 {}
#[doc = "Output Endpoint_5: Configuration"]
pub mod usboepcnf_5;
#[doc = "Output Endpoint_5: X-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbbax_5](usboepbbax_5) module"]
pub type USBOEPBBAX_5 = crate::Reg<u8, _USBOEPBBAX_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBBAX_5;
#[doc = "`read()` method returns [usboepbbax_5::R](usboepbbax_5::R) reader structure"]
impl crate::Readable for USBOEPBBAX_5 {}
#[doc = "`write(|w| ..)` method takes [usboepbbax_5::W](usboepbbax_5::W) writer structure"]
impl crate::Writable for USBOEPBBAX_5 {}
#[doc = "Output Endpoint_5: X-buffer base addr."]
pub mod usboepbbax_5;
#[doc = "Output Endpoint_5: X-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbctx_5](usboepbctx_5) module"]
pub type USBOEPBCTX_5 = crate::Reg<u8, _USBOEPBCTX_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBCTX_5;
#[doc = "`read()` method returns [usboepbctx_5::R](usboepbctx_5::R) reader structure"]
impl crate::Readable for USBOEPBCTX_5 {}
#[doc = "`write(|w| ..)` method takes [usboepbctx_5::W](usboepbctx_5::W) writer structure"]
impl crate::Writable for USBOEPBCTX_5 {}
#[doc = "Output Endpoint_5: X-byte count"]
pub mod usboepbctx_5;
#[doc = "Output Endpoint_5: Y-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbbay_5](usboepbbay_5) module"]
pub type USBOEPBBAY_5 = crate::Reg<u8, _USBOEPBBAY_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBBAY_5;
#[doc = "`read()` method returns [usboepbbay_5::R](usboepbbay_5::R) reader structure"]
impl crate::Readable for USBOEPBBAY_5 {}
#[doc = "`write(|w| ..)` method takes [usboepbbay_5::W](usboepbbay_5::W) writer structure"]
impl crate::Writable for USBOEPBBAY_5 {}
#[doc = "Output Endpoint_5: Y-buffer base addr."]
pub mod usboepbbay_5;
#[doc = "Output Endpoint_5: Y-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbcty_5](usboepbcty_5) module"]
pub type USBOEPBCTY_5 = crate::Reg<u8, _USBOEPBCTY_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBCTY_5;
#[doc = "`read()` method returns [usboepbcty_5::R](usboepbcty_5::R) reader structure"]
impl crate::Readable for USBOEPBCTY_5 {}
#[doc = "`write(|w| ..)` method takes [usboepbcty_5::W](usboepbcty_5::W) writer structure"]
impl crate::Writable for USBOEPBCTY_5 {}
#[doc = "Output Endpoint_5: Y-byte count"]
pub mod usboepbcty_5;
#[doc = "Output Endpoint_5: X/Y-buffer size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepsizxy_5](usboepsizxy_5) module"]
pub type USBOEPSIZXY_5 = crate::Reg<u8, _USBOEPSIZXY_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPSIZXY_5;
#[doc = "`read()` method returns [usboepsizxy_5::R](usboepsizxy_5::R) reader structure"]
impl crate::Readable for USBOEPSIZXY_5 {}
#[doc = "`write(|w| ..)` method takes [usboepsizxy_5::W](usboepsizxy_5::W) writer structure"]
impl crate::Writable for USBOEPSIZXY_5 {}
#[doc = "Output Endpoint_5: X/Y-buffer size"]
pub mod usboepsizxy_5;
#[doc = "Output Endpoint_6: Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepcnf_6](usboepcnf_6) module"]
pub type USBOEPCNF_6 = crate::Reg<u8, _USBOEPCNF_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPCNF_6;
#[doc = "`read()` method returns [usboepcnf_6::R](usboepcnf_6::R) reader structure"]
impl crate::Readable for USBOEPCNF_6 {}
#[doc = "`write(|w| ..)` method takes [usboepcnf_6::W](usboepcnf_6::W) writer structure"]
impl crate::Writable for USBOEPCNF_6 {}
#[doc = "Output Endpoint_6: Configuration"]
pub mod usboepcnf_6;
#[doc = "Output Endpoint_6: X-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbbax_6](usboepbbax_6) module"]
pub type USBOEPBBAX_6 = crate::Reg<u8, _USBOEPBBAX_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBBAX_6;
#[doc = "`read()` method returns [usboepbbax_6::R](usboepbbax_6::R) reader structure"]
impl crate::Readable for USBOEPBBAX_6 {}
#[doc = "`write(|w| ..)` method takes [usboepbbax_6::W](usboepbbax_6::W) writer structure"]
impl crate::Writable for USBOEPBBAX_6 {}
#[doc = "Output Endpoint_6: X-buffer base addr."]
pub mod usboepbbax_6;
#[doc = "Output Endpoint_6: X-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbctx_6](usboepbctx_6) module"]
pub type USBOEPBCTX_6 = crate::Reg<u8, _USBOEPBCTX_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBCTX_6;
#[doc = "`read()` method returns [usboepbctx_6::R](usboepbctx_6::R) reader structure"]
impl crate::Readable for USBOEPBCTX_6 {}
#[doc = "`write(|w| ..)` method takes [usboepbctx_6::W](usboepbctx_6::W) writer structure"]
impl crate::Writable for USBOEPBCTX_6 {}
#[doc = "Output Endpoint_6: X-byte count"]
pub mod usboepbctx_6;
#[doc = "Output Endpoint_6: Y-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbbay_6](usboepbbay_6) module"]
pub type USBOEPBBAY_6 = crate::Reg<u8, _USBOEPBBAY_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBBAY_6;
#[doc = "`read()` method returns [usboepbbay_6::R](usboepbbay_6::R) reader structure"]
impl crate::Readable for USBOEPBBAY_6 {}
#[doc = "`write(|w| ..)` method takes [usboepbbay_6::W](usboepbbay_6::W) writer structure"]
impl crate::Writable for USBOEPBBAY_6 {}
#[doc = "Output Endpoint_6: Y-buffer base addr."]
pub mod usboepbbay_6;
#[doc = "Output Endpoint_6: Y-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbcty_6](usboepbcty_6) module"]
pub type USBOEPBCTY_6 = crate::Reg<u8, _USBOEPBCTY_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBCTY_6;
#[doc = "`read()` method returns [usboepbcty_6::R](usboepbcty_6::R) reader structure"]
impl crate::Readable for USBOEPBCTY_6 {}
#[doc = "`write(|w| ..)` method takes [usboepbcty_6::W](usboepbcty_6::W) writer structure"]
impl crate::Writable for USBOEPBCTY_6 {}
#[doc = "Output Endpoint_6: Y-byte count"]
pub mod usboepbcty_6;
#[doc = "Output Endpoint_6: X/Y-buffer size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepsizxy_6](usboepsizxy_6) module"]
pub type USBOEPSIZXY_6 = crate::Reg<u8, _USBOEPSIZXY_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPSIZXY_6;
#[doc = "`read()` method returns [usboepsizxy_6::R](usboepsizxy_6::R) reader structure"]
impl crate::Readable for USBOEPSIZXY_6 {}
#[doc = "`write(|w| ..)` method takes [usboepsizxy_6::W](usboepsizxy_6::W) writer structure"]
impl crate::Writable for USBOEPSIZXY_6 {}
#[doc = "Output Endpoint_6: X/Y-buffer size"]
pub mod usboepsizxy_6;
#[doc = "Output Endpoint_7: Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepcnf_7](usboepcnf_7) module"]
pub type USBOEPCNF_7 = crate::Reg<u8, _USBOEPCNF_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPCNF_7;
#[doc = "`read()` method returns [usboepcnf_7::R](usboepcnf_7::R) reader structure"]
impl crate::Readable for USBOEPCNF_7 {}
#[doc = "`write(|w| ..)` method takes [usboepcnf_7::W](usboepcnf_7::W) writer structure"]
impl crate::Writable for USBOEPCNF_7 {}
#[doc = "Output Endpoint_7: Configuration"]
pub mod usboepcnf_7;
#[doc = "Output Endpoint_7: X-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbbax_7](usboepbbax_7) module"]
pub type USBOEPBBAX_7 = crate::Reg<u8, _USBOEPBBAX_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBBAX_7;
#[doc = "`read()` method returns [usboepbbax_7::R](usboepbbax_7::R) reader structure"]
impl crate::Readable for USBOEPBBAX_7 {}
#[doc = "`write(|w| ..)` method takes [usboepbbax_7::W](usboepbbax_7::W) writer structure"]
impl crate::Writable for USBOEPBBAX_7 {}
#[doc = "Output Endpoint_7: X-buffer base addr."]
pub mod usboepbbax_7;
#[doc = "Output Endpoint_7: X-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbctx_7](usboepbctx_7) module"]
pub type USBOEPBCTX_7 = crate::Reg<u8, _USBOEPBCTX_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBCTX_7;
#[doc = "`read()` method returns [usboepbctx_7::R](usboepbctx_7::R) reader structure"]
impl crate::Readable for USBOEPBCTX_7 {}
#[doc = "`write(|w| ..)` method takes [usboepbctx_7::W](usboepbctx_7::W) writer structure"]
impl crate::Writable for USBOEPBCTX_7 {}
#[doc = "Output Endpoint_7: X-byte count"]
pub mod usboepbctx_7;
#[doc = "Output Endpoint_7: Y-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbbay_7](usboepbbay_7) module"]
pub type USBOEPBBAY_7 = crate::Reg<u8, _USBOEPBBAY_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBBAY_7;
#[doc = "`read()` method returns [usboepbbay_7::R](usboepbbay_7::R) reader structure"]
impl crate::Readable for USBOEPBBAY_7 {}
#[doc = "`write(|w| ..)` method takes [usboepbbay_7::W](usboepbbay_7::W) writer structure"]
impl crate::Writable for USBOEPBBAY_7 {}
#[doc = "Output Endpoint_7: Y-buffer base addr."]
pub mod usboepbbay_7;
#[doc = "Output Endpoint_7: Y-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbcty_7](usboepbcty_7) module"]
pub type USBOEPBCTY_7 = crate::Reg<u8, _USBOEPBCTY_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPBCTY_7;
#[doc = "`read()` method returns [usboepbcty_7::R](usboepbcty_7::R) reader structure"]
impl crate::Readable for USBOEPBCTY_7 {}
#[doc = "`write(|w| ..)` method takes [usboepbcty_7::W](usboepbcty_7::W) writer structure"]
impl crate::Writable for USBOEPBCTY_7 {}
#[doc = "Output Endpoint_7: Y-byte count"]
pub mod usboepbcty_7;
#[doc = "Output Endpoint_7: X/Y-buffer size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepsizxy_7](usboepsizxy_7) module"]
pub type USBOEPSIZXY_7 = crate::Reg<u8, _USBOEPSIZXY_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPSIZXY_7;
#[doc = "`read()` method returns [usboepsizxy_7::R](usboepsizxy_7::R) reader structure"]
impl crate::Readable for USBOEPSIZXY_7 {}
#[doc = "`write(|w| ..)` method takes [usboepsizxy_7::W](usboepsizxy_7::W) writer structure"]
impl crate::Writable for USBOEPSIZXY_7 {}
#[doc = "Output Endpoint_7: X/Y-buffer size"]
pub mod usboepsizxy_7;
#[doc = "Input Endpoint_1: Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepcnf_1](usbiepcnf_1) module"]
pub type USBIEPCNF_1 = crate::Reg<u8, _USBIEPCNF_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPCNF_1;
#[doc = "`read()` method returns [usbiepcnf_1::R](usbiepcnf_1::R) reader structure"]
impl crate::Readable for USBIEPCNF_1 {}
#[doc = "`write(|w| ..)` method takes [usbiepcnf_1::W](usbiepcnf_1::W) writer structure"]
impl crate::Writable for USBIEPCNF_1 {}
#[doc = "Input Endpoint_1: Configuration"]
pub mod usbiepcnf_1;
#[doc = "Input Endpoint_1: X-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbbax_1](usbiepbbax_1) module"]
pub type USBIEPBBAX_1 = crate::Reg<u8, _USBIEPBBAX_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBBAX_1;
#[doc = "`read()` method returns [usbiepbbax_1::R](usbiepbbax_1::R) reader structure"]
impl crate::Readable for USBIEPBBAX_1 {}
#[doc = "`write(|w| ..)` method takes [usbiepbbax_1::W](usbiepbbax_1::W) writer structure"]
impl crate::Writable for USBIEPBBAX_1 {}
#[doc = "Input Endpoint_1: X-buffer base addr."]
pub mod usbiepbbax_1;
#[doc = "Input Endpoint_1: X-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbctx_1](usbiepbctx_1) module"]
pub type USBIEPBCTX_1 = crate::Reg<u8, _USBIEPBCTX_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBCTX_1;
#[doc = "`read()` method returns [usbiepbctx_1::R](usbiepbctx_1::R) reader structure"]
impl crate::Readable for USBIEPBCTX_1 {}
#[doc = "`write(|w| ..)` method takes [usbiepbctx_1::W](usbiepbctx_1::W) writer structure"]
impl crate::Writable for USBIEPBCTX_1 {}
#[doc = "Input Endpoint_1: X-byte count"]
pub mod usbiepbctx_1;
#[doc = "Input Endpoint_1: Y-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbbay_1](usbiepbbay_1) module"]
pub type USBIEPBBAY_1 = crate::Reg<u8, _USBIEPBBAY_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBBAY_1;
#[doc = "`read()` method returns [usbiepbbay_1::R](usbiepbbay_1::R) reader structure"]
impl crate::Readable for USBIEPBBAY_1 {}
#[doc = "`write(|w| ..)` method takes [usbiepbbay_1::W](usbiepbbay_1::W) writer structure"]
impl crate::Writable for USBIEPBBAY_1 {}
#[doc = "Input Endpoint_1: Y-buffer base addr."]
pub mod usbiepbbay_1;
#[doc = "Input Endpoint_1: Y-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbcty_1](usbiepbcty_1) module"]
pub type USBIEPBCTY_1 = crate::Reg<u8, _USBIEPBCTY_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBCTY_1;
#[doc = "`read()` method returns [usbiepbcty_1::R](usbiepbcty_1::R) reader structure"]
impl crate::Readable for USBIEPBCTY_1 {}
#[doc = "`write(|w| ..)` method takes [usbiepbcty_1::W](usbiepbcty_1::W) writer structure"]
impl crate::Writable for USBIEPBCTY_1 {}
#[doc = "Input Endpoint_1: Y-byte count"]
pub mod usbiepbcty_1;
#[doc = "Input Endpoint_1: X/Y-buffer size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepsizxy_1](usbiepsizxy_1) module"]
pub type USBIEPSIZXY_1 = crate::Reg<u8, _USBIEPSIZXY_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPSIZXY_1;
#[doc = "`read()` method returns [usbiepsizxy_1::R](usbiepsizxy_1::R) reader structure"]
impl crate::Readable for USBIEPSIZXY_1 {}
#[doc = "`write(|w| ..)` method takes [usbiepsizxy_1::W](usbiepsizxy_1::W) writer structure"]
impl crate::Writable for USBIEPSIZXY_1 {}
#[doc = "Input Endpoint_1: X/Y-buffer size"]
pub mod usbiepsizxy_1;
#[doc = "Input Endpoint_2: Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepcnf_2](usbiepcnf_2) module"]
pub type USBIEPCNF_2 = crate::Reg<u8, _USBIEPCNF_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPCNF_2;
#[doc = "`read()` method returns [usbiepcnf_2::R](usbiepcnf_2::R) reader structure"]
impl crate::Readable for USBIEPCNF_2 {}
#[doc = "`write(|w| ..)` method takes [usbiepcnf_2::W](usbiepcnf_2::W) writer structure"]
impl crate::Writable for USBIEPCNF_2 {}
#[doc = "Input Endpoint_2: Configuration"]
pub mod usbiepcnf_2;
#[doc = "Input Endpoint_2: X-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbbax_2](usbiepbbax_2) module"]
pub type USBIEPBBAX_2 = crate::Reg<u8, _USBIEPBBAX_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBBAX_2;
#[doc = "`read()` method returns [usbiepbbax_2::R](usbiepbbax_2::R) reader structure"]
impl crate::Readable for USBIEPBBAX_2 {}
#[doc = "`write(|w| ..)` method takes [usbiepbbax_2::W](usbiepbbax_2::W) writer structure"]
impl crate::Writable for USBIEPBBAX_2 {}
#[doc = "Input Endpoint_2: X-buffer base addr."]
pub mod usbiepbbax_2;
#[doc = "Input Endpoint_2: X-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbctx_2](usbiepbctx_2) module"]
pub type USBIEPBCTX_2 = crate::Reg<u8, _USBIEPBCTX_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBCTX_2;
#[doc = "`read()` method returns [usbiepbctx_2::R](usbiepbctx_2::R) reader structure"]
impl crate::Readable for USBIEPBCTX_2 {}
#[doc = "`write(|w| ..)` method takes [usbiepbctx_2::W](usbiepbctx_2::W) writer structure"]
impl crate::Writable for USBIEPBCTX_2 {}
#[doc = "Input Endpoint_2: X-byte count"]
pub mod usbiepbctx_2;
#[doc = "Input Endpoint_2: Y-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbbay_2](usbiepbbay_2) module"]
pub type USBIEPBBAY_2 = crate::Reg<u8, _USBIEPBBAY_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBBAY_2;
#[doc = "`read()` method returns [usbiepbbay_2::R](usbiepbbay_2::R) reader structure"]
impl crate::Readable for USBIEPBBAY_2 {}
#[doc = "`write(|w| ..)` method takes [usbiepbbay_2::W](usbiepbbay_2::W) writer structure"]
impl crate::Writable for USBIEPBBAY_2 {}
#[doc = "Input Endpoint_2: Y-buffer base addr."]
pub mod usbiepbbay_2;
#[doc = "Input Endpoint_2: Y-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbcty_2](usbiepbcty_2) module"]
pub type USBIEPBCTY_2 = crate::Reg<u8, _USBIEPBCTY_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBCTY_2;
#[doc = "`read()` method returns [usbiepbcty_2::R](usbiepbcty_2::R) reader structure"]
impl crate::Readable for USBIEPBCTY_2 {}
#[doc = "`write(|w| ..)` method takes [usbiepbcty_2::W](usbiepbcty_2::W) writer structure"]
impl crate::Writable for USBIEPBCTY_2 {}
#[doc = "Input Endpoint_2: Y-byte count"]
pub mod usbiepbcty_2;
#[doc = "Input Endpoint_2: X/Y-buffer size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepsizxy_2](usbiepsizxy_2) module"]
pub type USBIEPSIZXY_2 = crate::Reg<u8, _USBIEPSIZXY_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPSIZXY_2;
#[doc = "`read()` method returns [usbiepsizxy_2::R](usbiepsizxy_2::R) reader structure"]
impl crate::Readable for USBIEPSIZXY_2 {}
#[doc = "`write(|w| ..)` method takes [usbiepsizxy_2::W](usbiepsizxy_2::W) writer structure"]
impl crate::Writable for USBIEPSIZXY_2 {}
#[doc = "Input Endpoint_2: X/Y-buffer size"]
pub mod usbiepsizxy_2;
#[doc = "Input Endpoint_3: Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepcnf_3](usbiepcnf_3) module"]
pub type USBIEPCNF_3 = crate::Reg<u8, _USBIEPCNF_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPCNF_3;
#[doc = "`read()` method returns [usbiepcnf_3::R](usbiepcnf_3::R) reader structure"]
impl crate::Readable for USBIEPCNF_3 {}
#[doc = "`write(|w| ..)` method takes [usbiepcnf_3::W](usbiepcnf_3::W) writer structure"]
impl crate::Writable for USBIEPCNF_3 {}
#[doc = "Input Endpoint_3: Configuration"]
pub mod usbiepcnf_3;
#[doc = "Input Endpoint_3: X-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbbax_3](usbiepbbax_3) module"]
pub type USBIEPBBAX_3 = crate::Reg<u8, _USBIEPBBAX_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBBAX_3;
#[doc = "`read()` method returns [usbiepbbax_3::R](usbiepbbax_3::R) reader structure"]
impl crate::Readable for USBIEPBBAX_3 {}
#[doc = "`write(|w| ..)` method takes [usbiepbbax_3::W](usbiepbbax_3::W) writer structure"]
impl crate::Writable for USBIEPBBAX_3 {}
#[doc = "Input Endpoint_3: X-buffer base addr."]
pub mod usbiepbbax_3;
#[doc = "Input Endpoint_3: X-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbctx_3](usbiepbctx_3) module"]
pub type USBIEPBCTX_3 = crate::Reg<u8, _USBIEPBCTX_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBCTX_3;
#[doc = "`read()` method returns [usbiepbctx_3::R](usbiepbctx_3::R) reader structure"]
impl crate::Readable for USBIEPBCTX_3 {}
#[doc = "`write(|w| ..)` method takes [usbiepbctx_3::W](usbiepbctx_3::W) writer structure"]
impl crate::Writable for USBIEPBCTX_3 {}
#[doc = "Input Endpoint_3: X-byte count"]
pub mod usbiepbctx_3;
#[doc = "Input Endpoint_3: Y-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbbay_3](usbiepbbay_3) module"]
pub type USBIEPBBAY_3 = crate::Reg<u8, _USBIEPBBAY_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBBAY_3;
#[doc = "`read()` method returns [usbiepbbay_3::R](usbiepbbay_3::R) reader structure"]
impl crate::Readable for USBIEPBBAY_3 {}
#[doc = "`write(|w| ..)` method takes [usbiepbbay_3::W](usbiepbbay_3::W) writer structure"]
impl crate::Writable for USBIEPBBAY_3 {}
#[doc = "Input Endpoint_3: Y-buffer base addr."]
pub mod usbiepbbay_3;
#[doc = "Input Endpoint_3: Y-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbcty_3](usbiepbcty_3) module"]
pub type USBIEPBCTY_3 = crate::Reg<u8, _USBIEPBCTY_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBCTY_3;
#[doc = "`read()` method returns [usbiepbcty_3::R](usbiepbcty_3::R) reader structure"]
impl crate::Readable for USBIEPBCTY_3 {}
#[doc = "`write(|w| ..)` method takes [usbiepbcty_3::W](usbiepbcty_3::W) writer structure"]
impl crate::Writable for USBIEPBCTY_3 {}
#[doc = "Input Endpoint_3: Y-byte count"]
pub mod usbiepbcty_3;
#[doc = "Input Endpoint_3: X/Y-buffer size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepsizxy_3](usbiepsizxy_3) module"]
pub type USBIEPSIZXY_3 = crate::Reg<u8, _USBIEPSIZXY_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPSIZXY_3;
#[doc = "`read()` method returns [usbiepsizxy_3::R](usbiepsizxy_3::R) reader structure"]
impl crate::Readable for USBIEPSIZXY_3 {}
#[doc = "`write(|w| ..)` method takes [usbiepsizxy_3::W](usbiepsizxy_3::W) writer structure"]
impl crate::Writable for USBIEPSIZXY_3 {}
#[doc = "Input Endpoint_3: X/Y-buffer size"]
pub mod usbiepsizxy_3;
#[doc = "Input Endpoint_4: Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepcnf_4](usbiepcnf_4) module"]
pub type USBIEPCNF_4 = crate::Reg<u8, _USBIEPCNF_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPCNF_4;
#[doc = "`read()` method returns [usbiepcnf_4::R](usbiepcnf_4::R) reader structure"]
impl crate::Readable for USBIEPCNF_4 {}
#[doc = "`write(|w| ..)` method takes [usbiepcnf_4::W](usbiepcnf_4::W) writer structure"]
impl crate::Writable for USBIEPCNF_4 {}
#[doc = "Input Endpoint_4: Configuration"]
pub mod usbiepcnf_4;
#[doc = "Input Endpoint_4: X-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbbax_4](usbiepbbax_4) module"]
pub type USBIEPBBAX_4 = crate::Reg<u8, _USBIEPBBAX_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBBAX_4;
#[doc = "`read()` method returns [usbiepbbax_4::R](usbiepbbax_4::R) reader structure"]
impl crate::Readable for USBIEPBBAX_4 {}
#[doc = "`write(|w| ..)` method takes [usbiepbbax_4::W](usbiepbbax_4::W) writer structure"]
impl crate::Writable for USBIEPBBAX_4 {}
#[doc = "Input Endpoint_4: X-buffer base addr."]
pub mod usbiepbbax_4;
#[doc = "Input Endpoint_4: X-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbctx_4](usbiepbctx_4) module"]
pub type USBIEPBCTX_4 = crate::Reg<u8, _USBIEPBCTX_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBCTX_4;
#[doc = "`read()` method returns [usbiepbctx_4::R](usbiepbctx_4::R) reader structure"]
impl crate::Readable for USBIEPBCTX_4 {}
#[doc = "`write(|w| ..)` method takes [usbiepbctx_4::W](usbiepbctx_4::W) writer structure"]
impl crate::Writable for USBIEPBCTX_4 {}
#[doc = "Input Endpoint_4: X-byte count"]
pub mod usbiepbctx_4;
#[doc = "Input Endpoint_4: Y-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbbay_4](usbiepbbay_4) module"]
pub type USBIEPBBAY_4 = crate::Reg<u8, _USBIEPBBAY_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBBAY_4;
#[doc = "`read()` method returns [usbiepbbay_4::R](usbiepbbay_4::R) reader structure"]
impl crate::Readable for USBIEPBBAY_4 {}
#[doc = "`write(|w| ..)` method takes [usbiepbbay_4::W](usbiepbbay_4::W) writer structure"]
impl crate::Writable for USBIEPBBAY_4 {}
#[doc = "Input Endpoint_4: Y-buffer base addr."]
pub mod usbiepbbay_4;
#[doc = "Input Endpoint_4: Y-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbcty_4](usbiepbcty_4) module"]
pub type USBIEPBCTY_4 = crate::Reg<u8, _USBIEPBCTY_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBCTY_4;
#[doc = "`read()` method returns [usbiepbcty_4::R](usbiepbcty_4::R) reader structure"]
impl crate::Readable for USBIEPBCTY_4 {}
#[doc = "`write(|w| ..)` method takes [usbiepbcty_4::W](usbiepbcty_4::W) writer structure"]
impl crate::Writable for USBIEPBCTY_4 {}
#[doc = "Input Endpoint_4: Y-byte count"]
pub mod usbiepbcty_4;
#[doc = "Input Endpoint_4: X/Y-buffer size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepsizxy_4](usbiepsizxy_4) module"]
pub type USBIEPSIZXY_4 = crate::Reg<u8, _USBIEPSIZXY_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPSIZXY_4;
#[doc = "`read()` method returns [usbiepsizxy_4::R](usbiepsizxy_4::R) reader structure"]
impl crate::Readable for USBIEPSIZXY_4 {}
#[doc = "`write(|w| ..)` method takes [usbiepsizxy_4::W](usbiepsizxy_4::W) writer structure"]
impl crate::Writable for USBIEPSIZXY_4 {}
#[doc = "Input Endpoint_4: X/Y-buffer size"]
pub mod usbiepsizxy_4;
#[doc = "Input Endpoint_5: Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepcnf_5](usbiepcnf_5) module"]
pub type USBIEPCNF_5 = crate::Reg<u8, _USBIEPCNF_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPCNF_5;
#[doc = "`read()` method returns [usbiepcnf_5::R](usbiepcnf_5::R) reader structure"]
impl crate::Readable for USBIEPCNF_5 {}
#[doc = "`write(|w| ..)` method takes [usbiepcnf_5::W](usbiepcnf_5::W) writer structure"]
impl crate::Writable for USBIEPCNF_5 {}
#[doc = "Input Endpoint_5: Configuration"]
pub mod usbiepcnf_5;
#[doc = "Input Endpoint_5: X-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbbax_5](usbiepbbax_5) module"]
pub type USBIEPBBAX_5 = crate::Reg<u8, _USBIEPBBAX_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBBAX_5;
#[doc = "`read()` method returns [usbiepbbax_5::R](usbiepbbax_5::R) reader structure"]
impl crate::Readable for USBIEPBBAX_5 {}
#[doc = "`write(|w| ..)` method takes [usbiepbbax_5::W](usbiepbbax_5::W) writer structure"]
impl crate::Writable for USBIEPBBAX_5 {}
#[doc = "Input Endpoint_5: X-buffer base addr."]
pub mod usbiepbbax_5;
#[doc = "Input Endpoint_5: X-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbctx_5](usbiepbctx_5) module"]
pub type USBIEPBCTX_5 = crate::Reg<u8, _USBIEPBCTX_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBCTX_5;
#[doc = "`read()` method returns [usbiepbctx_5::R](usbiepbctx_5::R) reader structure"]
impl crate::Readable for USBIEPBCTX_5 {}
#[doc = "`write(|w| ..)` method takes [usbiepbctx_5::W](usbiepbctx_5::W) writer structure"]
impl crate::Writable for USBIEPBCTX_5 {}
#[doc = "Input Endpoint_5: X-byte count"]
pub mod usbiepbctx_5;
#[doc = "Input Endpoint_5: Y-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbbay_5](usbiepbbay_5) module"]
pub type USBIEPBBAY_5 = crate::Reg<u8, _USBIEPBBAY_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBBAY_5;
#[doc = "`read()` method returns [usbiepbbay_5::R](usbiepbbay_5::R) reader structure"]
impl crate::Readable for USBIEPBBAY_5 {}
#[doc = "`write(|w| ..)` method takes [usbiepbbay_5::W](usbiepbbay_5::W) writer structure"]
impl crate::Writable for USBIEPBBAY_5 {}
#[doc = "Input Endpoint_5: Y-buffer base addr."]
pub mod usbiepbbay_5;
#[doc = "Input Endpoint_5: Y-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbcty_5](usbiepbcty_5) module"]
pub type USBIEPBCTY_5 = crate::Reg<u8, _USBIEPBCTY_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBCTY_5;
#[doc = "`read()` method returns [usbiepbcty_5::R](usbiepbcty_5::R) reader structure"]
impl crate::Readable for USBIEPBCTY_5 {}
#[doc = "`write(|w| ..)` method takes [usbiepbcty_5::W](usbiepbcty_5::W) writer structure"]
impl crate::Writable for USBIEPBCTY_5 {}
#[doc = "Input Endpoint_5: Y-byte count"]
pub mod usbiepbcty_5;
#[doc = "Input Endpoint_5: X/Y-buffer size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepsizxy_5](usbiepsizxy_5) module"]
pub type USBIEPSIZXY_5 = crate::Reg<u8, _USBIEPSIZXY_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPSIZXY_5;
#[doc = "`read()` method returns [usbiepsizxy_5::R](usbiepsizxy_5::R) reader structure"]
impl crate::Readable for USBIEPSIZXY_5 {}
#[doc = "`write(|w| ..)` method takes [usbiepsizxy_5::W](usbiepsizxy_5::W) writer structure"]
impl crate::Writable for USBIEPSIZXY_5 {}
#[doc = "Input Endpoint_5: X/Y-buffer size"]
pub mod usbiepsizxy_5;
#[doc = "Input Endpoint_6: Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepcnf_6](usbiepcnf_6) module"]
pub type USBIEPCNF_6 = crate::Reg<u8, _USBIEPCNF_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPCNF_6;
#[doc = "`read()` method returns [usbiepcnf_6::R](usbiepcnf_6::R) reader structure"]
impl crate::Readable for USBIEPCNF_6 {}
#[doc = "`write(|w| ..)` method takes [usbiepcnf_6::W](usbiepcnf_6::W) writer structure"]
impl crate::Writable for USBIEPCNF_6 {}
#[doc = "Input Endpoint_6: Configuration"]
pub mod usbiepcnf_6;
#[doc = "Input Endpoint_6: X-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbbax_6](usbiepbbax_6) module"]
pub type USBIEPBBAX_6 = crate::Reg<u8, _USBIEPBBAX_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBBAX_6;
#[doc = "`read()` method returns [usbiepbbax_6::R](usbiepbbax_6::R) reader structure"]
impl crate::Readable for USBIEPBBAX_6 {}
#[doc = "`write(|w| ..)` method takes [usbiepbbax_6::W](usbiepbbax_6::W) writer structure"]
impl crate::Writable for USBIEPBBAX_6 {}
#[doc = "Input Endpoint_6: X-buffer base addr."]
pub mod usbiepbbax_6;
#[doc = "Input Endpoint_6: X-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbctx_6](usbiepbctx_6) module"]
pub type USBIEPBCTX_6 = crate::Reg<u8, _USBIEPBCTX_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBCTX_6;
#[doc = "`read()` method returns [usbiepbctx_6::R](usbiepbctx_6::R) reader structure"]
impl crate::Readable for USBIEPBCTX_6 {}
#[doc = "`write(|w| ..)` method takes [usbiepbctx_6::W](usbiepbctx_6::W) writer structure"]
impl crate::Writable for USBIEPBCTX_6 {}
#[doc = "Input Endpoint_6: X-byte count"]
pub mod usbiepbctx_6;
#[doc = "Input Endpoint_6: Y-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbbay_6](usbiepbbay_6) module"]
pub type USBIEPBBAY_6 = crate::Reg<u8, _USBIEPBBAY_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBBAY_6;
#[doc = "`read()` method returns [usbiepbbay_6::R](usbiepbbay_6::R) reader structure"]
impl crate::Readable for USBIEPBBAY_6 {}
#[doc = "`write(|w| ..)` method takes [usbiepbbay_6::W](usbiepbbay_6::W) writer structure"]
impl crate::Writable for USBIEPBBAY_6 {}
#[doc = "Input Endpoint_6: Y-buffer base addr."]
pub mod usbiepbbay_6;
#[doc = "Input Endpoint_6: Y-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbcty_6](usbiepbcty_6) module"]
pub type USBIEPBCTY_6 = crate::Reg<u8, _USBIEPBCTY_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBCTY_6;
#[doc = "`read()` method returns [usbiepbcty_6::R](usbiepbcty_6::R) reader structure"]
impl crate::Readable for USBIEPBCTY_6 {}
#[doc = "`write(|w| ..)` method takes [usbiepbcty_6::W](usbiepbcty_6::W) writer structure"]
impl crate::Writable for USBIEPBCTY_6 {}
#[doc = "Input Endpoint_6: Y-byte count"]
pub mod usbiepbcty_6;
#[doc = "Input Endpoint_6: X/Y-buffer size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepsizxy_6](usbiepsizxy_6) module"]
pub type USBIEPSIZXY_6 = crate::Reg<u8, _USBIEPSIZXY_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPSIZXY_6;
#[doc = "`read()` method returns [usbiepsizxy_6::R](usbiepsizxy_6::R) reader structure"]
impl crate::Readable for USBIEPSIZXY_6 {}
#[doc = "`write(|w| ..)` method takes [usbiepsizxy_6::W](usbiepsizxy_6::W) writer structure"]
impl crate::Writable for USBIEPSIZXY_6 {}
#[doc = "Input Endpoint_6: X/Y-buffer size"]
pub mod usbiepsizxy_6;
#[doc = "Input Endpoint_7: Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepcnf_7](usbiepcnf_7) module"]
pub type USBIEPCNF_7 = crate::Reg<u8, _USBIEPCNF_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPCNF_7;
#[doc = "`read()` method returns [usbiepcnf_7::R](usbiepcnf_7::R) reader structure"]
impl crate::Readable for USBIEPCNF_7 {}
#[doc = "`write(|w| ..)` method takes [usbiepcnf_7::W](usbiepcnf_7::W) writer structure"]
impl crate::Writable for USBIEPCNF_7 {}
#[doc = "Input Endpoint_7: Configuration"]
pub mod usbiepcnf_7;
#[doc = "Input Endpoint_7: X-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbbax_7](usbiepbbax_7) module"]
pub type USBIEPBBAX_7 = crate::Reg<u8, _USBIEPBBAX_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBBAX_7;
#[doc = "`read()` method returns [usbiepbbax_7::R](usbiepbbax_7::R) reader structure"]
impl crate::Readable for USBIEPBBAX_7 {}
#[doc = "`write(|w| ..)` method takes [usbiepbbax_7::W](usbiepbbax_7::W) writer structure"]
impl crate::Writable for USBIEPBBAX_7 {}
#[doc = "Input Endpoint_7: X-buffer base addr."]
pub mod usbiepbbax_7;
#[doc = "Input Endpoint_7: X-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbctx_7](usbiepbctx_7) module"]
pub type USBIEPBCTX_7 = crate::Reg<u8, _USBIEPBCTX_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBCTX_7;
#[doc = "`read()` method returns [usbiepbctx_7::R](usbiepbctx_7::R) reader structure"]
impl crate::Readable for USBIEPBCTX_7 {}
#[doc = "`write(|w| ..)` method takes [usbiepbctx_7::W](usbiepbctx_7::W) writer structure"]
impl crate::Writable for USBIEPBCTX_7 {}
#[doc = "Input Endpoint_7: X-byte count"]
pub mod usbiepbctx_7;
#[doc = "Input Endpoint_7: Y-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbbay_7](usbiepbbay_7) module"]
pub type USBIEPBBAY_7 = crate::Reg<u8, _USBIEPBBAY_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBBAY_7;
#[doc = "`read()` method returns [usbiepbbay_7::R](usbiepbbay_7::R) reader structure"]
impl crate::Readable for USBIEPBBAY_7 {}
#[doc = "`write(|w| ..)` method takes [usbiepbbay_7::W](usbiepbbay_7::W) writer structure"]
impl crate::Writable for USBIEPBBAY_7 {}
#[doc = "Input Endpoint_7: Y-buffer base addr."]
pub mod usbiepbbay_7;
#[doc = "Input Endpoint_7: Y-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbcty_7](usbiepbcty_7) module"]
pub type USBIEPBCTY_7 = crate::Reg<u8, _USBIEPBCTY_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPBCTY_7;
#[doc = "`read()` method returns [usbiepbcty_7::R](usbiepbcty_7::R) reader structure"]
impl crate::Readable for USBIEPBCTY_7 {}
#[doc = "`write(|w| ..)` method takes [usbiepbcty_7::W](usbiepbcty_7::W) writer structure"]
impl crate::Writable for USBIEPBCTY_7 {}
#[doc = "Input Endpoint_7: Y-byte count"]
pub mod usbiepbcty_7;
#[doc = "Input Endpoint_7: X/Y-buffer size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepsizxy_7](usbiepsizxy_7) module"]
pub type USBIEPSIZXY_7 = crate::Reg<u8, _USBIEPSIZXY_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPSIZXY_7;
#[doc = "`read()` method returns [usbiepsizxy_7::R](usbiepsizxy_7::R) reader structure"]
impl crate::Readable for USBIEPSIZXY_7 {}
#[doc = "`write(|w| ..)` method takes [usbiepsizxy_7::W](usbiepsizxy_7::W) writer structure"]
impl crate::Writable for USBIEPSIZXY_7 {}
#[doc = "Input Endpoint_7: X/Y-buffer size"]
pub mod usbiepsizxy_7;
