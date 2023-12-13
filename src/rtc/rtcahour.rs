#[doc = "Register `RTCAHOUR` reader"]
pub type R = crate::R<RTCAHOUR_SPEC>;
#[doc = "Register `RTCAHOUR` writer"]
pub type W = crate::W<RTCAHOUR_SPEC>;
#[doc = "Field `HOUR0` reader - Real Time Clock Hour Bit: 0"]
pub type HOUR0_R = crate::BitReader;
#[doc = "Field `HOUR0` writer - Real Time Clock Hour Bit: 0"]
pub type HOUR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOUR1` reader - Real Time Clock Hour Bit: 1"]
pub type HOUR1_R = crate::BitReader;
#[doc = "Field `HOUR1` writer - Real Time Clock Hour Bit: 1"]
pub type HOUR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOUR2` reader - Real Time Clock Hour Bit: 2"]
pub type HOUR2_R = crate::BitReader;
#[doc = "Field `HOUR2` writer - Real Time Clock Hour Bit: 2"]
pub type HOUR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOUR3` reader - Real Time Clock Hour Bit: 3"]
pub type HOUR3_R = crate::BitReader;
#[doc = "Field `HOUR3` writer - Real Time Clock Hour Bit: 3"]
pub type HOUR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOUR4` reader - Real Time Clock Hour Bit: 4"]
pub type HOUR4_R = crate::BitReader;
#[doc = "Field `HOUR4` writer - Real Time Clock Hour Bit: 4"]
pub type HOUR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOUR5` reader - Real Time Clock Hour Bit: 5"]
pub type HOUR5_R = crate::BitReader;
#[doc = "Field `HOUR5` writer - Real Time Clock Hour Bit: 5"]
pub type HOUR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOUR6` reader - Real Time Clock Hour Bit: 6"]
pub type HOUR6_R = crate::BitReader;
#[doc = "Field `HOUR6` writer - Real Time Clock Hour Bit: 6"]
pub type HOUR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAE` reader - Real Time Clock Alarm enable"]
pub type RTCAE_R = crate::BitReader;
#[doc = "Field `RTCAE` writer - Real Time Clock Alarm enable"]
pub type RTCAE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Real Time Clock Hour Bit: 0"]
    #[inline(always)]
    pub fn hour0(&self) -> HOUR0_R {
        HOUR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock Hour Bit: 1"]
    #[inline(always)]
    pub fn hour1(&self) -> HOUR1_R {
        HOUR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock Hour Bit: 2"]
    #[inline(always)]
    pub fn hour2(&self) -> HOUR2_R {
        HOUR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Hour Bit: 3"]
    #[inline(always)]
    pub fn hour3(&self) -> HOUR3_R {
        HOUR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock Hour Bit: 4"]
    #[inline(always)]
    pub fn hour4(&self) -> HOUR4_R {
        HOUR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock Hour Bit: 5"]
    #[inline(always)]
    pub fn hour5(&self) -> HOUR5_R {
        HOUR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock Hour Bit: 6"]
    #[inline(always)]
    pub fn hour6(&self) -> HOUR6_R {
        HOUR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Real Time Clock Alarm enable"]
    #[inline(always)]
    pub fn rtcae(&self) -> RTCAE_R {
        RTCAE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Hour Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn hour0(&mut self) -> HOUR0_W<RTCAHOUR_SPEC> {
        HOUR0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Real Time Clock Hour Bit: 1"]
    #[inline(always)]
    #[must_use]
    pub fn hour1(&mut self) -> HOUR1_W<RTCAHOUR_SPEC> {
        HOUR1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Real Time Clock Hour Bit: 2"]
    #[inline(always)]
    #[must_use]
    pub fn hour2(&mut self) -> HOUR2_W<RTCAHOUR_SPEC> {
        HOUR2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Real Time Clock Hour Bit: 3"]
    #[inline(always)]
    #[must_use]
    pub fn hour3(&mut self) -> HOUR3_W<RTCAHOUR_SPEC> {
        HOUR3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Real Time Clock Hour Bit: 4"]
    #[inline(always)]
    #[must_use]
    pub fn hour4(&mut self) -> HOUR4_W<RTCAHOUR_SPEC> {
        HOUR4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Real Time Clock Hour Bit: 5"]
    #[inline(always)]
    #[must_use]
    pub fn hour5(&mut self) -> HOUR5_W<RTCAHOUR_SPEC> {
        HOUR5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Real Time Clock Hour Bit: 6"]
    #[inline(always)]
    #[must_use]
    pub fn hour6(&mut self) -> HOUR6_W<RTCAHOUR_SPEC> {
        HOUR6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Real Time Clock Alarm enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcae(&mut self) -> RTCAE_W<RTCAHOUR_SPEC> {
        RTCAE_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Real Time Clock Alarm Hour\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcahour::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcahour::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCAHOUR_SPEC;
impl crate::RegisterSpec for RTCAHOUR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rtcahour::R`](R) reader structure"]
impl crate::Readable for RTCAHOUR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtcahour::W`](W) writer structure"]
impl crate::Writable for RTCAHOUR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCAHOUR to value 0"]
impl crate::Resettable for RTCAHOUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
