#[doc = "Register `SYSBERRIV` reader"]
pub struct R(crate::R<SYSBERRIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSBERRIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSBERRIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSBERRIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSBERRIV` writer"]
pub struct W(crate::W<SYSBERRIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSBERRIV_SPEC>;
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
impl From<crate::W<SYSBERRIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSBERRIV_SPEC>) -> Self {
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
#[doc = "Bus Error vector generator\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysberriv](index.html) module"]
pub struct SYSBERRIV_SPEC;
impl crate::RegisterSpec for SYSBERRIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sysberriv::R](R) reader structure"]
impl crate::Readable for SYSBERRIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysberriv::W](W) writer structure"]
impl crate::Writable for SYSBERRIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSBERRIV to value 0"]
impl crate::Resettable for SYSBERRIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
