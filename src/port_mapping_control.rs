#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Mapping Key register"]
    pub pmapkeyid: PMAPKEYID,
    #[doc = "0x02 - Port Mapping control register"]
    pub pmapctl: PMAPCTL,
}
#[doc = "Port Mapping Key register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmapkeyid](pmapkeyid) module"]
pub type PMAPKEYID = crate::Reg<u16, _PMAPKEYID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMAPKEYID;
#[doc = "`read()` method returns [pmapkeyid::R](pmapkeyid::R) reader structure"]
impl crate::Readable for PMAPKEYID {}
#[doc = "`write(|w| ..)` method takes [pmapkeyid::W](pmapkeyid::W) writer structure"]
impl crate::Writable for PMAPKEYID {}
#[doc = "Port Mapping Key register"]
pub mod pmapkeyid;
#[doc = "Port Mapping control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmapctl](pmapctl) module"]
pub type PMAPCTL = crate::Reg<u16, _PMAPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMAPCTL;
#[doc = "`read()` method returns [pmapctl::R](pmapctl::R) reader structure"]
impl crate::Readable for PMAPCTL {}
#[doc = "`write(|w| ..)` method takes [pmapctl::W](pmapctl::W) writer structure"]
impl crate::Writable for PMAPCTL {}
#[doc = "Port Mapping control register"]
pub mod pmapctl;
