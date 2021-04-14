#[doc = "Register `UCA0IFG_SPI` reader"]
pub struct R(crate::R<UCA0IFG_SPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0IFG_SPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0IFG_SPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0IFG_SPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0IFG_SPI` writer"]
pub struct W(crate::W<UCA0IFG_SPI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0IFG_SPI_SPEC>;
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
impl From<crate::W<UCA0IFG_SPI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0IFG_SPI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCRXIFG` reader - USCI Receive Interrupt Flag"]
pub struct UCRXIFG_R(crate::FieldReader<bool, bool>);
impl UCRXIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCRXIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCRXIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCRXIFG` writer - USCI Receive Interrupt Flag"]
pub struct UCRXIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIFG_W<'a> {
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
#[doc = "Field `UCTXIFG` reader - USCI Transmit Interrupt Flag"]
pub struct UCTXIFG_R(crate::FieldReader<bool, bool>);
impl UCTXIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCTXIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCTXIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXIFG` writer - USCI Transmit Interrupt Flag"]
pub struct UCTXIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIFG_W<'a> {
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
    #[doc = "Bit 0 - USCI Receive Interrupt Flag"]
    #[inline(always)]
    pub fn ucrxifg(&self) -> UCRXIFG_R {
        UCRXIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Flag"]
    #[inline(always)]
    pub fn uctxifg(&self) -> UCTXIFG_R {
        UCTXIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Receive Interrupt Flag"]
    #[inline(always)]
    pub fn ucrxifg(&mut self) -> UCRXIFG_W {
        UCRXIFG_W { w: self }
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Flag"]
    #[inline(always)]
    pub fn uctxifg(&mut self) -> UCTXIFG_W {
        UCTXIFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A0 Interrupt Flags Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0ifg_spi](index.html) module"]
pub struct UCA0IFG_SPI_SPEC;
impl crate::RegisterSpec for UCA0IFG_SPI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca0ifg_spi::R](R) reader structure"]
impl crate::Readable for UCA0IFG_SPI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0ifg_spi::W](W) writer structure"]
impl crate::Writable for UCA0IFG_SPI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0IFG_SPI to value 0"]
impl crate::Resettable for UCA0IFG_SPI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
