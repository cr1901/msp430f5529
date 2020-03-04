#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PMM Control 0"]
    pub pmmctl0: PMMCTL0,
    #[doc = "0x02 - PMM Control 1"]
    pub pmmctl1: PMMCTL1,
    #[doc = "0x04 - SVS and SVM high side control register"]
    pub svsmhctl: SVSMHCTL,
    #[doc = "0x06 - SVS and SVM low side control register"]
    pub svsmlctl: SVSMLCTL,
    #[doc = "0x08 - SVSIN and SVSOUT control register"]
    pub svsmio: SVSMIO,
    _reserved5: [u8; 2usize],
    #[doc = "0x0c - PMM Interrupt Flag"]
    pub pmmifg: PMMIFG,
    #[doc = "0x0e - PMM and RESET Interrupt Enable"]
    pub pmmrie: PMMRIE,
    #[doc = "0x10 - PMM Power Mode 5 Control Register 0"]
    pub pm5ctl0: PM5CTL0,
}
#[doc = "PMM Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmctl0](pmmctl0) module"]
pub type PMMCTL0 = crate::Reg<u16, _PMMCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMMCTL0;
#[doc = "`read()` method returns [pmmctl0::R](pmmctl0::R) reader structure"]
impl crate::Readable for PMMCTL0 {}
#[doc = "`write(|w| ..)` method takes [pmmctl0::W](pmmctl0::W) writer structure"]
impl crate::Writable for PMMCTL0 {}
#[doc = "PMM Control 0"]
pub mod pmmctl0;
#[doc = "PMM Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmctl1](pmmctl1) module"]
pub type PMMCTL1 = crate::Reg<u16, _PMMCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMMCTL1;
#[doc = "`read()` method returns [pmmctl1::R](pmmctl1::R) reader structure"]
impl crate::Readable for PMMCTL1 {}
#[doc = "`write(|w| ..)` method takes [pmmctl1::W](pmmctl1::W) writer structure"]
impl crate::Writable for PMMCTL1 {}
#[doc = "PMM Control 1"]
pub mod pmmctl1;
#[doc = "SVS and SVM high side control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [svsmhctl](svsmhctl) module"]
pub type SVSMHCTL = crate::Reg<u16, _SVSMHCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SVSMHCTL;
#[doc = "`read()` method returns [svsmhctl::R](svsmhctl::R) reader structure"]
impl crate::Readable for SVSMHCTL {}
#[doc = "`write(|w| ..)` method takes [svsmhctl::W](svsmhctl::W) writer structure"]
impl crate::Writable for SVSMHCTL {}
#[doc = "SVS and SVM high side control register"]
pub mod svsmhctl;
#[doc = "SVS and SVM low side control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [svsmlctl](svsmlctl) module"]
pub type SVSMLCTL = crate::Reg<u16, _SVSMLCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SVSMLCTL;
#[doc = "`read()` method returns [svsmlctl::R](svsmlctl::R) reader structure"]
impl crate::Readable for SVSMLCTL {}
#[doc = "`write(|w| ..)` method takes [svsmlctl::W](svsmlctl::W) writer structure"]
impl crate::Writable for SVSMLCTL {}
#[doc = "SVS and SVM low side control register"]
pub mod svsmlctl;
#[doc = "SVSIN and SVSOUT control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [svsmio](svsmio) module"]
pub type SVSMIO = crate::Reg<u16, _SVSMIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SVSMIO;
#[doc = "`read()` method returns [svsmio::R](svsmio::R) reader structure"]
impl crate::Readable for SVSMIO {}
#[doc = "`write(|w| ..)` method takes [svsmio::W](svsmio::W) writer structure"]
impl crate::Writable for SVSMIO {}
#[doc = "SVSIN and SVSOUT control register"]
pub mod svsmio;
#[doc = "PMM Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmifg](pmmifg) module"]
pub type PMMIFG = crate::Reg<u16, _PMMIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMMIFG;
#[doc = "`read()` method returns [pmmifg::R](pmmifg::R) reader structure"]
impl crate::Readable for PMMIFG {}
#[doc = "`write(|w| ..)` method takes [pmmifg::W](pmmifg::W) writer structure"]
impl crate::Writable for PMMIFG {}
#[doc = "PMM Interrupt Flag"]
pub mod pmmifg;
#[doc = "PMM and RESET Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmrie](pmmrie) module"]
pub type PMMRIE = crate::Reg<u16, _PMMRIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMMRIE;
#[doc = "`read()` method returns [pmmrie::R](pmmrie::R) reader structure"]
impl crate::Readable for PMMRIE {}
#[doc = "`write(|w| ..)` method takes [pmmrie::W](pmmrie::W) writer structure"]
impl crate::Writable for PMMRIE {}
#[doc = "PMM and RESET Interrupt Enable"]
pub mod pmmrie;
#[doc = "PMM Power Mode 5 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pm5ctl0](pm5ctl0) module"]
pub type PM5CTL0 = crate::Reg<u16, _PM5CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PM5CTL0;
#[doc = "`read()` method returns [pm5ctl0::R](pm5ctl0::R) reader structure"]
impl crate::Readable for PM5CTL0 {}
#[doc = "`write(|w| ..)` method takes [pm5ctl0::W](pm5ctl0::W) writer structure"]
impl crate::Writable for PM5CTL0 {}
#[doc = "PMM Power Mode 5 Control Register 0"]
pub mod pm5ctl0;
