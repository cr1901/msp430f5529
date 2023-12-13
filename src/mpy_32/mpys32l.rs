#[doc = "Register `MPYS32L` reader"]
pub type R = crate::R<MPYS32L_SPEC>;
#[doc = "Register `MPYS32L` writer"]
pub type W = crate::W<MPYS32L_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<MPYS32L_SPEC> {
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
#[doc = "32-bit operand 1 - signed multiply - low word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpys32l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpys32l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPYS32L_SPEC;
impl crate::RegisterSpec for MPYS32L_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mpys32l::R`](R) reader structure"]
impl crate::Readable for MPYS32L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mpys32l::W`](W) writer structure"]
impl crate::Writable for MPYS32L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPYS32L to value 0"]
impl crate::Resettable for MPYS32L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
