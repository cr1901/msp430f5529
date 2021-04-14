#[doc = "Register `RTCADOW` reader"]
pub struct R(crate::R<RTCADOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCADOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCADOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCADOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCADOW` writer"]
pub struct W(crate::W<RTCADOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCADOW_SPEC>;
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
impl From<crate::W<RTCADOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCADOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOW0` reader - Real Time Clock DOW Bit: 0"]
pub struct DOW0_R(crate::FieldReader<bool, bool>);
impl DOW0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOW0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOW0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOW0` writer - Real Time Clock DOW Bit: 0"]
pub struct DOW0_W<'a> {
    w: &'a mut W,
}
impl<'a> DOW0_W<'a> {
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
#[doc = "Field `DOW1` reader - Real Time Clock DOW Bit: 1"]
pub struct DOW1_R(crate::FieldReader<bool, bool>);
impl DOW1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOW1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOW1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOW1` writer - Real Time Clock DOW Bit: 1"]
pub struct DOW1_W<'a> {
    w: &'a mut W,
}
impl<'a> DOW1_W<'a> {
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
#[doc = "Field `DOW2` reader - Real Time Clock DOW Bit: 2"]
pub struct DOW2_R(crate::FieldReader<bool, bool>);
impl DOW2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOW2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOW2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOW2` writer - Real Time Clock DOW Bit: 2"]
pub struct DOW2_W<'a> {
    w: &'a mut W,
}
impl<'a> DOW2_W<'a> {
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
#[doc = "Field `DOW3` reader - Real Time Clock DOW Bit: 3"]
pub struct DOW3_R(crate::FieldReader<bool, bool>);
impl DOW3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOW3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOW3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOW3` writer - Real Time Clock DOW Bit: 3"]
pub struct DOW3_W<'a> {
    w: &'a mut W,
}
impl<'a> DOW3_W<'a> {
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
#[doc = "Field `DOW4` reader - Real Time Clock DOW Bit: 4"]
pub struct DOW4_R(crate::FieldReader<bool, bool>);
impl DOW4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOW4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOW4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOW4` writer - Real Time Clock DOW Bit: 4"]
pub struct DOW4_W<'a> {
    w: &'a mut W,
}
impl<'a> DOW4_W<'a> {
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
#[doc = "Field `DOW5` reader - Real Time Clock DOW Bit: 5"]
pub struct DOW5_R(crate::FieldReader<bool, bool>);
impl DOW5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOW5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOW5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOW5` writer - Real Time Clock DOW Bit: 5"]
pub struct DOW5_W<'a> {
    w: &'a mut W,
}
impl<'a> DOW5_W<'a> {
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
#[doc = "Field `DOW6` reader - Real Time Clock DOW Bit: 6"]
pub struct DOW6_R(crate::FieldReader<bool, bool>);
impl DOW6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOW6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOW6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOW6` writer - Real Time Clock DOW Bit: 6"]
pub struct DOW6_W<'a> {
    w: &'a mut W,
}
impl<'a> DOW6_W<'a> {
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
    #[doc = "Bit 0 - Real Time Clock DOW Bit: 0"]
    #[inline(always)]
    pub fn dow0(&self) -> DOW0_R {
        DOW0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock DOW Bit: 1"]
    #[inline(always)]
    pub fn dow1(&self) -> DOW1_R {
        DOW1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock DOW Bit: 2"]
    #[inline(always)]
    pub fn dow2(&self) -> DOW2_R {
        DOW2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock DOW Bit: 3"]
    #[inline(always)]
    pub fn dow3(&self) -> DOW3_R {
        DOW3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock DOW Bit: 4"]
    #[inline(always)]
    pub fn dow4(&self) -> DOW4_R {
        DOW4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock DOW Bit: 5"]
    #[inline(always)]
    pub fn dow5(&self) -> DOW5_R {
        DOW5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock DOW Bit: 6"]
    #[inline(always)]
    pub fn dow6(&self) -> DOW6_R {
        DOW6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Real Time Clock Alarm enable"]
    #[inline(always)]
    pub fn rtcae(&self) -> RTCAE_R {
        RTCAE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock DOW Bit: 0"]
    #[inline(always)]
    pub fn dow0(&mut self) -> DOW0_W {
        DOW0_W { w: self }
    }
    #[doc = "Bit 1 - Real Time Clock DOW Bit: 1"]
    #[inline(always)]
    pub fn dow1(&mut self) -> DOW1_W {
        DOW1_W { w: self }
    }
    #[doc = "Bit 2 - Real Time Clock DOW Bit: 2"]
    #[inline(always)]
    pub fn dow2(&mut self) -> DOW2_W {
        DOW2_W { w: self }
    }
    #[doc = "Bit 3 - Real Time Clock DOW Bit: 3"]
    #[inline(always)]
    pub fn dow3(&mut self) -> DOW3_W {
        DOW3_W { w: self }
    }
    #[doc = "Bit 4 - Real Time Clock DOW Bit: 4"]
    #[inline(always)]
    pub fn dow4(&mut self) -> DOW4_W {
        DOW4_W { w: self }
    }
    #[doc = "Bit 5 - Real Time Clock DOW Bit: 5"]
    #[inline(always)]
    pub fn dow5(&mut self) -> DOW5_W {
        DOW5_W { w: self }
    }
    #[doc = "Bit 6 - Real Time Clock DOW Bit: 6"]
    #[inline(always)]
    pub fn dow6(&mut self) -> DOW6_W {
        DOW6_W { w: self }
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
#[doc = "Real Time Clock Alarm Day of week\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcadow](index.html) module"]
pub struct RTCADOW_SPEC;
impl crate::RegisterSpec for RTCADOW_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rtcadow::R](R) reader structure"]
impl crate::Readable for RTCADOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcadow::W](W) writer structure"]
impl crate::Writable for RTCADOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCADOW to value 0"]
impl crate::Resettable for RTCADOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
