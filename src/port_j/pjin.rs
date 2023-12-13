#[doc = "Register `PJIN` reader"]
pub type R = crate::R<PJIN_SPEC>;
#[doc = "Register `PJIN` writer"]
pub type W = crate::W<PJIN_SPEC>;
#[doc = "Field `PJIN0` reader - PJIN0"]
pub type PJIN0_R = crate::BitReader;
#[doc = "Field `PJIN0` writer - PJIN0"]
pub type PJIN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJIN1` reader - PJIN1"]
pub type PJIN1_R = crate::BitReader;
#[doc = "Field `PJIN1` writer - PJIN1"]
pub type PJIN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJIN2` reader - PJIN2"]
pub type PJIN2_R = crate::BitReader;
#[doc = "Field `PJIN2` writer - PJIN2"]
pub type PJIN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJIN3` reader - PJIN3"]
pub type PJIN3_R = crate::BitReader;
#[doc = "Field `PJIN3` writer - PJIN3"]
pub type PJIN3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PJIN0"]
    #[inline(always)]
    pub fn pjin0(&self) -> PJIN0_R {
        PJIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PJIN1"]
    #[inline(always)]
    pub fn pjin1(&self) -> PJIN1_R {
        PJIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PJIN2"]
    #[inline(always)]
    pub fn pjin2(&self) -> PJIN2_R {
        PJIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PJIN3"]
    #[inline(always)]
    pub fn pjin3(&self) -> PJIN3_R {
        PJIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJIN0"]
    #[inline(always)]
    #[must_use]
    pub fn pjin0(&mut self) -> PJIN0_W<PJIN_SPEC> {
        PJIN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - PJIN1"]
    #[inline(always)]
    #[must_use]
    pub fn pjin1(&mut self) -> PJIN1_W<PJIN_SPEC> {
        PJIN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - PJIN2"]
    #[inline(always)]
    #[must_use]
    pub fn pjin2(&mut self) -> PJIN2_W<PJIN_SPEC> {
        PJIN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - PJIN3"]
    #[inline(always)]
    #[must_use]
    pub fn pjin3(&mut self) -> PJIN3_W<PJIN_SPEC> {
        PJIN3_W::new(self, 3)
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
#[doc = "Port J Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pjin::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pjin::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PJIN_SPEC;
impl crate::RegisterSpec for PJIN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pjin::R`](R) reader structure"]
impl crate::Readable for PJIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pjin::W`](W) writer structure"]
impl crate::Writable for PJIN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PJIN to value 0"]
impl crate::Resettable for PJIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
