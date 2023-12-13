#[doc = "Register `CRCINIRES` reader"]
pub type R = crate::R<CRCINIRES_SPEC>;
#[doc = "Register `CRCINIRES` writer"]
pub type W = crate::W<CRCINIRES_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CRCINIRES_SPEC> {
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
#[doc = "CRC Initialisation Register and Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcinires::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcinires::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCINIRES_SPEC;
impl crate::RegisterSpec for CRCINIRES_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crcinires::R`](R) reader structure"]
impl crate::Readable for CRCINIRES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crcinires::W`](W) writer structure"]
impl crate::Writable for CRCINIRES_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCINIRES to value 0"]
impl crate::Resettable for CRCINIRES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
