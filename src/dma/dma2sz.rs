#[doc = "Register `DMA2SZ` reader"]
pub type R = crate::R<DMA2SZ_SPEC>;
#[doc = "Register `DMA2SZ` writer"]
pub type W = crate::W<DMA2SZ_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<DMA2SZ_SPEC> {
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
#[doc = "DMA Channel 2 Transfer Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma2sz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma2sz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA2SZ_SPEC;
impl crate::RegisterSpec for DMA2SZ_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma2sz::R`](R) reader structure"]
impl crate::Readable for DMA2SZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma2sz::W`](W) writer structure"]
impl crate::Writable for DMA2SZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA2SZ to value 0"]
impl crate::Resettable for DMA2SZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
