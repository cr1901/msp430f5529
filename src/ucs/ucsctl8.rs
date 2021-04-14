#[doc = "Register `UCSCTL8` reader"]
pub struct R(crate::R<UCSCTL8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCSCTL8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCSCTL8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCSCTL8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCSCTL8` writer"]
pub struct W(crate::W<UCSCTL8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCSCTL8_SPEC>;
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
impl From<crate::W<UCSCTL8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCSCTL8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACLKREQEN` reader - ACLK Clock Request Enable"]
pub struct ACLKREQEN_R(crate::FieldReader<bool, bool>);
impl ACLKREQEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACLKREQEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACLKREQEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACLKREQEN` writer - ACLK Clock Request Enable"]
pub struct ACLKREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLKREQEN_W<'a> {
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
#[doc = "Field `MCLKREQEN` reader - MCLK Clock Request Enable"]
pub struct MCLKREQEN_R(crate::FieldReader<bool, bool>);
impl MCLKREQEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCLKREQEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCLKREQEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCLKREQEN` writer - MCLK Clock Request Enable"]
pub struct MCLKREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLKREQEN_W<'a> {
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
#[doc = "Field `SMCLKREQEN` reader - SMCLK Clock Request Enable"]
pub struct SMCLKREQEN_R(crate::FieldReader<bool, bool>);
impl SMCLKREQEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMCLKREQEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMCLKREQEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMCLKREQEN` writer - SMCLK Clock Request Enable"]
pub struct SMCLKREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMCLKREQEN_W<'a> {
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
#[doc = "Field `MODOSCREQEN` reader - MODOSC Clock Request Enable"]
pub struct MODOSCREQEN_R(crate::FieldReader<bool, bool>);
impl MODOSCREQEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODOSCREQEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODOSCREQEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODOSCREQEN` writer - MODOSC Clock Request Enable"]
pub struct MODOSCREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MODOSCREQEN_W<'a> {
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
    #[doc = "Bit 0 - ACLK Clock Request Enable"]
    #[inline(always)]
    pub fn aclkreqen(&self) -> ACLKREQEN_R {
        ACLKREQEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MCLK Clock Request Enable"]
    #[inline(always)]
    pub fn mclkreqen(&self) -> MCLKREQEN_R {
        MCLKREQEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SMCLK Clock Request Enable"]
    #[inline(always)]
    pub fn smclkreqen(&self) -> SMCLKREQEN_R {
        SMCLKREQEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MODOSC Clock Request Enable"]
    #[inline(always)]
    pub fn modoscreqen(&self) -> MODOSCREQEN_R {
        MODOSCREQEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACLK Clock Request Enable"]
    #[inline(always)]
    pub fn aclkreqen(&mut self) -> ACLKREQEN_W {
        ACLKREQEN_W { w: self }
    }
    #[doc = "Bit 1 - MCLK Clock Request Enable"]
    #[inline(always)]
    pub fn mclkreqen(&mut self) -> MCLKREQEN_W {
        MCLKREQEN_W { w: self }
    }
    #[doc = "Bit 2 - SMCLK Clock Request Enable"]
    #[inline(always)]
    pub fn smclkreqen(&mut self) -> SMCLKREQEN_W {
        SMCLKREQEN_W { w: self }
    }
    #[doc = "Bit 3 - MODOSC Clock Request Enable"]
    #[inline(always)]
    pub fn modoscreqen(&mut self) -> MODOSCREQEN_W {
        MODOSCREQEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCS Control Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsctl8](index.html) module"]
pub struct UCSCTL8_SPEC;
impl crate::RegisterSpec for UCSCTL8_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucsctl8::R](R) reader structure"]
impl crate::Readable for UCSCTL8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucsctl8::W](W) writer structure"]
impl crate::Writable for UCSCTL8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCSCTL8 to value 0"]
impl crate::Resettable for UCSCTL8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
