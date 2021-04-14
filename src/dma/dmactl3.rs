#[doc = "Register `DMACTL3` reader"]
pub struct R(crate::R<DMACTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACTL3` writer"]
pub struct W(crate::W<DMACTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACTL3_SPEC>;
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
impl From<crate::W<DMACTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACTL3_SPEC>) -> Self {
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
#[doc = "DMA Module Control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl3](index.html) module"]
pub struct DMACTL3_SPEC;
impl crate::RegisterSpec for DMACTL3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dmactl3::R](R) reader structure"]
impl crate::Readable for DMACTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmactl3::W](W) writer structure"]
impl crate::Writable for DMACTL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACTL3 to value 0"]
impl crate::Resettable for DMACTL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
