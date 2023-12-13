#[doc = "Register `P1SEL` reader"]
pub type R = crate::R<P1SEL_SPEC>;
#[doc = "Register `P1SEL` writer"]
pub type W = crate::W<P1SEL_SPEC>;
#[doc = "Field `P1SEL0` reader - P1SEL0"]
pub type P1SEL0_R = crate::BitReader;
#[doc = "Field `P1SEL0` writer - P1SEL0"]
pub type P1SEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1SEL1` reader - P1SEL1"]
pub type P1SEL1_R = crate::BitReader;
#[doc = "Field `P1SEL1` writer - P1SEL1"]
pub type P1SEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1SEL2` reader - P1SEL2"]
pub type P1SEL2_R = crate::BitReader;
#[doc = "Field `P1SEL2` writer - P1SEL2"]
pub type P1SEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1SEL3` reader - P1SEL3"]
pub type P1SEL3_R = crate::BitReader;
#[doc = "Field `P1SEL3` writer - P1SEL3"]
pub type P1SEL3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1SEL4` reader - P1SEL4"]
pub type P1SEL4_R = crate::BitReader;
#[doc = "Field `P1SEL4` writer - P1SEL4"]
pub type P1SEL4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1SEL5` reader - P1SEL5"]
pub type P1SEL5_R = crate::BitReader;
#[doc = "Field `P1SEL5` writer - P1SEL5"]
pub type P1SEL5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1SEL6` reader - P1SEL6"]
pub type P1SEL6_R = crate::BitReader;
#[doc = "Field `P1SEL6` writer - P1SEL6"]
pub type P1SEL6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1SEL7` reader - P1SEL7"]
pub type P1SEL7_R = crate::BitReader;
#[doc = "Field `P1SEL7` writer - P1SEL7"]
pub type P1SEL7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P1SEL0"]
    #[inline(always)]
    pub fn p1sel0(&self) -> P1SEL0_R {
        P1SEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P1SEL1"]
    #[inline(always)]
    pub fn p1sel1(&self) -> P1SEL1_R {
        P1SEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P1SEL2"]
    #[inline(always)]
    pub fn p1sel2(&self) -> P1SEL2_R {
        P1SEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P1SEL3"]
    #[inline(always)]
    pub fn p1sel3(&self) -> P1SEL3_R {
        P1SEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P1SEL4"]
    #[inline(always)]
    pub fn p1sel4(&self) -> P1SEL4_R {
        P1SEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P1SEL5"]
    #[inline(always)]
    pub fn p1sel5(&self) -> P1SEL5_R {
        P1SEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P1SEL6"]
    #[inline(always)]
    pub fn p1sel6(&self) -> P1SEL6_R {
        P1SEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P1SEL7"]
    #[inline(always)]
    pub fn p1sel7(&self) -> P1SEL7_R {
        P1SEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1SEL0"]
    #[inline(always)]
    #[must_use]
    pub fn p1sel0(&mut self) -> P1SEL0_W<P1SEL_SPEC> {
        P1SEL0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P1SEL1"]
    #[inline(always)]
    #[must_use]
    pub fn p1sel1(&mut self) -> P1SEL1_W<P1SEL_SPEC> {
        P1SEL1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P1SEL2"]
    #[inline(always)]
    #[must_use]
    pub fn p1sel2(&mut self) -> P1SEL2_W<P1SEL_SPEC> {
        P1SEL2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P1SEL3"]
    #[inline(always)]
    #[must_use]
    pub fn p1sel3(&mut self) -> P1SEL3_W<P1SEL_SPEC> {
        P1SEL3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P1SEL4"]
    #[inline(always)]
    #[must_use]
    pub fn p1sel4(&mut self) -> P1SEL4_W<P1SEL_SPEC> {
        P1SEL4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P1SEL5"]
    #[inline(always)]
    #[must_use]
    pub fn p1sel5(&mut self) -> P1SEL5_W<P1SEL_SPEC> {
        P1SEL5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P1SEL6"]
    #[inline(always)]
    #[must_use]
    pub fn p1sel6(&mut self) -> P1SEL6_W<P1SEL_SPEC> {
        P1SEL6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P1SEL7"]
    #[inline(always)]
    #[must_use]
    pub fn p1sel7(&mut self) -> P1SEL7_W<P1SEL_SPEC> {
        P1SEL7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 1 Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p1sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p1sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1SEL_SPEC;
impl crate::RegisterSpec for P1SEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1sel::R`](R) reader structure"]
impl crate::Readable for P1SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p1sel::W`](W) writer structure"]
impl crate::Writable for P1SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P1SEL to value 0"]
impl crate::Resettable for P1SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
