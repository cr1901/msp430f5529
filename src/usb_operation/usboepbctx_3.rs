#[doc = "Register `USBOEPBCTX_3` reader"]
pub type R = crate::R<USBOEPBCTX_3_SPEC>;
#[doc = "Register `USBOEPBCTX_3` writer"]
pub type W = crate::W<USBOEPBCTX_3_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<USBOEPBCTX_3_SPEC> {
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
#[doc = "Output Endpoint_3: X-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbctx_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbctx_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBOEPBCTX_3_SPEC;
impl crate::RegisterSpec for USBOEPBCTX_3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usboepbctx_3::R`](R) reader structure"]
impl crate::Readable for USBOEPBCTX_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usboepbctx_3::W`](W) writer structure"]
impl crate::Writable for USBOEPBCTX_3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBOEPBCTX_3 to value 0"]
impl crate::Resettable for USBOEPBCTX_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
