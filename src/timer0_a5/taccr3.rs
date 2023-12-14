#[doc = "Register `TACCR3` reader"]
pub type R = crate::R<TACCR3_SPEC>;
#[doc = "Register `TACCR3` writer"]
pub type W = crate::W<TACCR3_SPEC>;
#[doc = "Field `TACCR3` reader - Timer A Capture/Compare register 3"]
pub type TACCR3_R = crate::FieldReader<u16>;
#[doc = "Field `TACCR3` writer - Timer A Capture/Compare register 3"]
pub type TACCR3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 3"]
    #[inline(always)]
    pub fn taccr3(&self) -> TACCR3_R {
        TACCR3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 3"]
    #[inline(always)]
    #[must_use]
    pub fn taccr3(&mut self) -> TACCR3_W<TACCR3_SPEC> {
        TACCR3_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer0_A5 Capture/Compare 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`taccr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`taccr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TACCR3_SPEC;
impl crate::RegisterSpec for TACCR3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`taccr3::R`](R) reader structure"]
impl crate::Readable for TACCR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`taccr3::W`](W) writer structure"]
impl crate::Writable for TACCR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TACCR3 to value 0"]
impl crate::Resettable for TACCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
