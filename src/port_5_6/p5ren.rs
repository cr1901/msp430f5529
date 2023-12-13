#[doc = "Register `P5REN` reader"]
pub type R = crate::R<P5REN_SPEC>;
#[doc = "Register `P5REN` writer"]
pub type W = crate::W<P5REN_SPEC>;
#[doc = "Field `P5REN0` reader - P5REN0"]
pub type P5REN0_R = crate::BitReader;
#[doc = "Field `P5REN0` writer - P5REN0"]
pub type P5REN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5REN1` reader - P5REN1"]
pub type P5REN1_R = crate::BitReader;
#[doc = "Field `P5REN1` writer - P5REN1"]
pub type P5REN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5REN2` reader - P5REN2"]
pub type P5REN2_R = crate::BitReader;
#[doc = "Field `P5REN2` writer - P5REN2"]
pub type P5REN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5REN3` reader - P5REN3"]
pub type P5REN3_R = crate::BitReader;
#[doc = "Field `P5REN3` writer - P5REN3"]
pub type P5REN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5REN4` reader - P5REN4"]
pub type P5REN4_R = crate::BitReader;
#[doc = "Field `P5REN4` writer - P5REN4"]
pub type P5REN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5REN5` reader - P5REN5"]
pub type P5REN5_R = crate::BitReader;
#[doc = "Field `P5REN5` writer - P5REN5"]
pub type P5REN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5REN6` reader - P5REN6"]
pub type P5REN6_R = crate::BitReader;
#[doc = "Field `P5REN6` writer - P5REN6"]
pub type P5REN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5REN7` reader - P5REN7"]
pub type P5REN7_R = crate::BitReader;
#[doc = "Field `P5REN7` writer - P5REN7"]
pub type P5REN7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P5REN0"]
    #[inline(always)]
    pub fn p5ren0(&self) -> P5REN0_R {
        P5REN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P5REN1"]
    #[inline(always)]
    pub fn p5ren1(&self) -> P5REN1_R {
        P5REN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P5REN2"]
    #[inline(always)]
    pub fn p5ren2(&self) -> P5REN2_R {
        P5REN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P5REN3"]
    #[inline(always)]
    pub fn p5ren3(&self) -> P5REN3_R {
        P5REN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P5REN4"]
    #[inline(always)]
    pub fn p5ren4(&self) -> P5REN4_R {
        P5REN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P5REN5"]
    #[inline(always)]
    pub fn p5ren5(&self) -> P5REN5_R {
        P5REN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P5REN6"]
    #[inline(always)]
    pub fn p5ren6(&self) -> P5REN6_R {
        P5REN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P5REN7"]
    #[inline(always)]
    pub fn p5ren7(&self) -> P5REN7_R {
        P5REN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P5REN0"]
    #[inline(always)]
    #[must_use]
    pub fn p5ren0(&mut self) -> P5REN0_W<P5REN_SPEC> {
        P5REN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P5REN1"]
    #[inline(always)]
    #[must_use]
    pub fn p5ren1(&mut self) -> P5REN1_W<P5REN_SPEC> {
        P5REN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P5REN2"]
    #[inline(always)]
    #[must_use]
    pub fn p5ren2(&mut self) -> P5REN2_W<P5REN_SPEC> {
        P5REN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P5REN3"]
    #[inline(always)]
    #[must_use]
    pub fn p5ren3(&mut self) -> P5REN3_W<P5REN_SPEC> {
        P5REN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P5REN4"]
    #[inline(always)]
    #[must_use]
    pub fn p5ren4(&mut self) -> P5REN4_W<P5REN_SPEC> {
        P5REN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P5REN5"]
    #[inline(always)]
    #[must_use]
    pub fn p5ren5(&mut self) -> P5REN5_W<P5REN_SPEC> {
        P5REN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P5REN6"]
    #[inline(always)]
    #[must_use]
    pub fn p5ren6(&mut self) -> P5REN6_W<P5REN_SPEC> {
        P5REN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P5REN7"]
    #[inline(always)]
    #[must_use]
    pub fn p5ren7(&mut self) -> P5REN7_W<P5REN_SPEC> {
        P5REN7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 5 Resistor Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p5ren::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p5ren::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P5REN_SPEC;
impl crate::RegisterSpec for P5REN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p5ren::R`](R) reader structure"]
impl crate::Readable for P5REN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p5ren::W`](W) writer structure"]
impl crate::Writable for P5REN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P5REN to value 0"]
impl crate::Resettable for P5REN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
