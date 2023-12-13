#[doc = "Register `RTCAMIN` reader"]
pub type R = crate::R<RTCAMIN_SPEC>;
#[doc = "Register `RTCAMIN` writer"]
pub type W = crate::W<RTCAMIN_SPEC>;
#[doc = "Field `MINUTES0` reader - Real Time Clock Minutes Bit: 0"]
pub type MINUTES0_R = crate::BitReader;
#[doc = "Field `MINUTES0` writer - Real Time Clock Minutes Bit: 0"]
pub type MINUTES0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINUTES1` reader - Real Time Clock Minutes Bit: 1"]
pub type MINUTES1_R = crate::BitReader;
#[doc = "Field `MINUTES1` writer - Real Time Clock Minutes Bit: 1"]
pub type MINUTES1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINUTES2` reader - Real Time Clock Minutes Bit: 2"]
pub type MINUTES2_R = crate::BitReader;
#[doc = "Field `MINUTES2` writer - Real Time Clock Minutes Bit: 2"]
pub type MINUTES2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINUTES3` reader - Real Time Clock Minutes Bit: 3"]
pub type MINUTES3_R = crate::BitReader;
#[doc = "Field `MINUTES3` writer - Real Time Clock Minutes Bit: 3"]
pub type MINUTES3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINUTES4` reader - Real Time Clock Minutes Bit: 4"]
pub type MINUTES4_R = crate::BitReader;
#[doc = "Field `MINUTES4` writer - Real Time Clock Minutes Bit: 4"]
pub type MINUTES4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINUTES5` reader - Real Time Clock Minutes Bit: 5"]
pub type MINUTES5_R = crate::BitReader;
#[doc = "Field `MINUTES5` writer - Real Time Clock Minutes Bit: 5"]
pub type MINUTES5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINUTES6` reader - Real Time Clock Minutes Bit: 6"]
pub type MINUTES6_R = crate::BitReader;
#[doc = "Field `MINUTES6` writer - Real Time Clock Minutes Bit: 6"]
pub type MINUTES6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAE` reader - Real Time Clock Alarm enable"]
pub type RTCAE_R = crate::BitReader;
#[doc = "Field `RTCAE` writer - Real Time Clock Alarm enable"]
pub type RTCAE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Real Time Clock Minutes Bit: 0"]
    #[inline(always)]
    pub fn minutes0(&self) -> MINUTES0_R {
        MINUTES0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock Minutes Bit: 1"]
    #[inline(always)]
    pub fn minutes1(&self) -> MINUTES1_R {
        MINUTES1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock Minutes Bit: 2"]
    #[inline(always)]
    pub fn minutes2(&self) -> MINUTES2_R {
        MINUTES2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Minutes Bit: 3"]
    #[inline(always)]
    pub fn minutes3(&self) -> MINUTES3_R {
        MINUTES3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock Minutes Bit: 4"]
    #[inline(always)]
    pub fn minutes4(&self) -> MINUTES4_R {
        MINUTES4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock Minutes Bit: 5"]
    #[inline(always)]
    pub fn minutes5(&self) -> MINUTES5_R {
        MINUTES5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock Minutes Bit: 6"]
    #[inline(always)]
    pub fn minutes6(&self) -> MINUTES6_R {
        MINUTES6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Real Time Clock Alarm enable"]
    #[inline(always)]
    pub fn rtcae(&self) -> RTCAE_R {
        RTCAE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Minutes Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn minutes0(&mut self) -> MINUTES0_W<RTCAMIN_SPEC> {
        MINUTES0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Real Time Clock Minutes Bit: 1"]
    #[inline(always)]
    #[must_use]
    pub fn minutes1(&mut self) -> MINUTES1_W<RTCAMIN_SPEC> {
        MINUTES1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Real Time Clock Minutes Bit: 2"]
    #[inline(always)]
    #[must_use]
    pub fn minutes2(&mut self) -> MINUTES2_W<RTCAMIN_SPEC> {
        MINUTES2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Real Time Clock Minutes Bit: 3"]
    #[inline(always)]
    #[must_use]
    pub fn minutes3(&mut self) -> MINUTES3_W<RTCAMIN_SPEC> {
        MINUTES3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Real Time Clock Minutes Bit: 4"]
    #[inline(always)]
    #[must_use]
    pub fn minutes4(&mut self) -> MINUTES4_W<RTCAMIN_SPEC> {
        MINUTES4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Real Time Clock Minutes Bit: 5"]
    #[inline(always)]
    #[must_use]
    pub fn minutes5(&mut self) -> MINUTES5_W<RTCAMIN_SPEC> {
        MINUTES5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Real Time Clock Minutes Bit: 6"]
    #[inline(always)]
    #[must_use]
    pub fn minutes6(&mut self) -> MINUTES6_W<RTCAMIN_SPEC> {
        MINUTES6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Real Time Clock Alarm enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcae(&mut self) -> RTCAE_W<RTCAMIN_SPEC> {
        RTCAE_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Real Time Clock Alarm Min\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcamin::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcamin::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCAMIN_SPEC;
impl crate::RegisterSpec for RTCAMIN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rtcamin::R`](R) reader structure"]
impl crate::Readable for RTCAMIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtcamin::W`](W) writer structure"]
impl crate::Writable for RTCAMIN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCAMIN to value 0"]
impl crate::Resettable for RTCAMIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
