#[doc = "Register `P3IN` reader"]
pub type R = crate::R<P3IN_SPEC>;
#[doc = "Register `P3IN` writer"]
pub type W = crate::W<P3IN_SPEC>;
#[doc = "Field `P3IN0` reader - P3IN0"]
pub type P3IN0_R = crate::BitReader;
#[doc = "Field `P3IN0` writer - P3IN0"]
pub type P3IN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3IN1` reader - P3IN1"]
pub type P3IN1_R = crate::BitReader;
#[doc = "Field `P3IN1` writer - P3IN1"]
pub type P3IN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3IN2` reader - P3IN2"]
pub type P3IN2_R = crate::BitReader;
#[doc = "Field `P3IN2` writer - P3IN2"]
pub type P3IN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3IN3` reader - P3IN3"]
pub type P3IN3_R = crate::BitReader;
#[doc = "Field `P3IN3` writer - P3IN3"]
pub type P3IN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3IN4` reader - P3IN4"]
pub type P3IN4_R = crate::BitReader;
#[doc = "Field `P3IN4` writer - P3IN4"]
pub type P3IN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3IN5` reader - P3IN5"]
pub type P3IN5_R = crate::BitReader;
#[doc = "Field `P3IN5` writer - P3IN5"]
pub type P3IN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3IN6` reader - P3IN6"]
pub type P3IN6_R = crate::BitReader;
#[doc = "Field `P3IN6` writer - P3IN6"]
pub type P3IN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3IN7` reader - P3IN7"]
pub type P3IN7_R = crate::BitReader;
#[doc = "Field `P3IN7` writer - P3IN7"]
pub type P3IN7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P3IN0"]
    #[inline(always)]
    pub fn p3in0(&self) -> P3IN0_R {
        P3IN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P3IN1"]
    #[inline(always)]
    pub fn p3in1(&self) -> P3IN1_R {
        P3IN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P3IN2"]
    #[inline(always)]
    pub fn p3in2(&self) -> P3IN2_R {
        P3IN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P3IN3"]
    #[inline(always)]
    pub fn p3in3(&self) -> P3IN3_R {
        P3IN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P3IN4"]
    #[inline(always)]
    pub fn p3in4(&self) -> P3IN4_R {
        P3IN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P3IN5"]
    #[inline(always)]
    pub fn p3in5(&self) -> P3IN5_R {
        P3IN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P3IN6"]
    #[inline(always)]
    pub fn p3in6(&self) -> P3IN6_R {
        P3IN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P3IN7"]
    #[inline(always)]
    pub fn p3in7(&self) -> P3IN7_R {
        P3IN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3IN0"]
    #[inline(always)]
    #[must_use]
    pub fn p3in0(&mut self) -> P3IN0_W<P3IN_SPEC> {
        P3IN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P3IN1"]
    #[inline(always)]
    #[must_use]
    pub fn p3in1(&mut self) -> P3IN1_W<P3IN_SPEC> {
        P3IN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P3IN2"]
    #[inline(always)]
    #[must_use]
    pub fn p3in2(&mut self) -> P3IN2_W<P3IN_SPEC> {
        P3IN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P3IN3"]
    #[inline(always)]
    #[must_use]
    pub fn p3in3(&mut self) -> P3IN3_W<P3IN_SPEC> {
        P3IN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P3IN4"]
    #[inline(always)]
    #[must_use]
    pub fn p3in4(&mut self) -> P3IN4_W<P3IN_SPEC> {
        P3IN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P3IN5"]
    #[inline(always)]
    #[must_use]
    pub fn p3in5(&mut self) -> P3IN5_W<P3IN_SPEC> {
        P3IN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P3IN6"]
    #[inline(always)]
    #[must_use]
    pub fn p3in6(&mut self) -> P3IN6_W<P3IN_SPEC> {
        P3IN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P3IN7"]
    #[inline(always)]
    #[must_use]
    pub fn p3in7(&mut self) -> P3IN7_W<P3IN_SPEC> {
        P3IN7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 3 Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p3in::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p3in::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3IN_SPEC;
impl crate::RegisterSpec for P3IN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3in::R`](R) reader structure"]
impl crate::Readable for P3IN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p3in::W`](W) writer structure"]
impl crate::Writable for P3IN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P3IN to value 0"]
impl crate::Resettable for P3IN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
