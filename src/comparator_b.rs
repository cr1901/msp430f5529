#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator B Control Register 0"]
    pub cbctl0: CBCTL0,
    #[doc = "0x02 - Comparator B Control Register 1"]
    pub cbctl1: CBCTL1,
    #[doc = "0x04 - Comparator B Control Register 2"]
    pub cbctl2: CBCTL2,
    #[doc = "0x06 - Comparator B Control Register 3"]
    pub cbctl3: CBCTL3,
    _reserved4: [u8; 4usize],
    #[doc = "0x0c - Comparator B Interrupt Register"]
    pub cbint: CBINT,
    #[doc = "0x0e - Comparator B Interrupt Vector Word"]
    pub cbiv: CBIV,
}
#[doc = "Comparator B Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbctl0](cbctl0) module"]
pub type CBCTL0 = crate::Reg<u16, _CBCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBCTL0;
#[doc = "`read()` method returns [cbctl0::R](cbctl0::R) reader structure"]
impl crate::Readable for CBCTL0 {}
#[doc = "`write(|w| ..)` method takes [cbctl0::W](cbctl0::W) writer structure"]
impl crate::Writable for CBCTL0 {}
#[doc = "Comparator B Control Register 0"]
pub mod cbctl0;
#[doc = "Comparator B Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbctl1](cbctl1) module"]
pub type CBCTL1 = crate::Reg<u16, _CBCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBCTL1;
#[doc = "`read()` method returns [cbctl1::R](cbctl1::R) reader structure"]
impl crate::Readable for CBCTL1 {}
#[doc = "`write(|w| ..)` method takes [cbctl1::W](cbctl1::W) writer structure"]
impl crate::Writable for CBCTL1 {}
#[doc = "Comparator B Control Register 1"]
pub mod cbctl1;
#[doc = "Comparator B Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbctl2](cbctl2) module"]
pub type CBCTL2 = crate::Reg<u16, _CBCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBCTL2;
#[doc = "`read()` method returns [cbctl2::R](cbctl2::R) reader structure"]
impl crate::Readable for CBCTL2 {}
#[doc = "`write(|w| ..)` method takes [cbctl2::W](cbctl2::W) writer structure"]
impl crate::Writable for CBCTL2 {}
#[doc = "Comparator B Control Register 2"]
pub mod cbctl2;
#[doc = "Comparator B Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbctl3](cbctl3) module"]
pub type CBCTL3 = crate::Reg<u16, _CBCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBCTL3;
#[doc = "`read()` method returns [cbctl3::R](cbctl3::R) reader structure"]
impl crate::Readable for CBCTL3 {}
#[doc = "`write(|w| ..)` method takes [cbctl3::W](cbctl3::W) writer structure"]
impl crate::Writable for CBCTL3 {}
#[doc = "Comparator B Control Register 3"]
pub mod cbctl3;
#[doc = "Comparator B Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbint](cbint) module"]
pub type CBINT = crate::Reg<u16, _CBINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBINT;
#[doc = "`read()` method returns [cbint::R](cbint::R) reader structure"]
impl crate::Readable for CBINT {}
#[doc = "`write(|w| ..)` method takes [cbint::W](cbint::W) writer structure"]
impl crate::Writable for CBINT {}
#[doc = "Comparator B Interrupt Register"]
pub mod cbint;
#[doc = "Comparator B Interrupt Vector Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbiv](cbiv) module"]
pub type CBIV = crate::Reg<u16, _CBIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBIV;
#[doc = "`read()` method returns [cbiv::R](cbiv::R) reader structure"]
impl crate::Readable for CBIV {}
#[doc = "`write(|w| ..)` method takes [cbiv::W](cbiv::W) writer structure"]
impl crate::Writable for CBIV {}
#[doc = "Comparator B Interrupt Vector Word"]
pub mod cbiv;
