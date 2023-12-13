#[doc = "Register `P6SEL` reader"]
pub type R = crate::R<P6SEL_SPEC>;
#[doc = "Register `P6SEL` writer"]
pub type W = crate::W<P6SEL_SPEC>;
#[doc = "Field `P6SEL0` reader - P6SEL0"]
pub type P6SEL0_R = crate::BitReader;
#[doc = "Field `P6SEL0` writer - P6SEL0"]
pub type P6SEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6SEL1` reader - P6SEL1"]
pub type P6SEL1_R = crate::BitReader;
#[doc = "Field `P6SEL1` writer - P6SEL1"]
pub type P6SEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6SEL2` reader - P6SEL2"]
pub type P6SEL2_R = crate::BitReader;
#[doc = "Field `P6SEL2` writer - P6SEL2"]
pub type P6SEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6SEL3` reader - P6SEL3"]
pub type P6SEL3_R = crate::BitReader;
#[doc = "Field `P6SEL3` writer - P6SEL3"]
pub type P6SEL3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6SEL4` reader - P6SEL4"]
pub type P6SEL4_R = crate::BitReader;
#[doc = "Field `P6SEL4` writer - P6SEL4"]
pub type P6SEL4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6SEL5` reader - P6SEL5"]
pub type P6SEL5_R = crate::BitReader;
#[doc = "Field `P6SEL5` writer - P6SEL5"]
pub type P6SEL5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6SEL6` reader - P6SEL6"]
pub type P6SEL6_R = crate::BitReader;
#[doc = "Field `P6SEL6` writer - P6SEL6"]
pub type P6SEL6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6SEL7` reader - P6SEL7"]
pub type P6SEL7_R = crate::BitReader;
#[doc = "Field `P6SEL7` writer - P6SEL7"]
pub type P6SEL7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P6SEL0"]
    #[inline(always)]
    pub fn p6sel0(&self) -> P6SEL0_R {
        P6SEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P6SEL1"]
    #[inline(always)]
    pub fn p6sel1(&self) -> P6SEL1_R {
        P6SEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P6SEL2"]
    #[inline(always)]
    pub fn p6sel2(&self) -> P6SEL2_R {
        P6SEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P6SEL3"]
    #[inline(always)]
    pub fn p6sel3(&self) -> P6SEL3_R {
        P6SEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P6SEL4"]
    #[inline(always)]
    pub fn p6sel4(&self) -> P6SEL4_R {
        P6SEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P6SEL5"]
    #[inline(always)]
    pub fn p6sel5(&self) -> P6SEL5_R {
        P6SEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P6SEL6"]
    #[inline(always)]
    pub fn p6sel6(&self) -> P6SEL6_R {
        P6SEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P6SEL7"]
    #[inline(always)]
    pub fn p6sel7(&self) -> P6SEL7_R {
        P6SEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P6SEL0"]
    #[inline(always)]
    #[must_use]
    pub fn p6sel0(&mut self) -> P6SEL0_W<P6SEL_SPEC> {
        P6SEL0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P6SEL1"]
    #[inline(always)]
    #[must_use]
    pub fn p6sel1(&mut self) -> P6SEL1_W<P6SEL_SPEC> {
        P6SEL1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P6SEL2"]
    #[inline(always)]
    #[must_use]
    pub fn p6sel2(&mut self) -> P6SEL2_W<P6SEL_SPEC> {
        P6SEL2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P6SEL3"]
    #[inline(always)]
    #[must_use]
    pub fn p6sel3(&mut self) -> P6SEL3_W<P6SEL_SPEC> {
        P6SEL3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P6SEL4"]
    #[inline(always)]
    #[must_use]
    pub fn p6sel4(&mut self) -> P6SEL4_W<P6SEL_SPEC> {
        P6SEL4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P6SEL5"]
    #[inline(always)]
    #[must_use]
    pub fn p6sel5(&mut self) -> P6SEL5_W<P6SEL_SPEC> {
        P6SEL5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P6SEL6"]
    #[inline(always)]
    #[must_use]
    pub fn p6sel6(&mut self) -> P6SEL6_W<P6SEL_SPEC> {
        P6SEL6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P6SEL7"]
    #[inline(always)]
    #[must_use]
    pub fn p6sel7(&mut self) -> P6SEL7_W<P6SEL_SPEC> {
        P6SEL7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 6 Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p6sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p6sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P6SEL_SPEC;
impl crate::RegisterSpec for P6SEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p6sel::R`](R) reader structure"]
impl crate::Readable for P6SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p6sel::W`](W) writer structure"]
impl crate::Writable for P6SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P6SEL to value 0"]
impl crate::Resettable for P6SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
