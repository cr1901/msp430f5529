#[doc = "Register `USBIEP0BUF` reader"]
pub struct R(crate::R<USBIEP0BUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBIEP0BUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBIEP0BUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBIEP0BUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBIEP0BUF` writer"]
pub struct W(crate::W<USBIEP0BUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBIEP0BUF_SPEC>;
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
impl From<crate::W<USBIEP0BUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBIEP0BUF_SPEC>) -> Self {
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
#[doc = "Input endpoint_0 buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiep0buf](index.html) module"]
pub struct USBIEP0BUF_SPEC;
impl crate::RegisterSpec for USBIEP0BUF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbiep0buf::R](R) reader structure"]
impl crate::Readable for USBIEP0BUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbiep0buf::W](W) writer structure"]
impl crate::Writable for USBIEP0BUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBIEP0BUF to value 0"]
impl crate::Resettable for USBIEP0BUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
