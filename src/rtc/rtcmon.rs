#[doc = "Register `RTCMON` reader"]
pub struct R(crate::R<RTCMON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCMON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCMON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCMON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCMON` writer"]
pub struct W(crate::W<RTCMON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCMON_SPEC>;
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
impl From<crate::W<RTCMON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCMON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MONTH0` reader - Real Time Clock Month Bit: 0"]
pub struct MONTH0_R(crate::FieldReader<bool, bool>);
impl MONTH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MONTH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONTH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONTH0` writer - Real Time Clock Month Bit: 0"]
pub struct MONTH0_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `MONTH1` reader - Real Time Clock Month Bit: 1"]
pub struct MONTH1_R(crate::FieldReader<bool, bool>);
impl MONTH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MONTH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONTH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONTH1` writer - Real Time Clock Month Bit: 1"]
pub struct MONTH1_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `MONTH2` reader - Real Time Clock Month Bit: 2"]
pub struct MONTH2_R(crate::FieldReader<bool, bool>);
impl MONTH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MONTH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONTH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONTH2` writer - Real Time Clock Month Bit: 2"]
pub struct MONTH2_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `MONTH3` reader - Real Time Clock Month Bit: 3"]
pub struct MONTH3_R(crate::FieldReader<bool, bool>);
impl MONTH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MONTH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONTH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONTH3` writer - Real Time Clock Month Bit: 3"]
pub struct MONTH3_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `MONTH4` reader - Real Time Clock Month Bit: 4"]
pub struct MONTH4_R(crate::FieldReader<bool, bool>);
impl MONTH4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MONTH4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONTH4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONTH4` writer - Real Time Clock Month Bit: 4"]
pub struct MONTH4_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `MONTH5` reader - Real Time Clock Month Bit: 5"]
pub struct MONTH5_R(crate::FieldReader<bool, bool>);
impl MONTH5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MONTH5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONTH5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONTH5` writer - Real Time Clock Month Bit: 5"]
pub struct MONTH5_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `MONTH6` reader - Real Time Clock Month Bit: 6"]
pub struct MONTH6_R(crate::FieldReader<bool, bool>);
impl MONTH6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MONTH6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONTH6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONTH6` writer - Real Time Clock Month Bit: 6"]
pub struct MONTH6_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Real Time Clock Month Bit: 0"]
    #[inline(always)]
    pub fn month0(&self) -> MONTH0_R {
        MONTH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock Month Bit: 1"]
    #[inline(always)]
    pub fn month1(&self) -> MONTH1_R {
        MONTH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock Month Bit: 2"]
    #[inline(always)]
    pub fn month2(&self) -> MONTH2_R {
        MONTH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Month Bit: 3"]
    #[inline(always)]
    pub fn month3(&self) -> MONTH3_R {
        MONTH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock Month Bit: 4"]
    #[inline(always)]
    pub fn month4(&self) -> MONTH4_R {
        MONTH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock Month Bit: 5"]
    #[inline(always)]
    pub fn month5(&self) -> MONTH5_R {
        MONTH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock Month Bit: 6"]
    #[inline(always)]
    pub fn month6(&self) -> MONTH6_R {
        MONTH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Month Bit: 0"]
    #[inline(always)]
    pub fn month0(&mut self) -> MONTH0_W {
        MONTH0_W { w: self }
    }
    #[doc = "Bit 1 - Real Time Clock Month Bit: 1"]
    #[inline(always)]
    pub fn month1(&mut self) -> MONTH1_W {
        MONTH1_W { w: self }
    }
    #[doc = "Bit 2 - Real Time Clock Month Bit: 2"]
    #[inline(always)]
    pub fn month2(&mut self) -> MONTH2_W {
        MONTH2_W { w: self }
    }
    #[doc = "Bit 3 - Real Time Clock Month Bit: 3"]
    #[inline(always)]
    pub fn month3(&mut self) -> MONTH3_W {
        MONTH3_W { w: self }
    }
    #[doc = "Bit 4 - Real Time Clock Month Bit: 4"]
    #[inline(always)]
    pub fn month4(&mut self) -> MONTH4_W {
        MONTH4_W { w: self }
    }
    #[doc = "Bit 5 - Real Time Clock Month Bit: 5"]
    #[inline(always)]
    pub fn month5(&mut self) -> MONTH5_W {
        MONTH5_W { w: self }
    }
    #[doc = "Bit 6 - Real Time Clock Month Bit: 6"]
    #[inline(always)]
    pub fn month6(&mut self) -> MONTH6_W {
        MONTH6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Time Clock Month\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcmon](index.html) module"]
pub struct RTCMON_SPEC;
impl crate::RegisterSpec for RTCMON_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rtcmon::R](R) reader structure"]
impl crate::Readable for RTCMON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcmon::W](W) writer structure"]
impl crate::Writable for RTCMON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCMON to value 0"]
impl crate::Resettable for RTCMON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
