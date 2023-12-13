#[doc = "Register `P5DS` reader"]
pub type R = crate::R<P5DS_SPEC>;
#[doc = "Register `P5DS` writer"]
pub type W = crate::W<P5DS_SPEC>;
#[doc = "Field `P5DS0` reader - P5DS0"]
pub type P5DS0_R = crate::BitReader;
#[doc = "Field `P5DS0` writer - P5DS0"]
pub type P5DS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5DS1` reader - P5DS1"]
pub type P5DS1_R = crate::BitReader;
#[doc = "Field `P5DS1` writer - P5DS1"]
pub type P5DS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5DS2` reader - P5DS2"]
pub type P5DS2_R = crate::BitReader;
#[doc = "Field `P5DS2` writer - P5DS2"]
pub type P5DS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5DS3` reader - P5DS3"]
pub type P5DS3_R = crate::BitReader;
#[doc = "Field `P5DS3` writer - P5DS3"]
pub type P5DS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5DS4` reader - P5DS4"]
pub type P5DS4_R = crate::BitReader;
#[doc = "Field `P5DS4` writer - P5DS4"]
pub type P5DS4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5DS5` reader - P5DS5"]
pub type P5DS5_R = crate::BitReader;
#[doc = "Field `P5DS5` writer - P5DS5"]
pub type P5DS5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5DS6` reader - P5DS6"]
pub type P5DS6_R = crate::BitReader;
#[doc = "Field `P5DS6` writer - P5DS6"]
pub type P5DS6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5DS7` reader - P5DS7"]
pub type P5DS7_R = crate::BitReader;
#[doc = "Field `P5DS7` writer - P5DS7"]
pub type P5DS7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P5DS0"]
    #[inline(always)]
    pub fn p5ds0(&self) -> P5DS0_R {
        P5DS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P5DS1"]
    #[inline(always)]
    pub fn p5ds1(&self) -> P5DS1_R {
        P5DS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P5DS2"]
    #[inline(always)]
    pub fn p5ds2(&self) -> P5DS2_R {
        P5DS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P5DS3"]
    #[inline(always)]
    pub fn p5ds3(&self) -> P5DS3_R {
        P5DS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P5DS4"]
    #[inline(always)]
    pub fn p5ds4(&self) -> P5DS4_R {
        P5DS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P5DS5"]
    #[inline(always)]
    pub fn p5ds5(&self) -> P5DS5_R {
        P5DS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P5DS6"]
    #[inline(always)]
    pub fn p5ds6(&self) -> P5DS6_R {
        P5DS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P5DS7"]
    #[inline(always)]
    pub fn p5ds7(&self) -> P5DS7_R {
        P5DS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P5DS0"]
    #[inline(always)]
    #[must_use]
    pub fn p5ds0(&mut self) -> P5DS0_W<P5DS_SPEC> {
        P5DS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P5DS1"]
    #[inline(always)]
    #[must_use]
    pub fn p5ds1(&mut self) -> P5DS1_W<P5DS_SPEC> {
        P5DS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P5DS2"]
    #[inline(always)]
    #[must_use]
    pub fn p5ds2(&mut self) -> P5DS2_W<P5DS_SPEC> {
        P5DS2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P5DS3"]
    #[inline(always)]
    #[must_use]
    pub fn p5ds3(&mut self) -> P5DS3_W<P5DS_SPEC> {
        P5DS3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P5DS4"]
    #[inline(always)]
    #[must_use]
    pub fn p5ds4(&mut self) -> P5DS4_W<P5DS_SPEC> {
        P5DS4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P5DS5"]
    #[inline(always)]
    #[must_use]
    pub fn p5ds5(&mut self) -> P5DS5_W<P5DS_SPEC> {
        P5DS5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P5DS6"]
    #[inline(always)]
    #[must_use]
    pub fn p5ds6(&mut self) -> P5DS6_W<P5DS_SPEC> {
        P5DS6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P5DS7"]
    #[inline(always)]
    #[must_use]
    pub fn p5ds7(&mut self) -> P5DS7_W<P5DS_SPEC> {
        P5DS7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 5 Drive Strenght\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p5ds::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p5ds::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P5DS_SPEC;
impl crate::RegisterSpec for P5DS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p5ds::R`](R) reader structure"]
impl crate::Readable for P5DS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p5ds::W`](W) writer structure"]
impl crate::Writable for P5DS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P5DS to value 0"]
impl crate::Resettable for P5DS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
