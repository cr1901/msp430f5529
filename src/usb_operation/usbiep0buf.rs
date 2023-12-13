#[doc = "Register `USBIEP0BUF` reader"]
pub type R = crate::R<USBIEP0BUF_SPEC>;
#[doc = "Register `USBIEP0BUF` writer"]
pub type W = crate::W<USBIEP0BUF_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<USBIEP0BUF_SPEC> {
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
#[doc = "Input endpoint_0 buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiep0buf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiep0buf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBIEP0BUF_SPEC;
impl crate::RegisterSpec for USBIEP0BUF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usbiep0buf::R`](R) reader structure"]
impl crate::Readable for USBIEP0BUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbiep0buf::W`](W) writer structure"]
impl crate::Writable for USBIEP0BUF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBIEP0BUF to value 0"]
impl crate::Resettable for USBIEP0BUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
