#[doc = "Register `UCA1RXBUF` reader"]
pub type R = crate::R<UCA1RXBUF_SPEC>;
#[doc = "Register `UCA1RXBUF` writer"]
pub type W = crate::W<UCA1RXBUF_SPEC>;
#[doc = "Field `UCA1RXBUF` reader - USCI A1 Receive Buffer register"]
pub type UCA1RXBUF_R = crate::FieldReader;
#[doc = "Field `UCA1RXBUF` writer - USCI A1 Receive Buffer register"]
pub type UCA1RXBUF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - USCI A1 Receive Buffer register"]
    #[inline(always)]
    pub fn uca1rxbuf(&self) -> UCA1RXBUF_R {
        UCA1RXBUF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI A1 Receive Buffer register"]
    #[inline(always)]
    #[must_use]
    pub fn uca1rxbuf(&mut self) -> UCA1RXBUF_W<UCA1RXBUF_SPEC> {
        UCA1RXBUF_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USCI A1 Receive Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1rxbuf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1rxbuf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA1RXBUF_SPEC;
impl crate::RegisterSpec for UCA1RXBUF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca1rxbuf::R`](R) reader structure"]
impl crate::Readable for UCA1RXBUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca1rxbuf::W`](W) writer structure"]
impl crate::Writable for UCA1RXBUF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA1RXBUF to value 0"]
impl crate::Resettable for UCA1RXBUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
