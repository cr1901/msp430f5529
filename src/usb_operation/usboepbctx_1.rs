#[doc = "Register `USBOEPBCTX_1` reader"]
pub struct R(crate::R<USBOEPBCTX_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBOEPBCTX_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBOEPBCTX_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBOEPBCTX_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBOEPBCTX_1` writer"]
pub struct W(crate::W<USBOEPBCTX_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBOEPBCTX_1_SPEC>;
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
impl From<crate::W<USBOEPBCTX_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBOEPBCTX_1_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Endpoint_1: X-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbctx_1](index.html) module"]
pub struct USBOEPBCTX_1_SPEC;
impl crate::RegisterSpec for USBOEPBCTX_1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usboepbctx_1::R](R) reader structure"]
impl crate::Readable for USBOEPBCTX_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usboepbctx_1::W](W) writer structure"]
impl crate::Writable for USBOEPBCTX_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBOEPBCTX_1 to value 0"]
impl crate::Resettable for USBOEPBCTX_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
