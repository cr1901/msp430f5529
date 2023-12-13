#[doc = "Register `P3DS` reader"]
pub type R = crate::R<P3DS_SPEC>;
#[doc = "Register `P3DS` writer"]
pub type W = crate::W<P3DS_SPEC>;
#[doc = "Field `P3DS0` reader - P3DS0"]
pub type P3DS0_R = crate::BitReader;
#[doc = "Field `P3DS0` writer - P3DS0"]
pub type P3DS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3DS1` reader - P3DS1"]
pub type P3DS1_R = crate::BitReader;
#[doc = "Field `P3DS1` writer - P3DS1"]
pub type P3DS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3DS2` reader - P3DS2"]
pub type P3DS2_R = crate::BitReader;
#[doc = "Field `P3DS2` writer - P3DS2"]
pub type P3DS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3DS3` reader - P3DS3"]
pub type P3DS3_R = crate::BitReader;
#[doc = "Field `P3DS3` writer - P3DS3"]
pub type P3DS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3DS4` reader - P3DS4"]
pub type P3DS4_R = crate::BitReader;
#[doc = "Field `P3DS4` writer - P3DS4"]
pub type P3DS4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3DS5` reader - P3DS5"]
pub type P3DS5_R = crate::BitReader;
#[doc = "Field `P3DS5` writer - P3DS5"]
pub type P3DS5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3DS6` reader - P3DS6"]
pub type P3DS6_R = crate::BitReader;
#[doc = "Field `P3DS6` writer - P3DS6"]
pub type P3DS6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3DS7` reader - P3DS7"]
pub type P3DS7_R = crate::BitReader;
#[doc = "Field `P3DS7` writer - P3DS7"]
pub type P3DS7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P3DS0"]
    #[inline(always)]
    pub fn p3ds0(&self) -> P3DS0_R {
        P3DS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P3DS1"]
    #[inline(always)]
    pub fn p3ds1(&self) -> P3DS1_R {
        P3DS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P3DS2"]
    #[inline(always)]
    pub fn p3ds2(&self) -> P3DS2_R {
        P3DS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P3DS3"]
    #[inline(always)]
    pub fn p3ds3(&self) -> P3DS3_R {
        P3DS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P3DS4"]
    #[inline(always)]
    pub fn p3ds4(&self) -> P3DS4_R {
        P3DS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P3DS5"]
    #[inline(always)]
    pub fn p3ds5(&self) -> P3DS5_R {
        P3DS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P3DS6"]
    #[inline(always)]
    pub fn p3ds6(&self) -> P3DS6_R {
        P3DS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P3DS7"]
    #[inline(always)]
    pub fn p3ds7(&self) -> P3DS7_R {
        P3DS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3DS0"]
    #[inline(always)]
    #[must_use]
    pub fn p3ds0(&mut self) -> P3DS0_W<P3DS_SPEC> {
        P3DS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P3DS1"]
    #[inline(always)]
    #[must_use]
    pub fn p3ds1(&mut self) -> P3DS1_W<P3DS_SPEC> {
        P3DS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P3DS2"]
    #[inline(always)]
    #[must_use]
    pub fn p3ds2(&mut self) -> P3DS2_W<P3DS_SPEC> {
        P3DS2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P3DS3"]
    #[inline(always)]
    #[must_use]
    pub fn p3ds3(&mut self) -> P3DS3_W<P3DS_SPEC> {
        P3DS3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P3DS4"]
    #[inline(always)]
    #[must_use]
    pub fn p3ds4(&mut self) -> P3DS4_W<P3DS_SPEC> {
        P3DS4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P3DS5"]
    #[inline(always)]
    #[must_use]
    pub fn p3ds5(&mut self) -> P3DS5_W<P3DS_SPEC> {
        P3DS5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P3DS6"]
    #[inline(always)]
    #[must_use]
    pub fn p3ds6(&mut self) -> P3DS6_W<P3DS_SPEC> {
        P3DS6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P3DS7"]
    #[inline(always)]
    #[must_use]
    pub fn p3ds7(&mut self) -> P3DS7_W<P3DS_SPEC> {
        P3DS7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 3 Drive Strenght\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p3ds::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p3ds::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3DS_SPEC;
impl crate::RegisterSpec for P3DS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3ds::R`](R) reader structure"]
impl crate::Readable for P3DS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p3ds::W`](W) writer structure"]
impl crate::Writable for P3DS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P3DS to value 0"]
impl crate::Resettable for P3DS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
