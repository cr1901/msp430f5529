#[doc = "Register `PJREN` reader"]
pub type R = crate::R<PJREN_SPEC>;
#[doc = "Register `PJREN` writer"]
pub type W = crate::W<PJREN_SPEC>;
#[doc = "Field `PJREN0` reader - PJREN0"]
pub type PJREN0_R = crate::BitReader;
#[doc = "Field `PJREN0` writer - PJREN0"]
pub type PJREN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJREN1` reader - PJREN1"]
pub type PJREN1_R = crate::BitReader;
#[doc = "Field `PJREN1` writer - PJREN1"]
pub type PJREN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJREN2` reader - PJREN2"]
pub type PJREN2_R = crate::BitReader;
#[doc = "Field `PJREN2` writer - PJREN2"]
pub type PJREN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJREN3` reader - PJREN3"]
pub type PJREN3_R = crate::BitReader;
#[doc = "Field `PJREN3` writer - PJREN3"]
pub type PJREN3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PJREN0"]
    #[inline(always)]
    pub fn pjren0(&self) -> PJREN0_R {
        PJREN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PJREN1"]
    #[inline(always)]
    pub fn pjren1(&self) -> PJREN1_R {
        PJREN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PJREN2"]
    #[inline(always)]
    pub fn pjren2(&self) -> PJREN2_R {
        PJREN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PJREN3"]
    #[inline(always)]
    pub fn pjren3(&self) -> PJREN3_R {
        PJREN3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJREN0"]
    #[inline(always)]
    #[must_use]
    pub fn pjren0(&mut self) -> PJREN0_W<PJREN_SPEC> {
        PJREN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - PJREN1"]
    #[inline(always)]
    #[must_use]
    pub fn pjren1(&mut self) -> PJREN1_W<PJREN_SPEC> {
        PJREN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - PJREN2"]
    #[inline(always)]
    #[must_use]
    pub fn pjren2(&mut self) -> PJREN2_W<PJREN_SPEC> {
        PJREN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - PJREN3"]
    #[inline(always)]
    #[must_use]
    pub fn pjren3(&mut self) -> PJREN3_W<PJREN_SPEC> {
        PJREN3_W::new(self, 3)
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
#[doc = "Port J Resistor Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pjren::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pjren::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PJREN_SPEC;
impl crate::RegisterSpec for PJREN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pjren::R`](R) reader structure"]
impl crate::Readable for PJREN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pjren::W`](W) writer structure"]
impl crate::Writable for PJREN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PJREN to value 0"]
impl crate::Resettable for PJREN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
