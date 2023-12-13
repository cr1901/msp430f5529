#[doc = "Register `SYSJMBO0` reader"]
pub type R = crate::R<SYSJMBO0_SPEC>;
#[doc = "Register `SYSJMBO0` writer"]
pub type W = crate::W<SYSJMBO0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SYSJMBO0_SPEC> {
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
#[doc = "JTAG mailbox output 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysjmbo0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysjmbo0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSJMBO0_SPEC;
impl crate::RegisterSpec for SYSJMBO0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysjmbo0::R`](R) reader structure"]
impl crate::Readable for SYSJMBO0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sysjmbo0::W`](W) writer structure"]
impl crate::Writable for SYSJMBO0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSJMBO0 to value 0"]
impl crate::Resettable for SYSJMBO0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
