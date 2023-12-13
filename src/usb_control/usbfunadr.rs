#[doc = "Register `USBFUNADR` reader"]
pub type R = crate::R<USBFUNADR_SPEC>;
#[doc = "Register `USBFUNADR` writer"]
pub type W = crate::W<USBFUNADR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<USBFUNADR_SPEC> {
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
#[doc = "USB Function address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbfunadr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbfunadr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBFUNADR_SPEC;
impl crate::RegisterSpec for USBFUNADR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usbfunadr::R`](R) reader structure"]
impl crate::Readable for USBFUNADR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbfunadr::W`](W) writer structure"]
impl crate::Writable for USBFUNADR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBFUNADR to value 0"]
impl crate::Resettable for USBFUNADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
