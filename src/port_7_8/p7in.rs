#[doc = "Register `P7IN` reader"]
pub type R = crate::R<P7IN_SPEC>;
#[doc = "Register `P7IN` writer"]
pub type W = crate::W<P7IN_SPEC>;
#[doc = "Field `P7IN0` reader - P7IN0"]
pub type P7IN0_R = crate::BitReader;
#[doc = "Field `P7IN0` writer - P7IN0"]
pub type P7IN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7IN1` reader - P7IN1"]
pub type P7IN1_R = crate::BitReader;
#[doc = "Field `P7IN1` writer - P7IN1"]
pub type P7IN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7IN2` reader - P7IN2"]
pub type P7IN2_R = crate::BitReader;
#[doc = "Field `P7IN2` writer - P7IN2"]
pub type P7IN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7IN3` reader - P7IN3"]
pub type P7IN3_R = crate::BitReader;
#[doc = "Field `P7IN3` writer - P7IN3"]
pub type P7IN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7IN4` reader - P7IN4"]
pub type P7IN4_R = crate::BitReader;
#[doc = "Field `P7IN4` writer - P7IN4"]
pub type P7IN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7IN5` reader - P7IN5"]
pub type P7IN5_R = crate::BitReader;
#[doc = "Field `P7IN5` writer - P7IN5"]
pub type P7IN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7IN6` reader - P7IN6"]
pub type P7IN6_R = crate::BitReader;
#[doc = "Field `P7IN6` writer - P7IN6"]
pub type P7IN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7IN7` reader - P7IN7"]
pub type P7IN7_R = crate::BitReader;
#[doc = "Field `P7IN7` writer - P7IN7"]
pub type P7IN7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P7IN0"]
    #[inline(always)]
    pub fn p7in0(&self) -> P7IN0_R {
        P7IN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P7IN1"]
    #[inline(always)]
    pub fn p7in1(&self) -> P7IN1_R {
        P7IN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P7IN2"]
    #[inline(always)]
    pub fn p7in2(&self) -> P7IN2_R {
        P7IN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P7IN3"]
    #[inline(always)]
    pub fn p7in3(&self) -> P7IN3_R {
        P7IN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P7IN4"]
    #[inline(always)]
    pub fn p7in4(&self) -> P7IN4_R {
        P7IN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P7IN5"]
    #[inline(always)]
    pub fn p7in5(&self) -> P7IN5_R {
        P7IN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P7IN6"]
    #[inline(always)]
    pub fn p7in6(&self) -> P7IN6_R {
        P7IN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P7IN7"]
    #[inline(always)]
    pub fn p7in7(&self) -> P7IN7_R {
        P7IN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P7IN0"]
    #[inline(always)]
    #[must_use]
    pub fn p7in0(&mut self) -> P7IN0_W<P7IN_SPEC> {
        P7IN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P7IN1"]
    #[inline(always)]
    #[must_use]
    pub fn p7in1(&mut self) -> P7IN1_W<P7IN_SPEC> {
        P7IN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P7IN2"]
    #[inline(always)]
    #[must_use]
    pub fn p7in2(&mut self) -> P7IN2_W<P7IN_SPEC> {
        P7IN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P7IN3"]
    #[inline(always)]
    #[must_use]
    pub fn p7in3(&mut self) -> P7IN3_W<P7IN_SPEC> {
        P7IN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P7IN4"]
    #[inline(always)]
    #[must_use]
    pub fn p7in4(&mut self) -> P7IN4_W<P7IN_SPEC> {
        P7IN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P7IN5"]
    #[inline(always)]
    #[must_use]
    pub fn p7in5(&mut self) -> P7IN5_W<P7IN_SPEC> {
        P7IN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P7IN6"]
    #[inline(always)]
    #[must_use]
    pub fn p7in6(&mut self) -> P7IN6_W<P7IN_SPEC> {
        P7IN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P7IN7"]
    #[inline(always)]
    #[must_use]
    pub fn p7in7(&mut self) -> P7IN7_W<P7IN_SPEC> {
        P7IN7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 7 Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p7in::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p7in::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P7IN_SPEC;
impl crate::RegisterSpec for P7IN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p7in::R`](R) reader structure"]
impl crate::Readable for P7IN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p7in::W`](W) writer structure"]
impl crate::Writable for P7IN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P7IN to value 0"]
impl crate::Resettable for P7IN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
