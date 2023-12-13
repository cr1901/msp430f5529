#[doc = "Register `UCB1BR1` reader"]
pub type R = crate::R<UCB1BR1_SPEC>;
#[doc = "Register `UCB1BR1` writer"]
pub type W = crate::W<UCB1BR1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<UCB1BR1_SPEC> {
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
#[doc = "USCI B1 Baud Rate 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1br1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1br1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCB1BR1_SPEC;
impl crate::RegisterSpec for UCB1BR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb1br1::R`](R) reader structure"]
impl crate::Readable for UCB1BR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucb1br1::W`](W) writer structure"]
impl crate::Writable for UCB1BR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCB1BR1 to value 0"]
impl crate::Resettable for UCB1BR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
