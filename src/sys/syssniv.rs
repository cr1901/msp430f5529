#[doc = "Register `SYSSNIV` reader"]
pub type R = crate::R<SYSSNIV_SPEC>;
#[doc = "Register `SYSSNIV` writer"]
pub type W = crate::W<SYSSNIV_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SYSSNIV_SPEC> {
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
#[doc = "System NMI vector generator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syssniv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syssniv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSSNIV_SPEC;
impl crate::RegisterSpec for SYSSNIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syssniv::R`](R) reader structure"]
impl crate::Readable for SYSSNIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`syssniv::W`](W) writer structure"]
impl crate::Writable for SYSSNIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSSNIV to value 0"]
impl crate::Resettable for SYSSNIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
