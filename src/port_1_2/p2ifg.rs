#[doc = "Register `P2IFG` reader"]
pub type R = crate::R<P2IFG_SPEC>;
#[doc = "Register `P2IFG` writer"]
pub type W = crate::W<P2IFG_SPEC>;
#[doc = "Field `P2IFG0` reader - P2IFG0"]
pub type P2IFG0_R = crate::BitReader;
#[doc = "Field `P2IFG0` writer - P2IFG0"]
pub type P2IFG0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IFG1` reader - P2IFG1"]
pub type P2IFG1_R = crate::BitReader;
#[doc = "Field `P2IFG1` writer - P2IFG1"]
pub type P2IFG1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IFG2` reader - P2IFG2"]
pub type P2IFG2_R = crate::BitReader;
#[doc = "Field `P2IFG2` writer - P2IFG2"]
pub type P2IFG2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IFG3` reader - P2IFG3"]
pub type P2IFG3_R = crate::BitReader;
#[doc = "Field `P2IFG3` writer - P2IFG3"]
pub type P2IFG3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IFG4` reader - P2IFG4"]
pub type P2IFG4_R = crate::BitReader;
#[doc = "Field `P2IFG4` writer - P2IFG4"]
pub type P2IFG4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IFG5` reader - P2IFG5"]
pub type P2IFG5_R = crate::BitReader;
#[doc = "Field `P2IFG5` writer - P2IFG5"]
pub type P2IFG5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IFG6` reader - P2IFG6"]
pub type P2IFG6_R = crate::BitReader;
#[doc = "Field `P2IFG6` writer - P2IFG6"]
pub type P2IFG6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IFG7` reader - P2IFG7"]
pub type P2IFG7_R = crate::BitReader;
#[doc = "Field `P2IFG7` writer - P2IFG7"]
pub type P2IFG7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P2IFG0"]
    #[inline(always)]
    pub fn p2ifg0(&self) -> P2IFG0_R {
        P2IFG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P2IFG1"]
    #[inline(always)]
    pub fn p2ifg1(&self) -> P2IFG1_R {
        P2IFG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2IFG2"]
    #[inline(always)]
    pub fn p2ifg2(&self) -> P2IFG2_R {
        P2IFG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P2IFG3"]
    #[inline(always)]
    pub fn p2ifg3(&self) -> P2IFG3_R {
        P2IFG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P2IFG4"]
    #[inline(always)]
    pub fn p2ifg4(&self) -> P2IFG4_R {
        P2IFG4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P2IFG5"]
    #[inline(always)]
    pub fn p2ifg5(&self) -> P2IFG5_R {
        P2IFG5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P2IFG6"]
    #[inline(always)]
    pub fn p2ifg6(&self) -> P2IFG6_R {
        P2IFG6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P2IFG7"]
    #[inline(always)]
    pub fn p2ifg7(&self) -> P2IFG7_R {
        P2IFG7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2IFG0"]
    #[inline(always)]
    #[must_use]
    pub fn p2ifg0(&mut self) -> P2IFG0_W<P2IFG_SPEC> {
        P2IFG0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P2IFG1"]
    #[inline(always)]
    #[must_use]
    pub fn p2ifg1(&mut self) -> P2IFG1_W<P2IFG_SPEC> {
        P2IFG1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P2IFG2"]
    #[inline(always)]
    #[must_use]
    pub fn p2ifg2(&mut self) -> P2IFG2_W<P2IFG_SPEC> {
        P2IFG2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P2IFG3"]
    #[inline(always)]
    #[must_use]
    pub fn p2ifg3(&mut self) -> P2IFG3_W<P2IFG_SPEC> {
        P2IFG3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P2IFG4"]
    #[inline(always)]
    #[must_use]
    pub fn p2ifg4(&mut self) -> P2IFG4_W<P2IFG_SPEC> {
        P2IFG4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P2IFG5"]
    #[inline(always)]
    #[must_use]
    pub fn p2ifg5(&mut self) -> P2IFG5_W<P2IFG_SPEC> {
        P2IFG5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P2IFG6"]
    #[inline(always)]
    #[must_use]
    pub fn p2ifg6(&mut self) -> P2IFG6_W<P2IFG_SPEC> {
        P2IFG6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P2IFG7"]
    #[inline(always)]
    #[must_use]
    pub fn p2ifg7(&mut self) -> P2IFG7_W<P2IFG_SPEC> {
        P2IFG7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 2 Interrupt Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p2ifg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p2ifg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2IFG_SPEC;
impl crate::RegisterSpec for P2IFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2ifg::R`](R) reader structure"]
impl crate::Readable for P2IFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p2ifg::W`](W) writer structure"]
impl crate::Writable for P2IFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P2IFG to value 0"]
impl crate::Resettable for P2IFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
