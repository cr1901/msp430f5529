#[doc = "Register `USBSTABUFF` reader"]
pub type R = crate::R<USBSTABUFF_SPEC>;
#[doc = "Register `USBSTABUFF` writer"]
pub type W = crate::W<USBSTABUFF_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<USBSTABUFF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Start of buffer space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbstabuff::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbstabuff::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBSTABUFF_SPEC;
impl crate::RegisterSpec for USBSTABUFF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usbstabuff::R`](R) reader structure"]
impl crate::Readable for USBSTABUFF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbstabuff::W`](W) writer structure"]
impl crate::Writable for USBSTABUFF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBSTABUFF to value 0"]
impl crate::Resettable for USBSTABUFF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
