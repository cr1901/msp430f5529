#[doc = "Register `P3SEL` reader"]
pub type R = crate::R<P3SEL_SPEC>;
#[doc = "Register `P3SEL` writer"]
pub type W = crate::W<P3SEL_SPEC>;
#[doc = "Field `P3SEL0` reader - P3SEL0"]
pub type P3SEL0_R = crate::BitReader;
#[doc = "Field `P3SEL0` writer - P3SEL0"]
pub type P3SEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3SEL1` reader - P3SEL1"]
pub type P3SEL1_R = crate::BitReader;
#[doc = "Field `P3SEL1` writer - P3SEL1"]
pub type P3SEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3SEL2` reader - P3SEL2"]
pub type P3SEL2_R = crate::BitReader;
#[doc = "Field `P3SEL2` writer - P3SEL2"]
pub type P3SEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3SEL3` reader - P3SEL3"]
pub type P3SEL3_R = crate::BitReader;
#[doc = "Field `P3SEL3` writer - P3SEL3"]
pub type P3SEL3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3SEL4` reader - P3SEL4"]
pub type P3SEL4_R = crate::BitReader;
#[doc = "Field `P3SEL4` writer - P3SEL4"]
pub type P3SEL4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3SEL5` reader - P3SEL5"]
pub type P3SEL5_R = crate::BitReader;
#[doc = "Field `P3SEL5` writer - P3SEL5"]
pub type P3SEL5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3SEL6` reader - P3SEL6"]
pub type P3SEL6_R = crate::BitReader;
#[doc = "Field `P3SEL6` writer - P3SEL6"]
pub type P3SEL6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3SEL7` reader - P3SEL7"]
pub type P3SEL7_R = crate::BitReader;
#[doc = "Field `P3SEL7` writer - P3SEL7"]
pub type P3SEL7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P3SEL0"]
    #[inline(always)]
    pub fn p3sel0(&self) -> P3SEL0_R {
        P3SEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P3SEL1"]
    #[inline(always)]
    pub fn p3sel1(&self) -> P3SEL1_R {
        P3SEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P3SEL2"]
    #[inline(always)]
    pub fn p3sel2(&self) -> P3SEL2_R {
        P3SEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P3SEL3"]
    #[inline(always)]
    pub fn p3sel3(&self) -> P3SEL3_R {
        P3SEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P3SEL4"]
    #[inline(always)]
    pub fn p3sel4(&self) -> P3SEL4_R {
        P3SEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P3SEL5"]
    #[inline(always)]
    pub fn p3sel5(&self) -> P3SEL5_R {
        P3SEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P3SEL6"]
    #[inline(always)]
    pub fn p3sel6(&self) -> P3SEL6_R {
        P3SEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P3SEL7"]
    #[inline(always)]
    pub fn p3sel7(&self) -> P3SEL7_R {
        P3SEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3SEL0"]
    #[inline(always)]
    #[must_use]
    pub fn p3sel0(&mut self) -> P3SEL0_W<P3SEL_SPEC> {
        P3SEL0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P3SEL1"]
    #[inline(always)]
    #[must_use]
    pub fn p3sel1(&mut self) -> P3SEL1_W<P3SEL_SPEC> {
        P3SEL1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P3SEL2"]
    #[inline(always)]
    #[must_use]
    pub fn p3sel2(&mut self) -> P3SEL2_W<P3SEL_SPEC> {
        P3SEL2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P3SEL3"]
    #[inline(always)]
    #[must_use]
    pub fn p3sel3(&mut self) -> P3SEL3_W<P3SEL_SPEC> {
        P3SEL3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P3SEL4"]
    #[inline(always)]
    #[must_use]
    pub fn p3sel4(&mut self) -> P3SEL4_W<P3SEL_SPEC> {
        P3SEL4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P3SEL5"]
    #[inline(always)]
    #[must_use]
    pub fn p3sel5(&mut self) -> P3SEL5_W<P3SEL_SPEC> {
        P3SEL5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P3SEL6"]
    #[inline(always)]
    #[must_use]
    pub fn p3sel6(&mut self) -> P3SEL6_W<P3SEL_SPEC> {
        P3SEL6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P3SEL7"]
    #[inline(always)]
    #[must_use]
    pub fn p3sel7(&mut self) -> P3SEL7_W<P3SEL_SPEC> {
        P3SEL7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 3 Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p3sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p3sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3SEL_SPEC;
impl crate::RegisterSpec for P3SEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3sel::R`](R) reader structure"]
impl crate::Readable for P3SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p3sel::W`](W) writer structure"]
impl crate::Writable for P3SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P3SEL to value 0"]
impl crate::Resettable for P3SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
