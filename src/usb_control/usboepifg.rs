#[doc = "Register `USBOEPIFG` reader"]
pub type R = crate::R<USBOEPIFG_SPEC>;
#[doc = "Register `USBOEPIFG` writer"]
pub type W = crate::W<USBOEPIFG_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<USBOEPIFG_SPEC> {
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
#[doc = "USB Output endpoint interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepifg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepifg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBOEPIFG_SPEC;
impl crate::RegisterSpec for USBOEPIFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usboepifg::R`](R) reader structure"]
impl crate::Readable for USBOEPIFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usboepifg::W`](W) writer structure"]
impl crate::Writable for USBOEPIFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBOEPIFG to value 0"]
impl crate::Resettable for USBOEPIFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
