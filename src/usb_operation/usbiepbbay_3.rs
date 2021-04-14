#[doc = "Register `USBIEPBBAY_3` reader"]
pub struct R(crate::R<USBIEPBBAY_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBIEPBBAY_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBIEPBBAY_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBIEPBBAY_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBIEPBBAY_3` writer"]
pub struct W(crate::W<USBIEPBBAY_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBIEPBBAY_3_SPEC>;
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
impl From<crate::W<USBIEPBBAY_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBIEPBBAY_3_SPEC>) -> Self {
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
#[doc = "Input Endpoint_3: Y-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepbbay_3](index.html) module"]
pub struct USBIEPBBAY_3_SPEC;
impl crate::RegisterSpec for USBIEPBBAY_3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbiepbbay_3::R](R) reader structure"]
impl crate::Readable for USBIEPBBAY_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbiepbbay_3::W](W) writer structure"]
impl crate::Writable for USBIEPBBAY_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBIEPBBAY_3 to value 0"]
impl crate::Resettable for USBIEPBBAY_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
