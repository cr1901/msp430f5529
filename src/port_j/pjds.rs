#[doc = "Register `PJDS` reader"]
pub struct R(crate::R<PJDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PJDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PJDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PJDS` writer"]
pub struct W(crate::W<PJDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PJDS_SPEC>;
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
impl From<crate::W<PJDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PJDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PJDS0` reader - PJDS0"]
pub struct PJDS0_R(crate::FieldReader<bool, bool>);
impl PJDS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PJDS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJDS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJDS0` writer - PJDS0"]
pub struct PJDS0_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDS0_W<'a> {
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
#[doc = "Field `PJDS1` reader - PJDS1"]
pub struct PJDS1_R(crate::FieldReader<bool, bool>);
impl PJDS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PJDS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJDS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJDS1` writer - PJDS1"]
pub struct PJDS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDS1_W<'a> {
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
#[doc = "Field `PJDS2` reader - PJDS2"]
pub struct PJDS2_R(crate::FieldReader<bool, bool>);
impl PJDS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PJDS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJDS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJDS2` writer - PJDS2"]
pub struct PJDS2_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDS2_W<'a> {
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
#[doc = "Field `PJDS3` reader - PJDS3"]
pub struct PJDS3_R(crate::FieldReader<bool, bool>);
impl PJDS3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PJDS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJDS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJDS3` writer - PJDS3"]
pub struct PJDS3_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDS3_W<'a> {
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
    #[doc = "Bit 0 - PJDS0"]
    #[inline(always)]
    pub fn pjds0(&self) -> PJDS0_R {
        PJDS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PJDS1"]
    #[inline(always)]
    pub fn pjds1(&self) -> PJDS1_R {
        PJDS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PJDS2"]
    #[inline(always)]
    pub fn pjds2(&self) -> PJDS2_R {
        PJDS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PJDS3"]
    #[inline(always)]
    pub fn pjds3(&self) -> PJDS3_R {
        PJDS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJDS0"]
    #[inline(always)]
    pub fn pjds0(&mut self) -> PJDS0_W {
        PJDS0_W { w: self }
    }
    #[doc = "Bit 1 - PJDS1"]
    #[inline(always)]
    pub fn pjds1(&mut self) -> PJDS1_W {
        PJDS1_W { w: self }
    }
    #[doc = "Bit 2 - PJDS2"]
    #[inline(always)]
    pub fn pjds2(&mut self) -> PJDS2_W {
        PJDS2_W { w: self }
    }
    #[doc = "Bit 3 - PJDS3"]
    #[inline(always)]
    pub fn pjds3(&mut self) -> PJDS3_W {
        PJDS3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port J Drive Strenght\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjds](index.html) module"]
pub struct PJDS_SPEC;
impl crate::RegisterSpec for PJDS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjds::R](R) reader structure"]
impl crate::Readable for PJDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pjds::W](W) writer structure"]
impl crate::Writable for PJDS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PJDS to value 0"]
impl crate::Resettable for PJDS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
