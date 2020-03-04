#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Module Control 0"]
    pub dmactl0: DMACTL0,
    #[doc = "0x02 - DMA Module Control 1"]
    pub dmactl1: DMACTL1,
    #[doc = "0x04 - DMA Module Control 2"]
    pub dmactl2: DMACTL2,
    #[doc = "0x06 - DMA Module Control 3"]
    pub dmactl3: DMACTL3,
    #[doc = "0x08 - DMA Module Control 4"]
    pub dmactl4: DMACTL4,
    _reserved5: [u8; 4usize],
    #[doc = "0x0e - DMA Interrupt Vector Word"]
    pub dmaiv: DMAIV,
    #[doc = "0x10 - DMA Channel 0 Control"]
    pub dma0ctl: DMA0CTL,
    #[doc = "0x12 - DMA Channel 0 Source Address"]
    pub dma0sa: DMA0SA,
    #[doc = "0x16 - DMA Channel 0 Destination Address"]
    pub dma0da: DMA0DA,
    #[doc = "0x1a - DMA Channel 0 Transfer Size"]
    pub dma0sz: DMA0SZ,
    _reserved10: [u8; 4usize],
    #[doc = "0x20 - DMA Channel 1 Control"]
    pub dma1ctl: DMA1CTL,
    #[doc = "0x22 - DMA Channel 1 Source Address"]
    pub dma1sa: DMA1SA,
    #[doc = "0x26 - DMA Channel 1 Destination Address"]
    pub dma1da: DMA1DA,
    #[doc = "0x2a - DMA Channel 1 Transfer Size"]
    pub dma1sz: DMA1SZ,
    _reserved14: [u8; 4usize],
    #[doc = "0x30 - DMA Channel 2 Control"]
    pub dma2ctl: DMA2CTL,
    #[doc = "0x32 - DMA Channel 2 Source Address"]
    pub dma2sa: DMA2SA,
    #[doc = "0x36 - DMA Channel 2 Destination Address"]
    pub dma2da: DMA2DA,
    #[doc = "0x3a - DMA Channel 2 Transfer Size"]
    pub dma2sz: DMA2SZ,
}
#[doc = "DMA Module Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl0](dmactl0) module"]
pub type DMACTL0 = crate::Reg<u16, _DMACTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL0;
#[doc = "`read()` method returns [dmactl0::R](dmactl0::R) reader structure"]
impl crate::Readable for DMACTL0 {}
#[doc = "`write(|w| ..)` method takes [dmactl0::W](dmactl0::W) writer structure"]
impl crate::Writable for DMACTL0 {}
#[doc = "DMA Module Control 0"]
pub mod dmactl0;
#[doc = "DMA Module Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl1](dmactl1) module"]
pub type DMACTL1 = crate::Reg<u16, _DMACTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL1;
#[doc = "`read()` method returns [dmactl1::R](dmactl1::R) reader structure"]
impl crate::Readable for DMACTL1 {}
#[doc = "`write(|w| ..)` method takes [dmactl1::W](dmactl1::W) writer structure"]
impl crate::Writable for DMACTL1 {}
#[doc = "DMA Module Control 1"]
pub mod dmactl1;
#[doc = "DMA Module Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl2](dmactl2) module"]
pub type DMACTL2 = crate::Reg<u16, _DMACTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL2;
#[doc = "`read()` method returns [dmactl2::R](dmactl2::R) reader structure"]
impl crate::Readable for DMACTL2 {}
#[doc = "`write(|w| ..)` method takes [dmactl2::W](dmactl2::W) writer structure"]
impl crate::Writable for DMACTL2 {}
#[doc = "DMA Module Control 2"]
pub mod dmactl2;
#[doc = "DMA Module Control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl3](dmactl3) module"]
pub type DMACTL3 = crate::Reg<u16, _DMACTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL3;
#[doc = "`read()` method returns [dmactl3::R](dmactl3::R) reader structure"]
impl crate::Readable for DMACTL3 {}
#[doc = "`write(|w| ..)` method takes [dmactl3::W](dmactl3::W) writer structure"]
impl crate::Writable for DMACTL3 {}
#[doc = "DMA Module Control 3"]
pub mod dmactl3;
#[doc = "DMA Module Control 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl4](dmactl4) module"]
pub type DMACTL4 = crate::Reg<u16, _DMACTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL4;
#[doc = "`read()` method returns [dmactl4::R](dmactl4::R) reader structure"]
impl crate::Readable for DMACTL4 {}
#[doc = "`write(|w| ..)` method takes [dmactl4::W](dmactl4::W) writer structure"]
impl crate::Writable for DMACTL4 {}
#[doc = "DMA Module Control 4"]
pub mod dmactl4;
#[doc = "DMA Interrupt Vector Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaiv](dmaiv) module"]
pub type DMAIV = crate::Reg<u16, _DMAIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAIV;
#[doc = "`read()` method returns [dmaiv::R](dmaiv::R) reader structure"]
impl crate::Readable for DMAIV {}
#[doc = "`write(|w| ..)` method takes [dmaiv::W](dmaiv::W) writer structure"]
impl crate::Writable for DMAIV {}
#[doc = "DMA Interrupt Vector Word"]
pub mod dmaiv;
#[doc = "DMA Channel 0 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0ctl](dma0ctl) module"]
pub type DMA0CTL = crate::Reg<u16, _DMA0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA0CTL;
#[doc = "`read()` method returns [dma0ctl::R](dma0ctl::R) reader structure"]
impl crate::Readable for DMA0CTL {}
#[doc = "`write(|w| ..)` method takes [dma0ctl::W](dma0ctl::W) writer structure"]
impl crate::Writable for DMA0CTL {}
#[doc = "DMA Channel 0 Control"]
pub mod dma0ctl;
#[doc = "DMA Channel 0 Transfer Size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0sz](dma0sz) module"]
pub type DMA0SZ = crate::Reg<u16, _DMA0SZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA0SZ;
#[doc = "`read()` method returns [dma0sz::R](dma0sz::R) reader structure"]
impl crate::Readable for DMA0SZ {}
#[doc = "`write(|w| ..)` method takes [dma0sz::W](dma0sz::W) writer structure"]
impl crate::Writable for DMA0SZ {}
#[doc = "DMA Channel 0 Transfer Size"]
pub mod dma0sz;
#[doc = "DMA Channel 1 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1ctl](dma1ctl) module"]
pub type DMA1CTL = crate::Reg<u16, _DMA1CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA1CTL;
#[doc = "`read()` method returns [dma1ctl::R](dma1ctl::R) reader structure"]
impl crate::Readable for DMA1CTL {}
#[doc = "`write(|w| ..)` method takes [dma1ctl::W](dma1ctl::W) writer structure"]
impl crate::Writable for DMA1CTL {}
#[doc = "DMA Channel 1 Control"]
pub mod dma1ctl;
#[doc = "DMA Channel 1 Transfer Size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1sz](dma1sz) module"]
pub type DMA1SZ = crate::Reg<u16, _DMA1SZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA1SZ;
#[doc = "`read()` method returns [dma1sz::R](dma1sz::R) reader structure"]
impl crate::Readable for DMA1SZ {}
#[doc = "`write(|w| ..)` method takes [dma1sz::W](dma1sz::W) writer structure"]
impl crate::Writable for DMA1SZ {}
#[doc = "DMA Channel 1 Transfer Size"]
pub mod dma1sz;
#[doc = "DMA Channel 2 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2ctl](dma2ctl) module"]
pub type DMA2CTL = crate::Reg<u16, _DMA2CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2CTL;
#[doc = "`read()` method returns [dma2ctl::R](dma2ctl::R) reader structure"]
impl crate::Readable for DMA2CTL {}
#[doc = "`write(|w| ..)` method takes [dma2ctl::W](dma2ctl::W) writer structure"]
impl crate::Writable for DMA2CTL {}
#[doc = "DMA Channel 2 Control"]
pub mod dma2ctl;
#[doc = "DMA Channel 2 Transfer Size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2sz](dma2sz) module"]
pub type DMA2SZ = crate::Reg<u16, _DMA2SZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2SZ;
#[doc = "`read()` method returns [dma2sz::R](dma2sz::R) reader structure"]
impl crate::Readable for DMA2SZ {}
#[doc = "`write(|w| ..)` method takes [dma2sz::W](dma2sz::W) writer structure"]
impl crate::Writable for DMA2SZ {}
#[doc = "DMA Channel 2 Transfer Size"]
pub mod dma2sz;
#[doc = "DMA Channel 0 Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0sa](dma0sa) module"]
pub type DMA0SA = crate::Reg<u32, _DMA0SA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA0SA;
#[doc = "`read()` method returns [dma0sa::R](dma0sa::R) reader structure"]
impl crate::Readable for DMA0SA {}
#[doc = "`write(|w| ..)` method takes [dma0sa::W](dma0sa::W) writer structure"]
impl crate::Writable for DMA0SA {}
#[doc = "DMA Channel 0 Source Address"]
pub mod dma0sa;
#[doc = "DMA Channel 0 Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0da](dma0da) module"]
pub type DMA0DA = crate::Reg<u32, _DMA0DA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA0DA;
#[doc = "`read()` method returns [dma0da::R](dma0da::R) reader structure"]
impl crate::Readable for DMA0DA {}
#[doc = "`write(|w| ..)` method takes [dma0da::W](dma0da::W) writer structure"]
impl crate::Writable for DMA0DA {}
#[doc = "DMA Channel 0 Destination Address"]
pub mod dma0da;
#[doc = "DMA Channel 1 Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1sa](dma1sa) module"]
pub type DMA1SA = crate::Reg<u32, _DMA1SA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA1SA;
#[doc = "`read()` method returns [dma1sa::R](dma1sa::R) reader structure"]
impl crate::Readable for DMA1SA {}
#[doc = "`write(|w| ..)` method takes [dma1sa::W](dma1sa::W) writer structure"]
impl crate::Writable for DMA1SA {}
#[doc = "DMA Channel 1 Source Address"]
pub mod dma1sa;
#[doc = "DMA Channel 1 Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1da](dma1da) module"]
pub type DMA1DA = crate::Reg<u32, _DMA1DA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA1DA;
#[doc = "`read()` method returns [dma1da::R](dma1da::R) reader structure"]
impl crate::Readable for DMA1DA {}
#[doc = "`write(|w| ..)` method takes [dma1da::W](dma1da::W) writer structure"]
impl crate::Writable for DMA1DA {}
#[doc = "DMA Channel 1 Destination Address"]
pub mod dma1da;
#[doc = "DMA Channel 2 Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2sa](dma2sa) module"]
pub type DMA2SA = crate::Reg<u32, _DMA2SA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2SA;
#[doc = "`read()` method returns [dma2sa::R](dma2sa::R) reader structure"]
impl crate::Readable for DMA2SA {}
#[doc = "`write(|w| ..)` method takes [dma2sa::W](dma2sa::W) writer structure"]
impl crate::Writable for DMA2SA {}
#[doc = "DMA Channel 2 Source Address"]
pub mod dma2sa;
#[doc = "DMA Channel 2 Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2da](dma2da) module"]
pub type DMA2DA = crate::Reg<u32, _DMA2DA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2DA;
#[doc = "`read()` method returns [dma2da::R](dma2da::R) reader structure"]
impl crate::Readable for DMA2DA {}
#[doc = "`write(|w| ..)` method takes [dma2da::W](dma2da::W) writer structure"]
impl crate::Writable for DMA2DA {}
#[doc = "DMA Channel 2 Destination Address"]
pub mod dma2da;
