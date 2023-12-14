#[doc = "Register `UCA0TXBUF` reader"]
pub type R = crate::R<UCA0TXBUF_SPEC>;
#[doc = "Register `UCA0TXBUF` writer"]
pub type W = crate::W<UCA0TXBUF_SPEC>;
#[doc = "Field `UCA0TXBUF` reader - USCI A0 Transmit Buffer register"]
pub type UCA0TXBUF_R = crate::FieldReader;
#[doc = "Field `UCA0TXBUF` writer - USCI A0 Transmit Buffer register"]
pub type UCA0TXBUF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - USCI A0 Transmit Buffer register"]
    #[inline(always)]
    pub fn uca0txbuf(&self) -> UCA0TXBUF_R {
        UCA0TXBUF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI A0 Transmit Buffer register"]
    #[inline(always)]
    #[must_use]
    pub fn uca0txbuf(&mut self) -> UCA0TXBUF_W<UCA0TXBUF_SPEC> {
        UCA0TXBUF_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USCI A0 Transmit Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0txbuf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0txbuf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA0TXBUF_SPEC;
impl crate::RegisterSpec for UCA0TXBUF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0txbuf::R`](R) reader structure"]
impl crate::Readable for UCA0TXBUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca0txbuf::W`](W) writer structure"]
impl crate::Writable for UCA0TXBUF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA0TXBUF to value 0"]
impl crate::Resettable for UCA0TXBUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
