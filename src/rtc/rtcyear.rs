#[doc = "Register `RTCYEAR` reader"]
pub type R = crate::R<RTCYEAR_SPEC>;
#[doc = "Register `RTCYEAR` writer"]
pub type W = crate::W<RTCYEAR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RTCYEAR_SPEC> {
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
#[doc = "Real Time Clock Year\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcyear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcyear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCYEAR_SPEC;
impl crate::RegisterSpec for RTCYEAR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcyear::R`](R) reader structure"]
impl crate::Readable for RTCYEAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtcyear::W`](W) writer structure"]
impl crate::Writable for RTCYEAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCYEAR to value 0"]
impl crate::Resettable for RTCYEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
