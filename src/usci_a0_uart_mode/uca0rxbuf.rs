#[doc = "Register `UCA0RXBUF` reader"]
pub type R = crate::R<UCA0RXBUF_SPEC>;
#[doc = "Register `UCA0RXBUF` writer"]
pub type W = crate::W<UCA0RXBUF_SPEC>;
#[doc = "Field `UCA0RXBUF` reader - USCI A0 Receive Buffer register"]
pub type UCA0RXBUF_R = crate::FieldReader;
#[doc = "Field `UCA0RXBUF` writer - USCI A0 Receive Buffer register"]
pub type UCA0RXBUF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - USCI A0 Receive Buffer register"]
    #[inline(always)]
    pub fn uca0rxbuf(&self) -> UCA0RXBUF_R {
        UCA0RXBUF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI A0 Receive Buffer register"]
    #[inline(always)]
    #[must_use]
    pub fn uca0rxbuf(&mut self) -> UCA0RXBUF_W<UCA0RXBUF_SPEC> {
        UCA0RXBUF_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USCI A0 Receive Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0rxbuf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0rxbuf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA0RXBUF_SPEC;
impl crate::RegisterSpec for UCA0RXBUF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0rxbuf::R`](R) reader structure"]
impl crate::Readable for UCA0RXBUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca0rxbuf::W`](W) writer structure"]
impl crate::Writable for UCA0RXBUF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA0RXBUF to value 0"]
impl crate::Resettable for UCA0RXBUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
