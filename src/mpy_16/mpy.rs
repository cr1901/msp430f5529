#[doc = "Register `MPY` reader"]
pub type R = crate::R<MPY_SPEC>;
#[doc = "Register `MPY` writer"]
pub type W = crate::W<MPY_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<MPY_SPEC> {
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
#[doc = "Multiply Unsigned/Operand 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPY_SPEC;
impl crate::RegisterSpec for MPY_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mpy::R`](R) reader structure"]
impl crate::Readable for MPY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mpy::W`](W) writer structure"]
impl crate::Writable for MPY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPY to value 0"]
impl crate::Resettable for MPY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
