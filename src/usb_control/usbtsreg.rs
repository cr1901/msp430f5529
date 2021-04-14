#[doc = "Register `USBTSREG` reader"]
pub struct R(crate::R<USBTSREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBTSREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBTSREG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBTSREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBTSREG` writer"]
pub struct W(crate::W<USBTSREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBTSREG_SPEC>;
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
impl From<crate::W<USBTSREG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBTSREG_SPEC>) -> Self {
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
#[doc = "USB Time Stamp register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbtsreg](index.html) module"]
pub struct USBTSREG_SPEC;
impl crate::RegisterSpec for USBTSREG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbtsreg::R](R) reader structure"]
impl crate::Readable for USBTSREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbtsreg::W](W) writer structure"]
impl crate::Writable for USBTSREG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBTSREG to value 0"]
impl crate::Resettable for USBTSREG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
