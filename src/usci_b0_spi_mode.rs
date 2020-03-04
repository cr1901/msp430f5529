#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI B0 Control Register 1"]
    pub ucb0ctl1_spi: UCB0CTL1_SPI,
    #[doc = "0x01 - USCI B0 Control Register 0"]
    pub ucb0ctl0_spi: UCB0CTL0_SPI,
    _reserved2: [u8; 4usize],
    #[doc = "0x06 - USCI B0 Baud Rate 0"]
    pub ucb0br0_spi: UCB0BR0_SPI,
    #[doc = "0x07 - USCI B0 Baud Rate 1"]
    pub ucb0br1_spi: UCB0BR1_SPI,
    _reserved4: [u8; 2usize],
    #[doc = "0x0a - USCI B0 Status Register"]
    pub ucb0stat_spi: UCB0STAT_SPI,
    _reserved5: [u8; 1usize],
    #[doc = "0x0c - USCI B0 Receive Buffer"]
    pub ucb0rxbuf_spi: UCB0RXBUF_SPI,
    _reserved6: [u8; 1usize],
    #[doc = "0x0e - USCI B0 Transmit Buffer"]
    pub ucb0txbuf_spi: UCB0TXBUF_SPI,
    _reserved7: [u8; 13usize],
    #[doc = "0x1c - USCI B0 Interrupt Enable Register"]
    pub ucb0ie_spi: UCB0IE_SPI,
    #[doc = "0x1d - USCI B0 Interrupt Flags Register"]
    pub ucb0ifg_spi: UCB0IFG_SPI,
    #[doc = "0x1e - USCI B0 Interrupt Vector Register"]
    pub ucb0iv_spi: UCB0IV_SPI,
}
#[doc = "USCI B0 Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ctl1_spi](ucb0ctl1_spi) module"]
pub type UCB0CTL1_SPI = crate::Reg<u8, _UCB0CTL1_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0CTL1_SPI;
#[doc = "`read()` method returns [ucb0ctl1_spi::R](ucb0ctl1_spi::R) reader structure"]
impl crate::Readable for UCB0CTL1_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb0ctl1_spi::W](ucb0ctl1_spi::W) writer structure"]
impl crate::Writable for UCB0CTL1_SPI {}
#[doc = "USCI B0 Control Register 1"]
pub mod ucb0ctl1_spi;
#[doc = "USCI B0 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ctl0_spi](ucb0ctl0_spi) module"]
pub type UCB0CTL0_SPI = crate::Reg<u8, _UCB0CTL0_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0CTL0_SPI;
#[doc = "`read()` method returns [ucb0ctl0_spi::R](ucb0ctl0_spi::R) reader structure"]
impl crate::Readable for UCB0CTL0_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb0ctl0_spi::W](ucb0ctl0_spi::W) writer structure"]
impl crate::Writable for UCB0CTL0_SPI {}
#[doc = "USCI B0 Control Register 0"]
pub mod ucb0ctl0_spi;
#[doc = "USCI B0 Baud Rate 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0br0_spi](ucb0br0_spi) module"]
pub type UCB0BR0_SPI = crate::Reg<u8, _UCB0BR0_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0BR0_SPI;
#[doc = "`read()` method returns [ucb0br0_spi::R](ucb0br0_spi::R) reader structure"]
impl crate::Readable for UCB0BR0_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb0br0_spi::W](ucb0br0_spi::W) writer structure"]
impl crate::Writable for UCB0BR0_SPI {}
#[doc = "USCI B0 Baud Rate 0"]
pub mod ucb0br0_spi;
#[doc = "USCI B0 Baud Rate 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0br1_spi](ucb0br1_spi) module"]
pub type UCB0BR1_SPI = crate::Reg<u8, _UCB0BR1_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0BR1_SPI;
#[doc = "`read()` method returns [ucb0br1_spi::R](ucb0br1_spi::R) reader structure"]
impl crate::Readable for UCB0BR1_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb0br1_spi::W](ucb0br1_spi::W) writer structure"]
impl crate::Writable for UCB0BR1_SPI {}
#[doc = "USCI B0 Baud Rate 1"]
pub mod ucb0br1_spi;
#[doc = "USCI B0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0stat_spi](ucb0stat_spi) module"]
pub type UCB0STAT_SPI = crate::Reg<u8, _UCB0STAT_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0STAT_SPI;
#[doc = "`read()` method returns [ucb0stat_spi::R](ucb0stat_spi::R) reader structure"]
impl crate::Readable for UCB0STAT_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb0stat_spi::W](ucb0stat_spi::W) writer structure"]
impl crate::Writable for UCB0STAT_SPI {}
#[doc = "USCI B0 Status Register"]
pub mod ucb0stat_spi;
#[doc = "USCI B0 Receive Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0rxbuf_spi](ucb0rxbuf_spi) module"]
pub type UCB0RXBUF_SPI = crate::Reg<u8, _UCB0RXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0RXBUF_SPI;
#[doc = "`read()` method returns [ucb0rxbuf_spi::R](ucb0rxbuf_spi::R) reader structure"]
impl crate::Readable for UCB0RXBUF_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb0rxbuf_spi::W](ucb0rxbuf_spi::W) writer structure"]
impl crate::Writable for UCB0RXBUF_SPI {}
#[doc = "USCI B0 Receive Buffer"]
pub mod ucb0rxbuf_spi;
#[doc = "USCI B0 Transmit Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0txbuf_spi](ucb0txbuf_spi) module"]
pub type UCB0TXBUF_SPI = crate::Reg<u8, _UCB0TXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0TXBUF_SPI;
#[doc = "`read()` method returns [ucb0txbuf_spi::R](ucb0txbuf_spi::R) reader structure"]
impl crate::Readable for UCB0TXBUF_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb0txbuf_spi::W](ucb0txbuf_spi::W) writer structure"]
impl crate::Writable for UCB0TXBUF_SPI {}
#[doc = "USCI B0 Transmit Buffer"]
pub mod ucb0txbuf_spi;
#[doc = "USCI B0 Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ie_spi](ucb0ie_spi) module"]
pub type UCB0IE_SPI = crate::Reg<u8, _UCB0IE_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0IE_SPI;
#[doc = "`read()` method returns [ucb0ie_spi::R](ucb0ie_spi::R) reader structure"]
impl crate::Readable for UCB0IE_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb0ie_spi::W](ucb0ie_spi::W) writer structure"]
impl crate::Writable for UCB0IE_SPI {}
#[doc = "USCI B0 Interrupt Enable Register"]
pub mod ucb0ie_spi;
#[doc = "USCI B0 Interrupt Flags Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ifg_spi](ucb0ifg_spi) module"]
pub type UCB0IFG_SPI = crate::Reg<u8, _UCB0IFG_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0IFG_SPI;
#[doc = "`read()` method returns [ucb0ifg_spi::R](ucb0ifg_spi::R) reader structure"]
impl crate::Readable for UCB0IFG_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb0ifg_spi::W](ucb0ifg_spi::W) writer structure"]
impl crate::Writable for UCB0IFG_SPI {}
#[doc = "USCI B0 Interrupt Flags Register"]
pub mod ucb0ifg_spi;
#[doc = "USCI B0 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0iv_spi](ucb0iv_spi) module"]
pub type UCB0IV_SPI = crate::Reg<u16, _UCB0IV_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0IV_SPI;
#[doc = "`read()` method returns [ucb0iv_spi::R](ucb0iv_spi::R) reader structure"]
impl crate::Readable for UCB0IV_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb0iv_spi::W](ucb0iv_spi::W) writer structure"]
impl crate::Writable for UCB0IV_SPI {}
#[doc = "USCI B0 Interrupt Vector Register"]
pub mod ucb0iv_spi;
