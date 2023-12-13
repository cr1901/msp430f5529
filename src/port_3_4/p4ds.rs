#[doc = "Register `P4DS` reader"]
pub type R = crate::R<P4DS_SPEC>;
#[doc = "Register `P4DS` writer"]
pub type W = crate::W<P4DS_SPEC>;
#[doc = "Field `P4DS0` reader - P4DS0"]
pub type P4DS0_R = crate::BitReader;
#[doc = "Field `P4DS0` writer - P4DS0"]
pub type P4DS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4DS1` reader - P4DS1"]
pub type P4DS1_R = crate::BitReader;
#[doc = "Field `P4DS1` writer - P4DS1"]
pub type P4DS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4DS2` reader - P4DS2"]
pub type P4DS2_R = crate::BitReader;
#[doc = "Field `P4DS2` writer - P4DS2"]
pub type P4DS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4DS3` reader - P4DS3"]
pub type P4DS3_R = crate::BitReader;
#[doc = "Field `P4DS3` writer - P4DS3"]
pub type P4DS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4DS4` reader - P4DS4"]
pub type P4DS4_R = crate::BitReader;
#[doc = "Field `P4DS4` writer - P4DS4"]
pub type P4DS4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4DS5` reader - P4DS5"]
pub type P4DS5_R = crate::BitReader;
#[doc = "Field `P4DS5` writer - P4DS5"]
pub type P4DS5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4DS6` reader - P4DS6"]
pub type P4DS6_R = crate::BitReader;
#[doc = "Field `P4DS6` writer - P4DS6"]
pub type P4DS6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4DS7` reader - P4DS7"]
pub type P4DS7_R = crate::BitReader;
#[doc = "Field `P4DS7` writer - P4DS7"]
pub type P4DS7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P4DS0"]
    #[inline(always)]
    pub fn p4ds0(&self) -> P4DS0_R {
        P4DS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P4DS1"]
    #[inline(always)]
    pub fn p4ds1(&self) -> P4DS1_R {
        P4DS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P4DS2"]
    #[inline(always)]
    pub fn p4ds2(&self) -> P4DS2_R {
        P4DS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P4DS3"]
    #[inline(always)]
    pub fn p4ds3(&self) -> P4DS3_R {
        P4DS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P4DS4"]
    #[inline(always)]
    pub fn p4ds4(&self) -> P4DS4_R {
        P4DS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P4DS5"]
    #[inline(always)]
    pub fn p4ds5(&self) -> P4DS5_R {
        P4DS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P4DS6"]
    #[inline(always)]
    pub fn p4ds6(&self) -> P4DS6_R {
        P4DS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P4DS7"]
    #[inline(always)]
    pub fn p4ds7(&self) -> P4DS7_R {
        P4DS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4DS0"]
    #[inline(always)]
    #[must_use]
    pub fn p4ds0(&mut self) -> P4DS0_W<P4DS_SPEC> {
        P4DS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P4DS1"]
    #[inline(always)]
    #[must_use]
    pub fn p4ds1(&mut self) -> P4DS1_W<P4DS_SPEC> {
        P4DS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P4DS2"]
    #[inline(always)]
    #[must_use]
    pub fn p4ds2(&mut self) -> P4DS2_W<P4DS_SPEC> {
        P4DS2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P4DS3"]
    #[inline(always)]
    #[must_use]
    pub fn p4ds3(&mut self) -> P4DS3_W<P4DS_SPEC> {
        P4DS3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P4DS4"]
    #[inline(always)]
    #[must_use]
    pub fn p4ds4(&mut self) -> P4DS4_W<P4DS_SPEC> {
        P4DS4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P4DS5"]
    #[inline(always)]
    #[must_use]
    pub fn p4ds5(&mut self) -> P4DS5_W<P4DS_SPEC> {
        P4DS5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P4DS6"]
    #[inline(always)]
    #[must_use]
    pub fn p4ds6(&mut self) -> P4DS6_W<P4DS_SPEC> {
        P4DS6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P4DS7"]
    #[inline(always)]
    #[must_use]
    pub fn p4ds7(&mut self) -> P4DS7_W<P4DS_SPEC> {
        P4DS7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 4 Drive Strenght\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p4ds::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p4ds::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4DS_SPEC;
impl crate::RegisterSpec for P4DS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4ds::R`](R) reader structure"]
impl crate::Readable for P4DS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p4ds::W`](W) writer structure"]
impl crate::Writable for P4DS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P4DS to value 0"]
impl crate::Resettable for P4DS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
