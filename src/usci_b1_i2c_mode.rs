#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI B1 Control Register 1"]
    pub ucb1ctl1: UCB1CTL1,
    #[doc = "0x01 - USCI B1 Control Register 0"]
    pub ucb1ctl0: UCB1CTL0,
    _reserved2: [u8; 4usize],
    #[doc = "0x06 - USCI B1 Baud Rate 0"]
    pub ucb1br0: UCB1BR0,
    #[doc = "0x07 - USCI B1 Baud Rate 1"]
    pub ucb1br1: UCB1BR1,
    _reserved4: [u8; 2usize],
    #[doc = "0x0a - USCI B1 Status Register"]
    pub ucb1stat: UCB1STAT,
    _reserved5: [u8; 1usize],
    #[doc = "0x0c - USCI B1 Receive Buffer"]
    pub ucb1rxbuf: UCB1RXBUF,
    _reserved6: [u8; 1usize],
    #[doc = "0x0e - USCI B1 Transmit Buffer"]
    pub ucb1txbuf: UCB1TXBUF,
    _reserved7: [u8; 1usize],
    #[doc = "0x10 - USCI B1 I2C Own Address"]
    pub ucb1i2coa: UCB1I2COA,
    #[doc = "0x12 - USCI B1 I2C Slave Address"]
    pub ucb1i2csa: UCB1I2CSA,
    _reserved9: [u8; 8usize],
    #[doc = "0x1c - USCI B1 Interrupt Enable Register"]
    pub ucb1ie: UCB1IE,
    #[doc = "0x1d - USCI B1 Interrupt Flags Register"]
    pub ucb1ifg: UCB1IFG,
    #[doc = "0x1e - USCI B1 Interrupt Vector Register"]
    pub ucb1iv: UCB1IV,
}
#[doc = "USCI B1 Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ctl1](ucb1ctl1) module"]
pub type UCB1CTL1 = crate::Reg<u8, _UCB1CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1CTL1;
#[doc = "`read()` method returns [ucb1ctl1::R](ucb1ctl1::R) reader structure"]
impl crate::Readable for UCB1CTL1 {}
#[doc = "`write(|w| ..)` method takes [ucb1ctl1::W](ucb1ctl1::W) writer structure"]
impl crate::Writable for UCB1CTL1 {}
#[doc = "USCI B1 Control Register 1"]
pub mod ucb1ctl1;
#[doc = "USCI B1 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ctl0](ucb1ctl0) module"]
pub type UCB1CTL0 = crate::Reg<u8, _UCB1CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1CTL0;
#[doc = "`read()` method returns [ucb1ctl0::R](ucb1ctl0::R) reader structure"]
impl crate::Readable for UCB1CTL0 {}
#[doc = "`write(|w| ..)` method takes [ucb1ctl0::W](ucb1ctl0::W) writer structure"]
impl crate::Writable for UCB1CTL0 {}
#[doc = "USCI B1 Control Register 0"]
pub mod ucb1ctl0;
#[doc = "USCI B1 Baud Rate 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1br0](ucb1br0) module"]
pub type UCB1BR0 = crate::Reg<u8, _UCB1BR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1BR0;
#[doc = "`read()` method returns [ucb1br0::R](ucb1br0::R) reader structure"]
impl crate::Readable for UCB1BR0 {}
#[doc = "`write(|w| ..)` method takes [ucb1br0::W](ucb1br0::W) writer structure"]
impl crate::Writable for UCB1BR0 {}
#[doc = "USCI B1 Baud Rate 0"]
pub mod ucb1br0;
#[doc = "USCI B1 Baud Rate 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1br1](ucb1br1) module"]
pub type UCB1BR1 = crate::Reg<u8, _UCB1BR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1BR1;
#[doc = "`read()` method returns [ucb1br1::R](ucb1br1::R) reader structure"]
impl crate::Readable for UCB1BR1 {}
#[doc = "`write(|w| ..)` method takes [ucb1br1::W](ucb1br1::W) writer structure"]
impl crate::Writable for UCB1BR1 {}
#[doc = "USCI B1 Baud Rate 1"]
pub mod ucb1br1;
#[doc = "USCI B1 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1stat](ucb1stat) module"]
pub type UCB1STAT = crate::Reg<u8, _UCB1STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1STAT;
#[doc = "`read()` method returns [ucb1stat::R](ucb1stat::R) reader structure"]
impl crate::Readable for UCB1STAT {}
#[doc = "`write(|w| ..)` method takes [ucb1stat::W](ucb1stat::W) writer structure"]
impl crate::Writable for UCB1STAT {}
#[doc = "USCI B1 Status Register"]
pub mod ucb1stat;
#[doc = "USCI B1 Receive Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1rxbuf](ucb1rxbuf) module"]
pub type UCB1RXBUF = crate::Reg<u8, _UCB1RXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1RXBUF;
#[doc = "`read()` method returns [ucb1rxbuf::R](ucb1rxbuf::R) reader structure"]
impl crate::Readable for UCB1RXBUF {}
#[doc = "`write(|w| ..)` method takes [ucb1rxbuf::W](ucb1rxbuf::W) writer structure"]
impl crate::Writable for UCB1RXBUF {}
#[doc = "USCI B1 Receive Buffer"]
pub mod ucb1rxbuf;
#[doc = "USCI B1 Transmit Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1txbuf](ucb1txbuf) module"]
pub type UCB1TXBUF = crate::Reg<u8, _UCB1TXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1TXBUF;
#[doc = "`read()` method returns [ucb1txbuf::R](ucb1txbuf::R) reader structure"]
impl crate::Readable for UCB1TXBUF {}
#[doc = "`write(|w| ..)` method takes [ucb1txbuf::W](ucb1txbuf::W) writer structure"]
impl crate::Writable for UCB1TXBUF {}
#[doc = "USCI B1 Transmit Buffer"]
pub mod ucb1txbuf;
#[doc = "USCI B1 Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ie](ucb1ie) module"]
pub type UCB1IE = crate::Reg<u8, _UCB1IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1IE;
#[doc = "`read()` method returns [ucb1ie::R](ucb1ie::R) reader structure"]
impl crate::Readable for UCB1IE {}
#[doc = "`write(|w| ..)` method takes [ucb1ie::W](ucb1ie::W) writer structure"]
impl crate::Writable for UCB1IE {}
#[doc = "USCI B1 Interrupt Enable Register"]
pub mod ucb1ie;
#[doc = "USCI B1 Interrupt Flags Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ifg](ucb1ifg) module"]
pub type UCB1IFG = crate::Reg<u8, _UCB1IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1IFG;
#[doc = "`read()` method returns [ucb1ifg::R](ucb1ifg::R) reader structure"]
impl crate::Readable for UCB1IFG {}
#[doc = "`write(|w| ..)` method takes [ucb1ifg::W](ucb1ifg::W) writer structure"]
impl crate::Writable for UCB1IFG {}
#[doc = "USCI B1 Interrupt Flags Register"]
pub mod ucb1ifg;
#[doc = "USCI B1 I2C Own Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1i2coa](ucb1i2coa) module"]
pub type UCB1I2COA = crate::Reg<u16, _UCB1I2COA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1I2COA;
#[doc = "`read()` method returns [ucb1i2coa::R](ucb1i2coa::R) reader structure"]
impl crate::Readable for UCB1I2COA {}
#[doc = "`write(|w| ..)` method takes [ucb1i2coa::W](ucb1i2coa::W) writer structure"]
impl crate::Writable for UCB1I2COA {}
#[doc = "USCI B1 I2C Own Address"]
pub mod ucb1i2coa;
#[doc = "USCI B1 I2C Slave Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1i2csa](ucb1i2csa) module"]
pub type UCB1I2CSA = crate::Reg<u16, _UCB1I2CSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1I2CSA;
#[doc = "`read()` method returns [ucb1i2csa::R](ucb1i2csa::R) reader structure"]
impl crate::Readable for UCB1I2CSA {}
#[doc = "`write(|w| ..)` method takes [ucb1i2csa::W](ucb1i2csa::W) writer structure"]
impl crate::Writable for UCB1I2CSA {}
#[doc = "USCI B1 I2C Slave Address"]
pub mod ucb1i2csa;
#[doc = "USCI B1 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1iv](ucb1iv) module"]
pub type UCB1IV = crate::Reg<u16, _UCB1IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1IV;
#[doc = "`read()` method returns [ucb1iv::R](ucb1iv::R) reader structure"]
impl crate::Readable for UCB1IV {}
#[doc = "`write(|w| ..)` method takes [ucb1iv::W](ucb1iv::W) writer structure"]
impl crate::Writable for UCB1IV {}
#[doc = "USCI B1 Interrupt Vector Register"]
pub mod ucb1iv;
