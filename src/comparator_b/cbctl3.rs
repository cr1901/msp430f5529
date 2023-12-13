#[doc = "Register `CBCTL3` reader"]
pub type R = crate::R<CBCTL3_SPEC>;
#[doc = "Register `CBCTL3` writer"]
pub type W = crate::W<CBCTL3_SPEC>;
#[doc = "Field `CBPD0` reader - Comp. B Disable Input Buffer of Port Register .0"]
pub type CBPD0_R = crate::BitReader;
#[doc = "Field `CBPD0` writer - Comp. B Disable Input Buffer of Port Register .0"]
pub type CBPD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBPD1` reader - Comp. B Disable Input Buffer of Port Register .1"]
pub type CBPD1_R = crate::BitReader;
#[doc = "Field `CBPD1` writer - Comp. B Disable Input Buffer of Port Register .1"]
pub type CBPD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBPD2` reader - Comp. B Disable Input Buffer of Port Register .2"]
pub type CBPD2_R = crate::BitReader;
#[doc = "Field `CBPD2` writer - Comp. B Disable Input Buffer of Port Register .2"]
pub type CBPD2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBPD3` reader - Comp. B Disable Input Buffer of Port Register .3"]
pub type CBPD3_R = crate::BitReader;
#[doc = "Field `CBPD3` writer - Comp. B Disable Input Buffer of Port Register .3"]
pub type CBPD3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBPD4` reader - Comp. B Disable Input Buffer of Port Register .4"]
pub type CBPD4_R = crate::BitReader;
#[doc = "Field `CBPD4` writer - Comp. B Disable Input Buffer of Port Register .4"]
pub type CBPD4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBPD5` reader - Comp. B Disable Input Buffer of Port Register .5"]
pub type CBPD5_R = crate::BitReader;
#[doc = "Field `CBPD5` writer - Comp. B Disable Input Buffer of Port Register .5"]
pub type CBPD5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBPD6` reader - Comp. B Disable Input Buffer of Port Register .6"]
pub type CBPD6_R = crate::BitReader;
#[doc = "Field `CBPD6` writer - Comp. B Disable Input Buffer of Port Register .6"]
pub type CBPD6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBPD7` reader - Comp. B Disable Input Buffer of Port Register .7"]
pub type CBPD7_R = crate::BitReader;
#[doc = "Field `CBPD7` writer - Comp. B Disable Input Buffer of Port Register .7"]
pub type CBPD7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBPD8` reader - Comp. B Disable Input Buffer of Port Register .8"]
pub type CBPD8_R = crate::BitReader;
#[doc = "Field `CBPD8` writer - Comp. B Disable Input Buffer of Port Register .8"]
pub type CBPD8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBPD9` reader - Comp. B Disable Input Buffer of Port Register .9"]
pub type CBPD9_R = crate::BitReader;
#[doc = "Field `CBPD9` writer - Comp. B Disable Input Buffer of Port Register .9"]
pub type CBPD9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBPD10` reader - Comp. B Disable Input Buffer of Port Register .10"]
pub type CBPD10_R = crate::BitReader;
#[doc = "Field `CBPD10` writer - Comp. B Disable Input Buffer of Port Register .10"]
pub type CBPD10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBPD11` reader - Comp. B Disable Input Buffer of Port Register .11"]
pub type CBPD11_R = crate::BitReader;
#[doc = "Field `CBPD11` writer - Comp. B Disable Input Buffer of Port Register .11"]
pub type CBPD11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBPD12` reader - Comp. B Disable Input Buffer of Port Register .12"]
pub type CBPD12_R = crate::BitReader;
#[doc = "Field `CBPD12` writer - Comp. B Disable Input Buffer of Port Register .12"]
pub type CBPD12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBPD13` reader - Comp. B Disable Input Buffer of Port Register .13"]
pub type CBPD13_R = crate::BitReader;
#[doc = "Field `CBPD13` writer - Comp. B Disable Input Buffer of Port Register .13"]
pub type CBPD13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBPD14` reader - Comp. B Disable Input Buffer of Port Register .14"]
pub type CBPD14_R = crate::BitReader;
#[doc = "Field `CBPD14` writer - Comp. B Disable Input Buffer of Port Register .14"]
pub type CBPD14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBPD15` reader - Comp. B Disable Input Buffer of Port Register .15"]
pub type CBPD15_R = crate::BitReader;
#[doc = "Field `CBPD15` writer - Comp. B Disable Input Buffer of Port Register .15"]
pub type CBPD15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comp. B Disable Input Buffer of Port Register .0"]
    #[inline(always)]
    pub fn cbpd0(&self) -> CBPD0_R {
        CBPD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comp. B Disable Input Buffer of Port Register .1"]
    #[inline(always)]
    pub fn cbpd1(&self) -> CBPD1_R {
        CBPD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comp. B Disable Input Buffer of Port Register .2"]
    #[inline(always)]
    pub fn cbpd2(&self) -> CBPD2_R {
        CBPD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comp. B Disable Input Buffer of Port Register .3"]
    #[inline(always)]
    pub fn cbpd3(&self) -> CBPD3_R {
        CBPD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comp. B Disable Input Buffer of Port Register .4"]
    #[inline(always)]
    pub fn cbpd4(&self) -> CBPD4_R {
        CBPD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comp. B Disable Input Buffer of Port Register .5"]
    #[inline(always)]
    pub fn cbpd5(&self) -> CBPD5_R {
        CBPD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comp. B Disable Input Buffer of Port Register .6"]
    #[inline(always)]
    pub fn cbpd6(&self) -> CBPD6_R {
        CBPD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comp. B Disable Input Buffer of Port Register .7"]
    #[inline(always)]
    pub fn cbpd7(&self) -> CBPD7_R {
        CBPD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Comp. B Disable Input Buffer of Port Register .8"]
    #[inline(always)]
    pub fn cbpd8(&self) -> CBPD8_R {
        CBPD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Comp. B Disable Input Buffer of Port Register .9"]
    #[inline(always)]
    pub fn cbpd9(&self) -> CBPD9_R {
        CBPD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Comp. B Disable Input Buffer of Port Register .10"]
    #[inline(always)]
    pub fn cbpd10(&self) -> CBPD10_R {
        CBPD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Comp. B Disable Input Buffer of Port Register .11"]
    #[inline(always)]
    pub fn cbpd11(&self) -> CBPD11_R {
        CBPD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Comp. B Disable Input Buffer of Port Register .12"]
    #[inline(always)]
    pub fn cbpd12(&self) -> CBPD12_R {
        CBPD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Comp. B Disable Input Buffer of Port Register .13"]
    #[inline(always)]
    pub fn cbpd13(&self) -> CBPD13_R {
        CBPD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Comp. B Disable Input Buffer of Port Register .14"]
    #[inline(always)]
    pub fn cbpd14(&self) -> CBPD14_R {
        CBPD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Comp. B Disable Input Buffer of Port Register .15"]
    #[inline(always)]
    pub fn cbpd15(&self) -> CBPD15_R {
        CBPD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. B Disable Input Buffer of Port Register .0"]
    #[inline(always)]
    #[must_use]
    pub fn cbpd0(&mut self) -> CBPD0_W<CBCTL3_SPEC> {
        CBPD0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Comp. B Disable Input Buffer of Port Register .1"]
    #[inline(always)]
    #[must_use]
    pub fn cbpd1(&mut self) -> CBPD1_W<CBCTL3_SPEC> {
        CBPD1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Comp. B Disable Input Buffer of Port Register .2"]
    #[inline(always)]
    #[must_use]
    pub fn cbpd2(&mut self) -> CBPD2_W<CBCTL3_SPEC> {
        CBPD2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Comp. B Disable Input Buffer of Port Register .3"]
    #[inline(always)]
    #[must_use]
    pub fn cbpd3(&mut self) -> CBPD3_W<CBCTL3_SPEC> {
        CBPD3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Comp. B Disable Input Buffer of Port Register .4"]
    #[inline(always)]
    #[must_use]
    pub fn cbpd4(&mut self) -> CBPD4_W<CBCTL3_SPEC> {
        CBPD4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Comp. B Disable Input Buffer of Port Register .5"]
    #[inline(always)]
    #[must_use]
    pub fn cbpd5(&mut self) -> CBPD5_W<CBCTL3_SPEC> {
        CBPD5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Comp. B Disable Input Buffer of Port Register .6"]
    #[inline(always)]
    #[must_use]
    pub fn cbpd6(&mut self) -> CBPD6_W<CBCTL3_SPEC> {
        CBPD6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Comp. B Disable Input Buffer of Port Register .7"]
    #[inline(always)]
    #[must_use]
    pub fn cbpd7(&mut self) -> CBPD7_W<CBCTL3_SPEC> {
        CBPD7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Comp. B Disable Input Buffer of Port Register .8"]
    #[inline(always)]
    #[must_use]
    pub fn cbpd8(&mut self) -> CBPD8_W<CBCTL3_SPEC> {
        CBPD8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Comp. B Disable Input Buffer of Port Register .9"]
    #[inline(always)]
    #[must_use]
    pub fn cbpd9(&mut self) -> CBPD9_W<CBCTL3_SPEC> {
        CBPD9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Comp. B Disable Input Buffer of Port Register .10"]
    #[inline(always)]
    #[must_use]
    pub fn cbpd10(&mut self) -> CBPD10_W<CBCTL3_SPEC> {
        CBPD10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Comp. B Disable Input Buffer of Port Register .11"]
    #[inline(always)]
    #[must_use]
    pub fn cbpd11(&mut self) -> CBPD11_W<CBCTL3_SPEC> {
        CBPD11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Comp. B Disable Input Buffer of Port Register .12"]
    #[inline(always)]
    #[must_use]
    pub fn cbpd12(&mut self) -> CBPD12_W<CBCTL3_SPEC> {
        CBPD12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Comp. B Disable Input Buffer of Port Register .13"]
    #[inline(always)]
    #[must_use]
    pub fn cbpd13(&mut self) -> CBPD13_W<CBCTL3_SPEC> {
        CBPD13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Comp. B Disable Input Buffer of Port Register .14"]
    #[inline(always)]
    #[must_use]
    pub fn cbpd14(&mut self) -> CBPD14_W<CBCTL3_SPEC> {
        CBPD14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Comp. B Disable Input Buffer of Port Register .15"]
    #[inline(always)]
    #[must_use]
    pub fn cbpd15(&mut self) -> CBPD15_W<CBCTL3_SPEC> {
        CBPD15_W::new(self, 15)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Comparator B Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cbctl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cbctl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CBCTL3_SPEC;
impl crate::RegisterSpec for CBCTL3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cbctl3::R`](R) reader structure"]
impl crate::Readable for CBCTL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cbctl3::W`](W) writer structure"]
impl crate::Writable for CBCTL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CBCTL3 to value 0"]
impl crate::Resettable for CBCTL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
