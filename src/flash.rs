#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FLASH Control 1"]
    pub fctl1: FCTL1,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - FLASH Control 3"]
    pub fctl3: FCTL3,
    #[doc = "0x06 - FLASH Control 4"]
    pub fctl4: FCTL4,
}
#[doc = "FLASH Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctl1](fctl1) module"]
pub type FCTL1 = crate::Reg<u16, _FCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCTL1;
#[doc = "`read()` method returns [fctl1::R](fctl1::R) reader structure"]
impl crate::Readable for FCTL1 {}
#[doc = "`write(|w| ..)` method takes [fctl1::W](fctl1::W) writer structure"]
impl crate::Writable for FCTL1 {}
#[doc = "FLASH Control 1"]
pub mod fctl1;
#[doc = "FLASH Control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctl3](fctl3) module"]
pub type FCTL3 = crate::Reg<u16, _FCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCTL3;
#[doc = "`read()` method returns [fctl3::R](fctl3::R) reader structure"]
impl crate::Readable for FCTL3 {}
#[doc = "`write(|w| ..)` method takes [fctl3::W](fctl3::W) writer structure"]
impl crate::Writable for FCTL3 {}
#[doc = "FLASH Control 3"]
pub mod fctl3;
#[doc = "FLASH Control 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctl4](fctl4) module"]
pub type FCTL4 = crate::Reg<u16, _FCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCTL4;
#[doc = "`read()` method returns [fctl4::R](fctl4::R) reader structure"]
impl crate::Readable for FCTL4 {}
#[doc = "`write(|w| ..)` method takes [fctl4::W](fctl4::W) writer structure"]
impl crate::Writable for FCTL4 {}
#[doc = "FLASH Control 4"]
pub mod fctl4;
