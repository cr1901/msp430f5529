#[doc = "Register `P2DS` reader"]
pub type R = crate::R<P2DS_SPEC>;
#[doc = "Register `P2DS` writer"]
pub type W = crate::W<P2DS_SPEC>;
#[doc = "Field `P2DS0` reader - P2DS0"]
pub type P2DS0_R = crate::BitReader;
#[doc = "Field `P2DS0` writer - P2DS0"]
pub type P2DS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2DS1` reader - P2DS1"]
pub type P2DS1_R = crate::BitReader;
#[doc = "Field `P2DS1` writer - P2DS1"]
pub type P2DS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2DS2` reader - P2DS2"]
pub type P2DS2_R = crate::BitReader;
#[doc = "Field `P2DS2` writer - P2DS2"]
pub type P2DS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2DS3` reader - P2DS3"]
pub type P2DS3_R = crate::BitReader;
#[doc = "Field `P2DS3` writer - P2DS3"]
pub type P2DS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2DS4` reader - P2DS4"]
pub type P2DS4_R = crate::BitReader;
#[doc = "Field `P2DS4` writer - P2DS4"]
pub type P2DS4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2DS5` reader - P2DS5"]
pub type P2DS5_R = crate::BitReader;
#[doc = "Field `P2DS5` writer - P2DS5"]
pub type P2DS5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2DS6` reader - P2DS6"]
pub type P2DS6_R = crate::BitReader;
#[doc = "Field `P2DS6` writer - P2DS6"]
pub type P2DS6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2DS7` reader - P2DS7"]
pub type P2DS7_R = crate::BitReader;
#[doc = "Field `P2DS7` writer - P2DS7"]
pub type P2DS7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P2DS0"]
    #[inline(always)]
    pub fn p2ds0(&self) -> P2DS0_R {
        P2DS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P2DS1"]
    #[inline(always)]
    pub fn p2ds1(&self) -> P2DS1_R {
        P2DS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2DS2"]
    #[inline(always)]
    pub fn p2ds2(&self) -> P2DS2_R {
        P2DS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P2DS3"]
    #[inline(always)]
    pub fn p2ds3(&self) -> P2DS3_R {
        P2DS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P2DS4"]
    #[inline(always)]
    pub fn p2ds4(&self) -> P2DS4_R {
        P2DS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P2DS5"]
    #[inline(always)]
    pub fn p2ds5(&self) -> P2DS5_R {
        P2DS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P2DS6"]
    #[inline(always)]
    pub fn p2ds6(&self) -> P2DS6_R {
        P2DS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P2DS7"]
    #[inline(always)]
    pub fn p2ds7(&self) -> P2DS7_R {
        P2DS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2DS0"]
    #[inline(always)]
    #[must_use]
    pub fn p2ds0(&mut self) -> P2DS0_W<P2DS_SPEC> {
        P2DS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P2DS1"]
    #[inline(always)]
    #[must_use]
    pub fn p2ds1(&mut self) -> P2DS1_W<P2DS_SPEC> {
        P2DS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P2DS2"]
    #[inline(always)]
    #[must_use]
    pub fn p2ds2(&mut self) -> P2DS2_W<P2DS_SPEC> {
        P2DS2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P2DS3"]
    #[inline(always)]
    #[must_use]
    pub fn p2ds3(&mut self) -> P2DS3_W<P2DS_SPEC> {
        P2DS3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P2DS4"]
    #[inline(always)]
    #[must_use]
    pub fn p2ds4(&mut self) -> P2DS4_W<P2DS_SPEC> {
        P2DS4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P2DS5"]
    #[inline(always)]
    #[must_use]
    pub fn p2ds5(&mut self) -> P2DS5_W<P2DS_SPEC> {
        P2DS5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P2DS6"]
    #[inline(always)]
    #[must_use]
    pub fn p2ds6(&mut self) -> P2DS6_W<P2DS_SPEC> {
        P2DS6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P2DS7"]
    #[inline(always)]
    #[must_use]
    pub fn p2ds7(&mut self) -> P2DS7_W<P2DS_SPEC> {
        P2DS7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 2 Drive Strenght\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p2ds::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p2ds::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2DS_SPEC;
impl crate::RegisterSpec for P2DS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2ds::R`](R) reader structure"]
impl crate::Readable for P2DS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p2ds::W`](W) writer structure"]
impl crate::Writable for P2DS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P2DS to value 0"]
impl crate::Resettable for P2DS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
