#[doc = "Register `P6OUT` reader"]
pub type R = crate::R<P6OUT_SPEC>;
#[doc = "Register `P6OUT` writer"]
pub type W = crate::W<P6OUT_SPEC>;
#[doc = "Field `P6OUT0` reader - P6OUT0"]
pub type P6OUT0_R = crate::BitReader;
#[doc = "Field `P6OUT0` writer - P6OUT0"]
pub type P6OUT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6OUT1` reader - P6OUT1"]
pub type P6OUT1_R = crate::BitReader;
#[doc = "Field `P6OUT1` writer - P6OUT1"]
pub type P6OUT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6OUT2` reader - P6OUT2"]
pub type P6OUT2_R = crate::BitReader;
#[doc = "Field `P6OUT2` writer - P6OUT2"]
pub type P6OUT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6OUT3` reader - P6OUT3"]
pub type P6OUT3_R = crate::BitReader;
#[doc = "Field `P6OUT3` writer - P6OUT3"]
pub type P6OUT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6OUT4` reader - P6OUT4"]
pub type P6OUT4_R = crate::BitReader;
#[doc = "Field `P6OUT4` writer - P6OUT4"]
pub type P6OUT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6OUT5` reader - P6OUT5"]
pub type P6OUT5_R = crate::BitReader;
#[doc = "Field `P6OUT5` writer - P6OUT5"]
pub type P6OUT5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6OUT6` reader - P6OUT6"]
pub type P6OUT6_R = crate::BitReader;
#[doc = "Field `P6OUT6` writer - P6OUT6"]
pub type P6OUT6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6OUT7` reader - P6OUT7"]
pub type P6OUT7_R = crate::BitReader;
#[doc = "Field `P6OUT7` writer - P6OUT7"]
pub type P6OUT7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P6OUT0"]
    #[inline(always)]
    pub fn p6out0(&self) -> P6OUT0_R {
        P6OUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P6OUT1"]
    #[inline(always)]
    pub fn p6out1(&self) -> P6OUT1_R {
        P6OUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P6OUT2"]
    #[inline(always)]
    pub fn p6out2(&self) -> P6OUT2_R {
        P6OUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P6OUT3"]
    #[inline(always)]
    pub fn p6out3(&self) -> P6OUT3_R {
        P6OUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P6OUT4"]
    #[inline(always)]
    pub fn p6out4(&self) -> P6OUT4_R {
        P6OUT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P6OUT5"]
    #[inline(always)]
    pub fn p6out5(&self) -> P6OUT5_R {
        P6OUT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P6OUT6"]
    #[inline(always)]
    pub fn p6out6(&self) -> P6OUT6_R {
        P6OUT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P6OUT7"]
    #[inline(always)]
    pub fn p6out7(&self) -> P6OUT7_R {
        P6OUT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P6OUT0"]
    #[inline(always)]
    #[must_use]
    pub fn p6out0(&mut self) -> P6OUT0_W<P6OUT_SPEC> {
        P6OUT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P6OUT1"]
    #[inline(always)]
    #[must_use]
    pub fn p6out1(&mut self) -> P6OUT1_W<P6OUT_SPEC> {
        P6OUT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P6OUT2"]
    #[inline(always)]
    #[must_use]
    pub fn p6out2(&mut self) -> P6OUT2_W<P6OUT_SPEC> {
        P6OUT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P6OUT3"]
    #[inline(always)]
    #[must_use]
    pub fn p6out3(&mut self) -> P6OUT3_W<P6OUT_SPEC> {
        P6OUT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P6OUT4"]
    #[inline(always)]
    #[must_use]
    pub fn p6out4(&mut self) -> P6OUT4_W<P6OUT_SPEC> {
        P6OUT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P6OUT5"]
    #[inline(always)]
    #[must_use]
    pub fn p6out5(&mut self) -> P6OUT5_W<P6OUT_SPEC> {
        P6OUT5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P6OUT6"]
    #[inline(always)]
    #[must_use]
    pub fn p6out6(&mut self) -> P6OUT6_W<P6OUT_SPEC> {
        P6OUT6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P6OUT7"]
    #[inline(always)]
    #[must_use]
    pub fn p6out7(&mut self) -> P6OUT7_W<P6OUT_SPEC> {
        P6OUT7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 6 Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p6out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p6out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P6OUT_SPEC;
impl crate::RegisterSpec for P6OUT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p6out::R`](R) reader structure"]
impl crate::Readable for P6OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p6out::W`](W) writer structure"]
impl crate::Writable for P6OUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P6OUT to value 0"]
impl crate::Resettable for P6OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
