#[doc = "Register `UCB1TXBUF` reader"]
pub type R = crate::R<UCB1TXBUF_SPEC>;
#[doc = "Register `UCB1TXBUF` writer"]
pub type W = crate::W<UCB1TXBUF_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<UCB1TXBUF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
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
#[doc = "USCI B1 Transmit Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1txbuf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1txbuf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCB1TXBUF_SPEC;
impl crate::RegisterSpec for UCB1TXBUF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb1txbuf::R`](R) reader structure"]
impl crate::Readable for UCB1TXBUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucb1txbuf::W`](W) writer structure"]
impl crate::Writable for UCB1TXBUF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCB1TXBUF to value 0"]
impl crate::Resettable for UCB1TXBUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
