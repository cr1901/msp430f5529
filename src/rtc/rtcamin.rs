#[doc = "Register `RTCAMIN` reader"]
pub struct R(crate::R<RTCAMIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCAMIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCAMIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCAMIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCAMIN` writer"]
pub struct W(crate::W<RTCAMIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCAMIN_SPEC>;
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
impl From<crate::W<RTCAMIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCAMIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MINUTES0` reader - Real Time Clock Minutes Bit: 0"]
pub struct MINUTES0_R(crate::FieldReader<bool, bool>);
impl MINUTES0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MINUTES0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINUTES0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MINUTES0` writer - Real Time Clock Minutes Bit: 0"]
pub struct MINUTES0_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTES0_W<'a> {
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
#[doc = "Field `MINUTES1` reader - Real Time Clock Minutes Bit: 1"]
pub struct MINUTES1_R(crate::FieldReader<bool, bool>);
impl MINUTES1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MINUTES1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINUTES1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MINUTES1` writer - Real Time Clock Minutes Bit: 1"]
pub struct MINUTES1_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTES1_W<'a> {
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
#[doc = "Field `MINUTES2` reader - Real Time Clock Minutes Bit: 2"]
pub struct MINUTES2_R(crate::FieldReader<bool, bool>);
impl MINUTES2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MINUTES2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINUTES2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MINUTES2` writer - Real Time Clock Minutes Bit: 2"]
pub struct MINUTES2_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTES2_W<'a> {
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
#[doc = "Field `MINUTES3` reader - Real Time Clock Minutes Bit: 3"]
pub struct MINUTES3_R(crate::FieldReader<bool, bool>);
impl MINUTES3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MINUTES3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINUTES3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MINUTES3` writer - Real Time Clock Minutes Bit: 3"]
pub struct MINUTES3_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTES3_W<'a> {
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
#[doc = "Field `MINUTES4` reader - Real Time Clock Minutes Bit: 4"]
pub struct MINUTES4_R(crate::FieldReader<bool, bool>);
impl MINUTES4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MINUTES4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINUTES4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MINUTES4` writer - Real Time Clock Minutes Bit: 4"]
pub struct MINUTES4_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTES4_W<'a> {
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
#[doc = "Field `MINUTES5` reader - Real Time Clock Minutes Bit: 5"]
pub struct MINUTES5_R(crate::FieldReader<bool, bool>);
impl MINUTES5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MINUTES5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINUTES5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MINUTES5` writer - Real Time Clock Minutes Bit: 5"]
pub struct MINUTES5_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTES5_W<'a> {
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
#[doc = "Field `MINUTES6` reader - Real Time Clock Minutes Bit: 6"]
pub struct MINUTES6_R(crate::FieldReader<bool, bool>);
impl MINUTES6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MINUTES6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINUTES6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MINUTES6` writer - Real Time Clock Minutes Bit: 6"]
pub struct MINUTES6_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTES6_W<'a> {
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
    #[doc = "Bit 0 - Real Time Clock Minutes Bit: 0"]
    #[inline(always)]
    pub fn minutes0(&self) -> MINUTES0_R {
        MINUTES0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock Minutes Bit: 1"]
    #[inline(always)]
    pub fn minutes1(&self) -> MINUTES1_R {
        MINUTES1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock Minutes Bit: 2"]
    #[inline(always)]
    pub fn minutes2(&self) -> MINUTES2_R {
        MINUTES2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Minutes Bit: 3"]
    #[inline(always)]
    pub fn minutes3(&self) -> MINUTES3_R {
        MINUTES3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock Minutes Bit: 4"]
    #[inline(always)]
    pub fn minutes4(&self) -> MINUTES4_R {
        MINUTES4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock Minutes Bit: 5"]
    #[inline(always)]
    pub fn minutes5(&self) -> MINUTES5_R {
        MINUTES5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock Minutes Bit: 6"]
    #[inline(always)]
    pub fn minutes6(&self) -> MINUTES6_R {
        MINUTES6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Real Time Clock Alarm enable"]
    #[inline(always)]
    pub fn rtcae(&self) -> RTCAE_R {
        RTCAE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Minutes Bit: 0"]
    #[inline(always)]
    pub fn minutes0(&mut self) -> MINUTES0_W {
        MINUTES0_W { w: self }
    }
    #[doc = "Bit 1 - Real Time Clock Minutes Bit: 1"]
    #[inline(always)]
    pub fn minutes1(&mut self) -> MINUTES1_W {
        MINUTES1_W { w: self }
    }
    #[doc = "Bit 2 - Real Time Clock Minutes Bit: 2"]
    #[inline(always)]
    pub fn minutes2(&mut self) -> MINUTES2_W {
        MINUTES2_W { w: self }
    }
    #[doc = "Bit 3 - Real Time Clock Minutes Bit: 3"]
    #[inline(always)]
    pub fn minutes3(&mut self) -> MINUTES3_W {
        MINUTES3_W { w: self }
    }
    #[doc = "Bit 4 - Real Time Clock Minutes Bit: 4"]
    #[inline(always)]
    pub fn minutes4(&mut self) -> MINUTES4_W {
        MINUTES4_W { w: self }
    }
    #[doc = "Bit 5 - Real Time Clock Minutes Bit: 5"]
    #[inline(always)]
    pub fn minutes5(&mut self) -> MINUTES5_W {
        MINUTES5_W { w: self }
    }
    #[doc = "Bit 6 - Real Time Clock Minutes Bit: 6"]
    #[inline(always)]
    pub fn minutes6(&mut self) -> MINUTES6_W {
        MINUTES6_W { w: self }
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
#[doc = "Real Time Clock Alarm Min\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcamin](index.html) module"]
pub struct RTCAMIN_SPEC;
impl crate::RegisterSpec for RTCAMIN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rtcamin::R](R) reader structure"]
impl crate::Readable for RTCAMIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcamin::W](W) writer structure"]
impl crate::Writable for RTCAMIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCAMIN to value 0"]
impl crate::Resettable for RTCAMIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
