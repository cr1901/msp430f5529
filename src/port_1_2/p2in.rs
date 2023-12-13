#[doc = "Register `P2IN` reader"]
pub type R = crate::R<P2IN_SPEC>;
#[doc = "Register `P2IN` writer"]
pub type W = crate::W<P2IN_SPEC>;
#[doc = "Field `P2IN0` reader - P2IN0"]
pub type P2IN0_R = crate::BitReader;
#[doc = "Field `P2IN0` writer - P2IN0"]
pub type P2IN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IN1` reader - P2IN1"]
pub type P2IN1_R = crate::BitReader;
#[doc = "Field `P2IN1` writer - P2IN1"]
pub type P2IN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IN2` reader - P2IN2"]
pub type P2IN2_R = crate::BitReader;
#[doc = "Field `P2IN2` writer - P2IN2"]
pub type P2IN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IN3` reader - P2IN3"]
pub type P2IN3_R = crate::BitReader;
#[doc = "Field `P2IN3` writer - P2IN3"]
pub type P2IN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IN4` reader - P2IN4"]
pub type P2IN4_R = crate::BitReader;
#[doc = "Field `P2IN4` writer - P2IN4"]
pub type P2IN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IN5` reader - P2IN5"]
pub type P2IN5_R = crate::BitReader;
#[doc = "Field `P2IN5` writer - P2IN5"]
pub type P2IN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IN6` reader - P2IN6"]
pub type P2IN6_R = crate::BitReader;
#[doc = "Field `P2IN6` writer - P2IN6"]
pub type P2IN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IN7` reader - P2IN7"]
pub type P2IN7_R = crate::BitReader;
#[doc = "Field `P2IN7` writer - P2IN7"]
pub type P2IN7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P2IN0"]
    #[inline(always)]
    pub fn p2in0(&self) -> P2IN0_R {
        P2IN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P2IN1"]
    #[inline(always)]
    pub fn p2in1(&self) -> P2IN1_R {
        P2IN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2IN2"]
    #[inline(always)]
    pub fn p2in2(&self) -> P2IN2_R {
        P2IN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P2IN3"]
    #[inline(always)]
    pub fn p2in3(&self) -> P2IN3_R {
        P2IN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P2IN4"]
    #[inline(always)]
    pub fn p2in4(&self) -> P2IN4_R {
        P2IN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P2IN5"]
    #[inline(always)]
    pub fn p2in5(&self) -> P2IN5_R {
        P2IN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P2IN6"]
    #[inline(always)]
    pub fn p2in6(&self) -> P2IN6_R {
        P2IN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P2IN7"]
    #[inline(always)]
    pub fn p2in7(&self) -> P2IN7_R {
        P2IN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2IN0"]
    #[inline(always)]
    #[must_use]
    pub fn p2in0(&mut self) -> P2IN0_W<P2IN_SPEC> {
        P2IN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P2IN1"]
    #[inline(always)]
    #[must_use]
    pub fn p2in1(&mut self) -> P2IN1_W<P2IN_SPEC> {
        P2IN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P2IN2"]
    #[inline(always)]
    #[must_use]
    pub fn p2in2(&mut self) -> P2IN2_W<P2IN_SPEC> {
        P2IN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P2IN3"]
    #[inline(always)]
    #[must_use]
    pub fn p2in3(&mut self) -> P2IN3_W<P2IN_SPEC> {
        P2IN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P2IN4"]
    #[inline(always)]
    #[must_use]
    pub fn p2in4(&mut self) -> P2IN4_W<P2IN_SPEC> {
        P2IN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P2IN5"]
    #[inline(always)]
    #[must_use]
    pub fn p2in5(&mut self) -> P2IN5_W<P2IN_SPEC> {
        P2IN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P2IN6"]
    #[inline(always)]
    #[must_use]
    pub fn p2in6(&mut self) -> P2IN6_W<P2IN_SPEC> {
        P2IN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P2IN7"]
    #[inline(always)]
    #[must_use]
    pub fn p2in7(&mut self) -> P2IN7_W<P2IN_SPEC> {
        P2IN7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 2 Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p2in::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p2in::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2IN_SPEC;
impl crate::RegisterSpec for P2IN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2in::R`](R) reader structure"]
impl crate::Readable for P2IN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p2in::W`](W) writer structure"]
impl crate::Writable for P2IN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P2IN to value 0"]
impl crate::Resettable for P2IN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
