#[doc = "Register `RTCHOUR` reader"]
pub struct R(crate::R<RTCHOUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCHOUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCHOUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCHOUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCHOUR` writer"]
pub struct W(crate::W<RTCHOUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCHOUR_SPEC>;
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
impl From<crate::W<RTCHOUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCHOUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOUR0` reader - Real Time Clock Hour Bit: 0"]
pub struct HOUR0_R(crate::FieldReader<bool, bool>);
impl HOUR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOUR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOUR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOUR0` writer - Real Time Clock Hour Bit: 0"]
pub struct HOUR0_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR0_W<'a> {
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
#[doc = "Field `HOUR1` reader - Real Time Clock Hour Bit: 1"]
pub struct HOUR1_R(crate::FieldReader<bool, bool>);
impl HOUR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOUR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOUR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOUR1` writer - Real Time Clock Hour Bit: 1"]
pub struct HOUR1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR1_W<'a> {
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
#[doc = "Field `HOUR2` reader - Real Time Clock Hour Bit: 2"]
pub struct HOUR2_R(crate::FieldReader<bool, bool>);
impl HOUR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOUR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOUR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOUR2` writer - Real Time Clock Hour Bit: 2"]
pub struct HOUR2_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR2_W<'a> {
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
#[doc = "Field `HOUR3` reader - Real Time Clock Hour Bit: 3"]
pub struct HOUR3_R(crate::FieldReader<bool, bool>);
impl HOUR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOUR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOUR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOUR3` writer - Real Time Clock Hour Bit: 3"]
pub struct HOUR3_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR3_W<'a> {
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
#[doc = "Field `HOUR4` reader - Real Time Clock Hour Bit: 4"]
pub struct HOUR4_R(crate::FieldReader<bool, bool>);
impl HOUR4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOUR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOUR4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOUR4` writer - Real Time Clock Hour Bit: 4"]
pub struct HOUR4_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR4_W<'a> {
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
#[doc = "Field `HOUR5` reader - Real Time Clock Hour Bit: 5"]
pub struct HOUR5_R(crate::FieldReader<bool, bool>);
impl HOUR5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOUR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOUR5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOUR5` writer - Real Time Clock Hour Bit: 5"]
pub struct HOUR5_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR5_W<'a> {
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
#[doc = "Field `HOUR6` reader - Real Time Clock Hour Bit: 6"]
pub struct HOUR6_R(crate::FieldReader<bool, bool>);
impl HOUR6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOUR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOUR6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOUR6` writer - Real Time Clock Hour Bit: 6"]
pub struct HOUR6_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR6_W<'a> {
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
    #[doc = "Bit 0 - Real Time Clock Hour Bit: 0"]
    #[inline(always)]
    pub fn hour0(&self) -> HOUR0_R {
        HOUR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock Hour Bit: 1"]
    #[inline(always)]
    pub fn hour1(&self) -> HOUR1_R {
        HOUR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock Hour Bit: 2"]
    #[inline(always)]
    pub fn hour2(&self) -> HOUR2_R {
        HOUR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Hour Bit: 3"]
    #[inline(always)]
    pub fn hour3(&self) -> HOUR3_R {
        HOUR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock Hour Bit: 4"]
    #[inline(always)]
    pub fn hour4(&self) -> HOUR4_R {
        HOUR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock Hour Bit: 5"]
    #[inline(always)]
    pub fn hour5(&self) -> HOUR5_R {
        HOUR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock Hour Bit: 6"]
    #[inline(always)]
    pub fn hour6(&self) -> HOUR6_R {
        HOUR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Hour Bit: 0"]
    #[inline(always)]
    pub fn hour0(&mut self) -> HOUR0_W {
        HOUR0_W { w: self }
    }
    #[doc = "Bit 1 - Real Time Clock Hour Bit: 1"]
    #[inline(always)]
    pub fn hour1(&mut self) -> HOUR1_W {
        HOUR1_W { w: self }
    }
    #[doc = "Bit 2 - Real Time Clock Hour Bit: 2"]
    #[inline(always)]
    pub fn hour2(&mut self) -> HOUR2_W {
        HOUR2_W { w: self }
    }
    #[doc = "Bit 3 - Real Time Clock Hour Bit: 3"]
    #[inline(always)]
    pub fn hour3(&mut self) -> HOUR3_W {
        HOUR3_W { w: self }
    }
    #[doc = "Bit 4 - Real Time Clock Hour Bit: 4"]
    #[inline(always)]
    pub fn hour4(&mut self) -> HOUR4_W {
        HOUR4_W { w: self }
    }
    #[doc = "Bit 5 - Real Time Clock Hour Bit: 5"]
    #[inline(always)]
    pub fn hour5(&mut self) -> HOUR5_W {
        HOUR5_W { w: self }
    }
    #[doc = "Bit 6 - Real Time Clock Hour Bit: 6"]
    #[inline(always)]
    pub fn hour6(&mut self) -> HOUR6_W {
        HOUR6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Time Clock Hour\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtchour](index.html) module"]
pub struct RTCHOUR_SPEC;
impl crate::RegisterSpec for RTCHOUR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rtchour::R](R) reader structure"]
impl crate::Readable for RTCHOUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtchour::W](W) writer structure"]
impl crate::Writable for RTCHOUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCHOUR to value 0"]
impl crate::Resettable for RTCHOUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
