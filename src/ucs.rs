#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UCS Control Register 0"]
    pub ucsctl0: UCSCTL0,
    #[doc = "0x02 - UCS Control Register 1"]
    pub ucsctl1: UCSCTL1,
    #[doc = "0x04 - UCS Control Register 2"]
    pub ucsctl2: UCSCTL2,
    #[doc = "0x06 - UCS Control Register 3"]
    pub ucsctl3: UCSCTL3,
    #[doc = "0x08 - UCS Control Register 4"]
    pub ucsctl4: UCSCTL4,
    #[doc = "0x0a - UCS Control Register 5"]
    pub ucsctl5: UCSCTL5,
    #[doc = "0x0c - UCS Control Register 6"]
    pub ucsctl6: UCSCTL6,
    #[doc = "0x0e - UCS Control Register 7"]
    pub ucsctl7: UCSCTL7,
    #[doc = "0x10 - UCS Control Register 8"]
    pub ucsctl8: UCSCTL8,
}
#[doc = "UCS Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsctl0](ucsctl0) module"]
pub type UCSCTL0 = crate::Reg<u16, _UCSCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCSCTL0;
#[doc = "`read()` method returns [ucsctl0::R](ucsctl0::R) reader structure"]
impl crate::Readable for UCSCTL0 {}
#[doc = "`write(|w| ..)` method takes [ucsctl0::W](ucsctl0::W) writer structure"]
impl crate::Writable for UCSCTL0 {}
#[doc = "UCS Control Register 0"]
pub mod ucsctl0;
#[doc = "UCS Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsctl1](ucsctl1) module"]
pub type UCSCTL1 = crate::Reg<u16, _UCSCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCSCTL1;
#[doc = "`read()` method returns [ucsctl1::R](ucsctl1::R) reader structure"]
impl crate::Readable for UCSCTL1 {}
#[doc = "`write(|w| ..)` method takes [ucsctl1::W](ucsctl1::W) writer structure"]
impl crate::Writable for UCSCTL1 {}
#[doc = "UCS Control Register 1"]
pub mod ucsctl1;
#[doc = "UCS Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsctl2](ucsctl2) module"]
pub type UCSCTL2 = crate::Reg<u16, _UCSCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCSCTL2;
#[doc = "`read()` method returns [ucsctl2::R](ucsctl2::R) reader structure"]
impl crate::Readable for UCSCTL2 {}
#[doc = "`write(|w| ..)` method takes [ucsctl2::W](ucsctl2::W) writer structure"]
impl crate::Writable for UCSCTL2 {}
#[doc = "UCS Control Register 2"]
pub mod ucsctl2;
#[doc = "UCS Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsctl3](ucsctl3) module"]
pub type UCSCTL3 = crate::Reg<u16, _UCSCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCSCTL3;
#[doc = "`read()` method returns [ucsctl3::R](ucsctl3::R) reader structure"]
impl crate::Readable for UCSCTL3 {}
#[doc = "`write(|w| ..)` method takes [ucsctl3::W](ucsctl3::W) writer structure"]
impl crate::Writable for UCSCTL3 {}
#[doc = "UCS Control Register 3"]
pub mod ucsctl3;
#[doc = "UCS Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsctl4](ucsctl4) module"]
pub type UCSCTL4 = crate::Reg<u16, _UCSCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCSCTL4;
#[doc = "`read()` method returns [ucsctl4::R](ucsctl4::R) reader structure"]
impl crate::Readable for UCSCTL4 {}
#[doc = "`write(|w| ..)` method takes [ucsctl4::W](ucsctl4::W) writer structure"]
impl crate::Writable for UCSCTL4 {}
#[doc = "UCS Control Register 4"]
pub mod ucsctl4;
#[doc = "UCS Control Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsctl5](ucsctl5) module"]
pub type UCSCTL5 = crate::Reg<u16, _UCSCTL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCSCTL5;
#[doc = "`read()` method returns [ucsctl5::R](ucsctl5::R) reader structure"]
impl crate::Readable for UCSCTL5 {}
#[doc = "`write(|w| ..)` method takes [ucsctl5::W](ucsctl5::W) writer structure"]
impl crate::Writable for UCSCTL5 {}
#[doc = "UCS Control Register 5"]
pub mod ucsctl5;
#[doc = "UCS Control Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsctl6](ucsctl6) module"]
pub type UCSCTL6 = crate::Reg<u16, _UCSCTL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCSCTL6;
#[doc = "`read()` method returns [ucsctl6::R](ucsctl6::R) reader structure"]
impl crate::Readable for UCSCTL6 {}
#[doc = "`write(|w| ..)` method takes [ucsctl6::W](ucsctl6::W) writer structure"]
impl crate::Writable for UCSCTL6 {}
#[doc = "UCS Control Register 6"]
pub mod ucsctl6;
#[doc = "UCS Control Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsctl7](ucsctl7) module"]
pub type UCSCTL7 = crate::Reg<u16, _UCSCTL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCSCTL7;
#[doc = "`read()` method returns [ucsctl7::R](ucsctl7::R) reader structure"]
impl crate::Readable for UCSCTL7 {}
#[doc = "`write(|w| ..)` method takes [ucsctl7::W](ucsctl7::W) writer structure"]
impl crate::Writable for UCSCTL7 {}
#[doc = "UCS Control Register 7"]
pub mod ucsctl7;
#[doc = "UCS Control Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsctl8](ucsctl8) module"]
pub type UCSCTL8 = crate::Reg<u16, _UCSCTL8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCSCTL8;
#[doc = "`read()` method returns [ucsctl8::R](ucsctl8::R) reader structure"]
impl crate::Readable for UCSCTL8 {}
#[doc = "`write(|w| ..)` method takes [ucsctl8::W](ucsctl8::W) writer structure"]
impl crate::Writable for UCSCTL8 {}
#[doc = "UCS Control Register 8"]
pub mod ucsctl8;
