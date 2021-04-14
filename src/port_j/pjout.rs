#[doc = "Register `PJOUT` reader"]
pub struct R(crate::R<PJOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PJOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PJOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PJOUT` writer"]
pub struct W(crate::W<PJOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PJOUT_SPEC>;
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
impl From<crate::W<PJOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PJOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PJOUT0` reader - PJOUT0"]
pub struct PJOUT0_R(crate::FieldReader<bool, bool>);
impl PJOUT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PJOUT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJOUT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJOUT0` writer - PJOUT0"]
pub struct PJOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PJOUT0_W<'a> {
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
#[doc = "Field `PJOUT1` reader - PJOUT1"]
pub struct PJOUT1_R(crate::FieldReader<bool, bool>);
impl PJOUT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PJOUT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJOUT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJOUT1` writer - PJOUT1"]
pub struct PJOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PJOUT1_W<'a> {
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
#[doc = "Field `PJOUT2` reader - PJOUT2"]
pub struct PJOUT2_R(crate::FieldReader<bool, bool>);
impl PJOUT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PJOUT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJOUT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJOUT2` writer - PJOUT2"]
pub struct PJOUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> PJOUT2_W<'a> {
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
#[doc = "Field `PJOUT3` reader - PJOUT3"]
pub struct PJOUT3_R(crate::FieldReader<bool, bool>);
impl PJOUT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PJOUT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJOUT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJOUT3` writer - PJOUT3"]
pub struct PJOUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> PJOUT3_W<'a> {
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
    #[doc = "Bit 0 - PJOUT0"]
    #[inline(always)]
    pub fn pjout0(&self) -> PJOUT0_R {
        PJOUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PJOUT1"]
    #[inline(always)]
    pub fn pjout1(&self) -> PJOUT1_R {
        PJOUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PJOUT2"]
    #[inline(always)]
    pub fn pjout2(&self) -> PJOUT2_R {
        PJOUT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PJOUT3"]
    #[inline(always)]
    pub fn pjout3(&self) -> PJOUT3_R {
        PJOUT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJOUT0"]
    #[inline(always)]
    pub fn pjout0(&mut self) -> PJOUT0_W {
        PJOUT0_W { w: self }
    }
    #[doc = "Bit 1 - PJOUT1"]
    #[inline(always)]
    pub fn pjout1(&mut self) -> PJOUT1_W {
        PJOUT1_W { w: self }
    }
    #[doc = "Bit 2 - PJOUT2"]
    #[inline(always)]
    pub fn pjout2(&mut self) -> PJOUT2_W {
        PJOUT2_W { w: self }
    }
    #[doc = "Bit 3 - PJOUT3"]
    #[inline(always)]
    pub fn pjout3(&mut self) -> PJOUT3_W {
        PJOUT3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port J Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjout](index.html) module"]
pub struct PJOUT_SPEC;
impl crate::RegisterSpec for PJOUT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjout::R](R) reader structure"]
impl crate::Readable for PJOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pjout::W](W) writer structure"]
impl crate::Writable for PJOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PJOUT to value 0"]
impl crate::Resettable for PJOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
