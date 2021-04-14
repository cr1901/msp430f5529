#[doc = "Register `USBKEYID` reader"]
pub struct R(crate::R<USBKEYID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBKEYID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBKEYID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBKEYID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBKEYID` writer"]
pub struct W(crate::W<USBKEYID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBKEYID_SPEC>;
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
impl From<crate::W<USBKEYID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBKEYID_SPEC>) -> Self {
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
#[doc = "USB Controller key register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbkeyid](index.html) module"]
pub struct USBKEYID_SPEC;
impl crate::RegisterSpec for USBKEYID_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbkeyid::R](R) reader structure"]
impl crate::Readable for USBKEYID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbkeyid::W](W) writer structure"]
impl crate::Writable for USBKEYID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBKEYID to value 0"]
impl crate::Resettable for USBKEYID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
