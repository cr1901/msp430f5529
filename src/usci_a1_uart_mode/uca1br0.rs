#[doc = "Register `UCA1BR0` reader"]
pub type R = crate::R<UCA1BR0_SPEC>;
#[doc = "Register `UCA1BR0` writer"]
pub type W = crate::W<UCA1BR0_SPEC>;
#[doc = "Field `UCA1BR0` reader - USCI A1 Baud Rate 0 register"]
pub type UCA1BR0_R = crate::FieldReader;
#[doc = "Field `UCA1BR0` writer - USCI A1 Baud Rate 0 register"]
pub type UCA1BR0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - USCI A1 Baud Rate 0 register"]
    #[inline(always)]
    pub fn uca1br0(&self) -> UCA1BR0_R {
        UCA1BR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI A1 Baud Rate 0 register"]
    #[inline(always)]
    #[must_use]
    pub fn uca1br0(&mut self) -> UCA1BR0_W<UCA1BR0_SPEC> {
        UCA1BR0_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USCI A1 Baud Rate 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1br0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1br0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA1BR0_SPEC;
impl crate::RegisterSpec for UCA1BR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca1br0::R`](R) reader structure"]
impl crate::Readable for UCA1BR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca1br0::W`](W) writer structure"]
impl crate::Writable for UCA1BR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA1BR0 to value 0"]
impl crate::Resettable for UCA1BR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
