#[doc = "Register `P8REN` reader"]
pub type R = crate::R<P8REN_SPEC>;
#[doc = "Register `P8REN` writer"]
pub type W = crate::W<P8REN_SPEC>;
#[doc = "Field `P8REN0` reader - P8REN0"]
pub type P8REN0_R = crate::BitReader;
#[doc = "Field `P8REN0` writer - P8REN0"]
pub type P8REN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8REN1` reader - P8REN1"]
pub type P8REN1_R = crate::BitReader;
#[doc = "Field `P8REN1` writer - P8REN1"]
pub type P8REN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8REN2` reader - P8REN2"]
pub type P8REN2_R = crate::BitReader;
#[doc = "Field `P8REN2` writer - P8REN2"]
pub type P8REN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8REN3` reader - P8REN3"]
pub type P8REN3_R = crate::BitReader;
#[doc = "Field `P8REN3` writer - P8REN3"]
pub type P8REN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8REN4` reader - P8REN4"]
pub type P8REN4_R = crate::BitReader;
#[doc = "Field `P8REN4` writer - P8REN4"]
pub type P8REN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8REN5` reader - P8REN5"]
pub type P8REN5_R = crate::BitReader;
#[doc = "Field `P8REN5` writer - P8REN5"]
pub type P8REN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8REN6` reader - P8REN6"]
pub type P8REN6_R = crate::BitReader;
#[doc = "Field `P8REN6` writer - P8REN6"]
pub type P8REN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8REN7` reader - P8REN7"]
pub type P8REN7_R = crate::BitReader;
#[doc = "Field `P8REN7` writer - P8REN7"]
pub type P8REN7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P8REN0"]
    #[inline(always)]
    pub fn p8ren0(&self) -> P8REN0_R {
        P8REN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P8REN1"]
    #[inline(always)]
    pub fn p8ren1(&self) -> P8REN1_R {
        P8REN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P8REN2"]
    #[inline(always)]
    pub fn p8ren2(&self) -> P8REN2_R {
        P8REN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P8REN3"]
    #[inline(always)]
    pub fn p8ren3(&self) -> P8REN3_R {
        P8REN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P8REN4"]
    #[inline(always)]
    pub fn p8ren4(&self) -> P8REN4_R {
        P8REN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P8REN5"]
    #[inline(always)]
    pub fn p8ren5(&self) -> P8REN5_R {
        P8REN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P8REN6"]
    #[inline(always)]
    pub fn p8ren6(&self) -> P8REN6_R {
        P8REN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P8REN7"]
    #[inline(always)]
    pub fn p8ren7(&self) -> P8REN7_R {
        P8REN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P8REN0"]
    #[inline(always)]
    #[must_use]
    pub fn p8ren0(&mut self) -> P8REN0_W<P8REN_SPEC> {
        P8REN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P8REN1"]
    #[inline(always)]
    #[must_use]
    pub fn p8ren1(&mut self) -> P8REN1_W<P8REN_SPEC> {
        P8REN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P8REN2"]
    #[inline(always)]
    #[must_use]
    pub fn p8ren2(&mut self) -> P8REN2_W<P8REN_SPEC> {
        P8REN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P8REN3"]
    #[inline(always)]
    #[must_use]
    pub fn p8ren3(&mut self) -> P8REN3_W<P8REN_SPEC> {
        P8REN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P8REN4"]
    #[inline(always)]
    #[must_use]
    pub fn p8ren4(&mut self) -> P8REN4_W<P8REN_SPEC> {
        P8REN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P8REN5"]
    #[inline(always)]
    #[must_use]
    pub fn p8ren5(&mut self) -> P8REN5_W<P8REN_SPEC> {
        P8REN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P8REN6"]
    #[inline(always)]
    #[must_use]
    pub fn p8ren6(&mut self) -> P8REN6_W<P8REN_SPEC> {
        P8REN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P8REN7"]
    #[inline(always)]
    #[must_use]
    pub fn p8ren7(&mut self) -> P8REN7_W<P8REN_SPEC> {
        P8REN7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 8 Resistor Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p8ren::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p8ren::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P8REN_SPEC;
impl crate::RegisterSpec for P8REN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p8ren::R`](R) reader structure"]
impl crate::Readable for P8REN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p8ren::W`](W) writer structure"]
impl crate::Writable for P8REN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P8REN to value 0"]
impl crate::Resettable for P8REN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
