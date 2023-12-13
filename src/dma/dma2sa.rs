#[doc = "Register `DMA2SA` reader"]
pub type R = crate::R<DMA2SA_SPEC>;
#[doc = "Register `DMA2SA` writer"]
pub type W = crate::W<DMA2SA_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<DMA2SA_SPEC> {
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA Channel 2 Source Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma2sa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma2sa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA2SA_SPEC;
impl crate::RegisterSpec for DMA2SA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma2sa::R`](R) reader structure"]
impl crate::Readable for DMA2SA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma2sa::W`](W) writer structure"]
impl crate::Writable for DMA2SA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA2SA to value 0"]
impl crate::Resettable for DMA2SA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
