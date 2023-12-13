#[doc = "Register `P1IV` reader"]
pub type R = crate::R<P1IV_SPEC>;
#[doc = "Register `P1IV` writer"]
pub type W = crate::W<P1IV_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<P1IV_SPEC> {
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
#[doc = "Port 1 Interrupt Vector Word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p1iv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p1iv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1IV_SPEC;
impl crate::RegisterSpec for P1IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p1iv::R`](R) reader structure"]
impl crate::Readable for P1IV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p1iv::W`](W) writer structure"]
impl crate::Writable for P1IV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P1IV to value 0"]
impl crate::Resettable for P1IV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
