#[doc = "Register `P8DS` reader"]
pub type R = crate::R<P8DS_SPEC>;
#[doc = "Register `P8DS` writer"]
pub type W = crate::W<P8DS_SPEC>;
#[doc = "Field `P8DS0` reader - P8DS0"]
pub type P8DS0_R = crate::BitReader;
#[doc = "Field `P8DS0` writer - P8DS0"]
pub type P8DS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8DS1` reader - P8DS1"]
pub type P8DS1_R = crate::BitReader;
#[doc = "Field `P8DS1` writer - P8DS1"]
pub type P8DS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8DS2` reader - P8DS2"]
pub type P8DS2_R = crate::BitReader;
#[doc = "Field `P8DS2` writer - P8DS2"]
pub type P8DS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8DS3` reader - P8DS3"]
pub type P8DS3_R = crate::BitReader;
#[doc = "Field `P8DS3` writer - P8DS3"]
pub type P8DS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8DS4` reader - P8DS4"]
pub type P8DS4_R = crate::BitReader;
#[doc = "Field `P8DS4` writer - P8DS4"]
pub type P8DS4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8DS5` reader - P8DS5"]
pub type P8DS5_R = crate::BitReader;
#[doc = "Field `P8DS5` writer - P8DS5"]
pub type P8DS5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8DS6` reader - P8DS6"]
pub type P8DS6_R = crate::BitReader;
#[doc = "Field `P8DS6` writer - P8DS6"]
pub type P8DS6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8DS7` reader - P8DS7"]
pub type P8DS7_R = crate::BitReader;
#[doc = "Field `P8DS7` writer - P8DS7"]
pub type P8DS7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P8DS0"]
    #[inline(always)]
    pub fn p8ds0(&self) -> P8DS0_R {
        P8DS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P8DS1"]
    #[inline(always)]
    pub fn p8ds1(&self) -> P8DS1_R {
        P8DS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P8DS2"]
    #[inline(always)]
    pub fn p8ds2(&self) -> P8DS2_R {
        P8DS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P8DS3"]
    #[inline(always)]
    pub fn p8ds3(&self) -> P8DS3_R {
        P8DS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P8DS4"]
    #[inline(always)]
    pub fn p8ds4(&self) -> P8DS4_R {
        P8DS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P8DS5"]
    #[inline(always)]
    pub fn p8ds5(&self) -> P8DS5_R {
        P8DS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P8DS6"]
    #[inline(always)]
    pub fn p8ds6(&self) -> P8DS6_R {
        P8DS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P8DS7"]
    #[inline(always)]
    pub fn p8ds7(&self) -> P8DS7_R {
        P8DS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P8DS0"]
    #[inline(always)]
    #[must_use]
    pub fn p8ds0(&mut self) -> P8DS0_W<P8DS_SPEC> {
        P8DS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P8DS1"]
    #[inline(always)]
    #[must_use]
    pub fn p8ds1(&mut self) -> P8DS1_W<P8DS_SPEC> {
        P8DS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P8DS2"]
    #[inline(always)]
    #[must_use]
    pub fn p8ds2(&mut self) -> P8DS2_W<P8DS_SPEC> {
        P8DS2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P8DS3"]
    #[inline(always)]
    #[must_use]
    pub fn p8ds3(&mut self) -> P8DS3_W<P8DS_SPEC> {
        P8DS3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P8DS4"]
    #[inline(always)]
    #[must_use]
    pub fn p8ds4(&mut self) -> P8DS4_W<P8DS_SPEC> {
        P8DS4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P8DS5"]
    #[inline(always)]
    #[must_use]
    pub fn p8ds5(&mut self) -> P8DS5_W<P8DS_SPEC> {
        P8DS5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P8DS6"]
    #[inline(always)]
    #[must_use]
    pub fn p8ds6(&mut self) -> P8DS6_W<P8DS_SPEC> {
        P8DS6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P8DS7"]
    #[inline(always)]
    #[must_use]
    pub fn p8ds7(&mut self) -> P8DS7_W<P8DS_SPEC> {
        P8DS7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 8 Drive Strenght\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p8ds::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p8ds::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P8DS_SPEC;
impl crate::RegisterSpec for P8DS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p8ds::R`](R) reader structure"]
impl crate::Readable for P8DS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p8ds::W`](W) writer structure"]
impl crate::Writable for P8DS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P8DS to value 0"]
impl crate::Resettable for P8DS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
