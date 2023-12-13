#[doc = "Register `P8SEL` reader"]
pub type R = crate::R<P8SEL_SPEC>;
#[doc = "Register `P8SEL` writer"]
pub type W = crate::W<P8SEL_SPEC>;
#[doc = "Field `P8SEL0` reader - P8SEL0"]
pub type P8SEL0_R = crate::BitReader;
#[doc = "Field `P8SEL0` writer - P8SEL0"]
pub type P8SEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8SEL1` reader - P8SEL1"]
pub type P8SEL1_R = crate::BitReader;
#[doc = "Field `P8SEL1` writer - P8SEL1"]
pub type P8SEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8SEL2` reader - P8SEL2"]
pub type P8SEL2_R = crate::BitReader;
#[doc = "Field `P8SEL2` writer - P8SEL2"]
pub type P8SEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8SEL3` reader - P8SEL3"]
pub type P8SEL3_R = crate::BitReader;
#[doc = "Field `P8SEL3` writer - P8SEL3"]
pub type P8SEL3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8SEL4` reader - P8SEL4"]
pub type P8SEL4_R = crate::BitReader;
#[doc = "Field `P8SEL4` writer - P8SEL4"]
pub type P8SEL4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8SEL5` reader - P8SEL5"]
pub type P8SEL5_R = crate::BitReader;
#[doc = "Field `P8SEL5` writer - P8SEL5"]
pub type P8SEL5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8SEL6` reader - P8SEL6"]
pub type P8SEL6_R = crate::BitReader;
#[doc = "Field `P8SEL6` writer - P8SEL6"]
pub type P8SEL6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8SEL7` reader - P8SEL7"]
pub type P8SEL7_R = crate::BitReader;
#[doc = "Field `P8SEL7` writer - P8SEL7"]
pub type P8SEL7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P8SEL0"]
    #[inline(always)]
    pub fn p8sel0(&self) -> P8SEL0_R {
        P8SEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P8SEL1"]
    #[inline(always)]
    pub fn p8sel1(&self) -> P8SEL1_R {
        P8SEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P8SEL2"]
    #[inline(always)]
    pub fn p8sel2(&self) -> P8SEL2_R {
        P8SEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P8SEL3"]
    #[inline(always)]
    pub fn p8sel3(&self) -> P8SEL3_R {
        P8SEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P8SEL4"]
    #[inline(always)]
    pub fn p8sel4(&self) -> P8SEL4_R {
        P8SEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P8SEL5"]
    #[inline(always)]
    pub fn p8sel5(&self) -> P8SEL5_R {
        P8SEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P8SEL6"]
    #[inline(always)]
    pub fn p8sel6(&self) -> P8SEL6_R {
        P8SEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P8SEL7"]
    #[inline(always)]
    pub fn p8sel7(&self) -> P8SEL7_R {
        P8SEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P8SEL0"]
    #[inline(always)]
    #[must_use]
    pub fn p8sel0(&mut self) -> P8SEL0_W<P8SEL_SPEC> {
        P8SEL0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P8SEL1"]
    #[inline(always)]
    #[must_use]
    pub fn p8sel1(&mut self) -> P8SEL1_W<P8SEL_SPEC> {
        P8SEL1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P8SEL2"]
    #[inline(always)]
    #[must_use]
    pub fn p8sel2(&mut self) -> P8SEL2_W<P8SEL_SPEC> {
        P8SEL2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P8SEL3"]
    #[inline(always)]
    #[must_use]
    pub fn p8sel3(&mut self) -> P8SEL3_W<P8SEL_SPEC> {
        P8SEL3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P8SEL4"]
    #[inline(always)]
    #[must_use]
    pub fn p8sel4(&mut self) -> P8SEL4_W<P8SEL_SPEC> {
        P8SEL4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P8SEL5"]
    #[inline(always)]
    #[must_use]
    pub fn p8sel5(&mut self) -> P8SEL5_W<P8SEL_SPEC> {
        P8SEL5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P8SEL6"]
    #[inline(always)]
    #[must_use]
    pub fn p8sel6(&mut self) -> P8SEL6_W<P8SEL_SPEC> {
        P8SEL6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P8SEL7"]
    #[inline(always)]
    #[must_use]
    pub fn p8sel7(&mut self) -> P8SEL7_W<P8SEL_SPEC> {
        P8SEL7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 8 Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p8sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p8sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P8SEL_SPEC;
impl crate::RegisterSpec for P8SEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p8sel::R`](R) reader structure"]
impl crate::Readable for P8SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p8sel::W`](W) writer structure"]
impl crate::Writable for P8SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P8SEL to value 0"]
impl crate::Resettable for P8SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
