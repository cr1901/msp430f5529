#[doc = "Register `PJDIR` reader"]
pub struct R(crate::R<PJDIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJDIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PJDIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PJDIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PJDIR` writer"]
pub struct W(crate::W<PJDIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PJDIR_SPEC>;
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
impl From<crate::W<PJDIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PJDIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PJDIR0` reader - PJDIR0"]
pub struct PJDIR0_R(crate::FieldReader<bool, bool>);
impl PJDIR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PJDIR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJDIR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJDIR0` writer - PJDIR0"]
pub struct PJDIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDIR0_W<'a> {
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
#[doc = "Field `PJDIR1` reader - PJDIR1"]
pub struct PJDIR1_R(crate::FieldReader<bool, bool>);
impl PJDIR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PJDIR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJDIR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJDIR1` writer - PJDIR1"]
pub struct PJDIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDIR1_W<'a> {
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
#[doc = "Field `PJDIR2` reader - PJDIR2"]
pub struct PJDIR2_R(crate::FieldReader<bool, bool>);
impl PJDIR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PJDIR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJDIR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJDIR2` writer - PJDIR2"]
pub struct PJDIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDIR2_W<'a> {
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
#[doc = "Field `PJDIR3` reader - PJDIR3"]
pub struct PJDIR3_R(crate::FieldReader<bool, bool>);
impl PJDIR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PJDIR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJDIR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJDIR3` writer - PJDIR3"]
pub struct PJDIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDIR3_W<'a> {
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
    #[doc = "Bit 0 - PJDIR0"]
    #[inline(always)]
    pub fn pjdir0(&self) -> PJDIR0_R {
        PJDIR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PJDIR1"]
    #[inline(always)]
    pub fn pjdir1(&self) -> PJDIR1_R {
        PJDIR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PJDIR2"]
    #[inline(always)]
    pub fn pjdir2(&self) -> PJDIR2_R {
        PJDIR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PJDIR3"]
    #[inline(always)]
    pub fn pjdir3(&self) -> PJDIR3_R {
        PJDIR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJDIR0"]
    #[inline(always)]
    pub fn pjdir0(&mut self) -> PJDIR0_W {
        PJDIR0_W { w: self }
    }
    #[doc = "Bit 1 - PJDIR1"]
    #[inline(always)]
    pub fn pjdir1(&mut self) -> PJDIR1_W {
        PJDIR1_W { w: self }
    }
    #[doc = "Bit 2 - PJDIR2"]
    #[inline(always)]
    pub fn pjdir2(&mut self) -> PJDIR2_W {
        PJDIR2_W { w: self }
    }
    #[doc = "Bit 3 - PJDIR3"]
    #[inline(always)]
    pub fn pjdir3(&mut self) -> PJDIR3_W {
        PJDIR3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port J Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjdir](index.html) module"]
pub struct PJDIR_SPEC;
impl crate::RegisterSpec for PJDIR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjdir::R](R) reader structure"]
impl crate::Readable for PJDIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pjdir::W](W) writer structure"]
impl crate::Writable for PJDIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PJDIR to value 0"]
impl crate::Resettable for PJDIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
