#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI A1 Control Register 1"]
    pub uca1ctl1: UCA1CTL1,
    #[doc = "0x01 - USCI A1 Control Register 0"]
    pub uca1ctl0: UCA1CTL0,
    _reserved2: [u8; 4usize],
    #[doc = "0x06 - USCI A1 Baud Rate 0"]
    pub uca1br0: UCA1BR0,
    #[doc = "0x07 - USCI A1 Baud Rate 1"]
    pub uca1br1: UCA1BR1,
    #[doc = "0x08 - USCI A1 Modulation Control"]
    pub uca1mctl: UCA1MCTL,
    _reserved5: [u8; 1usize],
    #[doc = "0x0a - USCI A1 Status Register"]
    pub uca1stat: UCA1STAT,
    _reserved6: [u8; 1usize],
    #[doc = "0x0c - USCI A1 Receive Buffer"]
    pub uca1rxbuf: UCA1RXBUF,
    _reserved7: [u8; 1usize],
    #[doc = "0x0e - USCI A1 Transmit Buffer"]
    pub uca1txbuf: UCA1TXBUF,
    _reserved8: [u8; 1usize],
    #[doc = "0x10 - USCI A1 LIN Control"]
    pub uca1abctl: UCA1ABCTL,
    _reserved9: [u8; 1usize],
    #[doc = "0x12 - USCI A1 IrDA Transmit Control"]
    pub uca1irtctl: UCA1IRTCTL,
    #[doc = "0x13 - USCI A1 IrDA Receive Control"]
    pub uca1irrctl: UCA1IRRCTL,
    _reserved11: [u8; 8usize],
    #[doc = "0x1c - USCI A1 Interrupt Enable Register"]
    pub uca1ie: UCA1IE,
    #[doc = "0x1d - USCI A1 Interrupt Flags Register"]
    pub uca1ifg: UCA1IFG,
    #[doc = "0x1e - USCI A1 Interrupt Vector Register"]
    pub uca1iv: UCA1IV,
}
#[doc = "USCI A1 Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ctl1](uca1ctl1) module"]
pub type UCA1CTL1 = crate::Reg<u8, _UCA1CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1CTL1;
#[doc = "`read()` method returns [uca1ctl1::R](uca1ctl1::R) reader structure"]
impl crate::Readable for UCA1CTL1 {}
#[doc = "`write(|w| ..)` method takes [uca1ctl1::W](uca1ctl1::W) writer structure"]
impl crate::Writable for UCA1CTL1 {}
#[doc = "USCI A1 Control Register 1"]
pub mod uca1ctl1;
#[doc = "USCI A1 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ctl0](uca1ctl0) module"]
pub type UCA1CTL0 = crate::Reg<u8, _UCA1CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1CTL0;
#[doc = "`read()` method returns [uca1ctl0::R](uca1ctl0::R) reader structure"]
impl crate::Readable for UCA1CTL0 {}
#[doc = "`write(|w| ..)` method takes [uca1ctl0::W](uca1ctl0::W) writer structure"]
impl crate::Writable for UCA1CTL0 {}
#[doc = "USCI A1 Control Register 0"]
pub mod uca1ctl0;
#[doc = "USCI A1 Baud Rate 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1br0](uca1br0) module"]
pub type UCA1BR0 = crate::Reg<u8, _UCA1BR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1BR0;
#[doc = "`read()` method returns [uca1br0::R](uca1br0::R) reader structure"]
impl crate::Readable for UCA1BR0 {}
#[doc = "`write(|w| ..)` method takes [uca1br0::W](uca1br0::W) writer structure"]
impl crate::Writable for UCA1BR0 {}
#[doc = "USCI A1 Baud Rate 0"]
pub mod uca1br0;
#[doc = "USCI A1 Baud Rate 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1br1](uca1br1) module"]
pub type UCA1BR1 = crate::Reg<u8, _UCA1BR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1BR1;
#[doc = "`read()` method returns [uca1br1::R](uca1br1::R) reader structure"]
impl crate::Readable for UCA1BR1 {}
#[doc = "`write(|w| ..)` method takes [uca1br1::W](uca1br1::W) writer structure"]
impl crate::Writable for UCA1BR1 {}
#[doc = "USCI A1 Baud Rate 1"]
pub mod uca1br1;
#[doc = "USCI A1 Modulation Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1mctl](uca1mctl) module"]
pub type UCA1MCTL = crate::Reg<u8, _UCA1MCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1MCTL;
#[doc = "`read()` method returns [uca1mctl::R](uca1mctl::R) reader structure"]
impl crate::Readable for UCA1MCTL {}
#[doc = "`write(|w| ..)` method takes [uca1mctl::W](uca1mctl::W) writer structure"]
impl crate::Writable for UCA1MCTL {}
#[doc = "USCI A1 Modulation Control"]
pub mod uca1mctl;
#[doc = "USCI A1 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1stat](uca1stat) module"]
pub type UCA1STAT = crate::Reg<u8, _UCA1STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1STAT;
#[doc = "`read()` method returns [uca1stat::R](uca1stat::R) reader structure"]
impl crate::Readable for UCA1STAT {}
#[doc = "`write(|w| ..)` method takes [uca1stat::W](uca1stat::W) writer structure"]
impl crate::Writable for UCA1STAT {}
#[doc = "USCI A1 Status Register"]
pub mod uca1stat;
#[doc = "USCI A1 Receive Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1rxbuf](uca1rxbuf) module"]
pub type UCA1RXBUF = crate::Reg<u8, _UCA1RXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1RXBUF;
#[doc = "`read()` method returns [uca1rxbuf::R](uca1rxbuf::R) reader structure"]
impl crate::Readable for UCA1RXBUF {}
#[doc = "`write(|w| ..)` method takes [uca1rxbuf::W](uca1rxbuf::W) writer structure"]
impl crate::Writable for UCA1RXBUF {}
#[doc = "USCI A1 Receive Buffer"]
pub mod uca1rxbuf;
#[doc = "USCI A1 Transmit Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1txbuf](uca1txbuf) module"]
pub type UCA1TXBUF = crate::Reg<u8, _UCA1TXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1TXBUF;
#[doc = "`read()` method returns [uca1txbuf::R](uca1txbuf::R) reader structure"]
impl crate::Readable for UCA1TXBUF {}
#[doc = "`write(|w| ..)` method takes [uca1txbuf::W](uca1txbuf::W) writer structure"]
impl crate::Writable for UCA1TXBUF {}
#[doc = "USCI A1 Transmit Buffer"]
pub mod uca1txbuf;
#[doc = "USCI A1 LIN Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1abctl](uca1abctl) module"]
pub type UCA1ABCTL = crate::Reg<u8, _UCA1ABCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1ABCTL;
#[doc = "`read()` method returns [uca1abctl::R](uca1abctl::R) reader structure"]
impl crate::Readable for UCA1ABCTL {}
#[doc = "`write(|w| ..)` method takes [uca1abctl::W](uca1abctl::W) writer structure"]
impl crate::Writable for UCA1ABCTL {}
#[doc = "USCI A1 LIN Control"]
pub mod uca1abctl;
#[doc = "USCI A1 IrDA Transmit Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1irtctl](uca1irtctl) module"]
pub type UCA1IRTCTL = crate::Reg<u8, _UCA1IRTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IRTCTL;
#[doc = "`read()` method returns [uca1irtctl::R](uca1irtctl::R) reader structure"]
impl crate::Readable for UCA1IRTCTL {}
#[doc = "`write(|w| ..)` method takes [uca1irtctl::W](uca1irtctl::W) writer structure"]
impl crate::Writable for UCA1IRTCTL {}
#[doc = "USCI A1 IrDA Transmit Control"]
pub mod uca1irtctl;
#[doc = "USCI A1 IrDA Receive Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1irrctl](uca1irrctl) module"]
pub type UCA1IRRCTL = crate::Reg<u8, _UCA1IRRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IRRCTL;
#[doc = "`read()` method returns [uca1irrctl::R](uca1irrctl::R) reader structure"]
impl crate::Readable for UCA1IRRCTL {}
#[doc = "`write(|w| ..)` method takes [uca1irrctl::W](uca1irrctl::W) writer structure"]
impl crate::Writable for UCA1IRRCTL {}
#[doc = "USCI A1 IrDA Receive Control"]
pub mod uca1irrctl;
#[doc = "USCI A1 Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ie](uca1ie) module"]
pub type UCA1IE = crate::Reg<u8, _UCA1IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IE;
#[doc = "`read()` method returns [uca1ie::R](uca1ie::R) reader structure"]
impl crate::Readable for UCA1IE {}
#[doc = "`write(|w| ..)` method takes [uca1ie::W](uca1ie::W) writer structure"]
impl crate::Writable for UCA1IE {}
#[doc = "USCI A1 Interrupt Enable Register"]
pub mod uca1ie;
#[doc = "USCI A1 Interrupt Flags Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ifg](uca1ifg) module"]
pub type UCA1IFG = crate::Reg<u8, _UCA1IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IFG;
#[doc = "`read()` method returns [uca1ifg::R](uca1ifg::R) reader structure"]
impl crate::Readable for UCA1IFG {}
#[doc = "`write(|w| ..)` method takes [uca1ifg::W](uca1ifg::W) writer structure"]
impl crate::Writable for UCA1IFG {}
#[doc = "USCI A1 Interrupt Flags Register"]
pub mod uca1ifg;
#[doc = "USCI A1 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1iv](uca1iv) module"]
pub type UCA1IV = crate::Reg<u16, _UCA1IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IV;
#[doc = "`read()` method returns [uca1iv::R](uca1iv::R) reader structure"]
impl crate::Readable for UCA1IV {}
#[doc = "`write(|w| ..)` method takes [uca1iv::W](uca1iv::W) writer structure"]
impl crate::Writable for UCA1IV {}
#[doc = "USCI A1 Interrupt Vector Register"]
pub mod uca1iv;
