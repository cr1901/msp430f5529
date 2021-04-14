#[doc = "Register `SYSSNIV` reader"]
pub struct R(crate::R<SYSSNIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSSNIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSSNIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSSNIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSSNIV` writer"]
pub struct W(crate::W<SYSSNIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSSNIV_SPEC>;
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
impl From<crate::W<SYSSNIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSSNIV_SPEC>) -> Self {
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
#[doc = "System NMI vector generator\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syssniv](index.html) module"]
pub struct SYSSNIV_SPEC;
impl crate::RegisterSpec for SYSSNIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [syssniv::R](R) reader structure"]
impl crate::Readable for SYSSNIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syssniv::W](W) writer structure"]
impl crate::Writable for SYSSNIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSSNIV to value 0"]
impl crate::Resettable for SYSSNIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
