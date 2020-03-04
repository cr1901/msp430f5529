#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port J Input"]
    pub pjin: PJIN,
    #[doc = "0x02 - Port J Output"]
    pub pjout: PJOUT,
    #[doc = "0x04 - Port J Direction"]
    pub pjdir: PJDIR,
    #[doc = "0x06 - Port J Resistor Enable"]
    pub pjren: PJREN,
    #[doc = "0x08 - Port J Drive Strenght"]
    pub pjds: PJDS,
}
#[doc = "Port J Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjin](pjin) module"]
pub type PJIN = crate::Reg<u16, _PJIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJIN;
#[doc = "`read()` method returns [pjin::R](pjin::R) reader structure"]
impl crate::Readable for PJIN {}
#[doc = "`write(|w| ..)` method takes [pjin::W](pjin::W) writer structure"]
impl crate::Writable for PJIN {}
#[doc = "Port J Input"]
pub mod pjin;
#[doc = "Port J Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjout](pjout) module"]
pub type PJOUT = crate::Reg<u16, _PJOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJOUT;
#[doc = "`read()` method returns [pjout::R](pjout::R) reader structure"]
impl crate::Readable for PJOUT {}
#[doc = "`write(|w| ..)` method takes [pjout::W](pjout::W) writer structure"]
impl crate::Writable for PJOUT {}
#[doc = "Port J Output"]
pub mod pjout;
#[doc = "Port J Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjdir](pjdir) module"]
pub type PJDIR = crate::Reg<u16, _PJDIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJDIR;
#[doc = "`read()` method returns [pjdir::R](pjdir::R) reader structure"]
impl crate::Readable for PJDIR {}
#[doc = "`write(|w| ..)` method takes [pjdir::W](pjdir::W) writer structure"]
impl crate::Writable for PJDIR {}
#[doc = "Port J Direction"]
pub mod pjdir;
#[doc = "Port J Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjren](pjren) module"]
pub type PJREN = crate::Reg<u16, _PJREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJREN;
#[doc = "`read()` method returns [pjren::R](pjren::R) reader structure"]
impl crate::Readable for PJREN {}
#[doc = "`write(|w| ..)` method takes [pjren::W](pjren::W) writer structure"]
impl crate::Writable for PJREN {}
#[doc = "Port J Resistor Enable"]
pub mod pjren;
#[doc = "Port J Drive Strenght\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjds](pjds) module"]
pub type PJDS = crate::Reg<u16, _PJDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJDS;
#[doc = "`read()` method returns [pjds::R](pjds::R) reader structure"]
impl crate::Readable for PJDS {}
#[doc = "`write(|w| ..)` method takes [pjds::W](pjds::W) writer structure"]
impl crate::Writable for PJDS {}
#[doc = "Port J Drive Strenght"]
pub mod pjds;
