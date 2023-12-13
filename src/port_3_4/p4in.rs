#[doc = "Register `P4IN` reader"]
pub type R = crate::R<P4IN_SPEC>;
#[doc = "Register `P4IN` writer"]
pub type W = crate::W<P4IN_SPEC>;
#[doc = "Field `P4IN0` reader - P4IN0"]
pub type P4IN0_R = crate::BitReader;
#[doc = "Field `P4IN0` writer - P4IN0"]
pub type P4IN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4IN1` reader - P4IN1"]
pub type P4IN1_R = crate::BitReader;
#[doc = "Field `P4IN1` writer - P4IN1"]
pub type P4IN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4IN2` reader - P4IN2"]
pub type P4IN2_R = crate::BitReader;
#[doc = "Field `P4IN2` writer - P4IN2"]
pub type P4IN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4IN3` reader - P4IN3"]
pub type P4IN3_R = crate::BitReader;
#[doc = "Field `P4IN3` writer - P4IN3"]
pub type P4IN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4IN4` reader - P4IN4"]
pub type P4IN4_R = crate::BitReader;
#[doc = "Field `P4IN4` writer - P4IN4"]
pub type P4IN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4IN5` reader - P4IN5"]
pub type P4IN5_R = crate::BitReader;
#[doc = "Field `P4IN5` writer - P4IN5"]
pub type P4IN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4IN6` reader - P4IN6"]
pub type P4IN6_R = crate::BitReader;
#[doc = "Field `P4IN6` writer - P4IN6"]
pub type P4IN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4IN7` reader - P4IN7"]
pub type P4IN7_R = crate::BitReader;
#[doc = "Field `P4IN7` writer - P4IN7"]
pub type P4IN7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P4IN0"]
    #[inline(always)]
    pub fn p4in0(&self) -> P4IN0_R {
        P4IN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P4IN1"]
    #[inline(always)]
    pub fn p4in1(&self) -> P4IN1_R {
        P4IN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P4IN2"]
    #[inline(always)]
    pub fn p4in2(&self) -> P4IN2_R {
        P4IN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P4IN3"]
    #[inline(always)]
    pub fn p4in3(&self) -> P4IN3_R {
        P4IN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P4IN4"]
    #[inline(always)]
    pub fn p4in4(&self) -> P4IN4_R {
        P4IN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P4IN5"]
    #[inline(always)]
    pub fn p4in5(&self) -> P4IN5_R {
        P4IN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P4IN6"]
    #[inline(always)]
    pub fn p4in6(&self) -> P4IN6_R {
        P4IN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P4IN7"]
    #[inline(always)]
    pub fn p4in7(&self) -> P4IN7_R {
        P4IN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4IN0"]
    #[inline(always)]
    #[must_use]
    pub fn p4in0(&mut self) -> P4IN0_W<P4IN_SPEC> {
        P4IN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P4IN1"]
    #[inline(always)]
    #[must_use]
    pub fn p4in1(&mut self) -> P4IN1_W<P4IN_SPEC> {
        P4IN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P4IN2"]
    #[inline(always)]
    #[must_use]
    pub fn p4in2(&mut self) -> P4IN2_W<P4IN_SPEC> {
        P4IN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P4IN3"]
    #[inline(always)]
    #[must_use]
    pub fn p4in3(&mut self) -> P4IN3_W<P4IN_SPEC> {
        P4IN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P4IN4"]
    #[inline(always)]
    #[must_use]
    pub fn p4in4(&mut self) -> P4IN4_W<P4IN_SPEC> {
        P4IN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P4IN5"]
    #[inline(always)]
    #[must_use]
    pub fn p4in5(&mut self) -> P4IN5_W<P4IN_SPEC> {
        P4IN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P4IN6"]
    #[inline(always)]
    #[must_use]
    pub fn p4in6(&mut self) -> P4IN6_W<P4IN_SPEC> {
        P4IN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P4IN7"]
    #[inline(always)]
    #[must_use]
    pub fn p4in7(&mut self) -> P4IN7_W<P4IN_SPEC> {
        P4IN7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 4 Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p4in::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p4in::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4IN_SPEC;
impl crate::RegisterSpec for P4IN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4in::R`](R) reader structure"]
impl crate::Readable for P4IN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p4in::W`](W) writer structure"]
impl crate::Writable for P4IN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P4IN to value 0"]
impl crate::Resettable for P4IN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
