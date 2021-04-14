#[doc = "Register `USBIFG` reader"]
pub struct R(crate::R<USBIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBIFG` writer"]
pub struct W(crate::W<USBIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBIFG_SPEC>;
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
impl From<crate::W<USBIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STPOWIFG` reader - USB - Setup Overwrite Interrupt Flag"]
pub struct STPOWIFG_R(crate::FieldReader<bool, bool>);
impl STPOWIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STPOWIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STPOWIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STPOWIFG` writer - USB - Setup Overwrite Interrupt Flag"]
pub struct STPOWIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> STPOWIFG_W<'a> {
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
#[doc = "Field `SETUPIFG` reader - USB - Setup Interrupt Flag"]
pub struct SETUPIFG_R(crate::FieldReader<bool, bool>);
impl SETUPIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SETUPIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETUPIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETUPIFG` writer - USB - Setup Interrupt Flag"]
pub struct SETUPIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUPIFG_W<'a> {
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
#[doc = "Field `RESRIFG` reader - USB - Function Resume Request Interrupt Flag"]
pub struct RESRIFG_R(crate::FieldReader<bool, bool>);
impl RESRIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESRIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESRIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESRIFG` writer - USB - Function Resume Request Interrupt Flag"]
pub struct RESRIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RESRIFG_W<'a> {
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
#[doc = "Field `SUSRIFG` reader - USB - Function Suspend Request Interrupt Flag"]
pub struct SUSRIFG_R(crate::FieldReader<bool, bool>);
impl SUSRIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUSRIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSRIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSRIFG` writer - USB - Function Suspend Request Interrupt Flag"]
pub struct SUSRIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSRIFG_W<'a> {
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
#[doc = "Field `RSTRIFG` reader - USB - Function Reset Request Interrupt Flag"]
pub struct RSTRIFG_R(crate::FieldReader<bool, bool>);
impl RSTRIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RSTRIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTRIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTRIFG` writer - USB - Function Reset Request Interrupt Flag"]
pub struct RSTRIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTRIFG_W<'a> {
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
    #[doc = "Bit 0 - USB - Setup Overwrite Interrupt Flag"]
    #[inline(always)]
    pub fn stpowifg(&self) -> STPOWIFG_R {
        STPOWIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB - Setup Interrupt Flag"]
    #[inline(always)]
    pub fn setupifg(&self) -> SETUPIFG_R {
        SETUPIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB - Function Resume Request Interrupt Flag"]
    #[inline(always)]
    pub fn resrifg(&self) -> RESRIFG_R {
        RESRIFG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USB - Function Suspend Request Interrupt Flag"]
    #[inline(always)]
    pub fn susrifg(&self) -> SUSRIFG_R {
        SUSRIFG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB - Function Reset Request Interrupt Flag"]
    #[inline(always)]
    pub fn rstrifg(&self) -> RSTRIFG_R {
        RSTRIFG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - Setup Overwrite Interrupt Flag"]
    #[inline(always)]
    pub fn stpowifg(&mut self) -> STPOWIFG_W {
        STPOWIFG_W { w: self }
    }
    #[doc = "Bit 2 - USB - Setup Interrupt Flag"]
    #[inline(always)]
    pub fn setupifg(&mut self) -> SETUPIFG_W {
        SETUPIFG_W { w: self }
    }
    #[doc = "Bit 5 - USB - Function Resume Request Interrupt Flag"]
    #[inline(always)]
    pub fn resrifg(&mut self) -> RESRIFG_W {
        RESRIFG_W { w: self }
    }
    #[doc = "Bit 6 - USB - Function Suspend Request Interrupt Flag"]
    #[inline(always)]
    pub fn susrifg(&mut self) -> SUSRIFG_W {
        SUSRIFG_W { w: self }
    }
    #[doc = "Bit 7 - USB - Function Reset Request Interrupt Flag"]
    #[inline(always)]
    pub fn rstrifg(&mut self) -> RSTRIFG_W {
        RSTRIFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbifg](index.html) module"]
pub struct USBIFG_SPEC;
impl crate::RegisterSpec for USBIFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbifg::R](R) reader structure"]
impl crate::Readable for USBIFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbifg::W](W) writer structure"]
impl crate::Writable for USBIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBIFG to value 0"]
impl crate::Resettable for USBIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
