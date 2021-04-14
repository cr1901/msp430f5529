#[doc = "Register `MPY32CTL0` reader"]
pub struct R(crate::R<MPY32CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPY32CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPY32CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPY32CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPY32CTL0` writer"]
pub struct W(crate::W<MPY32CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPY32CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MPY32CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPY32CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPYC` reader - Carry of the multiplier"]
pub struct MPYC_R(crate::FieldReader<bool, bool>);
impl MPYC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MPYC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPYC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPYC` writer - Carry of the multiplier"]
pub struct MPYC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `MPYFRAC` reader - Fractional mode"]
pub struct MPYFRAC_R(crate::FieldReader<bool, bool>);
impl MPYFRAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MPYFRAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPYFRAC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPYFRAC` writer - Fractional mode"]
pub struct MPYFRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYFRAC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `MPYSAT` reader - Saturation mode"]
pub struct MPYSAT_R(crate::FieldReader<bool, bool>);
impl MPYSAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MPYSAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPYSAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPYSAT` writer - Saturation mode"]
pub struct MPYSAT_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYSAT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Multiplier mode Bit:0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MPYM_A {
    #[doc = "0: Multiplier mode: MPY"]
    MPYM_0 = 0,
    #[doc = "1: Multiplier mode: MPYS"]
    MPYM_1 = 1,
    #[doc = "2: Multiplier mode: MAC"]
    MPYM_2 = 2,
    #[doc = "3: Multiplier mode: MACS"]
    MPYM_3 = 3,
}
impl From<MPYM_A> for u8 {
    #[inline(always)]
    fn from(variant: MPYM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MPYM` reader - Multiplier mode Bit:0"]
pub struct MPYM_R(crate::FieldReader<u8, MPYM_A>);
impl MPYM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MPYM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPYM_A {
        match self.bits {
            0 => MPYM_A::MPYM_0,
            1 => MPYM_A::MPYM_1,
            2 => MPYM_A::MPYM_2,
            3 => MPYM_A::MPYM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MPYM_0`"]
    #[inline(always)]
    pub fn is_mpym_0(&self) -> bool {
        **self == MPYM_A::MPYM_0
    }
    #[doc = "Checks if the value of the field is `MPYM_1`"]
    #[inline(always)]
    pub fn is_mpym_1(&self) -> bool {
        **self == MPYM_A::MPYM_1
    }
    #[doc = "Checks if the value of the field is `MPYM_2`"]
    #[inline(always)]
    pub fn is_mpym_2(&self) -> bool {
        **self == MPYM_A::MPYM_2
    }
    #[doc = "Checks if the value of the field is `MPYM_3`"]
    #[inline(always)]
    pub fn is_mpym_3(&self) -> bool {
        **self == MPYM_A::MPYM_3
    }
}
impl core::ops::Deref for MPYM_R {
    type Target = crate::FieldReader<u8, MPYM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPYM` writer - Multiplier mode Bit:0"]
pub struct MPYM_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPYM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Multiplier mode: MPY"]
    #[inline(always)]
    pub fn mpym_0(self) -> &'a mut W {
        self.variant(MPYM_A::MPYM_0)
    }
    #[doc = "Multiplier mode: MPYS"]
    #[inline(always)]
    pub fn mpym_1(self) -> &'a mut W {
        self.variant(MPYM_A::MPYM_1)
    }
    #[doc = "Multiplier mode: MAC"]
    #[inline(always)]
    pub fn mpym_2(self) -> &'a mut W {
        self.variant(MPYM_A::MPYM_2)
    }
    #[doc = "Multiplier mode: MACS"]
    #[inline(always)]
    pub fn mpym_3(self) -> &'a mut W {
        self.variant(MPYM_A::MPYM_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u16 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `OP1_32` reader - Bit-width of operand 1 0:16Bit / 1:32Bit"]
pub struct OP1_32_R(crate::FieldReader<bool, bool>);
impl OP1_32_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OP1_32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OP1_32_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OP1_32` writer - Bit-width of operand 1 0:16Bit / 1:32Bit"]
pub struct OP1_32_W<'a> {
    w: &'a mut W,
}
impl<'a> OP1_32_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `OP2_32` reader - Bit-width of operand 2 0:16Bit / 1:32Bit"]
pub struct OP2_32_R(crate::FieldReader<bool, bool>);
impl OP2_32_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OP2_32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OP2_32_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OP2_32` writer - Bit-width of operand 2 0:16Bit / 1:32Bit"]
pub struct OP2_32_W<'a> {
    w: &'a mut W,
}
impl<'a> OP2_32_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `MPYDLYWRTEN` reader - Delayed write enable"]
pub struct MPYDLYWRTEN_R(crate::FieldReader<bool, bool>);
impl MPYDLYWRTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MPYDLYWRTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPYDLYWRTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPYDLYWRTEN` writer - Delayed write enable"]
pub struct MPYDLYWRTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYDLYWRTEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `MPYDLY32` reader - Delayed write mode"]
pub struct MPYDLY32_R(crate::FieldReader<bool, bool>);
impl MPYDLY32_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MPYDLY32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPYDLY32_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPYDLY32` writer - Delayed write mode"]
pub struct MPYDLY32_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYDLY32_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Carry of the multiplier"]
    #[inline(always)]
    pub fn mpyc(&self) -> MPYC_R {
        MPYC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fractional mode"]
    #[inline(always)]
    pub fn mpyfrac(&self) -> MPYFRAC_R {
        MPYFRAC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Saturation mode"]
    #[inline(always)]
    pub fn mpysat(&self) -> MPYSAT_R {
        MPYSAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Multiplier mode Bit:0"]
    #[inline(always)]
    pub fn mpym(&self) -> MPYM_R {
        MPYM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Bit-width of operand 1 0:16Bit / 1:32Bit"]
    #[inline(always)]
    pub fn op1_32(&self) -> OP1_32_R {
        OP1_32_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bit-width of operand 2 0:16Bit / 1:32Bit"]
    #[inline(always)]
    pub fn op2_32(&self) -> OP2_32_R {
        OP2_32_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Delayed write enable"]
    #[inline(always)]
    pub fn mpydlywrten(&self) -> MPYDLYWRTEN_R {
        MPYDLYWRTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Delayed write mode"]
    #[inline(always)]
    pub fn mpydly32(&self) -> MPYDLY32_R {
        MPYDLY32_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Carry of the multiplier"]
    #[inline(always)]
    pub fn mpyc(&mut self) -> MPYC_W {
        MPYC_W { w: self }
    }
    #[doc = "Bit 2 - Fractional mode"]
    #[inline(always)]
    pub fn mpyfrac(&mut self) -> MPYFRAC_W {
        MPYFRAC_W { w: self }
    }
    #[doc = "Bit 3 - Saturation mode"]
    #[inline(always)]
    pub fn mpysat(&mut self) -> MPYSAT_W {
        MPYSAT_W { w: self }
    }
    #[doc = "Bits 4:5 - Multiplier mode Bit:0"]
    #[inline(always)]
    pub fn mpym(&mut self) -> MPYM_W {
        MPYM_W { w: self }
    }
    #[doc = "Bit 6 - Bit-width of operand 1 0:16Bit / 1:32Bit"]
    #[inline(always)]
    pub fn op1_32(&mut self) -> OP1_32_W {
        OP1_32_W { w: self }
    }
    #[doc = "Bit 7 - Bit-width of operand 2 0:16Bit / 1:32Bit"]
    #[inline(always)]
    pub fn op2_32(&mut self) -> OP2_32_W {
        OP2_32_W { w: self }
    }
    #[doc = "Bit 8 - Delayed write enable"]
    #[inline(always)]
    pub fn mpydlywrten(&mut self) -> MPYDLYWRTEN_W {
        MPYDLYWRTEN_W { w: self }
    }
    #[doc = "Bit 9 - Delayed write mode"]
    #[inline(always)]
    pub fn mpydly32(&mut self) -> MPYDLY32_W {
        MPYDLY32_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPY32 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpy32ctl0](index.html) module"]
pub struct MPY32CTL0_SPEC;
impl crate::RegisterSpec for MPY32CTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mpy32ctl0::R](R) reader structure"]
impl crate::Readable for MPY32CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpy32ctl0::W](W) writer structure"]
impl crate::Writable for MPY32CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPY32CTL0 to value 0"]
impl crate::Resettable for MPY32CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
