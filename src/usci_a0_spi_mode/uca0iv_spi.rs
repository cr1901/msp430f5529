#[doc = "Register `UCA0IV_SPI` reader"]
pub struct R(crate::R<UCA0IV_SPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0IV_SPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0IV_SPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0IV_SPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0IV_SPI` writer"]
pub struct W(crate::W<UCA0IV_SPI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0IV_SPI_SPEC>;
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
impl From<crate::W<UCA0IV_SPI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0IV_SPI_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A0 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0iv_spi](index.html) module"]
pub struct UCA0IV_SPI_SPEC;
impl crate::RegisterSpec for UCA0IV_SPI_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uca0iv_spi::R](R) reader structure"]
impl crate::Readable for UCA0IV_SPI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0iv_spi::W](W) writer structure"]
impl crate::Writable for UCA0IV_SPI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0IV_SPI to value 0"]
impl crate::Resettable for UCA0IV_SPI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
