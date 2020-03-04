#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC12+ Control 0"]
    pub adc12ctl0: ADC12CTL0,
    #[doc = "0x02 - ADC12+ Control 1"]
    pub adc12ctl1: ADC12CTL1,
    #[doc = "0x04 - ADC12+ Control 2"]
    pub adc12ctl2: ADC12CTL2,
    _reserved3: [u8; 4usize],
    #[doc = "0x0a - ADC12+ Interrupt Flag"]
    pub adc12ifg: ADC12IFG,
    #[doc = "0x0c - ADC12+ Interrupt Enable"]
    pub adc12ie: ADC12IE,
    #[doc = "0x0e - ADC12+ Interrupt Vector Word"]
    pub adc12iv: ADC12IV,
    #[doc = "0x10 - ADC12 Memory Control 0"]
    pub adc12mctl0: ADC12MCTL0,
    #[doc = "0x11 - ADC12 Memory Control 1"]
    pub adc12mctl1: ADC12MCTL1,
    #[doc = "0x12 - ADC12 Memory Control 2"]
    pub adc12mctl2: ADC12MCTL2,
    #[doc = "0x13 - ADC12 Memory Control 3"]
    pub adc12mctl3: ADC12MCTL3,
    #[doc = "0x14 - ADC12 Memory Control 4"]
    pub adc12mctl4: ADC12MCTL4,
    #[doc = "0x15 - ADC12 Memory Control 5"]
    pub adc12mctl5: ADC12MCTL5,
    #[doc = "0x16 - ADC12 Memory Control 6"]
    pub adc12mctl6: ADC12MCTL6,
    #[doc = "0x17 - ADC12 Memory Control 7"]
    pub adc12mctl7: ADC12MCTL7,
    #[doc = "0x18 - ADC12 Memory Control 8"]
    pub adc12mctl8: ADC12MCTL8,
    #[doc = "0x19 - ADC12 Memory Control 9"]
    pub adc12mctl9: ADC12MCTL9,
    #[doc = "0x1a - ADC12 Memory Control 10"]
    pub adc12mctl10: ADC12MCTL10,
    #[doc = "0x1b - ADC12 Memory Control 11"]
    pub adc12mctl11: ADC12MCTL11,
    #[doc = "0x1c - ADC12 Memory Control 12"]
    pub adc12mctl12: ADC12MCTL12,
    #[doc = "0x1d - ADC12 Memory Control 13"]
    pub adc12mctl13: ADC12MCTL13,
    #[doc = "0x1e - ADC12 Memory Control 14"]
    pub adc12mctl14: ADC12MCTL14,
    #[doc = "0x1f - ADC12 Memory Control 15"]
    pub adc12mctl15: ADC12MCTL15,
    #[doc = "0x20 - ADC12 Conversion Memory 0"]
    pub adc12mem0: ADC12MEM0,
    #[doc = "0x22 - ADC12 Conversion Memory 1"]
    pub adc12mem1: ADC12MEM1,
    #[doc = "0x24 - ADC12 Conversion Memory 2"]
    pub adc12mem2: ADC12MEM2,
    #[doc = "0x26 - ADC12 Conversion Memory 3"]
    pub adc12mem3: ADC12MEM3,
    #[doc = "0x28 - ADC12 Conversion Memory 4"]
    pub adc12mem4: ADC12MEM4,
    #[doc = "0x2a - ADC12 Conversion Memory 5"]
    pub adc12mem5: ADC12MEM5,
    #[doc = "0x2c - ADC12 Conversion Memory 6"]
    pub adc12mem6: ADC12MEM6,
    #[doc = "0x2e - ADC12 Conversion Memory 7"]
    pub adc12mem7: ADC12MEM7,
    #[doc = "0x30 - ADC12 Conversion Memory 8"]
    pub adc12mem8: ADC12MEM8,
    #[doc = "0x32 - ADC12 Conversion Memory 9"]
    pub adc12mem9: ADC12MEM9,
    #[doc = "0x34 - ADC12 Conversion Memory 10"]
    pub adc12mem10: ADC12MEM10,
    #[doc = "0x36 - ADC12 Conversion Memory 11"]
    pub adc12mem11: ADC12MEM11,
    #[doc = "0x38 - ADC12 Conversion Memory 12"]
    pub adc12mem12: ADC12MEM12,
    #[doc = "0x3a - ADC12 Conversion Memory 13"]
    pub adc12mem13: ADC12MEM13,
    #[doc = "0x3c - ADC12 Conversion Memory 14"]
    pub adc12mem14: ADC12MEM14,
    #[doc = "0x3e - ADC12 Conversion Memory 15"]
    pub adc12mem15: ADC12MEM15,
}
#[doc = "ADC12 Memory Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl0](adc12mctl0) module"]
pub type ADC12MCTL0 = crate::Reg<u8, _ADC12MCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL0;
#[doc = "`read()` method returns [adc12mctl0::R](adc12mctl0::R) reader structure"]
impl crate::Readable for ADC12MCTL0 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl0::W](adc12mctl0::W) writer structure"]
impl crate::Writable for ADC12MCTL0 {}
#[doc = "ADC12 Memory Control 0"]
pub mod adc12mctl0;
#[doc = "ADC12 Memory Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl1](adc12mctl1) module"]
pub type ADC12MCTL1 = crate::Reg<u8, _ADC12MCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL1;
#[doc = "`read()` method returns [adc12mctl1::R](adc12mctl1::R) reader structure"]
impl crate::Readable for ADC12MCTL1 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl1::W](adc12mctl1::W) writer structure"]
impl crate::Writable for ADC12MCTL1 {}
#[doc = "ADC12 Memory Control 1"]
pub mod adc12mctl1;
#[doc = "ADC12 Memory Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl2](adc12mctl2) module"]
pub type ADC12MCTL2 = crate::Reg<u8, _ADC12MCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL2;
#[doc = "`read()` method returns [adc12mctl2::R](adc12mctl2::R) reader structure"]
impl crate::Readable for ADC12MCTL2 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl2::W](adc12mctl2::W) writer structure"]
impl crate::Writable for ADC12MCTL2 {}
#[doc = "ADC12 Memory Control 2"]
pub mod adc12mctl2;
#[doc = "ADC12 Memory Control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl3](adc12mctl3) module"]
pub type ADC12MCTL3 = crate::Reg<u8, _ADC12MCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL3;
#[doc = "`read()` method returns [adc12mctl3::R](adc12mctl3::R) reader structure"]
impl crate::Readable for ADC12MCTL3 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl3::W](adc12mctl3::W) writer structure"]
impl crate::Writable for ADC12MCTL3 {}
#[doc = "ADC12 Memory Control 3"]
pub mod adc12mctl3;
#[doc = "ADC12 Memory Control 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl4](adc12mctl4) module"]
pub type ADC12MCTL4 = crate::Reg<u8, _ADC12MCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL4;
#[doc = "`read()` method returns [adc12mctl4::R](adc12mctl4::R) reader structure"]
impl crate::Readable for ADC12MCTL4 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl4::W](adc12mctl4::W) writer structure"]
impl crate::Writable for ADC12MCTL4 {}
#[doc = "ADC12 Memory Control 4"]
pub mod adc12mctl4;
#[doc = "ADC12 Memory Control 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl5](adc12mctl5) module"]
pub type ADC12MCTL5 = crate::Reg<u8, _ADC12MCTL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL5;
#[doc = "`read()` method returns [adc12mctl5::R](adc12mctl5::R) reader structure"]
impl crate::Readable for ADC12MCTL5 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl5::W](adc12mctl5::W) writer structure"]
impl crate::Writable for ADC12MCTL5 {}
#[doc = "ADC12 Memory Control 5"]
pub mod adc12mctl5;
#[doc = "ADC12 Memory Control 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl6](adc12mctl6) module"]
pub type ADC12MCTL6 = crate::Reg<u8, _ADC12MCTL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL6;
#[doc = "`read()` method returns [adc12mctl6::R](adc12mctl6::R) reader structure"]
impl crate::Readable for ADC12MCTL6 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl6::W](adc12mctl6::W) writer structure"]
impl crate::Writable for ADC12MCTL6 {}
#[doc = "ADC12 Memory Control 6"]
pub mod adc12mctl6;
#[doc = "ADC12 Memory Control 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl7](adc12mctl7) module"]
pub type ADC12MCTL7 = crate::Reg<u8, _ADC12MCTL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL7;
#[doc = "`read()` method returns [adc12mctl7::R](adc12mctl7::R) reader structure"]
impl crate::Readable for ADC12MCTL7 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl7::W](adc12mctl7::W) writer structure"]
impl crate::Writable for ADC12MCTL7 {}
#[doc = "ADC12 Memory Control 7"]
pub mod adc12mctl7;
#[doc = "ADC12 Memory Control 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl8](adc12mctl8) module"]
pub type ADC12MCTL8 = crate::Reg<u8, _ADC12MCTL8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL8;
#[doc = "`read()` method returns [adc12mctl8::R](adc12mctl8::R) reader structure"]
impl crate::Readable for ADC12MCTL8 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl8::W](adc12mctl8::W) writer structure"]
impl crate::Writable for ADC12MCTL8 {}
#[doc = "ADC12 Memory Control 8"]
pub mod adc12mctl8;
#[doc = "ADC12 Memory Control 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl9](adc12mctl9) module"]
pub type ADC12MCTL9 = crate::Reg<u8, _ADC12MCTL9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL9;
#[doc = "`read()` method returns [adc12mctl9::R](adc12mctl9::R) reader structure"]
impl crate::Readable for ADC12MCTL9 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl9::W](adc12mctl9::W) writer structure"]
impl crate::Writable for ADC12MCTL9 {}
#[doc = "ADC12 Memory Control 9"]
pub mod adc12mctl9;
#[doc = "ADC12 Memory Control 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl10](adc12mctl10) module"]
pub type ADC12MCTL10 = crate::Reg<u8, _ADC12MCTL10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL10;
#[doc = "`read()` method returns [adc12mctl10::R](adc12mctl10::R) reader structure"]
impl crate::Readable for ADC12MCTL10 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl10::W](adc12mctl10::W) writer structure"]
impl crate::Writable for ADC12MCTL10 {}
#[doc = "ADC12 Memory Control 10"]
pub mod adc12mctl10;
#[doc = "ADC12 Memory Control 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl11](adc12mctl11) module"]
pub type ADC12MCTL11 = crate::Reg<u8, _ADC12MCTL11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL11;
#[doc = "`read()` method returns [adc12mctl11::R](adc12mctl11::R) reader structure"]
impl crate::Readable for ADC12MCTL11 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl11::W](adc12mctl11::W) writer structure"]
impl crate::Writable for ADC12MCTL11 {}
#[doc = "ADC12 Memory Control 11"]
pub mod adc12mctl11;
#[doc = "ADC12 Memory Control 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl12](adc12mctl12) module"]
pub type ADC12MCTL12 = crate::Reg<u8, _ADC12MCTL12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL12;
#[doc = "`read()` method returns [adc12mctl12::R](adc12mctl12::R) reader structure"]
impl crate::Readable for ADC12MCTL12 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl12::W](adc12mctl12::W) writer structure"]
impl crate::Writable for ADC12MCTL12 {}
#[doc = "ADC12 Memory Control 12"]
pub mod adc12mctl12;
#[doc = "ADC12 Memory Control 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl13](adc12mctl13) module"]
pub type ADC12MCTL13 = crate::Reg<u8, _ADC12MCTL13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL13;
#[doc = "`read()` method returns [adc12mctl13::R](adc12mctl13::R) reader structure"]
impl crate::Readable for ADC12MCTL13 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl13::W](adc12mctl13::W) writer structure"]
impl crate::Writable for ADC12MCTL13 {}
#[doc = "ADC12 Memory Control 13"]
pub mod adc12mctl13;
#[doc = "ADC12 Memory Control 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl14](adc12mctl14) module"]
pub type ADC12MCTL14 = crate::Reg<u8, _ADC12MCTL14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL14;
#[doc = "`read()` method returns [adc12mctl14::R](adc12mctl14::R) reader structure"]
impl crate::Readable for ADC12MCTL14 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl14::W](adc12mctl14::W) writer structure"]
impl crate::Writable for ADC12MCTL14 {}
#[doc = "ADC12 Memory Control 14"]
pub mod adc12mctl14;
#[doc = "ADC12 Memory Control 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl15](adc12mctl15) module"]
pub type ADC12MCTL15 = crate::Reg<u8, _ADC12MCTL15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL15;
#[doc = "`read()` method returns [adc12mctl15::R](adc12mctl15::R) reader structure"]
impl crate::Readable for ADC12MCTL15 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl15::W](adc12mctl15::W) writer structure"]
impl crate::Writable for ADC12MCTL15 {}
#[doc = "ADC12 Memory Control 15"]
pub mod adc12mctl15;
#[doc = "ADC12+ Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ctl0](adc12ctl0) module"]
pub type ADC12CTL0 = crate::Reg<u16, _ADC12CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12CTL0;
#[doc = "`read()` method returns [adc12ctl0::R](adc12ctl0::R) reader structure"]
impl crate::Readable for ADC12CTL0 {}
#[doc = "`write(|w| ..)` method takes [adc12ctl0::W](adc12ctl0::W) writer structure"]
impl crate::Writable for ADC12CTL0 {}
#[doc = "ADC12+ Control 0"]
pub mod adc12ctl0;
#[doc = "ADC12+ Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ctl1](adc12ctl1) module"]
pub type ADC12CTL1 = crate::Reg<u16, _ADC12CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12CTL1;
#[doc = "`read()` method returns [adc12ctl1::R](adc12ctl1::R) reader structure"]
impl crate::Readable for ADC12CTL1 {}
#[doc = "`write(|w| ..)` method takes [adc12ctl1::W](adc12ctl1::W) writer structure"]
impl crate::Writable for ADC12CTL1 {}
#[doc = "ADC12+ Control 1"]
pub mod adc12ctl1;
#[doc = "ADC12+ Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ctl2](adc12ctl2) module"]
pub type ADC12CTL2 = crate::Reg<u16, _ADC12CTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12CTL2;
#[doc = "`read()` method returns [adc12ctl2::R](adc12ctl2::R) reader structure"]
impl crate::Readable for ADC12CTL2 {}
#[doc = "`write(|w| ..)` method takes [adc12ctl2::W](adc12ctl2::W) writer structure"]
impl crate::Writable for ADC12CTL2 {}
#[doc = "ADC12+ Control 2"]
pub mod adc12ctl2;
#[doc = "ADC12+ Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ifg](adc12ifg) module"]
pub type ADC12IFG = crate::Reg<u16, _ADC12IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12IFG;
#[doc = "`read()` method returns [adc12ifg::R](adc12ifg::R) reader structure"]
impl crate::Readable for ADC12IFG {}
#[doc = "`write(|w| ..)` method takes [adc12ifg::W](adc12ifg::W) writer structure"]
impl crate::Writable for ADC12IFG {}
#[doc = "ADC12+ Interrupt Flag"]
pub mod adc12ifg;
#[doc = "ADC12+ Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ie](adc12ie) module"]
pub type ADC12IE = crate::Reg<u16, _ADC12IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12IE;
#[doc = "`read()` method returns [adc12ie::R](adc12ie::R) reader structure"]
impl crate::Readable for ADC12IE {}
#[doc = "`write(|w| ..)` method takes [adc12ie::W](adc12ie::W) writer structure"]
impl crate::Writable for ADC12IE {}
#[doc = "ADC12+ Interrupt Enable"]
pub mod adc12ie;
#[doc = "ADC12+ Interrupt Vector Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12iv](adc12iv) module"]
pub type ADC12IV = crate::Reg<u16, _ADC12IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12IV;
#[doc = "`read()` method returns [adc12iv::R](adc12iv::R) reader structure"]
impl crate::Readable for ADC12IV {}
#[doc = "`write(|w| ..)` method takes [adc12iv::W](adc12iv::W) writer structure"]
impl crate::Writable for ADC12IV {}
#[doc = "ADC12+ Interrupt Vector Word"]
pub mod adc12iv;
#[doc = "ADC12 Conversion Memory 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem0](adc12mem0) module"]
pub type ADC12MEM0 = crate::Reg<u16, _ADC12MEM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM0;
#[doc = "`read()` method returns [adc12mem0::R](adc12mem0::R) reader structure"]
impl crate::Readable for ADC12MEM0 {}
#[doc = "`write(|w| ..)` method takes [adc12mem0::W](adc12mem0::W) writer structure"]
impl crate::Writable for ADC12MEM0 {}
#[doc = "ADC12 Conversion Memory 0"]
pub mod adc12mem0;
#[doc = "ADC12 Conversion Memory 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem1](adc12mem1) module"]
pub type ADC12MEM1 = crate::Reg<u16, _ADC12MEM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM1;
#[doc = "`read()` method returns [adc12mem1::R](adc12mem1::R) reader structure"]
impl crate::Readable for ADC12MEM1 {}
#[doc = "`write(|w| ..)` method takes [adc12mem1::W](adc12mem1::W) writer structure"]
impl crate::Writable for ADC12MEM1 {}
#[doc = "ADC12 Conversion Memory 1"]
pub mod adc12mem1;
#[doc = "ADC12 Conversion Memory 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem2](adc12mem2) module"]
pub type ADC12MEM2 = crate::Reg<u16, _ADC12MEM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM2;
#[doc = "`read()` method returns [adc12mem2::R](adc12mem2::R) reader structure"]
impl crate::Readable for ADC12MEM2 {}
#[doc = "`write(|w| ..)` method takes [adc12mem2::W](adc12mem2::W) writer structure"]
impl crate::Writable for ADC12MEM2 {}
#[doc = "ADC12 Conversion Memory 2"]
pub mod adc12mem2;
#[doc = "ADC12 Conversion Memory 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem3](adc12mem3) module"]
pub type ADC12MEM3 = crate::Reg<u16, _ADC12MEM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM3;
#[doc = "`read()` method returns [adc12mem3::R](adc12mem3::R) reader structure"]
impl crate::Readable for ADC12MEM3 {}
#[doc = "`write(|w| ..)` method takes [adc12mem3::W](adc12mem3::W) writer structure"]
impl crate::Writable for ADC12MEM3 {}
#[doc = "ADC12 Conversion Memory 3"]
pub mod adc12mem3;
#[doc = "ADC12 Conversion Memory 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem4](adc12mem4) module"]
pub type ADC12MEM4 = crate::Reg<u16, _ADC12MEM4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM4;
#[doc = "`read()` method returns [adc12mem4::R](adc12mem4::R) reader structure"]
impl crate::Readable for ADC12MEM4 {}
#[doc = "`write(|w| ..)` method takes [adc12mem4::W](adc12mem4::W) writer structure"]
impl crate::Writable for ADC12MEM4 {}
#[doc = "ADC12 Conversion Memory 4"]
pub mod adc12mem4;
#[doc = "ADC12 Conversion Memory 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem5](adc12mem5) module"]
pub type ADC12MEM5 = crate::Reg<u16, _ADC12MEM5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM5;
#[doc = "`read()` method returns [adc12mem5::R](adc12mem5::R) reader structure"]
impl crate::Readable for ADC12MEM5 {}
#[doc = "`write(|w| ..)` method takes [adc12mem5::W](adc12mem5::W) writer structure"]
impl crate::Writable for ADC12MEM5 {}
#[doc = "ADC12 Conversion Memory 5"]
pub mod adc12mem5;
#[doc = "ADC12 Conversion Memory 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem6](adc12mem6) module"]
pub type ADC12MEM6 = crate::Reg<u16, _ADC12MEM6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM6;
#[doc = "`read()` method returns [adc12mem6::R](adc12mem6::R) reader structure"]
impl crate::Readable for ADC12MEM6 {}
#[doc = "`write(|w| ..)` method takes [adc12mem6::W](adc12mem6::W) writer structure"]
impl crate::Writable for ADC12MEM6 {}
#[doc = "ADC12 Conversion Memory 6"]
pub mod adc12mem6;
#[doc = "ADC12 Conversion Memory 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem7](adc12mem7) module"]
pub type ADC12MEM7 = crate::Reg<u16, _ADC12MEM7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM7;
#[doc = "`read()` method returns [adc12mem7::R](adc12mem7::R) reader structure"]
impl crate::Readable for ADC12MEM7 {}
#[doc = "`write(|w| ..)` method takes [adc12mem7::W](adc12mem7::W) writer structure"]
impl crate::Writable for ADC12MEM7 {}
#[doc = "ADC12 Conversion Memory 7"]
pub mod adc12mem7;
#[doc = "ADC12 Conversion Memory 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem8](adc12mem8) module"]
pub type ADC12MEM8 = crate::Reg<u16, _ADC12MEM8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM8;
#[doc = "`read()` method returns [adc12mem8::R](adc12mem8::R) reader structure"]
impl crate::Readable for ADC12MEM8 {}
#[doc = "`write(|w| ..)` method takes [adc12mem8::W](adc12mem8::W) writer structure"]
impl crate::Writable for ADC12MEM8 {}
#[doc = "ADC12 Conversion Memory 8"]
pub mod adc12mem8;
#[doc = "ADC12 Conversion Memory 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem9](adc12mem9) module"]
pub type ADC12MEM9 = crate::Reg<u16, _ADC12MEM9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM9;
#[doc = "`read()` method returns [adc12mem9::R](adc12mem9::R) reader structure"]
impl crate::Readable for ADC12MEM9 {}
#[doc = "`write(|w| ..)` method takes [adc12mem9::W](adc12mem9::W) writer structure"]
impl crate::Writable for ADC12MEM9 {}
#[doc = "ADC12 Conversion Memory 9"]
pub mod adc12mem9;
#[doc = "ADC12 Conversion Memory 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem10](adc12mem10) module"]
pub type ADC12MEM10 = crate::Reg<u16, _ADC12MEM10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM10;
#[doc = "`read()` method returns [adc12mem10::R](adc12mem10::R) reader structure"]
impl crate::Readable for ADC12MEM10 {}
#[doc = "`write(|w| ..)` method takes [adc12mem10::W](adc12mem10::W) writer structure"]
impl crate::Writable for ADC12MEM10 {}
#[doc = "ADC12 Conversion Memory 10"]
pub mod adc12mem10;
#[doc = "ADC12 Conversion Memory 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem11](adc12mem11) module"]
pub type ADC12MEM11 = crate::Reg<u16, _ADC12MEM11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM11;
#[doc = "`read()` method returns [adc12mem11::R](adc12mem11::R) reader structure"]
impl crate::Readable for ADC12MEM11 {}
#[doc = "`write(|w| ..)` method takes [adc12mem11::W](adc12mem11::W) writer structure"]
impl crate::Writable for ADC12MEM11 {}
#[doc = "ADC12 Conversion Memory 11"]
pub mod adc12mem11;
#[doc = "ADC12 Conversion Memory 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem12](adc12mem12) module"]
pub type ADC12MEM12 = crate::Reg<u16, _ADC12MEM12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM12;
#[doc = "`read()` method returns [adc12mem12::R](adc12mem12::R) reader structure"]
impl crate::Readable for ADC12MEM12 {}
#[doc = "`write(|w| ..)` method takes [adc12mem12::W](adc12mem12::W) writer structure"]
impl crate::Writable for ADC12MEM12 {}
#[doc = "ADC12 Conversion Memory 12"]
pub mod adc12mem12;
#[doc = "ADC12 Conversion Memory 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem13](adc12mem13) module"]
pub type ADC12MEM13 = crate::Reg<u16, _ADC12MEM13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM13;
#[doc = "`read()` method returns [adc12mem13::R](adc12mem13::R) reader structure"]
impl crate::Readable for ADC12MEM13 {}
#[doc = "`write(|w| ..)` method takes [adc12mem13::W](adc12mem13::W) writer structure"]
impl crate::Writable for ADC12MEM13 {}
#[doc = "ADC12 Conversion Memory 13"]
pub mod adc12mem13;
#[doc = "ADC12 Conversion Memory 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem14](adc12mem14) module"]
pub type ADC12MEM14 = crate::Reg<u16, _ADC12MEM14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM14;
#[doc = "`read()` method returns [adc12mem14::R](adc12mem14::R) reader structure"]
impl crate::Readable for ADC12MEM14 {}
#[doc = "`write(|w| ..)` method takes [adc12mem14::W](adc12mem14::W) writer structure"]
impl crate::Writable for ADC12MEM14 {}
#[doc = "ADC12 Conversion Memory 14"]
pub mod adc12mem14;
#[doc = "ADC12 Conversion Memory 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem15](adc12mem15) module"]
pub type ADC12MEM15 = crate::Reg<u16, _ADC12MEM15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM15;
#[doc = "`read()` method returns [adc12mem15::R](adc12mem15::R) reader structure"]
impl crate::Readable for ADC12MEM15 {}
#[doc = "`write(|w| ..)` method takes [adc12mem15::W](adc12mem15::W) writer structure"]
impl crate::Writable for ADC12MEM15 {}
#[doc = "ADC12 Conversion Memory 15"]
pub mod adc12mem15;
