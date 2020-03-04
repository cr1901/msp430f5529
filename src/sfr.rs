#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable 1"]
    pub sfrie1: SFRIE1,
    #[doc = "0x02 - Interrupt Flag 1"]
    pub sfrifg1: SFRIFG1,
    #[doc = "0x04 - RESET Pin Control Register"]
    pub sfrrpcr: SFRRPCR,
}
#[doc = "Interrupt Enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfrie1](sfrie1) module"]
pub type SFRIE1 = crate::Reg<u16, _SFRIE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFRIE1;
#[doc = "`read()` method returns [sfrie1::R](sfrie1::R) reader structure"]
impl crate::Readable for SFRIE1 {}
#[doc = "`write(|w| ..)` method takes [sfrie1::W](sfrie1::W) writer structure"]
impl crate::Writable for SFRIE1 {}
#[doc = "Interrupt Enable 1"]
pub mod sfrie1;
#[doc = "Interrupt Flag 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfrifg1](sfrifg1) module"]
pub type SFRIFG1 = crate::Reg<u16, _SFRIFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFRIFG1;
#[doc = "`read()` method returns [sfrifg1::R](sfrifg1::R) reader structure"]
impl crate::Readable for SFRIFG1 {}
#[doc = "`write(|w| ..)` method takes [sfrifg1::W](sfrifg1::W) writer structure"]
impl crate::Writable for SFRIFG1 {}
#[doc = "Interrupt Flag 1"]
pub mod sfrifg1;
#[doc = "RESET Pin Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfrrpcr](sfrrpcr) module"]
pub type SFRRPCR = crate::Reg<u16, _SFRRPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFRRPCR;
#[doc = "`read()` method returns [sfrrpcr::R](sfrrpcr::R) reader structure"]
impl crate::Readable for SFRRPCR {}
#[doc = "`write(|w| ..)` method takes [sfrrpcr::W](sfrrpcr::W) writer structure"]
impl crate::Writable for SFRRPCR {}
#[doc = "RESET Pin Control Register"]
pub mod sfrrpcr;
