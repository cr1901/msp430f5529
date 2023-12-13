#[doc = "Register `USBFN` reader"]
pub type R = crate::R<USBFN_SPEC>;
#[doc = "Register `USBFN` writer"]
pub type W = crate::W<USBFN_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<USBFN_SPEC> {
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB Frame number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbfn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbfn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBFN_SPEC;
impl crate::RegisterSpec for USBFN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usbfn::R`](R) reader structure"]
impl crate::Readable for USBFN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbfn::W`](W) writer structure"]
impl crate::Writable for USBFN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBFN to value 0"]
impl crate::Resettable for USBFN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
