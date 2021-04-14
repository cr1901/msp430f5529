#[doc = "Register `RCCTL0` reader"]
pub struct R(crate::R<RCCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCCTL0` writer"]
pub struct W(crate::W<RCCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCCTL0_SPEC>;
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
impl From<crate::W<RCCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCRS0OFF` reader - RAM Controller RAM Sector 0 Off"]
pub struct RCRS0OFF_R(crate::FieldReader<bool, bool>);
impl RCRS0OFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCRS0OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCRS0OFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCRS0OFF` writer - RAM Controller RAM Sector 0 Off"]
pub struct RCRS0OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRS0OFF_W<'a> {
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
#[doc = "Field `RCRS1OFF` reader - RAM Controller RAM Sector 1 Off"]
pub struct RCRS1OFF_R(crate::FieldReader<bool, bool>);
impl RCRS1OFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCRS1OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCRS1OFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCRS1OFF` writer - RAM Controller RAM Sector 1 Off"]
pub struct RCRS1OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRS1OFF_W<'a> {
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
#[doc = "Field `RCRS2OFF` reader - RAM Controller RAM Sector 2 Off"]
pub struct RCRS2OFF_R(crate::FieldReader<bool, bool>);
impl RCRS2OFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCRS2OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCRS2OFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCRS2OFF` writer - RAM Controller RAM Sector 2 Off"]
pub struct RCRS2OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRS2OFF_W<'a> {
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
#[doc = "Field `RCRS3OFF` reader - RAM Controller RAM Sector 3 Off"]
pub struct RCRS3OFF_R(crate::FieldReader<bool, bool>);
impl RCRS3OFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCRS3OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCRS3OFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCRS3OFF` writer - RAM Controller RAM Sector 3 Off"]
pub struct RCRS3OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRS3OFF_W<'a> {
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
#[doc = "Field `RCRS7OFF` reader - RAM Controller RAM Sector 7 (USB) Off"]
pub struct RCRS7OFF_R(crate::FieldReader<bool, bool>);
impl RCRS7OFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCRS7OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCRS7OFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCRS7OFF` writer - RAM Controller RAM Sector 7 (USB) Off"]
pub struct RCRS7OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRS7OFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RAM Controller RAM Sector 0 Off"]
    #[inline(always)]
    pub fn rcrs0off(&self) -> RCRS0OFF_R {
        RCRS0OFF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RAM Controller RAM Sector 1 Off"]
    #[inline(always)]
    pub fn rcrs1off(&self) -> RCRS1OFF_R {
        RCRS1OFF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RAM Controller RAM Sector 2 Off"]
    #[inline(always)]
    pub fn rcrs2off(&self) -> RCRS2OFF_R {
        RCRS2OFF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RAM Controller RAM Sector 3 Off"]
    #[inline(always)]
    pub fn rcrs3off(&self) -> RCRS3OFF_R {
        RCRS3OFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RAM Controller RAM Sector 7 (USB) Off"]
    #[inline(always)]
    pub fn rcrs7off(&self) -> RCRS7OFF_R {
        RCRS7OFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM Controller RAM Sector 0 Off"]
    #[inline(always)]
    pub fn rcrs0off(&mut self) -> RCRS0OFF_W {
        RCRS0OFF_W { w: self }
    }
    #[doc = "Bit 1 - RAM Controller RAM Sector 1 Off"]
    #[inline(always)]
    pub fn rcrs1off(&mut self) -> RCRS1OFF_W {
        RCRS1OFF_W { w: self }
    }
    #[doc = "Bit 2 - RAM Controller RAM Sector 2 Off"]
    #[inline(always)]
    pub fn rcrs2off(&mut self) -> RCRS2OFF_W {
        RCRS2OFF_W { w: self }
    }
    #[doc = "Bit 3 - RAM Controller RAM Sector 3 Off"]
    #[inline(always)]
    pub fn rcrs3off(&mut self) -> RCRS3OFF_W {
        RCRS3OFF_W { w: self }
    }
    #[doc = "Bit 7 - RAM Controller RAM Sector 7 (USB) Off"]
    #[inline(always)]
    pub fn rcrs7off(&mut self) -> RCRS7OFF_W {
        RCRS7OFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ram Controller Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcctl0](index.html) module"]
pub struct RCCTL0_SPEC;
impl crate::RegisterSpec for RCCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rcctl0::R](R) reader structure"]
impl crate::Readable for RCCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcctl0::W](W) writer structure"]
impl crate::Writable for RCCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCCTL0 to value 0"]
impl crate::Resettable for RCCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
