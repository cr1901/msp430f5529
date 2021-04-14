#[doc = "Register `USBPLLIR` reader"]
pub struct R(crate::R<USBPLLIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPLLIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPLLIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPLLIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPLLIR` writer"]
pub struct W(crate::W<USBPLLIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPLLIR_SPEC>;
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
impl From<crate::W<USBPLLIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPLLIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBOOLIFG` reader - USB - PLL out of lock Interrupt Flag"]
pub struct USBOOLIFG_R(crate::FieldReader<bool, bool>);
impl USBOOLIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBOOLIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBOOLIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBOOLIFG` writer - USB - PLL out of lock Interrupt Flag"]
pub struct USBOOLIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> USBOOLIFG_W<'a> {
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
#[doc = "Field `USBLOSIFG` reader - USB - PLL loss of signal Interrupt Flag"]
pub struct USBLOSIFG_R(crate::FieldReader<bool, bool>);
impl USBLOSIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBLOSIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBLOSIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBLOSIFG` writer - USB - PLL loss of signal Interrupt Flag"]
pub struct USBLOSIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> USBLOSIFG_W<'a> {
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
#[doc = "Field `USBOORIFG` reader - USB - PLL out of range Interrupt Flag"]
pub struct USBOORIFG_R(crate::FieldReader<bool, bool>);
impl USBOORIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBOORIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBOORIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBOORIFG` writer - USB - PLL out of range Interrupt Flag"]
pub struct USBOORIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> USBOORIFG_W<'a> {
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
#[doc = "Field `USBOOLIE` reader - USB - PLL out of lock Interrupt enable"]
pub struct USBOOLIE_R(crate::FieldReader<bool, bool>);
impl USBOOLIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBOOLIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBOOLIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBOOLIE` writer - USB - PLL out of lock Interrupt enable"]
pub struct USBOOLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBOOLIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `USBLOSIE` reader - USB - PLL loss of signal Interrupt enable"]
pub struct USBLOSIE_R(crate::FieldReader<bool, bool>);
impl USBLOSIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBLOSIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBLOSIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBLOSIE` writer - USB - PLL loss of signal Interrupt enable"]
pub struct USBLOSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBLOSIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `USBOORIE` reader - USB - PLL out of range Interrupt enable"]
pub struct USBOORIE_R(crate::FieldReader<bool, bool>);
impl USBOORIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBOORIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBOORIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBOORIE` writer - USB - PLL out of range Interrupt enable"]
pub struct USBOORIE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBOORIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB - PLL out of lock Interrupt Flag"]
    #[inline(always)]
    pub fn usboolifg(&self) -> USBOOLIFG_R {
        USBOOLIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB - PLL loss of signal Interrupt Flag"]
    #[inline(always)]
    pub fn usblosifg(&self) -> USBLOSIFG_R {
        USBLOSIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB - PLL out of range Interrupt Flag"]
    #[inline(always)]
    pub fn usboorifg(&self) -> USBOORIFG_R {
        USBOORIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USB - PLL out of lock Interrupt enable"]
    #[inline(always)]
    pub fn usboolie(&self) -> USBOOLIE_R {
        USBOOLIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - USB - PLL loss of signal Interrupt enable"]
    #[inline(always)]
    pub fn usblosie(&self) -> USBLOSIE_R {
        USBLOSIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USB - PLL out of range Interrupt enable"]
    #[inline(always)]
    pub fn usboorie(&self) -> USBOORIE_R {
        USBOORIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - PLL out of lock Interrupt Flag"]
    #[inline(always)]
    pub fn usboolifg(&mut self) -> USBOOLIFG_W {
        USBOOLIFG_W { w: self }
    }
    #[doc = "Bit 1 - USB - PLL loss of signal Interrupt Flag"]
    #[inline(always)]
    pub fn usblosifg(&mut self) -> USBLOSIFG_W {
        USBLOSIFG_W { w: self }
    }
    #[doc = "Bit 2 - USB - PLL out of range Interrupt Flag"]
    #[inline(always)]
    pub fn usboorifg(&mut self) -> USBOORIFG_W {
        USBOORIFG_W { w: self }
    }
    #[doc = "Bit 8 - USB - PLL out of lock Interrupt enable"]
    #[inline(always)]
    pub fn usboolie(&mut self) -> USBOOLIE_W {
        USBOOLIE_W { w: self }
    }
    #[doc = "Bit 9 - USB - PLL loss of signal Interrupt enable"]
    #[inline(always)]
    pub fn usblosie(&mut self) -> USBLOSIE_W {
        USBLOSIE_W { w: self }
    }
    #[doc = "Bit 10 - USB - PLL out of range Interrupt enable"]
    #[inline(always)]
    pub fn usboorie(&mut self) -> USBOORIE_W {
        USBOORIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PLL Interrupt control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpllir](index.html) module"]
pub struct USBPLLIR_SPEC;
impl crate::RegisterSpec for USBPLLIR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbpllir::R](R) reader structure"]
impl crate::Readable for USBPLLIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbpllir::W](W) writer structure"]
impl crate::Writable for USBPLLIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBPLLIR to value 0"]
impl crate::Resettable for USBPLLIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
