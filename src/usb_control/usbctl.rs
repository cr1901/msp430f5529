#[doc = "Register `USBCTL` reader"]
pub struct R(crate::R<USBCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCTL` writer"]
pub struct W(crate::W<USBCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCTL_SPEC>;
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
impl From<crate::W<USBCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIR` reader - USB - Data Response Bit"]
pub struct DIR_R(crate::FieldReader<bool, bool>);
impl DIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIR` writer - USB - Data Response Bit"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
#[doc = "Field `FRSTE` reader - USB - Function Reset Connection Enable"]
pub struct FRSTE_R(crate::FieldReader<bool, bool>);
impl FRSTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRSTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRSTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRSTE` writer - USB - Function Reset Connection Enable"]
pub struct FRSTE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRSTE_W<'a> {
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
#[doc = "Field `RWUP` reader - USB - Device Remote Wakeup Request"]
pub struct RWUP_R(crate::FieldReader<bool, bool>);
impl RWUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RWUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWUP` writer - USB - Device Remote Wakeup Request"]
pub struct RWUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RWUP_W<'a> {
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
#[doc = "Field `FEN` reader - USB - Function Enable Bit"]
pub struct FEN_R(crate::FieldReader<bool, bool>);
impl FEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEN` writer - USB - Function Enable Bit"]
pub struct FEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FEN_W<'a> {
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
    #[doc = "Bit 0 - USB - Data Response Bit"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - USB - Function Reset Connection Enable"]
    #[inline(always)]
    pub fn frste(&self) -> FRSTE_R {
        FRSTE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB - Device Remote Wakeup Request"]
    #[inline(always)]
    pub fn rwup(&self) -> RWUP_R {
        RWUP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USB - Function Enable Bit"]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - Data Response Bit"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 4 - USB - Function Reset Connection Enable"]
    #[inline(always)]
    pub fn frste(&mut self) -> FRSTE_W {
        FRSTE_W { w: self }
    }
    #[doc = "Bit 5 - USB - Device Remote Wakeup Request"]
    #[inline(always)]
    pub fn rwup(&mut self) -> RWUP_W {
        RWUP_W { w: self }
    }
    #[doc = "Bit 6 - USB - Function Enable Bit"]
    #[inline(always)]
    pub fn fen(&mut self) -> FEN_W {
        FEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbctl](index.html) module"]
pub struct USBCTL_SPEC;
impl crate::RegisterSpec for USBCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbctl::R](R) reader structure"]
impl crate::Readable for USBCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbctl::W](W) writer structure"]
impl crate::Writable for USBCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBCTL to value 0"]
impl crate::Resettable for USBCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
