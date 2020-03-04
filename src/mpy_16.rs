#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Multiply Unsigned/Operand 1"]
    pub mpy: MPY,
    #[doc = "0x02 - Multiply Signed/Operand 1"]
    pub mpys: MPYS,
    #[doc = "0x04 - Multiply Unsigned and Accumulate/Operand 1"]
    pub mac: MAC,
    #[doc = "0x06 - Multiply Signed and Accumulate/Operand 1"]
    pub macs: MACS,
    #[doc = "0x08 - Operand 2"]
    pub op2: OP2,
    #[doc = "0x0a - Result Low Word"]
    pub reslo: RESLO,
    #[doc = "0x0c - Result High Word"]
    pub reshi: RESHI,
    #[doc = "0x0e - Sum Extend"]
    pub sumext: SUMEXT,
    _reserved8: [u8; 28usize],
    #[doc = "0x2c - MPY32 Control Register 0"]
    pub mpy32ctl0: MPY32CTL0,
}
#[doc = "Multiply Unsigned/Operand 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpy](mpy) module"]
pub type MPY = crate::Reg<u16, _MPY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPY;
#[doc = "`read()` method returns [mpy::R](mpy::R) reader structure"]
impl crate::Readable for MPY {}
#[doc = "`write(|w| ..)` method takes [mpy::W](mpy::W) writer structure"]
impl crate::Writable for MPY {}
#[doc = "Multiply Unsigned/Operand 1"]
pub mod mpy;
#[doc = "Multiply Signed/Operand 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpys](mpys) module"]
pub type MPYS = crate::Reg<u16, _MPYS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPYS;
#[doc = "`read()` method returns [mpys::R](mpys::R) reader structure"]
impl crate::Readable for MPYS {}
#[doc = "`write(|w| ..)` method takes [mpys::W](mpys::W) writer structure"]
impl crate::Writable for MPYS {}
#[doc = "Multiply Signed/Operand 1"]
pub mod mpys;
#[doc = "Multiply Unsigned and Accumulate/Operand 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac](mac) module"]
pub type MAC = crate::Reg<u16, _MAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC;
#[doc = "`read()` method returns [mac::R](mac::R) reader structure"]
impl crate::Readable for MAC {}
#[doc = "`write(|w| ..)` method takes [mac::W](mac::W) writer structure"]
impl crate::Writable for MAC {}
#[doc = "Multiply Unsigned and Accumulate/Operand 1"]
pub mod mac;
#[doc = "Multiply Signed and Accumulate/Operand 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macs](macs) module"]
pub type MACS = crate::Reg<u16, _MACS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACS;
#[doc = "`read()` method returns [macs::R](macs::R) reader structure"]
impl crate::Readable for MACS {}
#[doc = "`write(|w| ..)` method takes [macs::W](macs::W) writer structure"]
impl crate::Writable for MACS {}
#[doc = "Multiply Signed and Accumulate/Operand 1"]
pub mod macs;
#[doc = "Operand 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [op2](op2) module"]
pub type OP2 = crate::Reg<u16, _OP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OP2;
#[doc = "`read()` method returns [op2::R](op2::R) reader structure"]
impl crate::Readable for OP2 {}
#[doc = "`write(|w| ..)` method takes [op2::W](op2::W) writer structure"]
impl crate::Writable for OP2 {}
#[doc = "Operand 2"]
pub mod op2;
#[doc = "Result Low Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reslo](reslo) module"]
pub type RESLO = crate::Reg<u16, _RESLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESLO;
#[doc = "`read()` method returns [reslo::R](reslo::R) reader structure"]
impl crate::Readable for RESLO {}
#[doc = "`write(|w| ..)` method takes [reslo::W](reslo::W) writer structure"]
impl crate::Writable for RESLO {}
#[doc = "Result Low Word"]
pub mod reslo;
#[doc = "Result High Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reshi](reshi) module"]
pub type RESHI = crate::Reg<u16, _RESHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESHI;
#[doc = "`read()` method returns [reshi::R](reshi::R) reader structure"]
impl crate::Readable for RESHI {}
#[doc = "`write(|w| ..)` method takes [reshi::W](reshi::W) writer structure"]
impl crate::Writable for RESHI {}
#[doc = "Result High Word"]
pub mod reshi;
#[doc = "Sum Extend\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sumext](sumext) module"]
pub type SUMEXT = crate::Reg<u16, _SUMEXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUMEXT;
#[doc = "`read()` method returns [sumext::R](sumext::R) reader structure"]
impl crate::Readable for SUMEXT {}
#[doc = "`write(|w| ..)` method takes [sumext::W](sumext::W) writer structure"]
impl crate::Writable for SUMEXT {}
#[doc = "Sum Extend"]
pub mod sumext;
#[doc = "MPY32 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpy32ctl0](mpy32ctl0) module"]
pub type MPY32CTL0 = crate::Reg<u16, _MPY32CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPY32CTL0;
#[doc = "`read()` method returns [mpy32ctl0::R](mpy32ctl0::R) reader structure"]
impl crate::Readable for MPY32CTL0 {}
#[doc = "`write(|w| ..)` method takes [mpy32ctl0::W](mpy32ctl0::W) writer structure"]
impl crate::Writable for MPY32CTL0 {}
#[doc = "MPY32 Control Register 0"]
pub mod mpy32ctl0;
