#[doc = "Register `USBOEP0BUF` reader"]
pub struct R(crate::R<USBOEP0BUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBOEP0BUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBOEP0BUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBOEP0BUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBOEP0BUF` writer"]
pub struct W(crate::W<USBOEP0BUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBOEP0BUF_SPEC>;
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
impl From<crate::W<USBOEP0BUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBOEP0BUF_SPEC>) -> Self {
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
#[doc = "Output endpoint_0 buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboep0buf](index.html) module"]
pub struct USBOEP0BUF_SPEC;
impl crate::RegisterSpec for USBOEP0BUF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usboep0buf::R](R) reader structure"]
impl crate::Readable for USBOEP0BUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usboep0buf::W](W) writer structure"]
impl crate::Writable for USBOEP0BUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBOEP0BUF to value 0"]
impl crate::Resettable for USBOEP0BUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
