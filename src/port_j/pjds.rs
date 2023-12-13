#[doc = "Register `PJDS` reader"]
pub type R = crate::R<PJDS_SPEC>;
#[doc = "Register `PJDS` writer"]
pub type W = crate::W<PJDS_SPEC>;
#[doc = "Field `PJDS0` reader - PJDS0"]
pub type PJDS0_R = crate::BitReader;
#[doc = "Field `PJDS0` writer - PJDS0"]
pub type PJDS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJDS1` reader - PJDS1"]
pub type PJDS1_R = crate::BitReader;
#[doc = "Field `PJDS1` writer - PJDS1"]
pub type PJDS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJDS2` reader - PJDS2"]
pub type PJDS2_R = crate::BitReader;
#[doc = "Field `PJDS2` writer - PJDS2"]
pub type PJDS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJDS3` reader - PJDS3"]
pub type PJDS3_R = crate::BitReader;
#[doc = "Field `PJDS3` writer - PJDS3"]
pub type PJDS3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PJDS0"]
    #[inline(always)]
    pub fn pjds0(&self) -> PJDS0_R {
        PJDS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PJDS1"]
    #[inline(always)]
    pub fn pjds1(&self) -> PJDS1_R {
        PJDS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PJDS2"]
    #[inline(always)]
    pub fn pjds2(&self) -> PJDS2_R {
        PJDS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PJDS3"]
    #[inline(always)]
    pub fn pjds3(&self) -> PJDS3_R {
        PJDS3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJDS0"]
    #[inline(always)]
    #[must_use]
    pub fn pjds0(&mut self) -> PJDS0_W<PJDS_SPEC> {
        PJDS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - PJDS1"]
    #[inline(always)]
    #[must_use]
    pub fn pjds1(&mut self) -> PJDS1_W<PJDS_SPEC> {
        PJDS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - PJDS2"]
    #[inline(always)]
    #[must_use]
    pub fn pjds2(&mut self) -> PJDS2_W<PJDS_SPEC> {
        PJDS2_W::new(self, 2)
    }
    #[doc = "Bit 3 - PJDS3"]
    #[inline(always)]
    #[must_use]
    pub fn pjds3(&mut self) -> PJDS3_W<PJDS_SPEC> {
        PJDS3_W::new(self, 3)
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
#[doc = "Port J Drive Strenght\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pjds::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pjds::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PJDS_SPEC;
impl crate::RegisterSpec for PJDS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pjds::R`](R) reader structure"]
impl crate::Readable for PJDS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pjds::W`](W) writer structure"]
impl crate::Writable for PJDS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PJDS to value 0"]
impl crate::Resettable for PJDS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
