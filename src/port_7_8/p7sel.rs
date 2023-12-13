#[doc = "Register `P7SEL` reader"]
pub type R = crate::R<P7SEL_SPEC>;
#[doc = "Register `P7SEL` writer"]
pub type W = crate::W<P7SEL_SPEC>;
#[doc = "Field `P7SEL0` reader - P7SEL0"]
pub type P7SEL0_R = crate::BitReader;
#[doc = "Field `P7SEL0` writer - P7SEL0"]
pub type P7SEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7SEL1` reader - P7SEL1"]
pub type P7SEL1_R = crate::BitReader;
#[doc = "Field `P7SEL1` writer - P7SEL1"]
pub type P7SEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7SEL2` reader - P7SEL2"]
pub type P7SEL2_R = crate::BitReader;
#[doc = "Field `P7SEL2` writer - P7SEL2"]
pub type P7SEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7SEL3` reader - P7SEL3"]
pub type P7SEL3_R = crate::BitReader;
#[doc = "Field `P7SEL3` writer - P7SEL3"]
pub type P7SEL3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7SEL4` reader - P7SEL4"]
pub type P7SEL4_R = crate::BitReader;
#[doc = "Field `P7SEL4` writer - P7SEL4"]
pub type P7SEL4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7SEL5` reader - P7SEL5"]
pub type P7SEL5_R = crate::BitReader;
#[doc = "Field `P7SEL5` writer - P7SEL5"]
pub type P7SEL5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7SEL6` reader - P7SEL6"]
pub type P7SEL6_R = crate::BitReader;
#[doc = "Field `P7SEL6` writer - P7SEL6"]
pub type P7SEL6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7SEL7` reader - P7SEL7"]
pub type P7SEL7_R = crate::BitReader;
#[doc = "Field `P7SEL7` writer - P7SEL7"]
pub type P7SEL7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P7SEL0"]
    #[inline(always)]
    pub fn p7sel0(&self) -> P7SEL0_R {
        P7SEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P7SEL1"]
    #[inline(always)]
    pub fn p7sel1(&self) -> P7SEL1_R {
        P7SEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P7SEL2"]
    #[inline(always)]
    pub fn p7sel2(&self) -> P7SEL2_R {
        P7SEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P7SEL3"]
    #[inline(always)]
    pub fn p7sel3(&self) -> P7SEL3_R {
        P7SEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P7SEL4"]
    #[inline(always)]
    pub fn p7sel4(&self) -> P7SEL4_R {
        P7SEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P7SEL5"]
    #[inline(always)]
    pub fn p7sel5(&self) -> P7SEL5_R {
        P7SEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P7SEL6"]
    #[inline(always)]
    pub fn p7sel6(&self) -> P7SEL6_R {
        P7SEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P7SEL7"]
    #[inline(always)]
    pub fn p7sel7(&self) -> P7SEL7_R {
        P7SEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P7SEL0"]
    #[inline(always)]
    #[must_use]
    pub fn p7sel0(&mut self) -> P7SEL0_W<P7SEL_SPEC> {
        P7SEL0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P7SEL1"]
    #[inline(always)]
    #[must_use]
    pub fn p7sel1(&mut self) -> P7SEL1_W<P7SEL_SPEC> {
        P7SEL1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P7SEL2"]
    #[inline(always)]
    #[must_use]
    pub fn p7sel2(&mut self) -> P7SEL2_W<P7SEL_SPEC> {
        P7SEL2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P7SEL3"]
    #[inline(always)]
    #[must_use]
    pub fn p7sel3(&mut self) -> P7SEL3_W<P7SEL_SPEC> {
        P7SEL3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P7SEL4"]
    #[inline(always)]
    #[must_use]
    pub fn p7sel4(&mut self) -> P7SEL4_W<P7SEL_SPEC> {
        P7SEL4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P7SEL5"]
    #[inline(always)]
    #[must_use]
    pub fn p7sel5(&mut self) -> P7SEL5_W<P7SEL_SPEC> {
        P7SEL5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P7SEL6"]
    #[inline(always)]
    #[must_use]
    pub fn p7sel6(&mut self) -> P7SEL6_W<P7SEL_SPEC> {
        P7SEL6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P7SEL7"]
    #[inline(always)]
    #[must_use]
    pub fn p7sel7(&mut self) -> P7SEL7_W<P7SEL_SPEC> {
        P7SEL7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 7 Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p7sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p7sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P7SEL_SPEC;
impl crate::RegisterSpec for P7SEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p7sel::R`](R) reader structure"]
impl crate::Readable for P7SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p7sel::W`](W) writer structure"]
impl crate::Writable for P7SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P7SEL to value 0"]
impl crate::Resettable for P7SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
