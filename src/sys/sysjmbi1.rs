#[doc = "Register `SYSJMBI1` reader"]
pub type R = crate::R<SYSJMBI1_SPEC>;
#[doc = "Register `SYSJMBI1` writer"]
pub type W = crate::W<SYSJMBI1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SYSJMBI1_SPEC> {
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
#[doc = "JTAG mailbox input 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysjmbi1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysjmbi1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSJMBI1_SPEC;
impl crate::RegisterSpec for SYSJMBI1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysjmbi1::R`](R) reader structure"]
impl crate::Readable for SYSJMBI1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sysjmbi1::W`](W) writer structure"]
impl crate::Writable for SYSJMBI1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSJMBI1 to value 0"]
impl crate::Resettable for SYSJMBI1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
