#[doc = "Register `USBIEPIE` reader"]
pub struct R(crate::R<USBIEPIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBIEPIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBIEPIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBIEPIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBIEPIE` writer"]
pub struct W(crate::W<USBIEPIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBIEPIE_SPEC>;
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
impl From<crate::W<USBIEPIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBIEPIE_SPEC>) -> Self {
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
#[doc = "USB Input endpoint interrupt enable flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepie](index.html) module"]
pub struct USBIEPIE_SPEC;
impl crate::RegisterSpec for USBIEPIE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbiepie::R](R) reader structure"]
impl crate::Readable for USBIEPIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbiepie::W](W) writer structure"]
impl crate::Writable for USBIEPIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBIEPIE to value 0"]
impl crate::Resettable for USBIEPIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
