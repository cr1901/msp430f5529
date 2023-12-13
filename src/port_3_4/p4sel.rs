#[doc = "Register `P4SEL` reader"]
pub type R = crate::R<P4SEL_SPEC>;
#[doc = "Register `P4SEL` writer"]
pub type W = crate::W<P4SEL_SPEC>;
#[doc = "Field `P4SEL0` reader - P4SEL0"]
pub type P4SEL0_R = crate::BitReader;
#[doc = "Field `P4SEL0` writer - P4SEL0"]
pub type P4SEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4SEL1` reader - P4SEL1"]
pub type P4SEL1_R = crate::BitReader;
#[doc = "Field `P4SEL1` writer - P4SEL1"]
pub type P4SEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4SEL2` reader - P4SEL2"]
pub type P4SEL2_R = crate::BitReader;
#[doc = "Field `P4SEL2` writer - P4SEL2"]
pub type P4SEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4SEL3` reader - P4SEL3"]
pub type P4SEL3_R = crate::BitReader;
#[doc = "Field `P4SEL3` writer - P4SEL3"]
pub type P4SEL3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4SEL4` reader - P4SEL4"]
pub type P4SEL4_R = crate::BitReader;
#[doc = "Field `P4SEL4` writer - P4SEL4"]
pub type P4SEL4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4SEL5` reader - P4SEL5"]
pub type P4SEL5_R = crate::BitReader;
#[doc = "Field `P4SEL5` writer - P4SEL5"]
pub type P4SEL5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4SEL6` reader - P4SEL6"]
pub type P4SEL6_R = crate::BitReader;
#[doc = "Field `P4SEL6` writer - P4SEL6"]
pub type P4SEL6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4SEL7` reader - P4SEL7"]
pub type P4SEL7_R = crate::BitReader;
#[doc = "Field `P4SEL7` writer - P4SEL7"]
pub type P4SEL7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P4SEL0"]
    #[inline(always)]
    pub fn p4sel0(&self) -> P4SEL0_R {
        P4SEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P4SEL1"]
    #[inline(always)]
    pub fn p4sel1(&self) -> P4SEL1_R {
        P4SEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P4SEL2"]
    #[inline(always)]
    pub fn p4sel2(&self) -> P4SEL2_R {
        P4SEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P4SEL3"]
    #[inline(always)]
    pub fn p4sel3(&self) -> P4SEL3_R {
        P4SEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P4SEL4"]
    #[inline(always)]
    pub fn p4sel4(&self) -> P4SEL4_R {
        P4SEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P4SEL5"]
    #[inline(always)]
    pub fn p4sel5(&self) -> P4SEL5_R {
        P4SEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P4SEL6"]
    #[inline(always)]
    pub fn p4sel6(&self) -> P4SEL6_R {
        P4SEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P4SEL7"]
    #[inline(always)]
    pub fn p4sel7(&self) -> P4SEL7_R {
        P4SEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4SEL0"]
    #[inline(always)]
    #[must_use]
    pub fn p4sel0(&mut self) -> P4SEL0_W<P4SEL_SPEC> {
        P4SEL0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P4SEL1"]
    #[inline(always)]
    #[must_use]
    pub fn p4sel1(&mut self) -> P4SEL1_W<P4SEL_SPEC> {
        P4SEL1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P4SEL2"]
    #[inline(always)]
    #[must_use]
    pub fn p4sel2(&mut self) -> P4SEL2_W<P4SEL_SPEC> {
        P4SEL2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P4SEL3"]
    #[inline(always)]
    #[must_use]
    pub fn p4sel3(&mut self) -> P4SEL3_W<P4SEL_SPEC> {
        P4SEL3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P4SEL4"]
    #[inline(always)]
    #[must_use]
    pub fn p4sel4(&mut self) -> P4SEL4_W<P4SEL_SPEC> {
        P4SEL4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P4SEL5"]
    #[inline(always)]
    #[must_use]
    pub fn p4sel5(&mut self) -> P4SEL5_W<P4SEL_SPEC> {
        P4SEL5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P4SEL6"]
    #[inline(always)]
    #[must_use]
    pub fn p4sel6(&mut self) -> P4SEL6_W<P4SEL_SPEC> {
        P4SEL6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P4SEL7"]
    #[inline(always)]
    #[must_use]
    pub fn p4sel7(&mut self) -> P4SEL7_W<P4SEL_SPEC> {
        P4SEL7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 4 Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p4sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p4sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4SEL_SPEC;
impl crate::RegisterSpec for P4SEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4sel::R`](R) reader structure"]
impl crate::Readable for P4SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p4sel::W`](W) writer structure"]
impl crate::Writable for P4SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P4SEL to value 0"]
impl crate::Resettable for P4SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
