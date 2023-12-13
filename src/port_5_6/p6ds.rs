#[doc = "Register `P6DS` reader"]
pub type R = crate::R<P6DS_SPEC>;
#[doc = "Register `P6DS` writer"]
pub type W = crate::W<P6DS_SPEC>;
#[doc = "Field `P6DS0` reader - P6DS0"]
pub type P6DS0_R = crate::BitReader;
#[doc = "Field `P6DS0` writer - P6DS0"]
pub type P6DS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6DS1` reader - P6DS1"]
pub type P6DS1_R = crate::BitReader;
#[doc = "Field `P6DS1` writer - P6DS1"]
pub type P6DS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6DS2` reader - P6DS2"]
pub type P6DS2_R = crate::BitReader;
#[doc = "Field `P6DS2` writer - P6DS2"]
pub type P6DS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6DS3` reader - P6DS3"]
pub type P6DS3_R = crate::BitReader;
#[doc = "Field `P6DS3` writer - P6DS3"]
pub type P6DS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6DS4` reader - P6DS4"]
pub type P6DS4_R = crate::BitReader;
#[doc = "Field `P6DS4` writer - P6DS4"]
pub type P6DS4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6DS5` reader - P6DS5"]
pub type P6DS5_R = crate::BitReader;
#[doc = "Field `P6DS5` writer - P6DS5"]
pub type P6DS5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6DS6` reader - P6DS6"]
pub type P6DS6_R = crate::BitReader;
#[doc = "Field `P6DS6` writer - P6DS6"]
pub type P6DS6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6DS7` reader - P6DS7"]
pub type P6DS7_R = crate::BitReader;
#[doc = "Field `P6DS7` writer - P6DS7"]
pub type P6DS7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P6DS0"]
    #[inline(always)]
    pub fn p6ds0(&self) -> P6DS0_R {
        P6DS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P6DS1"]
    #[inline(always)]
    pub fn p6ds1(&self) -> P6DS1_R {
        P6DS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P6DS2"]
    #[inline(always)]
    pub fn p6ds2(&self) -> P6DS2_R {
        P6DS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P6DS3"]
    #[inline(always)]
    pub fn p6ds3(&self) -> P6DS3_R {
        P6DS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P6DS4"]
    #[inline(always)]
    pub fn p6ds4(&self) -> P6DS4_R {
        P6DS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P6DS5"]
    #[inline(always)]
    pub fn p6ds5(&self) -> P6DS5_R {
        P6DS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P6DS6"]
    #[inline(always)]
    pub fn p6ds6(&self) -> P6DS6_R {
        P6DS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P6DS7"]
    #[inline(always)]
    pub fn p6ds7(&self) -> P6DS7_R {
        P6DS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P6DS0"]
    #[inline(always)]
    #[must_use]
    pub fn p6ds0(&mut self) -> P6DS0_W<P6DS_SPEC> {
        P6DS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P6DS1"]
    #[inline(always)]
    #[must_use]
    pub fn p6ds1(&mut self) -> P6DS1_W<P6DS_SPEC> {
        P6DS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P6DS2"]
    #[inline(always)]
    #[must_use]
    pub fn p6ds2(&mut self) -> P6DS2_W<P6DS_SPEC> {
        P6DS2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P6DS3"]
    #[inline(always)]
    #[must_use]
    pub fn p6ds3(&mut self) -> P6DS3_W<P6DS_SPEC> {
        P6DS3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P6DS4"]
    #[inline(always)]
    #[must_use]
    pub fn p6ds4(&mut self) -> P6DS4_W<P6DS_SPEC> {
        P6DS4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P6DS5"]
    #[inline(always)]
    #[must_use]
    pub fn p6ds5(&mut self) -> P6DS5_W<P6DS_SPEC> {
        P6DS5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P6DS6"]
    #[inline(always)]
    #[must_use]
    pub fn p6ds6(&mut self) -> P6DS6_W<P6DS_SPEC> {
        P6DS6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P6DS7"]
    #[inline(always)]
    #[must_use]
    pub fn p6ds7(&mut self) -> P6DS7_W<P6DS_SPEC> {
        P6DS7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 6 Drive Strenght\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p6ds::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p6ds::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P6DS_SPEC;
impl crate::RegisterSpec for P6DS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p6ds::R`](R) reader structure"]
impl crate::Readable for P6DS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p6ds::W`](W) writer structure"]
impl crate::Writable for P6DS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P6DS to value 0"]
impl crate::Resettable for P6DS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
