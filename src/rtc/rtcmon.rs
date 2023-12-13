#[doc = "Register `RTCMON` reader"]
pub type R = crate::R<RTCMON_SPEC>;
#[doc = "Register `RTCMON` writer"]
pub type W = crate::W<RTCMON_SPEC>;
#[doc = "Field `MONTH0` reader - Real Time Clock Month Bit: 0"]
pub type MONTH0_R = crate::BitReader;
#[doc = "Field `MONTH0` writer - Real Time Clock Month Bit: 0"]
pub type MONTH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONTH1` reader - Real Time Clock Month Bit: 1"]
pub type MONTH1_R = crate::BitReader;
#[doc = "Field `MONTH1` writer - Real Time Clock Month Bit: 1"]
pub type MONTH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONTH2` reader - Real Time Clock Month Bit: 2"]
pub type MONTH2_R = crate::BitReader;
#[doc = "Field `MONTH2` writer - Real Time Clock Month Bit: 2"]
pub type MONTH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONTH3` reader - Real Time Clock Month Bit: 3"]
pub type MONTH3_R = crate::BitReader;
#[doc = "Field `MONTH3` writer - Real Time Clock Month Bit: 3"]
pub type MONTH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONTH4` reader - Real Time Clock Month Bit: 4"]
pub type MONTH4_R = crate::BitReader;
#[doc = "Field `MONTH4` writer - Real Time Clock Month Bit: 4"]
pub type MONTH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONTH5` reader - Real Time Clock Month Bit: 5"]
pub type MONTH5_R = crate::BitReader;
#[doc = "Field `MONTH5` writer - Real Time Clock Month Bit: 5"]
pub type MONTH5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONTH6` reader - Real Time Clock Month Bit: 6"]
pub type MONTH6_R = crate::BitReader;
#[doc = "Field `MONTH6` writer - Real Time Clock Month Bit: 6"]
pub type MONTH6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Real Time Clock Month Bit: 0"]
    #[inline(always)]
    pub fn month0(&self) -> MONTH0_R {
        MONTH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock Month Bit: 1"]
    #[inline(always)]
    pub fn month1(&self) -> MONTH1_R {
        MONTH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock Month Bit: 2"]
    #[inline(always)]
    pub fn month2(&self) -> MONTH2_R {
        MONTH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Month Bit: 3"]
    #[inline(always)]
    pub fn month3(&self) -> MONTH3_R {
        MONTH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock Month Bit: 4"]
    #[inline(always)]
    pub fn month4(&self) -> MONTH4_R {
        MONTH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock Month Bit: 5"]
    #[inline(always)]
    pub fn month5(&self) -> MONTH5_R {
        MONTH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock Month Bit: 6"]
    #[inline(always)]
    pub fn month6(&self) -> MONTH6_R {
        MONTH6_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Month Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn month0(&mut self) -> MONTH0_W<RTCMON_SPEC> {
        MONTH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Real Time Clock Month Bit: 1"]
    #[inline(always)]
    #[must_use]
    pub fn month1(&mut self) -> MONTH1_W<RTCMON_SPEC> {
        MONTH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Real Time Clock Month Bit: 2"]
    #[inline(always)]
    #[must_use]
    pub fn month2(&mut self) -> MONTH2_W<RTCMON_SPEC> {
        MONTH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Real Time Clock Month Bit: 3"]
    #[inline(always)]
    #[must_use]
    pub fn month3(&mut self) -> MONTH3_W<RTCMON_SPEC> {
        MONTH3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Real Time Clock Month Bit: 4"]
    #[inline(always)]
    #[must_use]
    pub fn month4(&mut self) -> MONTH4_W<RTCMON_SPEC> {
        MONTH4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Real Time Clock Month Bit: 5"]
    #[inline(always)]
    #[must_use]
    pub fn month5(&mut self) -> MONTH5_W<RTCMON_SPEC> {
        MONTH5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Real Time Clock Month Bit: 6"]
    #[inline(always)]
    #[must_use]
    pub fn month6(&mut self) -> MONTH6_W<RTCMON_SPEC> {
        MONTH6_W::new(self, 6)
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
#[doc = "Real Time Clock Month\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcmon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcmon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCMON_SPEC;
impl crate::RegisterSpec for RTCMON_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rtcmon::R`](R) reader structure"]
impl crate::Readable for RTCMON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtcmon::W`](W) writer structure"]
impl crate::Writable for RTCMON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCMON to value 0"]
impl crate::Resettable for RTCMON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
