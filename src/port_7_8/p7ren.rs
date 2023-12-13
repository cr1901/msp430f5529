#[doc = "Register `P7REN` reader"]
pub type R = crate::R<P7REN_SPEC>;
#[doc = "Register `P7REN` writer"]
pub type W = crate::W<P7REN_SPEC>;
#[doc = "Field `P7REN0` reader - P7REN0"]
pub type P7REN0_R = crate::BitReader;
#[doc = "Field `P7REN0` writer - P7REN0"]
pub type P7REN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7REN1` reader - P7REN1"]
pub type P7REN1_R = crate::BitReader;
#[doc = "Field `P7REN1` writer - P7REN1"]
pub type P7REN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7REN2` reader - P7REN2"]
pub type P7REN2_R = crate::BitReader;
#[doc = "Field `P7REN2` writer - P7REN2"]
pub type P7REN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7REN3` reader - P7REN3"]
pub type P7REN3_R = crate::BitReader;
#[doc = "Field `P7REN3` writer - P7REN3"]
pub type P7REN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7REN4` reader - P7REN4"]
pub type P7REN4_R = crate::BitReader;
#[doc = "Field `P7REN4` writer - P7REN4"]
pub type P7REN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7REN5` reader - P7REN5"]
pub type P7REN5_R = crate::BitReader;
#[doc = "Field `P7REN5` writer - P7REN5"]
pub type P7REN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7REN6` reader - P7REN6"]
pub type P7REN6_R = crate::BitReader;
#[doc = "Field `P7REN6` writer - P7REN6"]
pub type P7REN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7REN7` reader - P7REN7"]
pub type P7REN7_R = crate::BitReader;
#[doc = "Field `P7REN7` writer - P7REN7"]
pub type P7REN7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P7REN0"]
    #[inline(always)]
    pub fn p7ren0(&self) -> P7REN0_R {
        P7REN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P7REN1"]
    #[inline(always)]
    pub fn p7ren1(&self) -> P7REN1_R {
        P7REN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P7REN2"]
    #[inline(always)]
    pub fn p7ren2(&self) -> P7REN2_R {
        P7REN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P7REN3"]
    #[inline(always)]
    pub fn p7ren3(&self) -> P7REN3_R {
        P7REN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P7REN4"]
    #[inline(always)]
    pub fn p7ren4(&self) -> P7REN4_R {
        P7REN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P7REN5"]
    #[inline(always)]
    pub fn p7ren5(&self) -> P7REN5_R {
        P7REN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P7REN6"]
    #[inline(always)]
    pub fn p7ren6(&self) -> P7REN6_R {
        P7REN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P7REN7"]
    #[inline(always)]
    pub fn p7ren7(&self) -> P7REN7_R {
        P7REN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P7REN0"]
    #[inline(always)]
    #[must_use]
    pub fn p7ren0(&mut self) -> P7REN0_W<P7REN_SPEC> {
        P7REN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P7REN1"]
    #[inline(always)]
    #[must_use]
    pub fn p7ren1(&mut self) -> P7REN1_W<P7REN_SPEC> {
        P7REN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P7REN2"]
    #[inline(always)]
    #[must_use]
    pub fn p7ren2(&mut self) -> P7REN2_W<P7REN_SPEC> {
        P7REN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P7REN3"]
    #[inline(always)]
    #[must_use]
    pub fn p7ren3(&mut self) -> P7REN3_W<P7REN_SPEC> {
        P7REN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P7REN4"]
    #[inline(always)]
    #[must_use]
    pub fn p7ren4(&mut self) -> P7REN4_W<P7REN_SPEC> {
        P7REN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P7REN5"]
    #[inline(always)]
    #[must_use]
    pub fn p7ren5(&mut self) -> P7REN5_W<P7REN_SPEC> {
        P7REN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P7REN6"]
    #[inline(always)]
    #[must_use]
    pub fn p7ren6(&mut self) -> P7REN6_W<P7REN_SPEC> {
        P7REN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P7REN7"]
    #[inline(always)]
    #[must_use]
    pub fn p7ren7(&mut self) -> P7REN7_W<P7REN_SPEC> {
        P7REN7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 7 Resistor Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p7ren::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p7ren::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P7REN_SPEC;
impl crate::RegisterSpec for P7REN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p7ren::R`](R) reader structure"]
impl crate::Readable for P7REN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p7ren::W`](W) writer structure"]
impl crate::Writable for P7REN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P7REN to value 0"]
impl crate::Resettable for P7REN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
