#[doc = "Register `UCA0IFG` reader"]
pub type R = crate::R<UCA0IFG_SPEC>;
#[doc = "Register `UCA0IFG` writer"]
pub type W = crate::W<UCA0IFG_SPEC>;
#[doc = "Field `UCRXIFG` reader - USCI Receive Interrupt Flag"]
pub type UCRXIFG_R = crate::BitReader;
#[doc = "Field `UCRXIFG` writer - USCI Receive Interrupt Flag"]
pub type UCRXIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXIFG` reader - USCI Transmit Interrupt Flag"]
pub type UCTXIFG_R = crate::BitReader;
#[doc = "Field `UCTXIFG` writer - USCI Transmit Interrupt Flag"]
pub type UCTXIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USCI Receive Interrupt Flag"]
    #[inline(always)]
    pub fn ucrxifg(&self) -> UCRXIFG_R {
        UCRXIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Flag"]
    #[inline(always)]
    pub fn uctxifg(&self) -> UCTXIFG_R {
        UCTXIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Receive Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucrxifg(&mut self) -> UCRXIFG_W<UCA0IFG_SPEC> {
        UCRXIFG_W::new(self, 0)
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn uctxifg(&mut self) -> UCTXIFG_W<UCA0IFG_SPEC> {
        UCTXIFG_W::new(self, 1)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USCI A0 Interrupt Flags Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0ifg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0ifg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA0IFG_SPEC;
impl crate::RegisterSpec for UCA0IFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0ifg::R`](R) reader structure"]
impl crate::Readable for UCA0IFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca0ifg::W`](W) writer structure"]
impl crate::Writable for UCA0IFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA0IFG to value 0"]
impl crate::Resettable for UCA0IFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
