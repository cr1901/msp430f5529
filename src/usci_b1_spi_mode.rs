#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI B1 Control Register 1"]
    pub ucb1ctl1_spi: UCB1CTL1_SPI,
    #[doc = "0x01 - USCI B1 Control Register 0"]
    pub ucb1ctl0_spi: UCB1CTL0_SPI,
    _reserved2: [u8; 4usize],
    #[doc = "0x06 - USCI B1 Baud Rate 0"]
    pub ucb1br0_spi: UCB1BR0_SPI,
    #[doc = "0x07 - USCI B1 Baud Rate 1"]
    pub ucb1br1_spi: UCB1BR1_SPI,
    _reserved4: [u8; 2usize],
    #[doc = "0x0a - USCI B1 Status Register"]
    pub ucb1stat_spi: UCB1STAT_SPI,
    _reserved5: [u8; 1usize],
    #[doc = "0x0c - USCI B1 Receive Buffer"]
    pub ucb1rxbuf_spi: UCB1RXBUF_SPI,
    _reserved6: [u8; 1usize],
    #[doc = "0x0e - USCI B1 Transmit Buffer"]
    pub ucb1txbuf_spi: UCB1TXBUF_SPI,
    _reserved7: [u8; 13usize],
    #[doc = "0x1c - USCI B1 Interrupt Enable Register"]
    pub ucb1ie_spi: UCB1IE_SPI,
    #[doc = "0x1d - USCI B1 Interrupt Flags Register"]
    pub ucb1ifg_spi: UCB1IFG_SPI,
    #[doc = "0x1e - USCI B1 Interrupt Vector Register"]
    pub ucb1iv_spi: UCB1IV_SPI,
}
#[doc = "USCI B1 Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ctl1_spi](ucb1ctl1_spi) module"]
pub type UCB1CTL1_SPI = crate::Reg<u8, _UCB1CTL1_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1CTL1_SPI;
#[doc = "`read()` method returns [ucb1ctl1_spi::R](ucb1ctl1_spi::R) reader structure"]
impl crate::Readable for UCB1CTL1_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb1ctl1_spi::W](ucb1ctl1_spi::W) writer structure"]
impl crate::Writable for UCB1CTL1_SPI {}
#[doc = "USCI B1 Control Register 1"]
pub mod ucb1ctl1_spi;
#[doc = "USCI B1 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ctl0_spi](ucb1ctl0_spi) module"]
pub type UCB1CTL0_SPI = crate::Reg<u8, _UCB1CTL0_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1CTL0_SPI;
#[doc = "`read()` method returns [ucb1ctl0_spi::R](ucb1ctl0_spi::R) reader structure"]
impl crate::Readable for UCB1CTL0_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb1ctl0_spi::W](ucb1ctl0_spi::W) writer structure"]
impl crate::Writable for UCB1CTL0_SPI {}
#[doc = "USCI B1 Control Register 0"]
pub mod ucb1ctl0_spi;
#[doc = "USCI B1 Baud Rate 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1br0_spi](ucb1br0_spi) module"]
pub type UCB1BR0_SPI = crate::Reg<u8, _UCB1BR0_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1BR0_SPI;
#[doc = "`read()` method returns [ucb1br0_spi::R](ucb1br0_spi::R) reader structure"]
impl crate::Readable for UCB1BR0_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb1br0_spi::W](ucb1br0_spi::W) writer structure"]
impl crate::Writable for UCB1BR0_SPI {}
#[doc = "USCI B1 Baud Rate 0"]
pub mod ucb1br0_spi;
#[doc = "USCI B1 Baud Rate 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1br1_spi](ucb1br1_spi) module"]
pub type UCB1BR1_SPI = crate::Reg<u8, _UCB1BR1_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1BR1_SPI;
#[doc = "`read()` method returns [ucb1br1_spi::R](ucb1br1_spi::R) reader structure"]
impl crate::Readable for UCB1BR1_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb1br1_spi::W](ucb1br1_spi::W) writer structure"]
impl crate::Writable for UCB1BR1_SPI {}
#[doc = "USCI B1 Baud Rate 1"]
pub mod ucb1br1_spi;
#[doc = "USCI B1 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1stat_spi](ucb1stat_spi) module"]
pub type UCB1STAT_SPI = crate::Reg<u8, _UCB1STAT_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1STAT_SPI;
#[doc = "`read()` method returns [ucb1stat_spi::R](ucb1stat_spi::R) reader structure"]
impl crate::Readable for UCB1STAT_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb1stat_spi::W](ucb1stat_spi::W) writer structure"]
impl crate::Writable for UCB1STAT_SPI {}
#[doc = "USCI B1 Status Register"]
pub mod ucb1stat_spi;
#[doc = "USCI B1 Receive Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1rxbuf_spi](ucb1rxbuf_spi) module"]
pub type UCB1RXBUF_SPI = crate::Reg<u8, _UCB1RXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1RXBUF_SPI;
#[doc = "`read()` method returns [ucb1rxbuf_spi::R](ucb1rxbuf_spi::R) reader structure"]
impl crate::Readable for UCB1RXBUF_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb1rxbuf_spi::W](ucb1rxbuf_spi::W) writer structure"]
impl crate::Writable for UCB1RXBUF_SPI {}
#[doc = "USCI B1 Receive Buffer"]
pub mod ucb1rxbuf_spi;
#[doc = "USCI B1 Transmit Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1txbuf_spi](ucb1txbuf_spi) module"]
pub type UCB1TXBUF_SPI = crate::Reg<u8, _UCB1TXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1TXBUF_SPI;
#[doc = "`read()` method returns [ucb1txbuf_spi::R](ucb1txbuf_spi::R) reader structure"]
impl crate::Readable for UCB1TXBUF_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb1txbuf_spi::W](ucb1txbuf_spi::W) writer structure"]
impl crate::Writable for UCB1TXBUF_SPI {}
#[doc = "USCI B1 Transmit Buffer"]
pub mod ucb1txbuf_spi;
#[doc = "USCI B1 Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ie_spi](ucb1ie_spi) module"]
pub type UCB1IE_SPI = crate::Reg<u8, _UCB1IE_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1IE_SPI;
#[doc = "`read()` method returns [ucb1ie_spi::R](ucb1ie_spi::R) reader structure"]
impl crate::Readable for UCB1IE_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb1ie_spi::W](ucb1ie_spi::W) writer structure"]
impl crate::Writable for UCB1IE_SPI {}
#[doc = "USCI B1 Interrupt Enable Register"]
pub mod ucb1ie_spi;
#[doc = "USCI B1 Interrupt Flags Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ifg_spi](ucb1ifg_spi) module"]
pub type UCB1IFG_SPI = crate::Reg<u8, _UCB1IFG_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1IFG_SPI;
#[doc = "`read()` method returns [ucb1ifg_spi::R](ucb1ifg_spi::R) reader structure"]
impl crate::Readable for UCB1IFG_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb1ifg_spi::W](ucb1ifg_spi::W) writer structure"]
impl crate::Writable for UCB1IFG_SPI {}
#[doc = "USCI B1 Interrupt Flags Register"]
pub mod ucb1ifg_spi;
#[doc = "USCI B1 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1iv_spi](ucb1iv_spi) module"]
pub type UCB1IV_SPI = crate::Reg<u16, _UCB1IV_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1IV_SPI;
#[doc = "`read()` method returns [ucb1iv_spi::R](ucb1iv_spi::R) reader structure"]
impl crate::Readable for UCB1IV_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb1iv_spi::W](ucb1iv_spi::W) writer structure"]
impl crate::Writable for UCB1IV_SPI {}
#[doc = "USCI B1 Interrupt Vector Register"]
pub mod ucb1iv_spi;
