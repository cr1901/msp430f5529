#[doc = "Register `RTCDAY` reader"]
pub type R = crate::R<RTCDAY_SPEC>;
#[doc = "Register `RTCDAY` writer"]
pub type W = crate::W<RTCDAY_SPEC>;
#[doc = "Field `DAY0` reader - Real Time Clock Day Bit: 0"]
pub type DAY0_R = crate::BitReader;
#[doc = "Field `DAY0` writer - Real Time Clock Day Bit: 0"]
pub type DAY0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAY1` reader - Real Time Clock Day Bit: 1"]
pub type DAY1_R = crate::BitReader;
#[doc = "Field `DAY1` writer - Real Time Clock Day Bit: 1"]
pub type DAY1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAY2` reader - Real Time Clock Day Bit: 2"]
pub type DAY2_R = crate::BitReader;
#[doc = "Field `DAY2` writer - Real Time Clock Day Bit: 2"]
pub type DAY2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAY3` reader - Real Time Clock Day Bit: 3"]
pub type DAY3_R = crate::BitReader;
#[doc = "Field `DAY3` writer - Real Time Clock Day Bit: 3"]
pub type DAY3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAY4` reader - Real Time Clock Day Bit: 4"]
pub type DAY4_R = crate::BitReader;
#[doc = "Field `DAY4` writer - Real Time Clock Day Bit: 4"]
pub type DAY4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAY5` reader - Real Time Clock Day Bit: 5"]
pub type DAY5_R = crate::BitReader;
#[doc = "Field `DAY5` writer - Real Time Clock Day Bit: 5"]
pub type DAY5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAY6` reader - Real Time Clock Day Bit: 6"]
pub type DAY6_R = crate::BitReader;
#[doc = "Field `DAY6` writer - Real Time Clock Day Bit: 6"]
pub type DAY6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Real Time Clock Day Bit: 0"]
    #[inline(always)]
    pub fn day0(&self) -> DAY0_R {
        DAY0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock Day Bit: 1"]
    #[inline(always)]
    pub fn day1(&self) -> DAY1_R {
        DAY1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock Day Bit: 2"]
    #[inline(always)]
    pub fn day2(&self) -> DAY2_R {
        DAY2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Day Bit: 3"]
    #[inline(always)]
    pub fn day3(&self) -> DAY3_R {
        DAY3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock Day Bit: 4"]
    #[inline(always)]
    pub fn day4(&self) -> DAY4_R {
        DAY4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock Day Bit: 5"]
    #[inline(always)]
    pub fn day5(&self) -> DAY5_R {
        DAY5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock Day Bit: 6"]
    #[inline(always)]
    pub fn day6(&self) -> DAY6_R {
        DAY6_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Day Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn day0(&mut self) -> DAY0_W<RTCDAY_SPEC> {
        DAY0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Real Time Clock Day Bit: 1"]
    #[inline(always)]
    #[must_use]
    pub fn day1(&mut self) -> DAY1_W<RTCDAY_SPEC> {
        DAY1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Real Time Clock Day Bit: 2"]
    #[inline(always)]
    #[must_use]
    pub fn day2(&mut self) -> DAY2_W<RTCDAY_SPEC> {
        DAY2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Real Time Clock Day Bit: 3"]
    #[inline(always)]
    #[must_use]
    pub fn day3(&mut self) -> DAY3_W<RTCDAY_SPEC> {
        DAY3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Real Time Clock Day Bit: 4"]
    #[inline(always)]
    #[must_use]
    pub fn day4(&mut self) -> DAY4_W<RTCDAY_SPEC> {
        DAY4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Real Time Clock Day Bit: 5"]
    #[inline(always)]
    #[must_use]
    pub fn day5(&mut self) -> DAY5_W<RTCDAY_SPEC> {
        DAY5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Real Time Clock Day Bit: 6"]
    #[inline(always)]
    #[must_use]
    pub fn day6(&mut self) -> DAY6_W<RTCDAY_SPEC> {
        DAY6_W::new(self, 6)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Real Time Clock Day\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcday::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcday::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCDAY_SPEC;
impl crate::RegisterSpec for RTCDAY_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rtcday::R`](R) reader structure"]
impl crate::Readable for RTCDAY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtcday::W`](W) writer structure"]
impl crate::Writable for RTCDAY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCDAY to value 0"]
impl crate::Resettable for RTCDAY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
