#[doc = "Register `RTCCTL23` reader"]
pub struct R(crate::R<RTCCTL23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCTL23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCTL23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCTL23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCCTL23` writer"]
pub struct W(crate::W<RTCCTL23_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCTL23_SPEC>;
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
impl From<crate::W<RTCCTL23_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCTL23_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCCAL0` reader - RTC Calibration Bit 0"]
pub struct RTCCAL0_R(crate::FieldReader<bool, bool>);
impl RTCCAL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTCCAL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCCAL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCAL0` writer - RTC Calibration Bit 0"]
pub struct RTCCAL0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCAL0_W<'a> {
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
#[doc = "Field `RTCCAL1` reader - RTC Calibration Bit 1"]
pub struct RTCCAL1_R(crate::FieldReader<bool, bool>);
impl RTCCAL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTCCAL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCCAL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCAL1` writer - RTC Calibration Bit 1"]
pub struct RTCCAL1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCAL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RTCCAL2` reader - RTC Calibration Bit 2"]
pub struct RTCCAL2_R(crate::FieldReader<bool, bool>);
impl RTCCAL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTCCAL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCCAL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCAL2` writer - RTC Calibration Bit 2"]
pub struct RTCCAL2_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCAL2_W<'a> {
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
#[doc = "Field `RTCCAL3` reader - RTC Calibration Bit 3"]
pub struct RTCCAL3_R(crate::FieldReader<bool, bool>);
impl RTCCAL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTCCAL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCCAL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCAL3` writer - RTC Calibration Bit 3"]
pub struct RTCCAL3_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCAL3_W<'a> {
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
#[doc = "Field `RTCCAL4` reader - RTC Calibration Bit 4"]
pub struct RTCCAL4_R(crate::FieldReader<bool, bool>);
impl RTCCAL4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTCCAL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCCAL4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCAL4` writer - RTC Calibration Bit 4"]
pub struct RTCCAL4_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCAL4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `RTCCAL5` reader - RTC Calibration Bit 5"]
pub struct RTCCAL5_R(crate::FieldReader<bool, bool>);
impl RTCCAL5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTCCAL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCCAL5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCAL5` writer - RTC Calibration Bit 5"]
pub struct RTCCAL5_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCAL5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RTCCALS` reader - RTC Calibration Sign"]
pub struct RTCCALS_R(crate::FieldReader<bool, bool>);
impl RTCCALS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTCCALS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCCALS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCALS` writer - RTC Calibration Sign"]
pub struct RTCCALS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCALS_W<'a> {
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
#[doc = "RTC Calibration Frequency Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCCALF_A {
    #[doc = "0: RTC Calibration Frequency: No Output"]
    RTCCALF_0 = 0,
    #[doc = "1: RTC Calibration Frequency: 512 Hz"]
    RTCCALF_1 = 1,
    #[doc = "2: RTC Calibration Frequency: 256 Hz"]
    RTCCALF_2 = 2,
    #[doc = "3: RTC Calibration Frequency: 1 Hz"]
    RTCCALF_3 = 3,
}
impl From<RTCCALF_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCCALF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RTCCALF` reader - RTC Calibration Frequency Bit 1"]
pub struct RTCCALF_R(crate::FieldReader<u8, RTCCALF_A>);
impl RTCCALF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTCCALF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCCALF_A {
        match self.bits {
            0 => RTCCALF_A::RTCCALF_0,
            1 => RTCCALF_A::RTCCALF_1,
            2 => RTCCALF_A::RTCCALF_2,
            3 => RTCCALF_A::RTCCALF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTCCALF_0`"]
    #[inline(always)]
    pub fn is_rtccalf_0(&self) -> bool {
        **self == RTCCALF_A::RTCCALF_0
    }
    #[doc = "Checks if the value of the field is `RTCCALF_1`"]
    #[inline(always)]
    pub fn is_rtccalf_1(&self) -> bool {
        **self == RTCCALF_A::RTCCALF_1
    }
    #[doc = "Checks if the value of the field is `RTCCALF_2`"]
    #[inline(always)]
    pub fn is_rtccalf_2(&self) -> bool {
        **self == RTCCALF_A::RTCCALF_2
    }
    #[doc = "Checks if the value of the field is `RTCCALF_3`"]
    #[inline(always)]
    pub fn is_rtccalf_3(&self) -> bool {
        **self == RTCCALF_A::RTCCALF_3
    }
}
impl core::ops::Deref for RTCCALF_R {
    type Target = crate::FieldReader<u8, RTCCALF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCALF` writer - RTC Calibration Frequency Bit 1"]
pub struct RTCCALF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCALF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCCALF_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "RTC Calibration Frequency: No Output"]
    #[inline(always)]
    pub fn rtccalf_0(self) -> &'a mut W {
        self.variant(RTCCALF_A::RTCCALF_0)
    }
    #[doc = "RTC Calibration Frequency: 512 Hz"]
    #[inline(always)]
    pub fn rtccalf_1(self) -> &'a mut W {
        self.variant(RTCCALF_A::RTCCALF_1)
    }
    #[doc = "RTC Calibration Frequency: 256 Hz"]
    #[inline(always)]
    pub fn rtccalf_2(self) -> &'a mut W {
        self.variant(RTCCALF_A::RTCCALF_2)
    }
    #[doc = "RTC Calibration Frequency: 1 Hz"]
    #[inline(always)]
    pub fn rtccalf_3(self) -> &'a mut W {
        self.variant(RTCCALF_A::RTCCALF_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u16 & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RTC Calibration Bit 0"]
    #[inline(always)]
    pub fn rtccal0(&self) -> RTCCAL0_R {
        RTCCAL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC Calibration Bit 1"]
    #[inline(always)]
    pub fn rtccal1(&self) -> RTCCAL1_R {
        RTCCAL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTC Calibration Bit 2"]
    #[inline(always)]
    pub fn rtccal2(&self) -> RTCCAL2_R {
        RTCCAL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC Calibration Bit 3"]
    #[inline(always)]
    pub fn rtccal3(&self) -> RTCCAL3_R {
        RTCCAL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTC Calibration Bit 4"]
    #[inline(always)]
    pub fn rtccal4(&self) -> RTCCAL4_R {
        RTCCAL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RTC Calibration Bit 5"]
    #[inline(always)]
    pub fn rtccal5(&self) -> RTCCAL5_R {
        RTCCAL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RTC Calibration Sign"]
    #[inline(always)]
    pub fn rtccals(&self) -> RTCCALS_R {
        RTCCALS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - RTC Calibration Frequency Bit 1"]
    #[inline(always)]
    pub fn rtccalf(&self) -> RTCCALF_R {
        RTCCALF_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Calibration Bit 0"]
    #[inline(always)]
    pub fn rtccal0(&mut self) -> RTCCAL0_W {
        RTCCAL0_W { w: self }
    }
    #[doc = "Bit 1 - RTC Calibration Bit 1"]
    #[inline(always)]
    pub fn rtccal1(&mut self) -> RTCCAL1_W {
        RTCCAL1_W { w: self }
    }
    #[doc = "Bit 2 - RTC Calibration Bit 2"]
    #[inline(always)]
    pub fn rtccal2(&mut self) -> RTCCAL2_W {
        RTCCAL2_W { w: self }
    }
    #[doc = "Bit 3 - RTC Calibration Bit 3"]
    #[inline(always)]
    pub fn rtccal3(&mut self) -> RTCCAL3_W {
        RTCCAL3_W { w: self }
    }
    #[doc = "Bit 4 - RTC Calibration Bit 4"]
    #[inline(always)]
    pub fn rtccal4(&mut self) -> RTCCAL4_W {
        RTCCAL4_W { w: self }
    }
    #[doc = "Bit 5 - RTC Calibration Bit 5"]
    #[inline(always)]
    pub fn rtccal5(&mut self) -> RTCCAL5_W {
        RTCCAL5_W { w: self }
    }
    #[doc = "Bit 7 - RTC Calibration Sign"]
    #[inline(always)]
    pub fn rtccals(&mut self) -> RTCCALS_W {
        RTCCALS_W { w: self }
    }
    #[doc = "Bits 8:9 - RTC Calibration Frequency Bit 1"]
    #[inline(always)]
    pub fn rtccalf(&mut self) -> RTCCALF_W {
        RTCCALF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Timer Control 2/3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcctl23](index.html) module"]
pub struct RTCCTL23_SPEC;
impl crate::RegisterSpec for RTCCTL23_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcctl23::R](R) reader structure"]
impl crate::Readable for RTCCTL23_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcctl23::W](W) writer structure"]
impl crate::Writable for RTCCTL23_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCCTL23 to value 0"]
impl crate::Resettable for RTCCTL23_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
