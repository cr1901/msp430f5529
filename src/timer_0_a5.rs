#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer0_A5 Control"]
    pub ta0ctl: TA0CTL,
    #[doc = "0x02 - Timer0_A5 Capture/Compare Control 0"]
    pub ta0cctl0: TA0CCTL0,
    #[doc = "0x04 - Timer0_A5 Capture/Compare Control 1"]
    pub ta0cctl1: TA0CCTL1,
    #[doc = "0x06 - Timer0_A5 Capture/Compare Control 2"]
    pub ta0cctl2: TA0CCTL2,
    #[doc = "0x08 - Timer0_A5 Capture/Compare Control 3"]
    pub ta0cctl3: TA0CCTL3,
    #[doc = "0x0a - Timer0_A5 Capture/Compare Control 4"]
    pub ta0cctl4: TA0CCTL4,
    _reserved6: [u8; 4usize],
    #[doc = "0x10 - Timer0_A5"]
    pub ta0r: TA0R,
    #[doc = "0x12 - Timer0_A5 Capture/Compare 0"]
    pub ta0ccr0: TA0CCR0,
    #[doc = "0x14 - Timer0_A5 Capture/Compare 1"]
    pub ta0ccr1: TA0CCR1,
    #[doc = "0x16 - Timer0_A5 Capture/Compare 2"]
    pub ta0ccr2: TA0CCR2,
    #[doc = "0x18 - Timer0_A5 Capture/Compare 3"]
    pub ta0ccr3: TA0CCR3,
    #[doc = "0x1a - Timer0_A5 Capture/Compare 4"]
    pub ta0ccr4: TA0CCR4,
    _reserved12: [u8; 4usize],
    #[doc = "0x20 - Timer0_A5 Expansion Register 0"]
    pub ta0ex0: TA0EX0,
    _reserved13: [u8; 12usize],
    #[doc = "0x2e - Timer0_A5 Interrupt Vector Word"]
    pub ta0iv: TA0IV,
}
#[doc = "Timer0_A5 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0ctl](ta0ctl) module"]
pub type TA0CTL = crate::Reg<u16, _TA0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0CTL;
#[doc = "`read()` method returns [ta0ctl::R](ta0ctl::R) reader structure"]
impl crate::Readable for TA0CTL {}
#[doc = "`write(|w| ..)` method takes [ta0ctl::W](ta0ctl::W) writer structure"]
impl crate::Writable for TA0CTL {}
#[doc = "Timer0_A5 Control"]
pub mod ta0ctl;
#[doc = "Timer0_A5 Capture/Compare Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0cctl0](ta0cctl0) module"]
pub type TA0CCTL0 = crate::Reg<u16, _TA0CCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0CCTL0;
#[doc = "`read()` method returns [ta0cctl0::R](ta0cctl0::R) reader structure"]
impl crate::Readable for TA0CCTL0 {}
#[doc = "`write(|w| ..)` method takes [ta0cctl0::W](ta0cctl0::W) writer structure"]
impl crate::Writable for TA0CCTL0 {}
#[doc = "Timer0_A5 Capture/Compare Control 0"]
pub mod ta0cctl0;
#[doc = "Timer0_A5 Capture/Compare Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0cctl1](ta0cctl1) module"]
pub type TA0CCTL1 = crate::Reg<u16, _TA0CCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0CCTL1;
#[doc = "`read()` method returns [ta0cctl1::R](ta0cctl1::R) reader structure"]
impl crate::Readable for TA0CCTL1 {}
#[doc = "`write(|w| ..)` method takes [ta0cctl1::W](ta0cctl1::W) writer structure"]
impl crate::Writable for TA0CCTL1 {}
#[doc = "Timer0_A5 Capture/Compare Control 1"]
pub mod ta0cctl1;
#[doc = "Timer0_A5 Capture/Compare Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0cctl2](ta0cctl2) module"]
pub type TA0CCTL2 = crate::Reg<u16, _TA0CCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0CCTL2;
#[doc = "`read()` method returns [ta0cctl2::R](ta0cctl2::R) reader structure"]
impl crate::Readable for TA0CCTL2 {}
#[doc = "`write(|w| ..)` method takes [ta0cctl2::W](ta0cctl2::W) writer structure"]
impl crate::Writable for TA0CCTL2 {}
#[doc = "Timer0_A5 Capture/Compare Control 2"]
pub mod ta0cctl2;
#[doc = "Timer0_A5 Capture/Compare Control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0cctl3](ta0cctl3) module"]
pub type TA0CCTL3 = crate::Reg<u16, _TA0CCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0CCTL3;
#[doc = "`read()` method returns [ta0cctl3::R](ta0cctl3::R) reader structure"]
impl crate::Readable for TA0CCTL3 {}
#[doc = "`write(|w| ..)` method takes [ta0cctl3::W](ta0cctl3::W) writer structure"]
impl crate::Writable for TA0CCTL3 {}
#[doc = "Timer0_A5 Capture/Compare Control 3"]
pub mod ta0cctl3;
#[doc = "Timer0_A5 Capture/Compare Control 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0cctl4](ta0cctl4) module"]
pub type TA0CCTL4 = crate::Reg<u16, _TA0CCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0CCTL4;
#[doc = "`read()` method returns [ta0cctl4::R](ta0cctl4::R) reader structure"]
impl crate::Readable for TA0CCTL4 {}
#[doc = "`write(|w| ..)` method takes [ta0cctl4::W](ta0cctl4::W) writer structure"]
impl crate::Writable for TA0CCTL4 {}
#[doc = "Timer0_A5 Capture/Compare Control 4"]
pub mod ta0cctl4;
#[doc = "Timer0_A5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0r](ta0r) module"]
pub type TA0R = crate::Reg<u16, _TA0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0R;
#[doc = "`read()` method returns [ta0r::R](ta0r::R) reader structure"]
impl crate::Readable for TA0R {}
#[doc = "`write(|w| ..)` method takes [ta0r::W](ta0r::W) writer structure"]
impl crate::Writable for TA0R {}
#[doc = "Timer0_A5"]
pub mod ta0r;
#[doc = "Timer0_A5 Capture/Compare 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0ccr0](ta0ccr0) module"]
pub type TA0CCR0 = crate::Reg<u16, _TA0CCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0CCR0;
#[doc = "`read()` method returns [ta0ccr0::R](ta0ccr0::R) reader structure"]
impl crate::Readable for TA0CCR0 {}
#[doc = "`write(|w| ..)` method takes [ta0ccr0::W](ta0ccr0::W) writer structure"]
impl crate::Writable for TA0CCR0 {}
#[doc = "Timer0_A5 Capture/Compare 0"]
pub mod ta0ccr0;
#[doc = "Timer0_A5 Capture/Compare 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0ccr1](ta0ccr1) module"]
pub type TA0CCR1 = crate::Reg<u16, _TA0CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0CCR1;
#[doc = "`read()` method returns [ta0ccr1::R](ta0ccr1::R) reader structure"]
impl crate::Readable for TA0CCR1 {}
#[doc = "`write(|w| ..)` method takes [ta0ccr1::W](ta0ccr1::W) writer structure"]
impl crate::Writable for TA0CCR1 {}
#[doc = "Timer0_A5 Capture/Compare 1"]
pub mod ta0ccr1;
#[doc = "Timer0_A5 Capture/Compare 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0ccr2](ta0ccr2) module"]
pub type TA0CCR2 = crate::Reg<u16, _TA0CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0CCR2;
#[doc = "`read()` method returns [ta0ccr2::R](ta0ccr2::R) reader structure"]
impl crate::Readable for TA0CCR2 {}
#[doc = "`write(|w| ..)` method takes [ta0ccr2::W](ta0ccr2::W) writer structure"]
impl crate::Writable for TA0CCR2 {}
#[doc = "Timer0_A5 Capture/Compare 2"]
pub mod ta0ccr2;
#[doc = "Timer0_A5 Capture/Compare 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0ccr3](ta0ccr3) module"]
pub type TA0CCR3 = crate::Reg<u16, _TA0CCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0CCR3;
#[doc = "`read()` method returns [ta0ccr3::R](ta0ccr3::R) reader structure"]
impl crate::Readable for TA0CCR3 {}
#[doc = "`write(|w| ..)` method takes [ta0ccr3::W](ta0ccr3::W) writer structure"]
impl crate::Writable for TA0CCR3 {}
#[doc = "Timer0_A5 Capture/Compare 3"]
pub mod ta0ccr3;
#[doc = "Timer0_A5 Capture/Compare 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0ccr4](ta0ccr4) module"]
pub type TA0CCR4 = crate::Reg<u16, _TA0CCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0CCR4;
#[doc = "`read()` method returns [ta0ccr4::R](ta0ccr4::R) reader structure"]
impl crate::Readable for TA0CCR4 {}
#[doc = "`write(|w| ..)` method takes [ta0ccr4::W](ta0ccr4::W) writer structure"]
impl crate::Writable for TA0CCR4 {}
#[doc = "Timer0_A5 Capture/Compare 4"]
pub mod ta0ccr4;
#[doc = "Timer0_A5 Expansion Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0ex0](ta0ex0) module"]
pub type TA0EX0 = crate::Reg<u16, _TA0EX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0EX0;
#[doc = "`read()` method returns [ta0ex0::R](ta0ex0::R) reader structure"]
impl crate::Readable for TA0EX0 {}
#[doc = "`write(|w| ..)` method takes [ta0ex0::W](ta0ex0::W) writer structure"]
impl crate::Writable for TA0EX0 {}
#[doc = "Timer0_A5 Expansion Register 0"]
pub mod ta0ex0;
#[doc = "Timer0_A5 Interrupt Vector Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0iv](ta0iv) module"]
pub type TA0IV = crate::Reg<u16, _TA0IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA0IV;
#[doc = "`read()` method returns [ta0iv::R](ta0iv::R) reader structure"]
impl crate::Readable for TA0IV {}
#[doc = "`write(|w| ..)` method takes [ta0iv::W](ta0iv::W) writer structure"]
impl crate::Writable for TA0IV {}
#[doc = "Timer0_A5 Interrupt Vector Word"]
pub mod ta0iv;
