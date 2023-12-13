#[doc = "Register `P6REN` reader"]
pub type R = crate::R<P6REN_SPEC>;
#[doc = "Register `P6REN` writer"]
pub type W = crate::W<P6REN_SPEC>;
#[doc = "Field `P6REN0` reader - P6REN0"]
pub type P6REN0_R = crate::BitReader;
#[doc = "Field `P6REN0` writer - P6REN0"]
pub type P6REN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6REN1` reader - P6REN1"]
pub type P6REN1_R = crate::BitReader;
#[doc = "Field `P6REN1` writer - P6REN1"]
pub type P6REN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6REN2` reader - P6REN2"]
pub type P6REN2_R = crate::BitReader;
#[doc = "Field `P6REN2` writer - P6REN2"]
pub type P6REN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6REN3` reader - P6REN3"]
pub type P6REN3_R = crate::BitReader;
#[doc = "Field `P6REN3` writer - P6REN3"]
pub type P6REN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6REN4` reader - P6REN4"]
pub type P6REN4_R = crate::BitReader;
#[doc = "Field `P6REN4` writer - P6REN4"]
pub type P6REN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6REN5` reader - P6REN5"]
pub type P6REN5_R = crate::BitReader;
#[doc = "Field `P6REN5` writer - P6REN5"]
pub type P6REN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6REN6` reader - P6REN6"]
pub type P6REN6_R = crate::BitReader;
#[doc = "Field `P6REN6` writer - P6REN6"]
pub type P6REN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6REN7` reader - P6REN7"]
pub type P6REN7_R = crate::BitReader;
#[doc = "Field `P6REN7` writer - P6REN7"]
pub type P6REN7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P6REN0"]
    #[inline(always)]
    pub fn p6ren0(&self) -> P6REN0_R {
        P6REN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P6REN1"]
    #[inline(always)]
    pub fn p6ren1(&self) -> P6REN1_R {
        P6REN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P6REN2"]
    #[inline(always)]
    pub fn p6ren2(&self) -> P6REN2_R {
        P6REN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P6REN3"]
    #[inline(always)]
    pub fn p6ren3(&self) -> P6REN3_R {
        P6REN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P6REN4"]
    #[inline(always)]
    pub fn p6ren4(&self) -> P6REN4_R {
        P6REN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P6REN5"]
    #[inline(always)]
    pub fn p6ren5(&self) -> P6REN5_R {
        P6REN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P6REN6"]
    #[inline(always)]
    pub fn p6ren6(&self) -> P6REN6_R {
        P6REN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P6REN7"]
    #[inline(always)]
    pub fn p6ren7(&self) -> P6REN7_R {
        P6REN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P6REN0"]
    #[inline(always)]
    #[must_use]
    pub fn p6ren0(&mut self) -> P6REN0_W<P6REN_SPEC> {
        P6REN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P6REN1"]
    #[inline(always)]
    #[must_use]
    pub fn p6ren1(&mut self) -> P6REN1_W<P6REN_SPEC> {
        P6REN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P6REN2"]
    #[inline(always)]
    #[must_use]
    pub fn p6ren2(&mut self) -> P6REN2_W<P6REN_SPEC> {
        P6REN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P6REN3"]
    #[inline(always)]
    #[must_use]
    pub fn p6ren3(&mut self) -> P6REN3_W<P6REN_SPEC> {
        P6REN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P6REN4"]
    #[inline(always)]
    #[must_use]
    pub fn p6ren4(&mut self) -> P6REN4_W<P6REN_SPEC> {
        P6REN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P6REN5"]
    #[inline(always)]
    #[must_use]
    pub fn p6ren5(&mut self) -> P6REN5_W<P6REN_SPEC> {
        P6REN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P6REN6"]
    #[inline(always)]
    #[must_use]
    pub fn p6ren6(&mut self) -> P6REN6_W<P6REN_SPEC> {
        P6REN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P6REN7"]
    #[inline(always)]
    #[must_use]
    pub fn p6ren7(&mut self) -> P6REN7_W<P6REN_SPEC> {
        P6REN7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 6 Resistor Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p6ren::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p6ren::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P6REN_SPEC;
impl crate::RegisterSpec for P6REN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p6ren::R`](R) reader structure"]
impl crate::Readable for P6REN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p6ren::W`](W) writer structure"]
impl crate::Writable for P6REN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P6REN to value 0"]
impl crate::Resettable for P6REN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
