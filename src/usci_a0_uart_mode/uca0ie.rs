#[doc = "Register `UCA0IE` reader"]
pub struct R(crate::R<UCA0IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0IE` writer"]
pub struct W(crate::W<UCA0IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0IE_SPEC>;
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
impl From<crate::W<UCA0IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCRXIE` reader - USCI Receive Interrupt Enable"]
pub struct UCRXIE_R(crate::FieldReader<bool, bool>);
impl UCRXIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCRXIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCRXIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCRXIE` writer - USCI Receive Interrupt Enable"]
pub struct UCRXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIE_W<'a> {
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
#[doc = "Field `UCTXIE` reader - USCI Transmit Interrupt Enable"]
pub struct UCTXIE_R(crate::FieldReader<bool, bool>);
impl UCTXIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCTXIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCTXIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXIE` writer - USCI Transmit Interrupt Enable"]
pub struct UCTXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USCI Receive Interrupt Enable"]
    #[inline(always)]
    pub fn ucrxie(&self) -> UCRXIE_R {
        UCRXIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn uctxie(&self) -> UCTXIE_R {
        UCTXIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Receive Interrupt Enable"]
    #[inline(always)]
    pub fn ucrxie(&mut self) -> UCRXIE_W {
        UCRXIE_W { w: self }
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn uctxie(&mut self) -> UCTXIE_W {
        UCTXIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A0 Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0ie](index.html) module"]
pub struct UCA0IE_SPEC;
impl crate::RegisterSpec for UCA0IE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca0ie::R](R) reader structure"]
impl crate::Readable for UCA0IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0ie::W](W) writer structure"]
impl crate::Writable for UCA0IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0IE to value 0"]
impl crate::Resettable for UCA0IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
