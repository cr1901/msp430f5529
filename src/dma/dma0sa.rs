#[doc = "Register `DMA0SA` reader"]
pub struct R(crate::R<DMA0SA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA0SA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA0SA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA0SA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA0SA` writer"]
pub struct W(crate::W<DMA0SA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA0SA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMA0SA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA0SA_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel 0 Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0sa](index.html) module"]
pub struct DMA0SA_SPEC;
impl crate::RegisterSpec for DMA0SA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma0sa::R](R) reader structure"]
impl crate::Readable for DMA0SA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma0sa::W](W) writer structure"]
impl crate::Writable for DMA0SA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA0SA to value 0"]
impl crate::Resettable for DMA0SA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
