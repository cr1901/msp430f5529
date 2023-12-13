#[doc = "Register `RTCCTL23` reader"]
pub type R = crate::R<RTCCTL23_SPEC>;
#[doc = "Register `RTCCTL23` writer"]
pub type W = crate::W<RTCCTL23_SPEC>;
#[doc = "Field `RTCCAL0` reader - RTC Calibration Bit 0"]
pub type RTCCAL0_R = crate::BitReader;
#[doc = "Field `RTCCAL0` writer - RTC Calibration Bit 0"]
pub type RTCCAL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCCAL1` reader - RTC Calibration Bit 1"]
pub type RTCCAL1_R = crate::BitReader;
#[doc = "Field `RTCCAL1` writer - RTC Calibration Bit 1"]
pub type RTCCAL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCCAL2` reader - RTC Calibration Bit 2"]
pub type RTCCAL2_R = crate::BitReader;
#[doc = "Field `RTCCAL2` writer - RTC Calibration Bit 2"]
pub type RTCCAL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCCAL3` reader - RTC Calibration Bit 3"]
pub type RTCCAL3_R = crate::BitReader;
#[doc = "Field `RTCCAL3` writer - RTC Calibration Bit 3"]
pub type RTCCAL3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCCAL4` reader - RTC Calibration Bit 4"]
pub type RTCCAL4_R = crate::BitReader;
#[doc = "Field `RTCCAL4` writer - RTC Calibration Bit 4"]
pub type RTCCAL4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCCAL5` reader - RTC Calibration Bit 5"]
pub type RTCCAL5_R = crate::BitReader;
#[doc = "Field `RTCCAL5` writer - RTC Calibration Bit 5"]
pub type RTCCAL5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCCALS` reader - RTC Calibration Sign"]
pub type RTCCALS_R = crate::BitReader;
#[doc = "Field `RTCCALS` writer - RTC Calibration Sign"]
pub type RTCCALS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCCALF` reader - RTC Calibration Frequency Bit 1"]
pub type RTCCALF_R = crate::FieldReader<RTCCALF_A>;
#[doc = "RTC Calibration Frequency Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCCALF_A {
    #[doc = "0: RTC Calibration Frequency: No Output"]
    RTCCALF_0 = 0,
    #[doc = "1: RTC Calibration Frequency: 512 Hz"]
    RTCCALF_1 = 1,
    #[doc = "2: RTC Calibration Frequency: 256 Hz"]
    RTCCALF_2 = 2,
    #[doc = "3: RTC Calibration Frequency: 1 Hz"]
    RTCCALF_3 = 3,
}
impl From<RTCCALF_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCCALF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTCCALF_A {
    type Ux = u8;
}
impl RTCCALF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTCCALF_A {
        match self.bits {
            0 => RTCCALF_A::RTCCALF_0,
            1 => RTCCALF_A::RTCCALF_1,
            2 => RTCCALF_A::RTCCALF_2,
            3 => RTCCALF_A::RTCCALF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "RTC Calibration Frequency: No Output"]
    #[inline(always)]
    pub fn is_rtccalf_0(&self) -> bool {
        *self == RTCCALF_A::RTCCALF_0
    }
    #[doc = "RTC Calibration Frequency: 512 Hz"]
    #[inline(always)]
    pub fn is_rtccalf_1(&self) -> bool {
        *self == RTCCALF_A::RTCCALF_1
    }
    #[doc = "RTC Calibration Frequency: 256 Hz"]
    #[inline(always)]
    pub fn is_rtccalf_2(&self) -> bool {
        *self == RTCCALF_A::RTCCALF_2
    }
    #[doc = "RTC Calibration Frequency: 1 Hz"]
    #[inline(always)]
    pub fn is_rtccalf_3(&self) -> bool {
        *self == RTCCALF_A::RTCCALF_3
    }
}
#[doc = "Field `RTCCALF` writer - RTC Calibration Frequency Bit 1"]
pub type RTCCALF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RTCCALF_A>;
impl<'a, REG> RTCCALF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RTC Calibration Frequency: No Output"]
    #[inline(always)]
    pub fn rtccalf_0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCCALF_A::RTCCALF_0)
    }
    #[doc = "RTC Calibration Frequency: 512 Hz"]
    #[inline(always)]
    pub fn rtccalf_1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCCALF_A::RTCCALF_1)
    }
    #[doc = "RTC Calibration Frequency: 256 Hz"]
    #[inline(always)]
    pub fn rtccalf_2(self) -> &'a mut crate::W<REG> {
        self.variant(RTCCALF_A::RTCCALF_2)
    }
    #[doc = "RTC Calibration Frequency: 1 Hz"]
    #[inline(always)]
    pub fn rtccalf_3(self) -> &'a mut crate::W<REG> {
        self.variant(RTCCALF_A::RTCCALF_3)
    }
}
impl R {
    #[doc = "Bit 0 - RTC Calibration Bit 0"]
    #[inline(always)]
    pub fn rtccal0(&self) -> RTCCAL0_R {
        RTCCAL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Calibration Bit 1"]
    #[inline(always)]
    pub fn rtccal1(&self) -> RTCCAL1_R {
        RTCCAL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Calibration Bit 2"]
    #[inline(always)]
    pub fn rtccal2(&self) -> RTCCAL2_R {
        RTCCAL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC Calibration Bit 3"]
    #[inline(always)]
    pub fn rtccal3(&self) -> RTCCAL3_R {
        RTCCAL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC Calibration Bit 4"]
    #[inline(always)]
    pub fn rtccal4(&self) -> RTCCAL4_R {
        RTCCAL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC Calibration Bit 5"]
    #[inline(always)]
    pub fn rtccal5(&self) -> RTCCAL5_R {
        RTCCAL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - RTC Calibration Sign"]
    #[inline(always)]
    pub fn rtccals(&self) -> RTCCALS_R {
        RTCCALS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC Calibration Frequency Bit 1"]
    #[inline(always)]
    pub fn rtccalf(&self) -> RTCCALF_R {
        RTCCALF_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Calibration Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rtccal0(&mut self) -> RTCCAL0_W<RTCCTL23_SPEC> {
        RTCCAL0_W::new(self, 0)
    }
    #[doc = "Bit 1 - RTC Calibration Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rtccal1(&mut self) -> RTCCAL1_W<RTCCTL23_SPEC> {
        RTCCAL1_W::new(self, 1)
    }
    #[doc = "Bit 2 - RTC Calibration Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rtccal2(&mut self) -> RTCCAL2_W<RTCCTL23_SPEC> {
        RTCCAL2_W::new(self, 2)
    }
    #[doc = "Bit 3 - RTC Calibration Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rtccal3(&mut self) -> RTCCAL3_W<RTCCTL23_SPEC> {
        RTCCAL3_W::new(self, 3)
    }
    #[doc = "Bit 4 - RTC Calibration Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rtccal4(&mut self) -> RTCCAL4_W<RTCCTL23_SPEC> {
        RTCCAL4_W::new(self, 4)
    }
    #[doc = "Bit 5 - RTC Calibration Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rtccal5(&mut self) -> RTCCAL5_W<RTCCTL23_SPEC> {
        RTCCAL5_W::new(self, 5)
    }
    #[doc = "Bit 7 - RTC Calibration Sign"]
    #[inline(always)]
    #[must_use]
    pub fn rtccals(&mut self) -> RTCCALS_W<RTCCTL23_SPEC> {
        RTCCALS_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - RTC Calibration Frequency Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rtccalf(&mut self) -> RTCCALF_W<RTCCTL23_SPEC> {
        RTCCALF_W::new(self, 8)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Real Timer Control 2/3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcctl23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcctl23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCCTL23_SPEC;
impl crate::RegisterSpec for RTCCTL23_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcctl23::R`](R) reader structure"]
impl crate::Readable for RTCCTL23_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtcctl23::W`](W) writer structure"]
impl crate::Writable for RTCCTL23_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCCTL23 to value 0"]
impl crate::Resettable for RTCCTL23_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
