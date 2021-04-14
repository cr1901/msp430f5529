#[doc = "Register `RTCADAY` reader"]
pub struct R(crate::R<RTCADAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCADAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCADAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCADAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCADAY` writer"]
pub struct W(crate::W<RTCADAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCADAY_SPEC>;
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
impl From<crate::W<RTCADAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCADAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAY0` reader - Real Time Clock Day Bit: 0"]
pub struct DAY0_R(crate::FieldReader<bool, bool>);
impl DAY0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAY0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAY0` writer - Real Time Clock Day Bit: 0"]
pub struct DAY0_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY0_W<'a> {
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
#[doc = "Field `DAY1` reader - Real Time Clock Day Bit: 1"]
pub struct DAY1_R(crate::FieldReader<bool, bool>);
impl DAY1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAY1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAY1` writer - Real Time Clock Day Bit: 1"]
pub struct DAY1_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY1_W<'a> {
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
#[doc = "Field `DAY2` reader - Real Time Clock Day Bit: 2"]
pub struct DAY2_R(crate::FieldReader<bool, bool>);
impl DAY2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAY2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAY2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAY2` writer - Real Time Clock Day Bit: 2"]
pub struct DAY2_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY2_W<'a> {
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
#[doc = "Field `DAY3` reader - Real Time Clock Day Bit: 3"]
pub struct DAY3_R(crate::FieldReader<bool, bool>);
impl DAY3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAY3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAY3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAY3` writer - Real Time Clock Day Bit: 3"]
pub struct DAY3_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY3_W<'a> {
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
#[doc = "Field `DAY4` reader - Real Time Clock Day Bit: 4"]
pub struct DAY4_R(crate::FieldReader<bool, bool>);
impl DAY4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAY4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAY4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAY4` writer - Real Time Clock Day Bit: 4"]
pub struct DAY4_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY4_W<'a> {
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
#[doc = "Field `DAY5` reader - Real Time Clock Day Bit: 5"]
pub struct DAY5_R(crate::FieldReader<bool, bool>);
impl DAY5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAY5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAY5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAY5` writer - Real Time Clock Day Bit: 5"]
pub struct DAY5_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY5_W<'a> {
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
#[doc = "Field `DAY6` reader - Real Time Clock Day Bit: 6"]
pub struct DAY6_R(crate::FieldReader<bool, bool>);
impl DAY6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAY6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAY6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAY6` writer - Real Time Clock Day Bit: 6"]
pub struct DAY6_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY6_W<'a> {
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
#[doc = "Field `RTCAE` reader - Real Time Clock Alarm enable"]
pub struct RTCAE_R(crate::FieldReader<bool, bool>);
impl RTCAE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTCAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCAE` writer - Real Time Clock Alarm enable"]
pub struct RTCAE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Real Time Clock Day Bit: 0"]
    #[inline(always)]
    pub fn day0(&self) -> DAY0_R {
        DAY0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock Day Bit: 1"]
    #[inline(always)]
    pub fn day1(&self) -> DAY1_R {
        DAY1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock Day Bit: 2"]
    #[inline(always)]
    pub fn day2(&self) -> DAY2_R {
        DAY2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Day Bit: 3"]
    #[inline(always)]
    pub fn day3(&self) -> DAY3_R {
        DAY3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock Day Bit: 4"]
    #[inline(always)]
    pub fn day4(&self) -> DAY4_R {
        DAY4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock Day Bit: 5"]
    #[inline(always)]
    pub fn day5(&self) -> DAY5_R {
        DAY5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock Day Bit: 6"]
    #[inline(always)]
    pub fn day6(&self) -> DAY6_R {
        DAY6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Real Time Clock Alarm enable"]
    #[inline(always)]
    pub fn rtcae(&self) -> RTCAE_R {
        RTCAE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Day Bit: 0"]
    #[inline(always)]
    pub fn day0(&mut self) -> DAY0_W {
        DAY0_W { w: self }
    }
    #[doc = "Bit 1 - Real Time Clock Day Bit: 1"]
    #[inline(always)]
    pub fn day1(&mut self) -> DAY1_W {
        DAY1_W { w: self }
    }
    #[doc = "Bit 2 - Real Time Clock Day Bit: 2"]
    #[inline(always)]
    pub fn day2(&mut self) -> DAY2_W {
        DAY2_W { w: self }
    }
    #[doc = "Bit 3 - Real Time Clock Day Bit: 3"]
    #[inline(always)]
    pub fn day3(&mut self) -> DAY3_W {
        DAY3_W { w: self }
    }
    #[doc = "Bit 4 - Real Time Clock Day Bit: 4"]
    #[inline(always)]
    pub fn day4(&mut self) -> DAY4_W {
        DAY4_W { w: self }
    }
    #[doc = "Bit 5 - Real Time Clock Day Bit: 5"]
    #[inline(always)]
    pub fn day5(&mut self) -> DAY5_W {
        DAY5_W { w: self }
    }
    #[doc = "Bit 6 - Real Time Clock Day Bit: 6"]
    #[inline(always)]
    pub fn day6(&mut self) -> DAY6_W {
        DAY6_W { w: self }
    }
    #[doc = "Bit 7 - Real Time Clock Alarm enable"]
    #[inline(always)]
    pub fn rtcae(&mut self) -> RTCAE_W {
        RTCAE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Time Clock Alarm Day\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcaday](index.html) module"]
pub struct RTCADAY_SPEC;
impl crate::RegisterSpec for RTCADAY_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rtcaday::R](R) reader structure"]
impl crate::Readable for RTCADAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcaday::W](W) writer structure"]
impl crate::Writable for RTCADAY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCADAY to value 0"]
impl crate::Resettable for RTCADAY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
