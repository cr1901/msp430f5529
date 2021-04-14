#[doc = "Register `PJREN` reader"]
pub struct R(crate::R<PJREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PJREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PJREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PJREN` writer"]
pub struct W(crate::W<PJREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PJREN_SPEC>;
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
impl From<crate::W<PJREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PJREN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PJREN0` reader - PJREN0"]
pub struct PJREN0_R(crate::FieldReader<bool, bool>);
impl PJREN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PJREN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJREN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJREN0` writer - PJREN0"]
pub struct PJREN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PJREN0_W<'a> {
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
#[doc = "Field `PJREN1` reader - PJREN1"]
pub struct PJREN1_R(crate::FieldReader<bool, bool>);
impl PJREN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PJREN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJREN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJREN1` writer - PJREN1"]
pub struct PJREN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PJREN1_W<'a> {
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
#[doc = "Field `PJREN2` reader - PJREN2"]
pub struct PJREN2_R(crate::FieldReader<bool, bool>);
impl PJREN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PJREN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJREN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJREN2` writer - PJREN2"]
pub struct PJREN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PJREN2_W<'a> {
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
#[doc = "Field `PJREN3` reader - PJREN3"]
pub struct PJREN3_R(crate::FieldReader<bool, bool>);
impl PJREN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PJREN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJREN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJREN3` writer - PJREN3"]
pub struct PJREN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PJREN3_W<'a> {
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
impl R {
    #[doc = "Bit 0 - PJREN0"]
    #[inline(always)]
    pub fn pjren0(&self) -> PJREN0_R {
        PJREN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PJREN1"]
    #[inline(always)]
    pub fn pjren1(&self) -> PJREN1_R {
        PJREN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PJREN2"]
    #[inline(always)]
    pub fn pjren2(&self) -> PJREN2_R {
        PJREN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PJREN3"]
    #[inline(always)]
    pub fn pjren3(&self) -> PJREN3_R {
        PJREN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJREN0"]
    #[inline(always)]
    pub fn pjren0(&mut self) -> PJREN0_W {
        PJREN0_W { w: self }
    }
    #[doc = "Bit 1 - PJREN1"]
    #[inline(always)]
    pub fn pjren1(&mut self) -> PJREN1_W {
        PJREN1_W { w: self }
    }
    #[doc = "Bit 2 - PJREN2"]
    #[inline(always)]
    pub fn pjren2(&mut self) -> PJREN2_W {
        PJREN2_W { w: self }
    }
    #[doc = "Bit 3 - PJREN3"]
    #[inline(always)]
    pub fn pjren3(&mut self) -> PJREN3_W {
        PJREN3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port J Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjren](index.html) module"]
pub struct PJREN_SPEC;
impl crate::RegisterSpec for PJREN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjren::R](R) reader structure"]
impl crate::Readable for PJREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pjren::W](W) writer structure"]
impl crate::Writable for PJREN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PJREN to value 0"]
impl crate::Resettable for PJREN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
