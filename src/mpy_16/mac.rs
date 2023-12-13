#[doc = "Register `MAC` reader"]
pub type R = crate::R<MAC_SPEC>;
#[doc = "Register `MAC` writer"]
pub type W = crate::W<MAC_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<MAC_SPEC> {
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
#[doc = "Multiply Unsigned and Accumulate/Operand 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_SPEC;
impl crate::RegisterSpec for MAC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mac::R`](R) reader structure"]
impl crate::Readable for MAC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac::W`](W) writer structure"]
impl crate::Writable for MAC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC to value 0"]
impl crate::Resettable for MAC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
