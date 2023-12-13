#[doc = "Register `P5OUT` reader"]
pub type R = crate::R<P5OUT_SPEC>;
#[doc = "Register `P5OUT` writer"]
pub type W = crate::W<P5OUT_SPEC>;
#[doc = "Field `P5OUT0` reader - P5OUT0"]
pub type P5OUT0_R = crate::BitReader;
#[doc = "Field `P5OUT0` writer - P5OUT0"]
pub type P5OUT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5OUT1` reader - P5OUT1"]
pub type P5OUT1_R = crate::BitReader;
#[doc = "Field `P5OUT1` writer - P5OUT1"]
pub type P5OUT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5OUT2` reader - P5OUT2"]
pub type P5OUT2_R = crate::BitReader;
#[doc = "Field `P5OUT2` writer - P5OUT2"]
pub type P5OUT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5OUT3` reader - P5OUT3"]
pub type P5OUT3_R = crate::BitReader;
#[doc = "Field `P5OUT3` writer - P5OUT3"]
pub type P5OUT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5OUT4` reader - P5OUT4"]
pub type P5OUT4_R = crate::BitReader;
#[doc = "Field `P5OUT4` writer - P5OUT4"]
pub type P5OUT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5OUT5` reader - P5OUT5"]
pub type P5OUT5_R = crate::BitReader;
#[doc = "Field `P5OUT5` writer - P5OUT5"]
pub type P5OUT5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5OUT6` reader - P5OUT6"]
pub type P5OUT6_R = crate::BitReader;
#[doc = "Field `P5OUT6` writer - P5OUT6"]
pub type P5OUT6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5OUT7` reader - P5OUT7"]
pub type P5OUT7_R = crate::BitReader;
#[doc = "Field `P5OUT7` writer - P5OUT7"]
pub type P5OUT7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P5OUT0"]
    #[inline(always)]
    pub fn p5out0(&self) -> P5OUT0_R {
        P5OUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P5OUT1"]
    #[inline(always)]
    pub fn p5out1(&self) -> P5OUT1_R {
        P5OUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P5OUT2"]
    #[inline(always)]
    pub fn p5out2(&self) -> P5OUT2_R {
        P5OUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P5OUT3"]
    #[inline(always)]
    pub fn p5out3(&self) -> P5OUT3_R {
        P5OUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P5OUT4"]
    #[inline(always)]
    pub fn p5out4(&self) -> P5OUT4_R {
        P5OUT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P5OUT5"]
    #[inline(always)]
    pub fn p5out5(&self) -> P5OUT5_R {
        P5OUT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P5OUT6"]
    #[inline(always)]
    pub fn p5out6(&self) -> P5OUT6_R {
        P5OUT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P5OUT7"]
    #[inline(always)]
    pub fn p5out7(&self) -> P5OUT7_R {
        P5OUT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P5OUT0"]
    #[inline(always)]
    #[must_use]
    pub fn p5out0(&mut self) -> P5OUT0_W<P5OUT_SPEC> {
        P5OUT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P5OUT1"]
    #[inline(always)]
    #[must_use]
    pub fn p5out1(&mut self) -> P5OUT1_W<P5OUT_SPEC> {
        P5OUT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P5OUT2"]
    #[inline(always)]
    #[must_use]
    pub fn p5out2(&mut self) -> P5OUT2_W<P5OUT_SPEC> {
        P5OUT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P5OUT3"]
    #[inline(always)]
    #[must_use]
    pub fn p5out3(&mut self) -> P5OUT3_W<P5OUT_SPEC> {
        P5OUT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P5OUT4"]
    #[inline(always)]
    #[must_use]
    pub fn p5out4(&mut self) -> P5OUT4_W<P5OUT_SPEC> {
        P5OUT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P5OUT5"]
    #[inline(always)]
    #[must_use]
    pub fn p5out5(&mut self) -> P5OUT5_W<P5OUT_SPEC> {
        P5OUT5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P5OUT6"]
    #[inline(always)]
    #[must_use]
    pub fn p5out6(&mut self) -> P5OUT6_W<P5OUT_SPEC> {
        P5OUT6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P5OUT7"]
    #[inline(always)]
    #[must_use]
    pub fn p5out7(&mut self) -> P5OUT7_W<P5OUT_SPEC> {
        P5OUT7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 5 Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p5out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p5out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P5OUT_SPEC;
impl crate::RegisterSpec for P5OUT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p5out::R`](R) reader structure"]
impl crate::Readable for P5OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p5out::W`](W) writer structure"]
impl crate::Writable for P5OUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P5OUT to value 0"]
impl crate::Resettable for P5OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
