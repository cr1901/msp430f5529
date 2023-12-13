#[doc = "Register `P3OUT` reader"]
pub type R = crate::R<P3OUT_SPEC>;
#[doc = "Register `P3OUT` writer"]
pub type W = crate::W<P3OUT_SPEC>;
#[doc = "Field `P3OUT0` reader - P3OUT0"]
pub type P3OUT0_R = crate::BitReader;
#[doc = "Field `P3OUT0` writer - P3OUT0"]
pub type P3OUT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3OUT1` reader - P3OUT1"]
pub type P3OUT1_R = crate::BitReader;
#[doc = "Field `P3OUT1` writer - P3OUT1"]
pub type P3OUT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3OUT2` reader - P3OUT2"]
pub type P3OUT2_R = crate::BitReader;
#[doc = "Field `P3OUT2` writer - P3OUT2"]
pub type P3OUT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3OUT3` reader - P3OUT3"]
pub type P3OUT3_R = crate::BitReader;
#[doc = "Field `P3OUT3` writer - P3OUT3"]
pub type P3OUT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3OUT4` reader - P3OUT4"]
pub type P3OUT4_R = crate::BitReader;
#[doc = "Field `P3OUT4` writer - P3OUT4"]
pub type P3OUT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3OUT5` reader - P3OUT5"]
pub type P3OUT5_R = crate::BitReader;
#[doc = "Field `P3OUT5` writer - P3OUT5"]
pub type P3OUT5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3OUT6` reader - P3OUT6"]
pub type P3OUT6_R = crate::BitReader;
#[doc = "Field `P3OUT6` writer - P3OUT6"]
pub type P3OUT6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3OUT7` reader - P3OUT7"]
pub type P3OUT7_R = crate::BitReader;
#[doc = "Field `P3OUT7` writer - P3OUT7"]
pub type P3OUT7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P3OUT0"]
    #[inline(always)]
    pub fn p3out0(&self) -> P3OUT0_R {
        P3OUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P3OUT1"]
    #[inline(always)]
    pub fn p3out1(&self) -> P3OUT1_R {
        P3OUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P3OUT2"]
    #[inline(always)]
    pub fn p3out2(&self) -> P3OUT2_R {
        P3OUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P3OUT3"]
    #[inline(always)]
    pub fn p3out3(&self) -> P3OUT3_R {
        P3OUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P3OUT4"]
    #[inline(always)]
    pub fn p3out4(&self) -> P3OUT4_R {
        P3OUT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P3OUT5"]
    #[inline(always)]
    pub fn p3out5(&self) -> P3OUT5_R {
        P3OUT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P3OUT6"]
    #[inline(always)]
    pub fn p3out6(&self) -> P3OUT6_R {
        P3OUT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P3OUT7"]
    #[inline(always)]
    pub fn p3out7(&self) -> P3OUT7_R {
        P3OUT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3OUT0"]
    #[inline(always)]
    #[must_use]
    pub fn p3out0(&mut self) -> P3OUT0_W<P3OUT_SPEC> {
        P3OUT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P3OUT1"]
    #[inline(always)]
    #[must_use]
    pub fn p3out1(&mut self) -> P3OUT1_W<P3OUT_SPEC> {
        P3OUT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P3OUT2"]
    #[inline(always)]
    #[must_use]
    pub fn p3out2(&mut self) -> P3OUT2_W<P3OUT_SPEC> {
        P3OUT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P3OUT3"]
    #[inline(always)]
    #[must_use]
    pub fn p3out3(&mut self) -> P3OUT3_W<P3OUT_SPEC> {
        P3OUT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P3OUT4"]
    #[inline(always)]
    #[must_use]
    pub fn p3out4(&mut self) -> P3OUT4_W<P3OUT_SPEC> {
        P3OUT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P3OUT5"]
    #[inline(always)]
    #[must_use]
    pub fn p3out5(&mut self) -> P3OUT5_W<P3OUT_SPEC> {
        P3OUT5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P3OUT6"]
    #[inline(always)]
    #[must_use]
    pub fn p3out6(&mut self) -> P3OUT6_W<P3OUT_SPEC> {
        P3OUT6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P3OUT7"]
    #[inline(always)]
    #[must_use]
    pub fn p3out7(&mut self) -> P3OUT7_W<P3OUT_SPEC> {
        P3OUT7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 3 Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p3out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p3out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3OUT_SPEC;
impl crate::RegisterSpec for P3OUT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3out::R`](R) reader structure"]
impl crate::Readable for P3OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p3out::W`](W) writer structure"]
impl crate::Writable for P3OUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P3OUT to value 0"]
impl crate::Resettable for P3OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
