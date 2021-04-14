#[doc = "Register `SYSRSTIV` reader"]
pub struct R(crate::R<SYSRSTIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSRSTIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSRSTIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSRSTIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSRSTIV` writer"]
pub struct W(crate::W<SYSRSTIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSRSTIV_SPEC>;
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
impl From<crate::W<SYSRSTIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSRSTIV_SPEC>) -> Self {
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
#[doc = "Reset vector generator\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysrstiv](index.html) module"]
pub struct SYSRSTIV_SPEC;
impl crate::RegisterSpec for SYSRSTIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sysrstiv::R](R) reader structure"]
impl crate::Readable for SYSRSTIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysrstiv::W](W) writer structure"]
impl crate::Writable for SYSRSTIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSRSTIV to value 0"]
impl crate::Resettable for SYSRSTIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
