#[doc = "Register `DMACTL2` reader"]
pub struct R(crate::R<DMACTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACTL2` writer"]
pub struct W(crate::W<DMACTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACTL2_SPEC>;
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
impl From<crate::W<DMACTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACTL2_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Module Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl2](index.html) module"]
pub struct DMACTL2_SPEC;
impl crate::RegisterSpec for DMACTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dmactl2::R](R) reader structure"]
impl crate::Readable for DMACTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmactl2::W](W) writer structure"]
impl crate::Writable for DMACTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACTL2 to value 0"]
impl crate::Resettable for DMACTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
