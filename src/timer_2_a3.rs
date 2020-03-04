#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer2_A3 Control"]
    pub ta2ctl: TA2CTL,
    #[doc = "0x02 - Timer2_A3 Capture/Compare Control 0"]
    pub ta2cctl0: TA2CCTL0,
    #[doc = "0x04 - Timer2_A3 Capture/Compare Control 1"]
    pub ta2cctl1: TA2CCTL1,
    #[doc = "0x06 - Timer2_A3 Capture/Compare Control 2"]
    pub ta2cctl2: TA2CCTL2,
    _reserved4: [u8; 8usize],
    #[doc = "0x10 - Timer2_A3"]
    pub ta2r: TA2R,
    #[doc = "0x12 - Timer2_A3 Capture/Compare 0"]
    pub ta2ccr0: TA2CCR0,
    #[doc = "0x14 - Timer2_A3 Capture/Compare 1"]
    pub ta2ccr1: TA2CCR1,
    #[doc = "0x16 - Timer2_A3 Capture/Compare 2"]
    pub ta2ccr2: TA2CCR2,
    _reserved8: [u8; 8usize],
    #[doc = "0x20 - Timer2_A3 Expansion Register 0"]
    pub ta2ex0: TA2EX0,
    _reserved9: [u8; 12usize],
    #[doc = "0x2e - Timer2_A3 Interrupt Vector Word"]
    pub ta2iv: TA2IV,
}
#[doc = "Timer2_A3 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta2ctl](ta2ctl) module"]
pub type TA2CTL = crate::Reg<u16, _TA2CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA2CTL;
#[doc = "`read()` method returns [ta2ctl::R](ta2ctl::R) reader structure"]
impl crate::Readable for TA2CTL {}
#[doc = "`write(|w| ..)` method takes [ta2ctl::W](ta2ctl::W) writer structure"]
impl crate::Writable for TA2CTL {}
#[doc = "Timer2_A3 Control"]
pub mod ta2ctl;
#[doc = "Timer2_A3 Capture/Compare Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta2cctl0](ta2cctl0) module"]
pub type TA2CCTL0 = crate::Reg<u16, _TA2CCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA2CCTL0;
#[doc = "`read()` method returns [ta2cctl0::R](ta2cctl0::R) reader structure"]
impl crate::Readable for TA2CCTL0 {}
#[doc = "`write(|w| ..)` method takes [ta2cctl0::W](ta2cctl0::W) writer structure"]
impl crate::Writable for TA2CCTL0 {}
#[doc = "Timer2_A3 Capture/Compare Control 0"]
pub mod ta2cctl0;
#[doc = "Timer2_A3 Capture/Compare Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta2cctl1](ta2cctl1) module"]
pub type TA2CCTL1 = crate::Reg<u16, _TA2CCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA2CCTL1;
#[doc = "`read()` method returns [ta2cctl1::R](ta2cctl1::R) reader structure"]
impl crate::Readable for TA2CCTL1 {}
#[doc = "`write(|w| ..)` method takes [ta2cctl1::W](ta2cctl1::W) writer structure"]
impl crate::Writable for TA2CCTL1 {}
#[doc = "Timer2_A3 Capture/Compare Control 1"]
pub mod ta2cctl1;
#[doc = "Timer2_A3 Capture/Compare Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta2cctl2](ta2cctl2) module"]
pub type TA2CCTL2 = crate::Reg<u16, _TA2CCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA2CCTL2;
#[doc = "`read()` method returns [ta2cctl2::R](ta2cctl2::R) reader structure"]
impl crate::Readable for TA2CCTL2 {}
#[doc = "`write(|w| ..)` method takes [ta2cctl2::W](ta2cctl2::W) writer structure"]
impl crate::Writable for TA2CCTL2 {}
#[doc = "Timer2_A3 Capture/Compare Control 2"]
pub mod ta2cctl2;
#[doc = "Timer2_A3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta2r](ta2r) module"]
pub type TA2R = crate::Reg<u16, _TA2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA2R;
#[doc = "`read()` method returns [ta2r::R](ta2r::R) reader structure"]
impl crate::Readable for TA2R {}
#[doc = "`write(|w| ..)` method takes [ta2r::W](ta2r::W) writer structure"]
impl crate::Writable for TA2R {}
#[doc = "Timer2_A3"]
pub mod ta2r;
#[doc = "Timer2_A3 Capture/Compare 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta2ccr0](ta2ccr0) module"]
pub type TA2CCR0 = crate::Reg<u16, _TA2CCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA2CCR0;
#[doc = "`read()` method returns [ta2ccr0::R](ta2ccr0::R) reader structure"]
impl crate::Readable for TA2CCR0 {}
#[doc = "`write(|w| ..)` method takes [ta2ccr0::W](ta2ccr0::W) writer structure"]
impl crate::Writable for TA2CCR0 {}
#[doc = "Timer2_A3 Capture/Compare 0"]
pub mod ta2ccr0;
#[doc = "Timer2_A3 Capture/Compare 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta2ccr1](ta2ccr1) module"]
pub type TA2CCR1 = crate::Reg<u16, _TA2CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA2CCR1;
#[doc = "`read()` method returns [ta2ccr1::R](ta2ccr1::R) reader structure"]
impl crate::Readable for TA2CCR1 {}
#[doc = "`write(|w| ..)` method takes [ta2ccr1::W](ta2ccr1::W) writer structure"]
impl crate::Writable for TA2CCR1 {}
#[doc = "Timer2_A3 Capture/Compare 1"]
pub mod ta2ccr1;
#[doc = "Timer2_A3 Capture/Compare 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta2ccr2](ta2ccr2) module"]
pub type TA2CCR2 = crate::Reg<u16, _TA2CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA2CCR2;
#[doc = "`read()` method returns [ta2ccr2::R](ta2ccr2::R) reader structure"]
impl crate::Readable for TA2CCR2 {}
#[doc = "`write(|w| ..)` method takes [ta2ccr2::W](ta2ccr2::W) writer structure"]
impl crate::Writable for TA2CCR2 {}
#[doc = "Timer2_A3 Capture/Compare 2"]
pub mod ta2ccr2;
#[doc = "Timer2_A3 Expansion Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta2ex0](ta2ex0) module"]
pub type TA2EX0 = crate::Reg<u16, _TA2EX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA2EX0;
#[doc = "`read()` method returns [ta2ex0::R](ta2ex0::R) reader structure"]
impl crate::Readable for TA2EX0 {}
#[doc = "`write(|w| ..)` method takes [ta2ex0::W](ta2ex0::W) writer structure"]
impl crate::Writable for TA2EX0 {}
#[doc = "Timer2_A3 Expansion Register 0"]
pub mod ta2ex0;
#[doc = "Timer2_A3 Interrupt Vector Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta2iv](ta2iv) module"]
pub type TA2IV = crate::Reg<u16, _TA2IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA2IV;
#[doc = "`read()` method returns [ta2iv::R](ta2iv::R) reader structure"]
impl crate::Readable for TA2IV {}
#[doc = "`write(|w| ..)` method takes [ta2iv::W](ta2iv::W) writer structure"]
impl crate::Writable for TA2IV {}
#[doc = "Timer2_A3 Interrupt Vector Word"]
pub mod ta2iv;
