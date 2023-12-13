#[doc = "Register `P1IN` reader"]
pub type R = crate::R<P1IN_SPEC>;
#[doc = "Register `P1IN` writer"]
pub type W = crate::W<P1IN_SPEC>;
#[doc = "Field `P1IN0` reader - P1IN0"]
pub type P1IN0_R = crate::BitReader;
#[doc = "Field `P1IN0` writer - P1IN0"]
pub type P1IN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IN1` reader - P1IN1"]
pub type P1IN1_R = crate::BitReader;
#[doc = "Field `P1IN1` writer - P1IN1"]
pub type P1IN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IN2` reader - P1IN2"]
pub type P1IN2_R = crate::BitReader;
#[doc = "Field `P1IN2` writer - P1IN2"]
pub type P1IN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IN3` reader - P1IN3"]
pub type P1IN3_R = crate::BitReader;
#[doc = "Field `P1IN3` writer - P1IN3"]
pub type P1IN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IN4` reader - P1IN4"]
pub type P1IN4_R = crate::BitReader;
#[doc = "Field `P1IN4` writer - P1IN4"]
pub type P1IN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IN5` reader - P1IN5"]
pub type P1IN5_R = crate::BitReader;
#[doc = "Field `P1IN5` writer - P1IN5"]
pub type P1IN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IN6` reader - P1IN6"]
pub type P1IN6_R = crate::BitReader;
#[doc = "Field `P1IN6` writer - P1IN6"]
pub type P1IN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IN7` reader - P1IN7"]
pub type P1IN7_R = crate::BitReader;
#[doc = "Field `P1IN7` writer - P1IN7"]
pub type P1IN7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P1IN0"]
    #[inline(always)]
    pub fn p1in0(&self) -> P1IN0_R {
        P1IN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P1IN1"]
    #[inline(always)]
    pub fn p1in1(&self) -> P1IN1_R {
        P1IN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P1IN2"]
    #[inline(always)]
    pub fn p1in2(&self) -> P1IN2_R {
        P1IN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P1IN3"]
    #[inline(always)]
    pub fn p1in3(&self) -> P1IN3_R {
        P1IN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P1IN4"]
    #[inline(always)]
    pub fn p1in4(&self) -> P1IN4_R {
        P1IN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P1IN5"]
    #[inline(always)]
    pub fn p1in5(&self) -> P1IN5_R {
        P1IN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P1IN6"]
    #[inline(always)]
    pub fn p1in6(&self) -> P1IN6_R {
        P1IN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P1IN7"]
    #[inline(always)]
    pub fn p1in7(&self) -> P1IN7_R {
        P1IN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1IN0"]
    #[inline(always)]
    #[must_use]
    pub fn p1in0(&mut self) -> P1IN0_W<P1IN_SPEC> {
        P1IN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P1IN1"]
    #[inline(always)]
    #[must_use]
    pub fn p1in1(&mut self) -> P1IN1_W<P1IN_SPEC> {
        P1IN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P1IN2"]
    #[inline(always)]
    #[must_use]
    pub fn p1in2(&mut self) -> P1IN2_W<P1IN_SPEC> {
        P1IN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P1IN3"]
    #[inline(always)]
    #[must_use]
    pub fn p1in3(&mut self) -> P1IN3_W<P1IN_SPEC> {
        P1IN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P1IN4"]
    #[inline(always)]
    #[must_use]
    pub fn p1in4(&mut self) -> P1IN4_W<P1IN_SPEC> {
        P1IN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P1IN5"]
    #[inline(always)]
    #[must_use]
    pub fn p1in5(&mut self) -> P1IN5_W<P1IN_SPEC> {
        P1IN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P1IN6"]
    #[inline(always)]
    #[must_use]
    pub fn p1in6(&mut self) -> P1IN6_W<P1IN_SPEC> {
        P1IN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P1IN7"]
    #[inline(always)]
    #[must_use]
    pub fn p1in7(&mut self) -> P1IN7_W<P1IN_SPEC> {
        P1IN7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 1 Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p1in::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p1in::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1IN_SPEC;
impl crate::RegisterSpec for P1IN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1in::R`](R) reader structure"]
impl crate::Readable for P1IN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p1in::W`](W) writer structure"]
impl crate::Writable for P1IN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P1IN to value 0"]
impl crate::Resettable for P1IN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
