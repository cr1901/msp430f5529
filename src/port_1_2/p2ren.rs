#[doc = "Register `P2REN` reader"]
pub type R = crate::R<P2REN_SPEC>;
#[doc = "Register `P2REN` writer"]
pub type W = crate::W<P2REN_SPEC>;
#[doc = "Field `P2REN0` reader - P2REN0"]
pub type P2REN0_R = crate::BitReader;
#[doc = "Field `P2REN0` writer - P2REN0"]
pub type P2REN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2REN1` reader - P2REN1"]
pub type P2REN1_R = crate::BitReader;
#[doc = "Field `P2REN1` writer - P2REN1"]
pub type P2REN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2REN2` reader - P2REN2"]
pub type P2REN2_R = crate::BitReader;
#[doc = "Field `P2REN2` writer - P2REN2"]
pub type P2REN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2REN3` reader - P2REN3"]
pub type P2REN3_R = crate::BitReader;
#[doc = "Field `P2REN3` writer - P2REN3"]
pub type P2REN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2REN4` reader - P2REN4"]
pub type P2REN4_R = crate::BitReader;
#[doc = "Field `P2REN4` writer - P2REN4"]
pub type P2REN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2REN5` reader - P2REN5"]
pub type P2REN5_R = crate::BitReader;
#[doc = "Field `P2REN5` writer - P2REN5"]
pub type P2REN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2REN6` reader - P2REN6"]
pub type P2REN6_R = crate::BitReader;
#[doc = "Field `P2REN6` writer - P2REN6"]
pub type P2REN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2REN7` reader - P2REN7"]
pub type P2REN7_R = crate::BitReader;
#[doc = "Field `P2REN7` writer - P2REN7"]
pub type P2REN7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P2REN0"]
    #[inline(always)]
    pub fn p2ren0(&self) -> P2REN0_R {
        P2REN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P2REN1"]
    #[inline(always)]
    pub fn p2ren1(&self) -> P2REN1_R {
        P2REN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2REN2"]
    #[inline(always)]
    pub fn p2ren2(&self) -> P2REN2_R {
        P2REN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P2REN3"]
    #[inline(always)]
    pub fn p2ren3(&self) -> P2REN3_R {
        P2REN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P2REN4"]
    #[inline(always)]
    pub fn p2ren4(&self) -> P2REN4_R {
        P2REN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P2REN5"]
    #[inline(always)]
    pub fn p2ren5(&self) -> P2REN5_R {
        P2REN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P2REN6"]
    #[inline(always)]
    pub fn p2ren6(&self) -> P2REN6_R {
        P2REN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P2REN7"]
    #[inline(always)]
    pub fn p2ren7(&self) -> P2REN7_R {
        P2REN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2REN0"]
    #[inline(always)]
    #[must_use]
    pub fn p2ren0(&mut self) -> P2REN0_W<P2REN_SPEC> {
        P2REN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P2REN1"]
    #[inline(always)]
    #[must_use]
    pub fn p2ren1(&mut self) -> P2REN1_W<P2REN_SPEC> {
        P2REN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P2REN2"]
    #[inline(always)]
    #[must_use]
    pub fn p2ren2(&mut self) -> P2REN2_W<P2REN_SPEC> {
        P2REN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P2REN3"]
    #[inline(always)]
    #[must_use]
    pub fn p2ren3(&mut self) -> P2REN3_W<P2REN_SPEC> {
        P2REN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P2REN4"]
    #[inline(always)]
    #[must_use]
    pub fn p2ren4(&mut self) -> P2REN4_W<P2REN_SPEC> {
        P2REN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P2REN5"]
    #[inline(always)]
    #[must_use]
    pub fn p2ren5(&mut self) -> P2REN5_W<P2REN_SPEC> {
        P2REN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P2REN6"]
    #[inline(always)]
    #[must_use]
    pub fn p2ren6(&mut self) -> P2REN6_W<P2REN_SPEC> {
        P2REN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P2REN7"]
    #[inline(always)]
    #[must_use]
    pub fn p2ren7(&mut self) -> P2REN7_W<P2REN_SPEC> {
        P2REN7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 2 Resistor Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p2ren::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p2ren::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2REN_SPEC;
impl crate::RegisterSpec for P2REN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2ren::R`](R) reader structure"]
impl crate::Readable for P2REN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p2ren::W`](W) writer structure"]
impl crate::Writable for P2REN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P2REN to value 0"]
impl crate::Resettable for P2REN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
