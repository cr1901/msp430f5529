#[doc = "Register `USBIEPBCTX_7` reader"]
pub struct R(crate::R<USBIEPBCTX_7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBIEPBCTX_7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBIEPBCTX_7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBIEPBCTX_7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBIEPBCTX_7` writer"]
pub struct W(crate::W<USBIEPBCTX_7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBIEPBCTX_7_SPEC>;
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
impl From<crate::W<USBIEPBCTX_7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBIEPBCTX_7_SPEC>) -> Self {
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
#[doc = "Input Endpoint_7: X-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbctx_7](index.html) module"]
pub struct USBIEPBCTX_7_SPEC;
impl crate::RegisterSpec for USBIEPBCTX_7_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbiepbctx_7::R](R) reader structure"]
impl crate::Readable for USBIEPBCTX_7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbiepbctx_7::W](W) writer structure"]
impl crate::Writable for USBIEPBCTX_7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBIEPBCTX_7 to value 0"]
impl crate::Resettable for USBIEPBCTX_7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
