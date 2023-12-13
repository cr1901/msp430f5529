#[doc = "Register `P8OUT` reader"]
pub type R = crate::R<P8OUT_SPEC>;
#[doc = "Register `P8OUT` writer"]
pub type W = crate::W<P8OUT_SPEC>;
#[doc = "Field `P8OUT0` reader - P8OUT0"]
pub type P8OUT0_R = crate::BitReader;
#[doc = "Field `P8OUT0` writer - P8OUT0"]
pub type P8OUT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8OUT1` reader - P8OUT1"]
pub type P8OUT1_R = crate::BitReader;
#[doc = "Field `P8OUT1` writer - P8OUT1"]
pub type P8OUT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8OUT2` reader - P8OUT2"]
pub type P8OUT2_R = crate::BitReader;
#[doc = "Field `P8OUT2` writer - P8OUT2"]
pub type P8OUT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8OUT3` reader - P8OUT3"]
pub type P8OUT3_R = crate::BitReader;
#[doc = "Field `P8OUT3` writer - P8OUT3"]
pub type P8OUT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8OUT4` reader - P8OUT4"]
pub type P8OUT4_R = crate::BitReader;
#[doc = "Field `P8OUT4` writer - P8OUT4"]
pub type P8OUT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8OUT5` reader - P8OUT5"]
pub type P8OUT5_R = crate::BitReader;
#[doc = "Field `P8OUT5` writer - P8OUT5"]
pub type P8OUT5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8OUT6` reader - P8OUT6"]
pub type P8OUT6_R = crate::BitReader;
#[doc = "Field `P8OUT6` writer - P8OUT6"]
pub type P8OUT6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8OUT7` reader - P8OUT7"]
pub type P8OUT7_R = crate::BitReader;
#[doc = "Field `P8OUT7` writer - P8OUT7"]
pub type P8OUT7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P8OUT0"]
    #[inline(always)]
    pub fn p8out0(&self) -> P8OUT0_R {
        P8OUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P8OUT1"]
    #[inline(always)]
    pub fn p8out1(&self) -> P8OUT1_R {
        P8OUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P8OUT2"]
    #[inline(always)]
    pub fn p8out2(&self) -> P8OUT2_R {
        P8OUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P8OUT3"]
    #[inline(always)]
    pub fn p8out3(&self) -> P8OUT3_R {
        P8OUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P8OUT4"]
    #[inline(always)]
    pub fn p8out4(&self) -> P8OUT4_R {
        P8OUT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P8OUT5"]
    #[inline(always)]
    pub fn p8out5(&self) -> P8OUT5_R {
        P8OUT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P8OUT6"]
    #[inline(always)]
    pub fn p8out6(&self) -> P8OUT6_R {
        P8OUT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P8OUT7"]
    #[inline(always)]
    pub fn p8out7(&self) -> P8OUT7_R {
        P8OUT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P8OUT0"]
    #[inline(always)]
    #[must_use]
    pub fn p8out0(&mut self) -> P8OUT0_W<P8OUT_SPEC> {
        P8OUT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P8OUT1"]
    #[inline(always)]
    #[must_use]
    pub fn p8out1(&mut self) -> P8OUT1_W<P8OUT_SPEC> {
        P8OUT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P8OUT2"]
    #[inline(always)]
    #[must_use]
    pub fn p8out2(&mut self) -> P8OUT2_W<P8OUT_SPEC> {
        P8OUT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P8OUT3"]
    #[inline(always)]
    #[must_use]
    pub fn p8out3(&mut self) -> P8OUT3_W<P8OUT_SPEC> {
        P8OUT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P8OUT4"]
    #[inline(always)]
    #[must_use]
    pub fn p8out4(&mut self) -> P8OUT4_W<P8OUT_SPEC> {
        P8OUT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P8OUT5"]
    #[inline(always)]
    #[must_use]
    pub fn p8out5(&mut self) -> P8OUT5_W<P8OUT_SPEC> {
        P8OUT5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P8OUT6"]
    #[inline(always)]
    #[must_use]
    pub fn p8out6(&mut self) -> P8OUT6_W<P8OUT_SPEC> {
        P8OUT6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P8OUT7"]
    #[inline(always)]
    #[must_use]
    pub fn p8out7(&mut self) -> P8OUT7_W<P8OUT_SPEC> {
        P8OUT7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 8 Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p8out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p8out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P8OUT_SPEC;
impl crate::RegisterSpec for P8OUT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p8out::R`](R) reader structure"]
impl crate::Readable for P8OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p8out::W`](W) writer structure"]
impl crate::Writable for P8OUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P8OUT to value 0"]
impl crate::Resettable for P8OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
