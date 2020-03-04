#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Real Timer Control 0/1"]
    pub rtcctl01: RTCCTL01,
    #[doc = "0x02 - Real Timer Control 2/3"]
    pub rtcctl23: RTCCTL23,
    _reserved2: [u8; 4usize],
    #[doc = "0x08 - Real Timer Prescale Timer 0 Control"]
    pub rtcps0ctl: RTCPS0CTL,
    #[doc = "0x0a - Real Timer Prescale Timer 1 Control"]
    pub rtcps1ctl: RTCPS1CTL,
    #[doc = "0x0c - Real Timer Prescale Timer Control"]
    pub rtcps: RTCPS,
    #[doc = "0x0e - Real Time Clock Interrupt Vector"]
    pub rtciv: RTCIV,
    #[doc = "0x10 - Real Time Clock Seconds"]
    pub rtcsec: RTCSEC,
    #[doc = "0x11 - Real Time Clock Minutes"]
    pub rtcmin: RTCMIN,
    #[doc = "0x12 - Real Time Clock Hour"]
    pub rtchour: RTCHOUR,
    #[doc = "0x13 - Real Time Clock Day of week"]
    pub rtcdow: RTCDOW,
    #[doc = "0x14 - Real Time Clock Day"]
    pub rtcday: RTCDAY,
    #[doc = "0x15 - Real Time Clock Month"]
    pub rtcmon: RTCMON,
    #[doc = "0x16 - Real Time Clock Year"]
    pub rtcyear: RTCYEAR,
    #[doc = "0x18 - Real Time Clock Alarm Min"]
    pub rtcamin: RTCAMIN,
    #[doc = "0x19 - Real Time Clock Alarm Hour"]
    pub rtcahour: RTCAHOUR,
    #[doc = "0x1a - Real Time Clock Alarm Day of week"]
    pub rtcadow: RTCADOW,
    #[doc = "0x1b - Real Time Clock Alarm Day"]
    pub rtcaday: RTCADAY,
}
#[doc = "Real Time Clock Seconds\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcsec](rtcsec) module"]
pub type RTCSEC = crate::Reg<u8, _RTCSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCSEC;
#[doc = "`read()` method returns [rtcsec::R](rtcsec::R) reader structure"]
impl crate::Readable for RTCSEC {}
#[doc = "`write(|w| ..)` method takes [rtcsec::W](rtcsec::W) writer structure"]
impl crate::Writable for RTCSEC {}
#[doc = "Real Time Clock Seconds"]
pub mod rtcsec;
#[doc = "Real Time Clock Minutes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcmin](rtcmin) module"]
pub type RTCMIN = crate::Reg<u8, _RTCMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCMIN;
#[doc = "`read()` method returns [rtcmin::R](rtcmin::R) reader structure"]
impl crate::Readable for RTCMIN {}
#[doc = "`write(|w| ..)` method takes [rtcmin::W](rtcmin::W) writer structure"]
impl crate::Writable for RTCMIN {}
#[doc = "Real Time Clock Minutes"]
pub mod rtcmin;
#[doc = "Real Time Clock Hour\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtchour](rtchour) module"]
pub type RTCHOUR = crate::Reg<u8, _RTCHOUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCHOUR;
#[doc = "`read()` method returns [rtchour::R](rtchour::R) reader structure"]
impl crate::Readable for RTCHOUR {}
#[doc = "`write(|w| ..)` method takes [rtchour::W](rtchour::W) writer structure"]
impl crate::Writable for RTCHOUR {}
#[doc = "Real Time Clock Hour"]
pub mod rtchour;
#[doc = "Real Time Clock Day of week\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcdow](rtcdow) module"]
pub type RTCDOW = crate::Reg<u8, _RTCDOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCDOW;
#[doc = "`read()` method returns [rtcdow::R](rtcdow::R) reader structure"]
impl crate::Readable for RTCDOW {}
#[doc = "`write(|w| ..)` method takes [rtcdow::W](rtcdow::W) writer structure"]
impl crate::Writable for RTCDOW {}
#[doc = "Real Time Clock Day of week"]
pub mod rtcdow;
#[doc = "Real Time Clock Day\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcday](rtcday) module"]
pub type RTCDAY = crate::Reg<u8, _RTCDAY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCDAY;
#[doc = "`read()` method returns [rtcday::R](rtcday::R) reader structure"]
impl crate::Readable for RTCDAY {}
#[doc = "`write(|w| ..)` method takes [rtcday::W](rtcday::W) writer structure"]
impl crate::Writable for RTCDAY {}
#[doc = "Real Time Clock Day"]
pub mod rtcday;
#[doc = "Real Time Clock Month\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcmon](rtcmon) module"]
pub type RTCMON = crate::Reg<u8, _RTCMON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCMON;
#[doc = "`read()` method returns [rtcmon::R](rtcmon::R) reader structure"]
impl crate::Readable for RTCMON {}
#[doc = "`write(|w| ..)` method takes [rtcmon::W](rtcmon::W) writer structure"]
impl crate::Writable for RTCMON {}
#[doc = "Real Time Clock Month"]
pub mod rtcmon;
#[doc = "Real Time Clock Alarm Min\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcamin](rtcamin) module"]
pub type RTCAMIN = crate::Reg<u8, _RTCAMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCAMIN;
#[doc = "`read()` method returns [rtcamin::R](rtcamin::R) reader structure"]
impl crate::Readable for RTCAMIN {}
#[doc = "`write(|w| ..)` method takes [rtcamin::W](rtcamin::W) writer structure"]
impl crate::Writable for RTCAMIN {}
#[doc = "Real Time Clock Alarm Min"]
pub mod rtcamin;
#[doc = "Real Time Clock Alarm Hour\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcahour](rtcahour) module"]
pub type RTCAHOUR = crate::Reg<u8, _RTCAHOUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCAHOUR;
#[doc = "`read()` method returns [rtcahour::R](rtcahour::R) reader structure"]
impl crate::Readable for RTCAHOUR {}
#[doc = "`write(|w| ..)` method takes [rtcahour::W](rtcahour::W) writer structure"]
impl crate::Writable for RTCAHOUR {}
#[doc = "Real Time Clock Alarm Hour"]
pub mod rtcahour;
#[doc = "Real Time Clock Alarm Day of week\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcadow](rtcadow) module"]
pub type RTCADOW = crate::Reg<u8, _RTCADOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCADOW;
#[doc = "`read()` method returns [rtcadow::R](rtcadow::R) reader structure"]
impl crate::Readable for RTCADOW {}
#[doc = "`write(|w| ..)` method takes [rtcadow::W](rtcadow::W) writer structure"]
impl crate::Writable for RTCADOW {}
#[doc = "Real Time Clock Alarm Day of week"]
pub mod rtcadow;
#[doc = "Real Time Clock Alarm Day\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcaday](rtcaday) module"]
pub type RTCADAY = crate::Reg<u8, _RTCADAY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCADAY;
#[doc = "`read()` method returns [rtcaday::R](rtcaday::R) reader structure"]
impl crate::Readable for RTCADAY {}
#[doc = "`write(|w| ..)` method takes [rtcaday::W](rtcaday::W) writer structure"]
impl crate::Writable for RTCADAY {}
#[doc = "Real Time Clock Alarm Day"]
pub mod rtcaday;
#[doc = "Real Timer Control 0/1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcctl01](rtcctl01) module"]
pub type RTCCTL01 = crate::Reg<u16, _RTCCTL01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCTL01;
#[doc = "`read()` method returns [rtcctl01::R](rtcctl01::R) reader structure"]
impl crate::Readable for RTCCTL01 {}
#[doc = "`write(|w| ..)` method takes [rtcctl01::W](rtcctl01::W) writer structure"]
impl crate::Writable for RTCCTL01 {}
#[doc = "Real Timer Control 0/1"]
pub mod rtcctl01;
#[doc = "Real Timer Control 2/3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcctl23](rtcctl23) module"]
pub type RTCCTL23 = crate::Reg<u16, _RTCCTL23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCTL23;
#[doc = "`read()` method returns [rtcctl23::R](rtcctl23::R) reader structure"]
impl crate::Readable for RTCCTL23 {}
#[doc = "`write(|w| ..)` method takes [rtcctl23::W](rtcctl23::W) writer structure"]
impl crate::Writable for RTCCTL23 {}
#[doc = "Real Timer Control 2/3"]
pub mod rtcctl23;
#[doc = "Real Timer Prescale Timer 0 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcps0ctl](rtcps0ctl) module"]
pub type RTCPS0CTL = crate::Reg<u16, _RTCPS0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCPS0CTL;
#[doc = "`read()` method returns [rtcps0ctl::R](rtcps0ctl::R) reader structure"]
impl crate::Readable for RTCPS0CTL {}
#[doc = "`write(|w| ..)` method takes [rtcps0ctl::W](rtcps0ctl::W) writer structure"]
impl crate::Writable for RTCPS0CTL {}
#[doc = "Real Timer Prescale Timer 0 Control"]
pub mod rtcps0ctl;
#[doc = "Real Timer Prescale Timer 1 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcps1ctl](rtcps1ctl) module"]
pub type RTCPS1CTL = crate::Reg<u16, _RTCPS1CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCPS1CTL;
#[doc = "`read()` method returns [rtcps1ctl::R](rtcps1ctl::R) reader structure"]
impl crate::Readable for RTCPS1CTL {}
#[doc = "`write(|w| ..)` method takes [rtcps1ctl::W](rtcps1ctl::W) writer structure"]
impl crate::Writable for RTCPS1CTL {}
#[doc = "Real Timer Prescale Timer 1 Control"]
pub mod rtcps1ctl;
#[doc = "Real Timer Prescale Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcps](rtcps) module"]
pub type RTCPS = crate::Reg<u16, _RTCPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCPS;
#[doc = "`read()` method returns [rtcps::R](rtcps::R) reader structure"]
impl crate::Readable for RTCPS {}
#[doc = "`write(|w| ..)` method takes [rtcps::W](rtcps::W) writer structure"]
impl crate::Writable for RTCPS {}
#[doc = "Real Timer Prescale Timer Control"]
pub mod rtcps;
#[doc = "Real Time Clock Interrupt Vector\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtciv](rtciv) module"]
pub type RTCIV = crate::Reg<u16, _RTCIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCIV;
#[doc = "`read()` method returns [rtciv::R](rtciv::R) reader structure"]
impl crate::Readable for RTCIV {}
#[doc = "`write(|w| ..)` method takes [rtciv::W](rtciv::W) writer structure"]
impl crate::Writable for RTCIV {}
#[doc = "Real Time Clock Interrupt Vector"]
pub mod rtciv;
#[doc = "Real Time Clock Year\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcyear](rtcyear) module"]
pub type RTCYEAR = crate::Reg<u16, _RTCYEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCYEAR;
#[doc = "`read()` method returns [rtcyear::R](rtcyear::R) reader structure"]
impl crate::Readable for RTCYEAR {}
#[doc = "`write(|w| ..)` method takes [rtcyear::W](rtcyear::W) writer structure"]
impl crate::Writable for RTCYEAR {}
#[doc = "Real Time Clock Year"]
pub mod rtcyear;
