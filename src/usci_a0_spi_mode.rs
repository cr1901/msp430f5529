#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI A0 Control Register 1"]
    pub uca0ctl1_spi: UCA0CTL1_SPI,
    #[doc = "0x01 - USCI A0 Control Register 0"]
    pub uca0ctl0_spi: UCA0CTL0_SPI,
    _reserved2: [u8; 4usize],
    #[doc = "0x06 - USCI A0 Baud Rate 0"]
    pub uca0br0_spi: UCA0BR0_SPI,
    #[doc = "0x07 - USCI A0 Baud Rate 1"]
    pub uca0br1_spi: UCA0BR1_SPI,
    #[doc = "0x08 - USCI A0 Modulation Control"]
    pub uca0mctl_spi: UCA0MCTL_SPI,
    _reserved5: [u8; 1usize],
    #[doc = "0x0a - USCI A0 Status Register"]
    pub uca0stat_spi: UCA0STAT_SPI,
    _reserved6: [u8; 1usize],
    #[doc = "0x0c - USCI A0 Receive Buffer"]
    pub uca0rxbuf_spi: UCA0RXBUF_SPI,
    _reserved7: [u8; 1usize],
    #[doc = "0x0e - USCI A0 Transmit Buffer"]
    pub uca0txbuf_spi: UCA0TXBUF_SPI,
    _reserved8: [u8; 13usize],
    #[doc = "0x1c - USCI A0 Interrupt Enable Register"]
    pub uca0ie_spi: UCA0IE_SPI,
    #[doc = "0x1d - USCI A0 Interrupt Flags Register"]
    pub uca0ifg_spi: UCA0IFG_SPI,
    #[doc = "0x1e - USCI A0 Interrupt Vector Register"]
    pub uca0iv_spi: UCA0IV_SPI,
}
#[doc = "USCI A0 Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0ctl1_spi](uca0ctl1_spi) module"]
pub type UCA0CTL1_SPI = crate::Reg<u8, _UCA0CTL1_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0CTL1_SPI;
#[doc = "`read()` method returns [uca0ctl1_spi::R](uca0ctl1_spi::R) reader structure"]
impl crate::Readable for UCA0CTL1_SPI {}
#[doc = "`write(|w| ..)` method takes [uca0ctl1_spi::W](uca0ctl1_spi::W) writer structure"]
impl crate::Writable for UCA0CTL1_SPI {}
#[doc = "USCI A0 Control Register 1"]
pub mod uca0ctl1_spi;
#[doc = "USCI A0 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0ctl0_spi](uca0ctl0_spi) module"]
pub type UCA0CTL0_SPI = crate::Reg<u8, _UCA0CTL0_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0CTL0_SPI;
#[doc = "`read()` method returns [uca0ctl0_spi::R](uca0ctl0_spi::R) reader structure"]
impl crate::Readable for UCA0CTL0_SPI {}
#[doc = "`write(|w| ..)` method takes [uca0ctl0_spi::W](uca0ctl0_spi::W) writer structure"]
impl crate::Writable for UCA0CTL0_SPI {}
#[doc = "USCI A0 Control Register 0"]
pub mod uca0ctl0_spi;
#[doc = "USCI A0 Baud Rate 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0br0_spi](uca0br0_spi) module"]
pub type UCA0BR0_SPI = crate::Reg<u8, _UCA0BR0_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0BR0_SPI;
#[doc = "`read()` method returns [uca0br0_spi::R](uca0br0_spi::R) reader structure"]
impl crate::Readable for UCA0BR0_SPI {}
#[doc = "`write(|w| ..)` method takes [uca0br0_spi::W](uca0br0_spi::W) writer structure"]
impl crate::Writable for UCA0BR0_SPI {}
#[doc = "USCI A0 Baud Rate 0"]
pub mod uca0br0_spi;
#[doc = "USCI A0 Baud Rate 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0br1_spi](uca0br1_spi) module"]
pub type UCA0BR1_SPI = crate::Reg<u8, _UCA0BR1_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0BR1_SPI;
#[doc = "`read()` method returns [uca0br1_spi::R](uca0br1_spi::R) reader structure"]
impl crate::Readable for UCA0BR1_SPI {}
#[doc = "`write(|w| ..)` method takes [uca0br1_spi::W](uca0br1_spi::W) writer structure"]
impl crate::Writable for UCA0BR1_SPI {}
#[doc = "USCI A0 Baud Rate 1"]
pub mod uca0br1_spi;
#[doc = "USCI A0 Modulation Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0mctl_spi](uca0mctl_spi) module"]
pub type UCA0MCTL_SPI = crate::Reg<u8, _UCA0MCTL_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0MCTL_SPI;
#[doc = "`read()` method returns [uca0mctl_spi::R](uca0mctl_spi::R) reader structure"]
impl crate::Readable for UCA0MCTL_SPI {}
#[doc = "`write(|w| ..)` method takes [uca0mctl_spi::W](uca0mctl_spi::W) writer structure"]
impl crate::Writable for UCA0MCTL_SPI {}
#[doc = "USCI A0 Modulation Control"]
pub mod uca0mctl_spi;
#[doc = "USCI A0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0stat_spi](uca0stat_spi) module"]
pub type UCA0STAT_SPI = crate::Reg<u8, _UCA0STAT_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0STAT_SPI;
#[doc = "`read()` method returns [uca0stat_spi::R](uca0stat_spi::R) reader structure"]
impl crate::Readable for UCA0STAT_SPI {}
#[doc = "`write(|w| ..)` method takes [uca0stat_spi::W](uca0stat_spi::W) writer structure"]
impl crate::Writable for UCA0STAT_SPI {}
#[doc = "USCI A0 Status Register"]
pub mod uca0stat_spi;
#[doc = "USCI A0 Receive Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0rxbuf_spi](uca0rxbuf_spi) module"]
pub type UCA0RXBUF_SPI = crate::Reg<u8, _UCA0RXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0RXBUF_SPI;
#[doc = "`read()` method returns [uca0rxbuf_spi::R](uca0rxbuf_spi::R) reader structure"]
impl crate::Readable for UCA0RXBUF_SPI {}
#[doc = "`write(|w| ..)` method takes [uca0rxbuf_spi::W](uca0rxbuf_spi::W) writer structure"]
impl crate::Writable for UCA0RXBUF_SPI {}
#[doc = "USCI A0 Receive Buffer"]
pub mod uca0rxbuf_spi;
#[doc = "USCI A0 Transmit Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0txbuf_spi](uca0txbuf_spi) module"]
pub type UCA0TXBUF_SPI = crate::Reg<u8, _UCA0TXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0TXBUF_SPI;
#[doc = "`read()` method returns [uca0txbuf_spi::R](uca0txbuf_spi::R) reader structure"]
impl crate::Readable for UCA0TXBUF_SPI {}
#[doc = "`write(|w| ..)` method takes [uca0txbuf_spi::W](uca0txbuf_spi::W) writer structure"]
impl crate::Writable for UCA0TXBUF_SPI {}
#[doc = "USCI A0 Transmit Buffer"]
pub mod uca0txbuf_spi;
#[doc = "USCI A0 Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0ie_spi](uca0ie_spi) module"]
pub type UCA0IE_SPI = crate::Reg<u8, _UCA0IE_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0IE_SPI;
#[doc = "`read()` method returns [uca0ie_spi::R](uca0ie_spi::R) reader structure"]
impl crate::Readable for UCA0IE_SPI {}
#[doc = "`write(|w| ..)` method takes [uca0ie_spi::W](uca0ie_spi::W) writer structure"]
impl crate::Writable for UCA0IE_SPI {}
#[doc = "USCI A0 Interrupt Enable Register"]
pub mod uca0ie_spi;
#[doc = "USCI A0 Interrupt Flags Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0ifg_spi](uca0ifg_spi) module"]
pub type UCA0IFG_SPI = crate::Reg<u8, _UCA0IFG_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0IFG_SPI;
#[doc = "`read()` method returns [uca0ifg_spi::R](uca0ifg_spi::R) reader structure"]
impl crate::Readable for UCA0IFG_SPI {}
#[doc = "`write(|w| ..)` method takes [uca0ifg_spi::W](uca0ifg_spi::W) writer structure"]
impl crate::Writable for UCA0IFG_SPI {}
#[doc = "USCI A0 Interrupt Flags Register"]
pub mod uca0ifg_spi;
#[doc = "USCI A0 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0iv_spi](uca0iv_spi) module"]
pub type UCA0IV_SPI = crate::Reg<u16, _UCA0IV_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0IV_SPI;
#[doc = "`read()` method returns [uca0iv_spi::R](uca0iv_spi::R) reader structure"]
impl crate::Readable for UCA0IV_SPI {}
#[doc = "`write(|w| ..)` method takes [uca0iv_spi::W](uca0iv_spi::W) writer structure"]
impl crate::Writable for UCA0IV_SPI {}
#[doc = "USCI A0 Interrupt Vector Register"]
pub mod uca0iv_spi;
