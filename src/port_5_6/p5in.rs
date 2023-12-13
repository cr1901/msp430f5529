#[doc = "Register `P5IN` reader"]
pub type R = crate::R<P5IN_SPEC>;
#[doc = "Register `P5IN` writer"]
pub type W = crate::W<P5IN_SPEC>;
#[doc = "Field `P5IN0` reader - P5IN0"]
pub type P5IN0_R = crate::BitReader;
#[doc = "Field `P5IN0` writer - P5IN0"]
pub type P5IN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5IN1` reader - P5IN1"]
pub type P5IN1_R = crate::BitReader;
#[doc = "Field `P5IN1` writer - P5IN1"]
pub type P5IN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5IN2` reader - P5IN2"]
pub type P5IN2_R = crate::BitReader;
#[doc = "Field `P5IN2` writer - P5IN2"]
pub type P5IN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5IN3` reader - P5IN3"]
pub type P5IN3_R = crate::BitReader;
#[doc = "Field `P5IN3` writer - P5IN3"]
pub type P5IN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5IN4` reader - P5IN4"]
pub type P5IN4_R = crate::BitReader;
#[doc = "Field `P5IN4` writer - P5IN4"]
pub type P5IN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5IN5` reader - P5IN5"]
pub type P5IN5_R = crate::BitReader;
#[doc = "Field `P5IN5` writer - P5IN5"]
pub type P5IN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5IN6` reader - P5IN6"]
pub type P5IN6_R = crate::BitReader;
#[doc = "Field `P5IN6` writer - P5IN6"]
pub type P5IN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5IN7` reader - P5IN7"]
pub type P5IN7_R = crate::BitReader;
#[doc = "Field `P5IN7` writer - P5IN7"]
pub type P5IN7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P5IN0"]
    #[inline(always)]
    pub fn p5in0(&self) -> P5IN0_R {
        P5IN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P5IN1"]
    #[inline(always)]
    pub fn p5in1(&self) -> P5IN1_R {
        P5IN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P5IN2"]
    #[inline(always)]
    pub fn p5in2(&self) -> P5IN2_R {
        P5IN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P5IN3"]
    #[inline(always)]
    pub fn p5in3(&self) -> P5IN3_R {
        P5IN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P5IN4"]
    #[inline(always)]
    pub fn p5in4(&self) -> P5IN4_R {
        P5IN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P5IN5"]
    #[inline(always)]
    pub fn p5in5(&self) -> P5IN5_R {
        P5IN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P5IN6"]
    #[inline(always)]
    pub fn p5in6(&self) -> P5IN6_R {
        P5IN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P5IN7"]
    #[inline(always)]
    pub fn p5in7(&self) -> P5IN7_R {
        P5IN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P5IN0"]
    #[inline(always)]
    #[must_use]
    pub fn p5in0(&mut self) -> P5IN0_W<P5IN_SPEC> {
        P5IN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P5IN1"]
    #[inline(always)]
    #[must_use]
    pub fn p5in1(&mut self) -> P5IN1_W<P5IN_SPEC> {
        P5IN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P5IN2"]
    #[inline(always)]
    #[must_use]
    pub fn p5in2(&mut self) -> P5IN2_W<P5IN_SPEC> {
        P5IN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P5IN3"]
    #[inline(always)]
    #[must_use]
    pub fn p5in3(&mut self) -> P5IN3_W<P5IN_SPEC> {
        P5IN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P5IN4"]
    #[inline(always)]
    #[must_use]
    pub fn p5in4(&mut self) -> P5IN4_W<P5IN_SPEC> {
        P5IN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P5IN5"]
    #[inline(always)]
    #[must_use]
    pub fn p5in5(&mut self) -> P5IN5_W<P5IN_SPEC> {
        P5IN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P5IN6"]
    #[inline(always)]
    #[must_use]
    pub fn p5in6(&mut self) -> P5IN6_W<P5IN_SPEC> {
        P5IN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P5IN7"]
    #[inline(always)]
    #[must_use]
    pub fn p5in7(&mut self) -> P5IN7_W<P5IN_SPEC> {
        P5IN7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 5 Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p5in::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p5in::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P5IN_SPEC;
impl crate::RegisterSpec for P5IN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p5in::R`](R) reader structure"]
impl crate::Readable for P5IN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p5in::W`](W) writer structure"]
impl crate::Writable for P5IN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P5IN to value 0"]
impl crate::Resettable for P5IN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
