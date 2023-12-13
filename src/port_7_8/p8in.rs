#[doc = "Register `P8IN` reader"]
pub type R = crate::R<P8IN_SPEC>;
#[doc = "Register `P8IN` writer"]
pub type W = crate::W<P8IN_SPEC>;
#[doc = "Field `P8IN0` reader - P8IN0"]
pub type P8IN0_R = crate::BitReader;
#[doc = "Field `P8IN0` writer - P8IN0"]
pub type P8IN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8IN1` reader - P8IN1"]
pub type P8IN1_R = crate::BitReader;
#[doc = "Field `P8IN1` writer - P8IN1"]
pub type P8IN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8IN2` reader - P8IN2"]
pub type P8IN2_R = crate::BitReader;
#[doc = "Field `P8IN2` writer - P8IN2"]
pub type P8IN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8IN3` reader - P8IN3"]
pub type P8IN3_R = crate::BitReader;
#[doc = "Field `P8IN3` writer - P8IN3"]
pub type P8IN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8IN4` reader - P8IN4"]
pub type P8IN4_R = crate::BitReader;
#[doc = "Field `P8IN4` writer - P8IN4"]
pub type P8IN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8IN5` reader - P8IN5"]
pub type P8IN5_R = crate::BitReader;
#[doc = "Field `P8IN5` writer - P8IN5"]
pub type P8IN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8IN6` reader - P8IN6"]
pub type P8IN6_R = crate::BitReader;
#[doc = "Field `P8IN6` writer - P8IN6"]
pub type P8IN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8IN7` reader - P8IN7"]
pub type P8IN7_R = crate::BitReader;
#[doc = "Field `P8IN7` writer - P8IN7"]
pub type P8IN7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P8IN0"]
    #[inline(always)]
    pub fn p8in0(&self) -> P8IN0_R {
        P8IN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P8IN1"]
    #[inline(always)]
    pub fn p8in1(&self) -> P8IN1_R {
        P8IN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P8IN2"]
    #[inline(always)]
    pub fn p8in2(&self) -> P8IN2_R {
        P8IN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P8IN3"]
    #[inline(always)]
    pub fn p8in3(&self) -> P8IN3_R {
        P8IN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P8IN4"]
    #[inline(always)]
    pub fn p8in4(&self) -> P8IN4_R {
        P8IN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P8IN5"]
    #[inline(always)]
    pub fn p8in5(&self) -> P8IN5_R {
        P8IN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P8IN6"]
    #[inline(always)]
    pub fn p8in6(&self) -> P8IN6_R {
        P8IN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P8IN7"]
    #[inline(always)]
    pub fn p8in7(&self) -> P8IN7_R {
        P8IN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P8IN0"]
    #[inline(always)]
    #[must_use]
    pub fn p8in0(&mut self) -> P8IN0_W<P8IN_SPEC> {
        P8IN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P8IN1"]
    #[inline(always)]
    #[must_use]
    pub fn p8in1(&mut self) -> P8IN1_W<P8IN_SPEC> {
        P8IN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P8IN2"]
    #[inline(always)]
    #[must_use]
    pub fn p8in2(&mut self) -> P8IN2_W<P8IN_SPEC> {
        P8IN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P8IN3"]
    #[inline(always)]
    #[must_use]
    pub fn p8in3(&mut self) -> P8IN3_W<P8IN_SPEC> {
        P8IN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P8IN4"]
    #[inline(always)]
    #[must_use]
    pub fn p8in4(&mut self) -> P8IN4_W<P8IN_SPEC> {
        P8IN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P8IN5"]
    #[inline(always)]
    #[must_use]
    pub fn p8in5(&mut self) -> P8IN5_W<P8IN_SPEC> {
        P8IN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P8IN6"]
    #[inline(always)]
    #[must_use]
    pub fn p8in6(&mut self) -> P8IN6_W<P8IN_SPEC> {
        P8IN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P8IN7"]
    #[inline(always)]
    #[must_use]
    pub fn p8in7(&mut self) -> P8IN7_W<P8IN_SPEC> {
        P8IN7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 8 Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p8in::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p8in::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P8IN_SPEC;
impl crate::RegisterSpec for P8IN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p8in::R`](R) reader structure"]
impl crate::Readable for P8IN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p8in::W`](W) writer structure"]
impl crate::Writable for P8IN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P8IN to value 0"]
impl crate::Resettable for P8IN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
