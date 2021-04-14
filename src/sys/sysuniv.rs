#[doc = "Register `SYSUNIV` reader"]
pub struct R(crate::R<SYSUNIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSUNIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSUNIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSUNIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSUNIV` writer"]
pub struct W(crate::W<SYSUNIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSUNIV_SPEC>;
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
impl From<crate::W<SYSUNIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSUNIV_SPEC>) -> Self {
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
#[doc = "User NMI vector generator\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysuniv](index.html) module"]
pub struct SYSUNIV_SPEC;
impl crate::RegisterSpec for SYSUNIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sysuniv::R](R) reader structure"]
impl crate::Readable for SYSUNIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysuniv::W](W) writer structure"]
impl crate::Writable for SYSUNIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSUNIV to value 0"]
impl crate::Resettable for SYSUNIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
