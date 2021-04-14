#[doc = "Register `TA0CCR3` reader"]
pub struct R(crate::R<TA0CCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TA0CCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TA0CCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TA0CCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TA0CCR3` writer"]
pub struct W(crate::W<TA0CCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TA0CCR3_SPEC>;
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
impl From<crate::W<TA0CCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TA0CCR3_SPEC>) -> Self {
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
#[doc = "Timer0_A5 Capture/Compare 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0ccr3](index.html) module"]
pub struct TA0CCR3_SPEC;
impl crate::RegisterSpec for TA0CCR3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ta0ccr3::R](R) reader structure"]
impl crate::Readable for TA0CCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ta0ccr3::W](W) writer structure"]
impl crate::Writable for TA0CCR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TA0CCR3 to value 0"]
impl crate::Resettable for TA0CCR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
