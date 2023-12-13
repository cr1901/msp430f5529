#[doc = "Register `PJOUT` reader"]
pub type R = crate::R<PJOUT_SPEC>;
#[doc = "Register `PJOUT` writer"]
pub type W = crate::W<PJOUT_SPEC>;
#[doc = "Field `PJOUT0` reader - PJOUT0"]
pub type PJOUT0_R = crate::BitReader;
#[doc = "Field `PJOUT0` writer - PJOUT0"]
pub type PJOUT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJOUT1` reader - PJOUT1"]
pub type PJOUT1_R = crate::BitReader;
#[doc = "Field `PJOUT1` writer - PJOUT1"]
pub type PJOUT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJOUT2` reader - PJOUT2"]
pub type PJOUT2_R = crate::BitReader;
#[doc = "Field `PJOUT2` writer - PJOUT2"]
pub type PJOUT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJOUT3` reader - PJOUT3"]
pub type PJOUT3_R = crate::BitReader;
#[doc = "Field `PJOUT3` writer - PJOUT3"]
pub type PJOUT3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PJOUT0"]
    #[inline(always)]
    pub fn pjout0(&self) -> PJOUT0_R {
        PJOUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PJOUT1"]
    #[inline(always)]
    pub fn pjout1(&self) -> PJOUT1_R {
        PJOUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PJOUT2"]
    #[inline(always)]
    pub fn pjout2(&self) -> PJOUT2_R {
        PJOUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PJOUT3"]
    #[inline(always)]
    pub fn pjout3(&self) -> PJOUT3_R {
        PJOUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJOUT0"]
    #[inline(always)]
    #[must_use]
    pub fn pjout0(&mut self) -> PJOUT0_W<PJOUT_SPEC> {
        PJOUT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - PJOUT1"]
    #[inline(always)]
    #[must_use]
    pub fn pjout1(&mut self) -> PJOUT1_W<PJOUT_SPEC> {
        PJOUT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - PJOUT2"]
    #[inline(always)]
    #[must_use]
    pub fn pjout2(&mut self) -> PJOUT2_W<PJOUT_SPEC> {
        PJOUT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - PJOUT3"]
    #[inline(always)]
    #[must_use]
    pub fn pjout3(&mut self) -> PJOUT3_W<PJOUT_SPEC> {
        PJOUT3_W::new(self, 3)
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
#[doc = "Port J Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pjout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pjout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PJOUT_SPEC;
impl crate::RegisterSpec for PJOUT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pjout::R`](R) reader structure"]
impl crate::Readable for PJOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pjout::W`](W) writer structure"]
impl crate::Writable for PJOUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PJOUT to value 0"]
impl crate::Resettable for PJOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
