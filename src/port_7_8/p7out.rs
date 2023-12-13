#[doc = "Register `P7OUT` reader"]
pub type R = crate::R<P7OUT_SPEC>;
#[doc = "Register `P7OUT` writer"]
pub type W = crate::W<P7OUT_SPEC>;
#[doc = "Field `P7OUT0` reader - P7OUT0"]
pub type P7OUT0_R = crate::BitReader;
#[doc = "Field `P7OUT0` writer - P7OUT0"]
pub type P7OUT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7OUT1` reader - P7OUT1"]
pub type P7OUT1_R = crate::BitReader;
#[doc = "Field `P7OUT1` writer - P7OUT1"]
pub type P7OUT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7OUT2` reader - P7OUT2"]
pub type P7OUT2_R = crate::BitReader;
#[doc = "Field `P7OUT2` writer - P7OUT2"]
pub type P7OUT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7OUT3` reader - P7OUT3"]
pub type P7OUT3_R = crate::BitReader;
#[doc = "Field `P7OUT3` writer - P7OUT3"]
pub type P7OUT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7OUT4` reader - P7OUT4"]
pub type P7OUT4_R = crate::BitReader;
#[doc = "Field `P7OUT4` writer - P7OUT4"]
pub type P7OUT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7OUT5` reader - P7OUT5"]
pub type P7OUT5_R = crate::BitReader;
#[doc = "Field `P7OUT5` writer - P7OUT5"]
pub type P7OUT5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7OUT6` reader - P7OUT6"]
pub type P7OUT6_R = crate::BitReader;
#[doc = "Field `P7OUT6` writer - P7OUT6"]
pub type P7OUT6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7OUT7` reader - P7OUT7"]
pub type P7OUT7_R = crate::BitReader;
#[doc = "Field `P7OUT7` writer - P7OUT7"]
pub type P7OUT7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P7OUT0"]
    #[inline(always)]
    pub fn p7out0(&self) -> P7OUT0_R {
        P7OUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P7OUT1"]
    #[inline(always)]
    pub fn p7out1(&self) -> P7OUT1_R {
        P7OUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P7OUT2"]
    #[inline(always)]
    pub fn p7out2(&self) -> P7OUT2_R {
        P7OUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P7OUT3"]
    #[inline(always)]
    pub fn p7out3(&self) -> P7OUT3_R {
        P7OUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P7OUT4"]
    #[inline(always)]
    pub fn p7out4(&self) -> P7OUT4_R {
        P7OUT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P7OUT5"]
    #[inline(always)]
    pub fn p7out5(&self) -> P7OUT5_R {
        P7OUT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P7OUT6"]
    #[inline(always)]
    pub fn p7out6(&self) -> P7OUT6_R {
        P7OUT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P7OUT7"]
    #[inline(always)]
    pub fn p7out7(&self) -> P7OUT7_R {
        P7OUT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P7OUT0"]
    #[inline(always)]
    #[must_use]
    pub fn p7out0(&mut self) -> P7OUT0_W<P7OUT_SPEC> {
        P7OUT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P7OUT1"]
    #[inline(always)]
    #[must_use]
    pub fn p7out1(&mut self) -> P7OUT1_W<P7OUT_SPEC> {
        P7OUT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P7OUT2"]
    #[inline(always)]
    #[must_use]
    pub fn p7out2(&mut self) -> P7OUT2_W<P7OUT_SPEC> {
        P7OUT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P7OUT3"]
    #[inline(always)]
    #[must_use]
    pub fn p7out3(&mut self) -> P7OUT3_W<P7OUT_SPEC> {
        P7OUT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P7OUT4"]
    #[inline(always)]
    #[must_use]
    pub fn p7out4(&mut self) -> P7OUT4_W<P7OUT_SPEC> {
        P7OUT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P7OUT5"]
    #[inline(always)]
    #[must_use]
    pub fn p7out5(&mut self) -> P7OUT5_W<P7OUT_SPEC> {
        P7OUT5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P7OUT6"]
    #[inline(always)]
    #[must_use]
    pub fn p7out6(&mut self) -> P7OUT6_W<P7OUT_SPEC> {
        P7OUT6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P7OUT7"]
    #[inline(always)]
    #[must_use]
    pub fn p7out7(&mut self) -> P7OUT7_W<P7OUT_SPEC> {
        P7OUT7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 7 Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p7out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p7out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P7OUT_SPEC;
impl crate::RegisterSpec for P7OUT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p7out::R`](R) reader structure"]
impl crate::Readable for P7OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p7out::W`](W) writer structure"]
impl crate::Writable for P7OUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P7OUT to value 0"]
impl crate::Resettable for P7OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
