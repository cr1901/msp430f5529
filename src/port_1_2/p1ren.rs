#[doc = "Register `P1REN` reader"]
pub type R = crate::R<P1REN_SPEC>;
#[doc = "Register `P1REN` writer"]
pub type W = crate::W<P1REN_SPEC>;
#[doc = "Field `P1REN0` reader - P1REN0"]
pub type P1REN0_R = crate::BitReader;
#[doc = "Field `P1REN0` writer - P1REN0"]
pub type P1REN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1REN1` reader - P1REN1"]
pub type P1REN1_R = crate::BitReader;
#[doc = "Field `P1REN1` writer - P1REN1"]
pub type P1REN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1REN2` reader - P1REN2"]
pub type P1REN2_R = crate::BitReader;
#[doc = "Field `P1REN2` writer - P1REN2"]
pub type P1REN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1REN3` reader - P1REN3"]
pub type P1REN3_R = crate::BitReader;
#[doc = "Field `P1REN3` writer - P1REN3"]
pub type P1REN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1REN4` reader - P1REN4"]
pub type P1REN4_R = crate::BitReader;
#[doc = "Field `P1REN4` writer - P1REN4"]
pub type P1REN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1REN5` reader - P1REN5"]
pub type P1REN5_R = crate::BitReader;
#[doc = "Field `P1REN5` writer - P1REN5"]
pub type P1REN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1REN6` reader - P1REN6"]
pub type P1REN6_R = crate::BitReader;
#[doc = "Field `P1REN6` writer - P1REN6"]
pub type P1REN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1REN7` reader - P1REN7"]
pub type P1REN7_R = crate::BitReader;
#[doc = "Field `P1REN7` writer - P1REN7"]
pub type P1REN7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P1REN0"]
    #[inline(always)]
    pub fn p1ren0(&self) -> P1REN0_R {
        P1REN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P1REN1"]
    #[inline(always)]
    pub fn p1ren1(&self) -> P1REN1_R {
        P1REN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P1REN2"]
    #[inline(always)]
    pub fn p1ren2(&self) -> P1REN2_R {
        P1REN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P1REN3"]
    #[inline(always)]
    pub fn p1ren3(&self) -> P1REN3_R {
        P1REN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P1REN4"]
    #[inline(always)]
    pub fn p1ren4(&self) -> P1REN4_R {
        P1REN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P1REN5"]
    #[inline(always)]
    pub fn p1ren5(&self) -> P1REN5_R {
        P1REN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P1REN6"]
    #[inline(always)]
    pub fn p1ren6(&self) -> P1REN6_R {
        P1REN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P1REN7"]
    #[inline(always)]
    pub fn p1ren7(&self) -> P1REN7_R {
        P1REN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1REN0"]
    #[inline(always)]
    #[must_use]
    pub fn p1ren0(&mut self) -> P1REN0_W<P1REN_SPEC> {
        P1REN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P1REN1"]
    #[inline(always)]
    #[must_use]
    pub fn p1ren1(&mut self) -> P1REN1_W<P1REN_SPEC> {
        P1REN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P1REN2"]
    #[inline(always)]
    #[must_use]
    pub fn p1ren2(&mut self) -> P1REN2_W<P1REN_SPEC> {
        P1REN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P1REN3"]
    #[inline(always)]
    #[must_use]
    pub fn p1ren3(&mut self) -> P1REN3_W<P1REN_SPEC> {
        P1REN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P1REN4"]
    #[inline(always)]
    #[must_use]
    pub fn p1ren4(&mut self) -> P1REN4_W<P1REN_SPEC> {
        P1REN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P1REN5"]
    #[inline(always)]
    #[must_use]
    pub fn p1ren5(&mut self) -> P1REN5_W<P1REN_SPEC> {
        P1REN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P1REN6"]
    #[inline(always)]
    #[must_use]
    pub fn p1ren6(&mut self) -> P1REN6_W<P1REN_SPEC> {
        P1REN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P1REN7"]
    #[inline(always)]
    #[must_use]
    pub fn p1ren7(&mut self) -> P1REN7_W<P1REN_SPEC> {
        P1REN7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 1 Resistor Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p1ren::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p1ren::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1REN_SPEC;
impl crate::RegisterSpec for P1REN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1ren::R`](R) reader structure"]
impl crate::Readable for P1REN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p1ren::W`](W) writer structure"]
impl crate::Writable for P1REN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P1REN to value 0"]
impl crate::Resettable for P1REN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
