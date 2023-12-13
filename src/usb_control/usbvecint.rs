#[doc = "Register `USBVECINT` reader"]
pub type R = crate::R<USBVECINT_SPEC>;
#[doc = "Register `USBVECINT` writer"]
pub type W = crate::W<USBVECINT_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<USBVECINT_SPEC> {
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
#[doc = "USB Vector interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbvecint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbvecint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBVECINT_SPEC;
impl crate::RegisterSpec for USBVECINT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usbvecint::R`](R) reader structure"]
impl crate::Readable for USBVECINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbvecint::W`](W) writer structure"]
impl crate::Writable for USBVECINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBVECINT to value 0"]
impl crate::Resettable for USBVECINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
