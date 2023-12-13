#[doc = "Register `P4REN` reader"]
pub type R = crate::R<P4REN_SPEC>;
#[doc = "Register `P4REN` writer"]
pub type W = crate::W<P4REN_SPEC>;
#[doc = "Field `P4REN0` reader - P4REN0"]
pub type P4REN0_R = crate::BitReader;
#[doc = "Field `P4REN0` writer - P4REN0"]
pub type P4REN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4REN1` reader - P4REN1"]
pub type P4REN1_R = crate::BitReader;
#[doc = "Field `P4REN1` writer - P4REN1"]
pub type P4REN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4REN2` reader - P4REN2"]
pub type P4REN2_R = crate::BitReader;
#[doc = "Field `P4REN2` writer - P4REN2"]
pub type P4REN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4REN3` reader - P4REN3"]
pub type P4REN3_R = crate::BitReader;
#[doc = "Field `P4REN3` writer - P4REN3"]
pub type P4REN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4REN4` reader - P4REN4"]
pub type P4REN4_R = crate::BitReader;
#[doc = "Field `P4REN4` writer - P4REN4"]
pub type P4REN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4REN5` reader - P4REN5"]
pub type P4REN5_R = crate::BitReader;
#[doc = "Field `P4REN5` writer - P4REN5"]
pub type P4REN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4REN6` reader - P4REN6"]
pub type P4REN6_R = crate::BitReader;
#[doc = "Field `P4REN6` writer - P4REN6"]
pub type P4REN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4REN7` reader - P4REN7"]
pub type P4REN7_R = crate::BitReader;
#[doc = "Field `P4REN7` writer - P4REN7"]
pub type P4REN7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P4REN0"]
    #[inline(always)]
    pub fn p4ren0(&self) -> P4REN0_R {
        P4REN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P4REN1"]
    #[inline(always)]
    pub fn p4ren1(&self) -> P4REN1_R {
        P4REN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P4REN2"]
    #[inline(always)]
    pub fn p4ren2(&self) -> P4REN2_R {
        P4REN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P4REN3"]
    #[inline(always)]
    pub fn p4ren3(&self) -> P4REN3_R {
        P4REN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P4REN4"]
    #[inline(always)]
    pub fn p4ren4(&self) -> P4REN4_R {
        P4REN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P4REN5"]
    #[inline(always)]
    pub fn p4ren5(&self) -> P4REN5_R {
        P4REN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P4REN6"]
    #[inline(always)]
    pub fn p4ren6(&self) -> P4REN6_R {
        P4REN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P4REN7"]
    #[inline(always)]
    pub fn p4ren7(&self) -> P4REN7_R {
        P4REN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4REN0"]
    #[inline(always)]
    #[must_use]
    pub fn p4ren0(&mut self) -> P4REN0_W<P4REN_SPEC> {
        P4REN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P4REN1"]
    #[inline(always)]
    #[must_use]
    pub fn p4ren1(&mut self) -> P4REN1_W<P4REN_SPEC> {
        P4REN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P4REN2"]
    #[inline(always)]
    #[must_use]
    pub fn p4ren2(&mut self) -> P4REN2_W<P4REN_SPEC> {
        P4REN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P4REN3"]
    #[inline(always)]
    #[must_use]
    pub fn p4ren3(&mut self) -> P4REN3_W<P4REN_SPEC> {
        P4REN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P4REN4"]
    #[inline(always)]
    #[must_use]
    pub fn p4ren4(&mut self) -> P4REN4_W<P4REN_SPEC> {
        P4REN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P4REN5"]
    #[inline(always)]
    #[must_use]
    pub fn p4ren5(&mut self) -> P4REN5_W<P4REN_SPEC> {
        P4REN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P4REN6"]
    #[inline(always)]
    #[must_use]
    pub fn p4ren6(&mut self) -> P4REN6_W<P4REN_SPEC> {
        P4REN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P4REN7"]
    #[inline(always)]
    #[must_use]
    pub fn p4ren7(&mut self) -> P4REN7_W<P4REN_SPEC> {
        P4REN7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 4 Resistor Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p4ren::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p4ren::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4REN_SPEC;
impl crate::RegisterSpec for P4REN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4ren::R`](R) reader structure"]
impl crate::Readable for P4REN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p4ren::W`](W) writer structure"]
impl crate::Writable for P4REN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P4REN to value 0"]
impl crate::Resettable for P4REN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
