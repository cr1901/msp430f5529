#[doc = "Register `P4OUT` reader"]
pub type R = crate::R<P4OUT_SPEC>;
#[doc = "Register `P4OUT` writer"]
pub type W = crate::W<P4OUT_SPEC>;
#[doc = "Field `P4OUT0` reader - P4OUT0"]
pub type P4OUT0_R = crate::BitReader;
#[doc = "Field `P4OUT0` writer - P4OUT0"]
pub type P4OUT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4OUT1` reader - P4OUT1"]
pub type P4OUT1_R = crate::BitReader;
#[doc = "Field `P4OUT1` writer - P4OUT1"]
pub type P4OUT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4OUT2` reader - P4OUT2"]
pub type P4OUT2_R = crate::BitReader;
#[doc = "Field `P4OUT2` writer - P4OUT2"]
pub type P4OUT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4OUT3` reader - P4OUT3"]
pub type P4OUT3_R = crate::BitReader;
#[doc = "Field `P4OUT3` writer - P4OUT3"]
pub type P4OUT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4OUT4` reader - P4OUT4"]
pub type P4OUT4_R = crate::BitReader;
#[doc = "Field `P4OUT4` writer - P4OUT4"]
pub type P4OUT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4OUT5` reader - P4OUT5"]
pub type P4OUT5_R = crate::BitReader;
#[doc = "Field `P4OUT5` writer - P4OUT5"]
pub type P4OUT5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4OUT6` reader - P4OUT6"]
pub type P4OUT6_R = crate::BitReader;
#[doc = "Field `P4OUT6` writer - P4OUT6"]
pub type P4OUT6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4OUT7` reader - P4OUT7"]
pub type P4OUT7_R = crate::BitReader;
#[doc = "Field `P4OUT7` writer - P4OUT7"]
pub type P4OUT7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P4OUT0"]
    #[inline(always)]
    pub fn p4out0(&self) -> P4OUT0_R {
        P4OUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P4OUT1"]
    #[inline(always)]
    pub fn p4out1(&self) -> P4OUT1_R {
        P4OUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P4OUT2"]
    #[inline(always)]
    pub fn p4out2(&self) -> P4OUT2_R {
        P4OUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P4OUT3"]
    #[inline(always)]
    pub fn p4out3(&self) -> P4OUT3_R {
        P4OUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P4OUT4"]
    #[inline(always)]
    pub fn p4out4(&self) -> P4OUT4_R {
        P4OUT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P4OUT5"]
    #[inline(always)]
    pub fn p4out5(&self) -> P4OUT5_R {
        P4OUT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P4OUT6"]
    #[inline(always)]
    pub fn p4out6(&self) -> P4OUT6_R {
        P4OUT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P4OUT7"]
    #[inline(always)]
    pub fn p4out7(&self) -> P4OUT7_R {
        P4OUT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4OUT0"]
    #[inline(always)]
    #[must_use]
    pub fn p4out0(&mut self) -> P4OUT0_W<P4OUT_SPEC> {
        P4OUT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P4OUT1"]
    #[inline(always)]
    #[must_use]
    pub fn p4out1(&mut self) -> P4OUT1_W<P4OUT_SPEC> {
        P4OUT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P4OUT2"]
    #[inline(always)]
    #[must_use]
    pub fn p4out2(&mut self) -> P4OUT2_W<P4OUT_SPEC> {
        P4OUT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P4OUT3"]
    #[inline(always)]
    #[must_use]
    pub fn p4out3(&mut self) -> P4OUT3_W<P4OUT_SPEC> {
        P4OUT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P4OUT4"]
    #[inline(always)]
    #[must_use]
    pub fn p4out4(&mut self) -> P4OUT4_W<P4OUT_SPEC> {
        P4OUT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P4OUT5"]
    #[inline(always)]
    #[must_use]
    pub fn p4out5(&mut self) -> P4OUT5_W<P4OUT_SPEC> {
        P4OUT5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P4OUT6"]
    #[inline(always)]
    #[must_use]
    pub fn p4out6(&mut self) -> P4OUT6_W<P4OUT_SPEC> {
        P4OUT6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P4OUT7"]
    #[inline(always)]
    #[must_use]
    pub fn p4out7(&mut self) -> P4OUT7_W<P4OUT_SPEC> {
        P4OUT7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 4 Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p4out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p4out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4OUT_SPEC;
impl crate::RegisterSpec for P4OUT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4out::R`](R) reader structure"]
impl crate::Readable for P4OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p4out::W`](W) writer structure"]
impl crate::Writable for P4OUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P4OUT to value 0"]
impl crate::Resettable for P4OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
