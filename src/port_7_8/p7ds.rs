#[doc = "Register `P7DS` reader"]
pub type R = crate::R<P7DS_SPEC>;
#[doc = "Register `P7DS` writer"]
pub type W = crate::W<P7DS_SPEC>;
#[doc = "Field `P7DS0` reader - P7DS0"]
pub type P7DS0_R = crate::BitReader;
#[doc = "Field `P7DS0` writer - P7DS0"]
pub type P7DS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7DS1` reader - P7DS1"]
pub type P7DS1_R = crate::BitReader;
#[doc = "Field `P7DS1` writer - P7DS1"]
pub type P7DS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7DS2` reader - P7DS2"]
pub type P7DS2_R = crate::BitReader;
#[doc = "Field `P7DS2` writer - P7DS2"]
pub type P7DS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7DS3` reader - P7DS3"]
pub type P7DS3_R = crate::BitReader;
#[doc = "Field `P7DS3` writer - P7DS3"]
pub type P7DS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7DS4` reader - P7DS4"]
pub type P7DS4_R = crate::BitReader;
#[doc = "Field `P7DS4` writer - P7DS4"]
pub type P7DS4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7DS5` reader - P7DS5"]
pub type P7DS5_R = crate::BitReader;
#[doc = "Field `P7DS5` writer - P7DS5"]
pub type P7DS5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7DS6` reader - P7DS6"]
pub type P7DS6_R = crate::BitReader;
#[doc = "Field `P7DS6` writer - P7DS6"]
pub type P7DS6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7DS7` reader - P7DS7"]
pub type P7DS7_R = crate::BitReader;
#[doc = "Field `P7DS7` writer - P7DS7"]
pub type P7DS7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P7DS0"]
    #[inline(always)]
    pub fn p7ds0(&self) -> P7DS0_R {
        P7DS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P7DS1"]
    #[inline(always)]
    pub fn p7ds1(&self) -> P7DS1_R {
        P7DS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P7DS2"]
    #[inline(always)]
    pub fn p7ds2(&self) -> P7DS2_R {
        P7DS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P7DS3"]
    #[inline(always)]
    pub fn p7ds3(&self) -> P7DS3_R {
        P7DS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P7DS4"]
    #[inline(always)]
    pub fn p7ds4(&self) -> P7DS4_R {
        P7DS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P7DS5"]
    #[inline(always)]
    pub fn p7ds5(&self) -> P7DS5_R {
        P7DS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P7DS6"]
    #[inline(always)]
    pub fn p7ds6(&self) -> P7DS6_R {
        P7DS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P7DS7"]
    #[inline(always)]
    pub fn p7ds7(&self) -> P7DS7_R {
        P7DS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P7DS0"]
    #[inline(always)]
    #[must_use]
    pub fn p7ds0(&mut self) -> P7DS0_W<P7DS_SPEC> {
        P7DS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P7DS1"]
    #[inline(always)]
    #[must_use]
    pub fn p7ds1(&mut self) -> P7DS1_W<P7DS_SPEC> {
        P7DS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P7DS2"]
    #[inline(always)]
    #[must_use]
    pub fn p7ds2(&mut self) -> P7DS2_W<P7DS_SPEC> {
        P7DS2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P7DS3"]
    #[inline(always)]
    #[must_use]
    pub fn p7ds3(&mut self) -> P7DS3_W<P7DS_SPEC> {
        P7DS3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P7DS4"]
    #[inline(always)]
    #[must_use]
    pub fn p7ds4(&mut self) -> P7DS4_W<P7DS_SPEC> {
        P7DS4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P7DS5"]
    #[inline(always)]
    #[must_use]
    pub fn p7ds5(&mut self) -> P7DS5_W<P7DS_SPEC> {
        P7DS5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P7DS6"]
    #[inline(always)]
    #[must_use]
    pub fn p7ds6(&mut self) -> P7DS6_W<P7DS_SPEC> {
        P7DS6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P7DS7"]
    #[inline(always)]
    #[must_use]
    pub fn p7ds7(&mut self) -> P7DS7_W<P7DS_SPEC> {
        P7DS7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 7 Drive Strenght\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p7ds::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p7ds::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P7DS_SPEC;
impl crate::RegisterSpec for P7DS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p7ds::R`](R) reader structure"]
impl crate::Readable for P7DS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p7ds::W`](W) writer structure"]
impl crate::Writable for P7DS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P7DS to value 0"]
impl crate::Resettable for P7DS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
