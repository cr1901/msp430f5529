#[doc = "Register `PM5CTL0` reader"]
pub type R = crate::R<PM5CTL0_SPEC>;
#[doc = "Register `PM5CTL0` writer"]
pub type W = crate::W<PM5CTL0_SPEC>;
#[doc = "Field `LOCKLPM5` reader - Lock I/O pin configuration upon entry/exit to/from LPM5"]
pub type LOCKLPM5_R = crate::BitReader;
#[doc = "Field `LOCKLPM5` writer - Lock I/O pin configuration upon entry/exit to/from LPM5"]
pub type LOCKLPM5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock I/O pin configuration upon entry/exit to/from LPM5"]
    #[inline(always)]
    pub fn locklpm5(&self) -> LOCKLPM5_R {
        LOCKLPM5_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock I/O pin configuration upon entry/exit to/from LPM5"]
    #[inline(always)]
    #[must_use]
    pub fn locklpm5(&mut self) -> LOCKLPM5_W<PM5CTL0_SPEC> {
        LOCKLPM5_W::new(self, 0)
    }
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
#[doc = "PMM Power Mode 5 Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pm5ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pm5ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PM5CTL0_SPEC;
impl crate::RegisterSpec for PM5CTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pm5ctl0::R`](R) reader structure"]
impl crate::Readable for PM5CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pm5ctl0::W`](W) writer structure"]
impl crate::Writable for PM5CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PM5CTL0 to value 0"]
impl crate::Resettable for PM5CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
