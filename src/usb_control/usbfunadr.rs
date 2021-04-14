#[doc = "Register `USBFUNADR` reader"]
pub struct R(crate::R<USBFUNADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBFUNADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBFUNADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBFUNADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBFUNADR` writer"]
pub struct W(crate::W<USBFUNADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBFUNADR_SPEC>;
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
impl From<crate::W<USBFUNADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBFUNADR_SPEC>) -> Self {
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
#[doc = "USB Function address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbfunadr](index.html) module"]
pub struct USBFUNADR_SPEC;
impl crate::RegisterSpec for USBFUNADR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbfunadr::R](R) reader structure"]
impl crate::Readable for USBFUNADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbfunadr::W](W) writer structure"]
impl crate::Writable for USBFUNADR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBFUNADR to value 0"]
impl crate::Resettable for USBFUNADR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
