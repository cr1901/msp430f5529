#[doc = "Register `USBVECINT` reader"]
pub struct R(crate::R<USBVECINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBVECINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBVECINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBVECINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBVECINT` writer"]
pub struct W(crate::W<USBVECINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBVECINT_SPEC>;
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
impl From<crate::W<USBVECINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBVECINT_SPEC>) -> Self {
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
#[doc = "USB Vector interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbvecint](index.html) module"]
pub struct USBVECINT_SPEC;
impl crate::RegisterSpec for USBVECINT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbvecint::R](R) reader structure"]
impl crate::Readable for USBVECINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbvecint::W](W) writer structure"]
impl crate::Writable for USBVECINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBVECINT to value 0"]
impl crate::Resettable for USBVECINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
