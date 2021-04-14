#[doc = "Register `USBIE` reader"]
pub struct R(crate::R<USBIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBIE` writer"]
pub struct W(crate::W<USBIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBIE_SPEC>;
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
impl From<crate::W<USBIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STPOWIE` reader - USB - Setup Overwrite Interrupt Enable"]
pub struct STPOWIE_R(crate::FieldReader<bool, bool>);
impl STPOWIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STPOWIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STPOWIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STPOWIE` writer - USB - Setup Overwrite Interrupt Enable"]
pub struct STPOWIE_W<'a> {
    w: &'a mut W,
}
impl<'a> STPOWIE_W<'a> {
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
#[doc = "Field `SETUPIE` reader - USB - Setup Interrupt Enable"]
pub struct SETUPIE_R(crate::FieldReader<bool, bool>);
impl SETUPIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SETUPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETUPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETUPIE` writer - USB - Setup Interrupt Enable"]
pub struct SETUPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUPIE_W<'a> {
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
#[doc = "Field `RESRIE` reader - USB - Function Resume Request Interrupt Enable"]
pub struct RESRIE_R(crate::FieldReader<bool, bool>);
impl RESRIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESRIE` writer - USB - Function Resume Request Interrupt Enable"]
pub struct RESRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESRIE_W<'a> {
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
#[doc = "Field `SUSRIE` reader - USB - Function Suspend Request Interrupt Enable"]
pub struct SUSRIE_R(crate::FieldReader<bool, bool>);
impl SUSRIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUSRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSRIE` writer - USB - Function Suspend Request Interrupt Enable"]
pub struct SUSRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSRIE_W<'a> {
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
#[doc = "Field `RSTRIE` reader - USB - Function Reset Request Interrupt Enable"]
pub struct RSTRIE_R(crate::FieldReader<bool, bool>);
impl RSTRIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RSTRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTRIE` writer - USB - Function Reset Request Interrupt Enable"]
pub struct RSTRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTRIE_W<'a> {
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
    #[doc = "Bit 0 - USB - Setup Overwrite Interrupt Enable"]
    #[inline(always)]
    pub fn stpowie(&self) -> STPOWIE_R {
        STPOWIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB - Setup Interrupt Enable"]
    #[inline(always)]
    pub fn setupie(&self) -> SETUPIE_R {
        SETUPIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB - Function Resume Request Interrupt Enable"]
    #[inline(always)]
    pub fn resrie(&self) -> RESRIE_R {
        RESRIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USB - Function Suspend Request Interrupt Enable"]
    #[inline(always)]
    pub fn susrie(&self) -> SUSRIE_R {
        SUSRIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB - Function Reset Request Interrupt Enable"]
    #[inline(always)]
    pub fn rstrie(&self) -> RSTRIE_R {
        RSTRIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - Setup Overwrite Interrupt Enable"]
    #[inline(always)]
    pub fn stpowie(&mut self) -> STPOWIE_W {
        STPOWIE_W { w: self }
    }
    #[doc = "Bit 2 - USB - Setup Interrupt Enable"]
    #[inline(always)]
    pub fn setupie(&mut self) -> SETUPIE_W {
        SETUPIE_W { w: self }
    }
    #[doc = "Bit 5 - USB - Function Resume Request Interrupt Enable"]
    #[inline(always)]
    pub fn resrie(&mut self) -> RESRIE_W {
        RESRIE_W { w: self }
    }
    #[doc = "Bit 6 - USB - Function Suspend Request Interrupt Enable"]
    #[inline(always)]
    pub fn susrie(&mut self) -> SUSRIE_W {
        SUSRIE_W { w: self }
    }
    #[doc = "Bit 7 - USB - Function Reset Request Interrupt Enable"]
    #[inline(always)]
    pub fn rstrie(&mut self) -> RSTRIE_W {
        RSTRIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbie](index.html) module"]
pub struct USBIE_SPEC;
impl crate::RegisterSpec for USBIE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbie::R](R) reader structure"]
impl crate::Readable for USBIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbie::W](W) writer structure"]
impl crate::Writable for USBIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBIE to value 0"]
impl crate::Resettable for USBIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
