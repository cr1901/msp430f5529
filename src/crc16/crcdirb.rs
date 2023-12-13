#[doc = "Register `CRCDIRB` reader"]
pub type R = crate::R<CRCDIRB_SPEC>;
#[doc = "Register `CRCDIRB` writer"]
pub type W = crate::W<CRCDIRB_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CRCDIRB_SPEC> {
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
#[doc = "CRC data in reverse byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcdirb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcdirb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCDIRB_SPEC;
impl crate::RegisterSpec for CRCDIRB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crcdirb::R`](R) reader structure"]
impl crate::Readable for CRCDIRB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crcdirb::W`](W) writer structure"]
impl crate::Writable for CRCDIRB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCDIRB to value 0"]
impl crate::Resettable for CRCDIRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
