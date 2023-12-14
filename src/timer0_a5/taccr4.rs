#[doc = "Register `TACCR4` reader"]
pub type R = crate::R<TACCR4_SPEC>;
#[doc = "Register `TACCR4` writer"]
pub type W = crate::W<TACCR4_SPEC>;
#[doc = "Field `TACCR4` reader - Timer A Capture/Compare register 4"]
pub type TACCR4_R = crate::FieldReader<u16>;
#[doc = "Field `TACCR4` writer - Timer A Capture/Compare register 4"]
pub type TACCR4_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 4"]
    #[inline(always)]
    pub fn taccr4(&self) -> TACCR4_R {
        TACCR4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 4"]
    #[inline(always)]
    #[must_use]
    pub fn taccr4(&mut self) -> TACCR4_W<TACCR4_SPEC> {
        TACCR4_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer0_A5 Capture/Compare 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`taccr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`taccr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TACCR4_SPEC;
impl crate::RegisterSpec for TACCR4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`taccr4::R`](R) reader structure"]
impl crate::Readable for TACCR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`taccr4::W`](W) writer structure"]
impl crate::Writable for TACCR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TACCR4 to value 0"]
impl crate::Resettable for TACCR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
