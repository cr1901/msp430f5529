#[doc = "Register `TA1CCR2` reader"]
pub type R = crate::R<TA1CCR2_SPEC>;
#[doc = "Register `TA1CCR2` writer"]
pub type W = crate::W<TA1CCR2_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TA1CCR2_SPEC> {
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer1_A3 Capture/Compare 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta1ccr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta1ccr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TA1CCR2_SPEC;
impl crate::RegisterSpec for TA1CCR2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ta1ccr2::R`](R) reader structure"]
impl crate::Readable for TA1CCR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ta1ccr2::W`](W) writer structure"]
impl crate::Writable for TA1CCR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TA1CCR2 to value 0"]
impl crate::Resettable for TA1CCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
