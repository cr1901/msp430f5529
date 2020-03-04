#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI A1 Control Register 1"]
    pub uca1ctl1_spi: UCA1CTL1_SPI,
    #[doc = "0x01 - USCI A1 Control Register 0"]
    pub uca1ctl0_spi: UCA1CTL0_SPI,
    _reserved2: [u8; 4usize],
    #[doc = "0x06 - USCI A1 Baud Rate 0"]
    pub uca1br0_spi: UCA1BR0_SPI,
    #[doc = "0x07 - USCI A1 Baud Rate 1"]
    pub uca1br1_spi: UCA1BR1_SPI,
    #[doc = "0x08 - USCI A1 Modulation Control"]
    pub uca1mctl_spi: UCA1MCTL_SPI,
    _reserved5: [u8; 1usize],
    #[doc = "0x0a - USCI A1 Status Register"]
    pub uca1stat_spi: UCA1STAT_SPI,
    _reserved6: [u8; 1usize],
    #[doc = "0x0c - USCI A1 Receive Buffer"]
    pub uca1rxbuf_spi: UCA1RXBUF_SPI,
    _reserved7: [u8; 1usize],
    #[doc = "0x0e - USCI A1 Transmit Buffer"]
    pub uca1txbuf_spi: UCA1TXBUF_SPI,
    _reserved8: [u8; 13usize],
    #[doc = "0x1c - USCI A1 Interrupt Enable Register"]
    pub uca1ie_spi: UCA1IE_SPI,
    #[doc = "0x1d - USCI A1 Interrupt Flags Register"]
    pub uca1ifg_spi: UCA1IFG_SPI,
    #[doc = "0x1e - USCI A1 Interrupt Vector Register"]
    pub uca1iv_spi: UCA1IV_SPI,
}
#[doc = "USCI A1 Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ctl1_spi](uca1ctl1_spi) module"]
pub type UCA1CTL1_SPI = crate::Reg<u8, _UCA1CTL1_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1CTL1_SPI;
#[doc = "`read()` method returns [uca1ctl1_spi::R](uca1ctl1_spi::R) reader structure"]
impl crate::Readable for UCA1CTL1_SPI {}
#[doc = "`write(|w| ..)` method takes [uca1ctl1_spi::W](uca1ctl1_spi::W) writer structure"]
impl crate::Writable for UCA1CTL1_SPI {}
#[doc = "USCI A1 Control Register 1"]
pub mod uca1ctl1_spi;
#[doc = "USCI A1 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ctl0_spi](uca1ctl0_spi) module"]
pub type UCA1CTL0_SPI = crate::Reg<u8, _UCA1CTL0_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1CTL0_SPI;
#[doc = "`read()` method returns [uca1ctl0_spi::R](uca1ctl0_spi::R) reader structure"]
impl crate::Readable for UCA1CTL0_SPI {}
#[doc = "`write(|w| ..)` method takes [uca1ctl0_spi::W](uca1ctl0_spi::W) writer structure"]
impl crate::Writable for UCA1CTL0_SPI {}
#[doc = "USCI A1 Control Register 0"]
pub mod uca1ctl0_spi;
#[doc = "USCI A1 Baud Rate 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1br0_spi](uca1br0_spi) module"]
pub type UCA1BR0_SPI = crate::Reg<u8, _UCA1BR0_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1BR0_SPI;
#[doc = "`read()` method returns [uca1br0_spi::R](uca1br0_spi::R) reader structure"]
impl crate::Readable for UCA1BR0_SPI {}
#[doc = "`write(|w| ..)` method takes [uca1br0_spi::W](uca1br0_spi::W) writer structure"]
impl crate::Writable for UCA1BR0_SPI {}
#[doc = "USCI A1 Baud Rate 0"]
pub mod uca1br0_spi;
#[doc = "USCI A1 Baud Rate 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1br1_spi](uca1br1_spi) module"]
pub type UCA1BR1_SPI = crate::Reg<u8, _UCA1BR1_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1BR1_SPI;
#[doc = "`read()` method returns [uca1br1_spi::R](uca1br1_spi::R) reader structure"]
impl crate::Readable for UCA1BR1_SPI {}
#[doc = "`write(|w| ..)` method takes [uca1br1_spi::W](uca1br1_spi::W) writer structure"]
impl crate::Writable for UCA1BR1_SPI {}
#[doc = "USCI A1 Baud Rate 1"]
pub mod uca1br1_spi;
#[doc = "USCI A1 Modulation Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1mctl_spi](uca1mctl_spi) module"]
pub type UCA1MCTL_SPI = crate::Reg<u8, _UCA1MCTL_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1MCTL_SPI;
#[doc = "`read()` method returns [uca1mctl_spi::R](uca1mctl_spi::R) reader structure"]
impl crate::Readable for UCA1MCTL_SPI {}
#[doc = "`write(|w| ..)` method takes [uca1mctl_spi::W](uca1mctl_spi::W) writer structure"]
impl crate::Writable for UCA1MCTL_SPI {}
#[doc = "USCI A1 Modulation Control"]
pub mod uca1mctl_spi;
#[doc = "USCI A1 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1stat_spi](uca1stat_spi) module"]
pub type UCA1STAT_SPI = crate::Reg<u8, _UCA1STAT_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1STAT_SPI;
#[doc = "`read()` method returns [uca1stat_spi::R](uca1stat_spi::R) reader structure"]
impl crate::Readable for UCA1STAT_SPI {}
#[doc = "`write(|w| ..)` method takes [uca1stat_spi::W](uca1stat_spi::W) writer structure"]
impl crate::Writable for UCA1STAT_SPI {}
#[doc = "USCI A1 Status Register"]
pub mod uca1stat_spi;
#[doc = "USCI A1 Receive Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1rxbuf_spi](uca1rxbuf_spi) module"]
pub type UCA1RXBUF_SPI = crate::Reg<u8, _UCA1RXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1RXBUF_SPI;
#[doc = "`read()` method returns [uca1rxbuf_spi::R](uca1rxbuf_spi::R) reader structure"]
impl crate::Readable for UCA1RXBUF_SPI {}
#[doc = "`write(|w| ..)` method takes [uca1rxbuf_spi::W](uca1rxbuf_spi::W) writer structure"]
impl crate::Writable for UCA1RXBUF_SPI {}
#[doc = "USCI A1 Receive Buffer"]
pub mod uca1rxbuf_spi;
#[doc = "USCI A1 Transmit Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1txbuf_spi](uca1txbuf_spi) module"]
pub type UCA1TXBUF_SPI = crate::Reg<u8, _UCA1TXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1TXBUF_SPI;
#[doc = "`read()` method returns [uca1txbuf_spi::R](uca1txbuf_spi::R) reader structure"]
impl crate::Readable for UCA1TXBUF_SPI {}
#[doc = "`write(|w| ..)` method takes [uca1txbuf_spi::W](uca1txbuf_spi::W) writer structure"]
impl crate::Writable for UCA1TXBUF_SPI {}
#[doc = "USCI A1 Transmit Buffer"]
pub mod uca1txbuf_spi;
#[doc = "USCI A1 Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ie_spi](uca1ie_spi) module"]
pub type UCA1IE_SPI = crate::Reg<u8, _UCA1IE_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IE_SPI;
#[doc = "`read()` method returns [uca1ie_spi::R](uca1ie_spi::R) reader structure"]
impl crate::Readable for UCA1IE_SPI {}
#[doc = "`write(|w| ..)` method takes [uca1ie_spi::W](uca1ie_spi::W) writer structure"]
impl crate::Writable for UCA1IE_SPI {}
#[doc = "USCI A1 Interrupt Enable Register"]
pub mod uca1ie_spi;
#[doc = "USCI A1 Interrupt Flags Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ifg_spi](uca1ifg_spi) module"]
pub type UCA1IFG_SPI = crate::Reg<u8, _UCA1IFG_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IFG_SPI;
#[doc = "`read()` method returns [uca1ifg_spi::R](uca1ifg_spi::R) reader structure"]
impl crate::Readable for UCA1IFG_SPI {}
#[doc = "`write(|w| ..)` method takes [uca1ifg_spi::W](uca1ifg_spi::W) writer structure"]
impl crate::Writable for UCA1IFG_SPI {}
#[doc = "USCI A1 Interrupt Flags Register"]
pub mod uca1ifg_spi;
#[doc = "USCI A1 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1iv_spi](uca1iv_spi) module"]
pub type UCA1IV_SPI = crate::Reg<u16, _UCA1IV_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IV_SPI;
#[doc = "`read()` method returns [uca1iv_spi::R](uca1iv_spi::R) reader structure"]
impl crate::Readable for UCA1IV_SPI {}
#[doc = "`write(|w| ..)` method takes [uca1iv_spi::W](uca1iv_spi::W) writer structure"]
impl crate::Writable for UCA1IV_SPI {}
#[doc = "USCI A1 Interrupt Vector Register"]
pub mod uca1iv_spi;
