#[doc = "Register `PJIN` reader"]
pub struct R(crate::R<PJIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PJIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PJIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PJIN` writer"]
pub struct W(crate::W<PJIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PJIN_SPEC>;
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
impl From<crate::W<PJIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PJIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PJIN0` reader - PJIN0"]
pub struct PJIN0_R(crate::FieldReader<bool, bool>);
impl PJIN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PJIN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJIN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJIN0` writer - PJIN0"]
pub struct PJIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PJIN0_W<'a> {
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
#[doc = "Field `PJIN1` reader - PJIN1"]
pub struct PJIN1_R(crate::FieldReader<bool, bool>);
impl PJIN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PJIN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJIN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJIN1` writer - PJIN1"]
pub struct PJIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PJIN1_W<'a> {
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
#[doc = "Field `PJIN2` reader - PJIN2"]
pub struct PJIN2_R(crate::FieldReader<bool, bool>);
impl PJIN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PJIN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJIN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJIN2` writer - PJIN2"]
pub struct PJIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PJIN2_W<'a> {
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
#[doc = "Field `PJIN3` reader - PJIN3"]
pub struct PJIN3_R(crate::FieldReader<bool, bool>);
impl PJIN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PJIN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJIN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJIN3` writer - PJIN3"]
pub struct PJIN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PJIN3_W<'a> {
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
    #[doc = "Bit 0 - PJIN0"]
    #[inline(always)]
    pub fn pjin0(&self) -> PJIN0_R {
        PJIN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PJIN1"]
    #[inline(always)]
    pub fn pjin1(&self) -> PJIN1_R {
        PJIN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PJIN2"]
    #[inline(always)]
    pub fn pjin2(&self) -> PJIN2_R {
        PJIN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PJIN3"]
    #[inline(always)]
    pub fn pjin3(&self) -> PJIN3_R {
        PJIN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJIN0"]
    #[inline(always)]
    pub fn pjin0(&mut self) -> PJIN0_W {
        PJIN0_W { w: self }
    }
    #[doc = "Bit 1 - PJIN1"]
    #[inline(always)]
    pub fn pjin1(&mut self) -> PJIN1_W {
        PJIN1_W { w: self }
    }
    #[doc = "Bit 2 - PJIN2"]
    #[inline(always)]
    pub fn pjin2(&mut self) -> PJIN2_W {
        PJIN2_W { w: self }
    }
    #[doc = "Bit 3 - PJIN3"]
    #[inline(always)]
    pub fn pjin3(&mut self) -> PJIN3_W {
        PJIN3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port J Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjin](index.html) module"]
pub struct PJIN_SPEC;
impl crate::RegisterSpec for PJIN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjin::R](R) reader structure"]
impl crate::Readable for PJIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pjin::W](W) writer structure"]
impl crate::Writable for PJIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PJIN to value 0"]
impl crate::Resettable for PJIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
