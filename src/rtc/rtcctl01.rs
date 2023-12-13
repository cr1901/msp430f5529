#[doc = "Register `RTCCTL01` reader"]
pub type R = crate::R<RTCCTL01_SPEC>;
#[doc = "Register `RTCCTL01` writer"]
pub type W = crate::W<RTCCTL01_SPEC>;
#[doc = "Field `RTCRDYIFG` reader - RTC Ready Interrupt Flag"]
pub type RTCRDYIFG_R = crate::BitReader;
#[doc = "Field `RTCRDYIFG` writer - RTC Ready Interrupt Flag"]
pub type RTCRDYIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAIFG` reader - RTC Alarm Interrupt Flag"]
pub type RTCAIFG_R = crate::BitReader;
#[doc = "Field `RTCAIFG` writer - RTC Alarm Interrupt Flag"]
pub type RTCAIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCTEVIFG` reader - RTC Time Event Interrupt Flag"]
pub type RTCTEVIFG_R = crate::BitReader;
#[doc = "Field `RTCTEVIFG` writer - RTC Time Event Interrupt Flag"]
pub type RTCTEVIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCRDYIE` reader - RTC Ready Interrupt Enable Flag"]
pub type RTCRDYIE_R = crate::BitReader;
#[doc = "Field `RTCRDYIE` writer - RTC Ready Interrupt Enable Flag"]
pub type RTCRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAIE` reader - RTC Alarm Interrupt Enable Flag"]
pub type RTCAIE_R = crate::BitReader;
#[doc = "Field `RTCAIE` writer - RTC Alarm Interrupt Enable Flag"]
pub type RTCAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCTEVIE` reader - RTC Time Event Interrupt Enable Flag"]
pub type RTCTEVIE_R = crate::BitReader;
#[doc = "Field `RTCTEVIE` writer - RTC Time Event Interrupt Enable Flag"]
pub type RTCTEVIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCTEV` reader - RTC Time Event 1"]
pub type RTCTEV_R = crate::FieldReader<RTCTEV_A>;
#[doc = "RTC Time Event 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCTEV_A {
    #[doc = "0: RTC Time Event: 0 (Min. changed)"]
    RTCTEV_0 = 0,
    #[doc = "1: RTC Time Event: 1 (Hour changed)"]
    RTCTEV_1 = 1,
    #[doc = "2: RTC Time Event: 2 (12:00 changed)"]
    RTCTEV_2 = 2,
    #[doc = "3: RTC Time Event: 3 (00:00 changed)"]
    RTCTEV_3 = 3,
}
impl From<RTCTEV_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCTEV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTCTEV_A {
    type Ux = u8;
}
impl RTCTEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTCTEV_A {
        match self.bits {
            0 => RTCTEV_A::RTCTEV_0,
            1 => RTCTEV_A::RTCTEV_1,
            2 => RTCTEV_A::RTCTEV_2,
            3 => RTCTEV_A::RTCTEV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "RTC Time Event: 0 (Min. changed)"]
    #[inline(always)]
    pub fn is_rtctev_0(&self) -> bool {
        *self == RTCTEV_A::RTCTEV_0
    }
    #[doc = "RTC Time Event: 1 (Hour changed)"]
    #[inline(always)]
    pub fn is_rtctev_1(&self) -> bool {
        *self == RTCTEV_A::RTCTEV_1
    }
    #[doc = "RTC Time Event: 2 (12:00 changed)"]
    #[inline(always)]
    pub fn is_rtctev_2(&self) -> bool {
        *self == RTCTEV_A::RTCTEV_2
    }
    #[doc = "RTC Time Event: 3 (00:00 changed)"]
    #[inline(always)]
    pub fn is_rtctev_3(&self) -> bool {
        *self == RTCTEV_A::RTCTEV_3
    }
}
#[doc = "Field `RTCTEV` writer - RTC Time Event 1"]
pub type RTCTEV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RTCTEV_A>;
impl<'a, REG> RTCTEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RTC Time Event: 0 (Min. changed)"]
    #[inline(always)]
    pub fn rtctev_0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCTEV_A::RTCTEV_0)
    }
    #[doc = "RTC Time Event: 1 (Hour changed)"]
    #[inline(always)]
    pub fn rtctev_1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCTEV_A::RTCTEV_1)
    }
    #[doc = "RTC Time Event: 2 (12:00 changed)"]
    #[inline(always)]
    pub fn rtctev_2(self) -> &'a mut crate::W<REG> {
        self.variant(RTCTEV_A::RTCTEV_2)
    }
    #[doc = "RTC Time Event: 3 (00:00 changed)"]
    #[inline(always)]
    pub fn rtctev_3(self) -> &'a mut crate::W<REG> {
        self.variant(RTCTEV_A::RTCTEV_3)
    }
}
#[doc = "Field `RTCSSEL` reader - RTC Source Select 1"]
pub type RTCSSEL_R = crate::FieldReader<RTCSSEL_A>;
#[doc = "RTC Source Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCSSEL_A {
    #[doc = "0: RTC Source Select ACLK"]
    RTCSSEL_0 = 0,
    #[doc = "1: RTC Source Select SMCLK"]
    RTCSSEL_1 = 1,
    #[doc = "2: RTC Source Select RT1PS"]
    RTCSSEL_2 = 2,
    #[doc = "3: RTC Source Select RT1PS"]
    RTCSSEL_3 = 3,
}
impl From<RTCSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTCSSEL_A {
    type Ux = u8;
}
impl RTCSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTCSSEL_A {
        match self.bits {
            0 => RTCSSEL_A::RTCSSEL_0,
            1 => RTCSSEL_A::RTCSSEL_1,
            2 => RTCSSEL_A::RTCSSEL_2,
            3 => RTCSSEL_A::RTCSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "RTC Source Select ACLK"]
    #[inline(always)]
    pub fn is_rtcssel_0(&self) -> bool {
        *self == RTCSSEL_A::RTCSSEL_0
    }
    #[doc = "RTC Source Select SMCLK"]
    #[inline(always)]
    pub fn is_rtcssel_1(&self) -> bool {
        *self == RTCSSEL_A::RTCSSEL_1
    }
    #[doc = "RTC Source Select RT1PS"]
    #[inline(always)]
    pub fn is_rtcssel_2(&self) -> bool {
        *self == RTCSSEL_A::RTCSSEL_2
    }
    #[doc = "RTC Source Select RT1PS"]
    #[inline(always)]
    pub fn is_rtcssel_3(&self) -> bool {
        *self == RTCSSEL_A::RTCSSEL_3
    }
}
#[doc = "Field `RTCSSEL` writer - RTC Source Select 1"]
pub type RTCSSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RTCSSEL_A>;
impl<'a, REG> RTCSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RTC Source Select ACLK"]
    #[inline(always)]
    pub fn rtcssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSSEL_A::RTCSSEL_0)
    }
    #[doc = "RTC Source Select SMCLK"]
    #[inline(always)]
    pub fn rtcssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSSEL_A::RTCSSEL_1)
    }
    #[doc = "RTC Source Select RT1PS"]
    #[inline(always)]
    pub fn rtcssel_2(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSSEL_A::RTCSSEL_2)
    }
    #[doc = "RTC Source Select RT1PS"]
    #[inline(always)]
    pub fn rtcssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSSEL_A::RTCSSEL_3)
    }
}
#[doc = "Field `RTCRDY` reader - RTC Ready"]
pub type RTCRDY_R = crate::BitReader;
#[doc = "Field `RTCRDY` writer - RTC Ready"]
pub type RTCRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCMODE` reader - RTC Mode 0:Counter / 1: Calendar"]
pub type RTCMODE_R = crate::BitReader;
#[doc = "Field `RTCMODE` writer - RTC Mode 0:Counter / 1: Calendar"]
pub type RTCMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCHOLD` reader - RTC Hold"]
pub type RTCHOLD_R = crate::BitReader;
#[doc = "Field `RTCHOLD` writer - RTC Hold"]
pub type RTCHOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCBCD` reader - RTC BCD 0:Binary / 1:BCD"]
pub type RTCBCD_R = crate::BitReader;
#[doc = "Field `RTCBCD` writer - RTC BCD 0:Binary / 1:BCD"]
pub type RTCBCD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RTC Ready Interrupt Flag"]
    #[inline(always)]
    pub fn rtcrdyifg(&self) -> RTCRDYIFG_R {
        RTCRDYIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Alarm Interrupt Flag"]
    #[inline(always)]
    pub fn rtcaifg(&self) -> RTCAIFG_R {
        RTCAIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Time Event Interrupt Flag"]
    #[inline(always)]
    pub fn rtctevifg(&self) -> RTCTEVIFG_R {
        RTCTEVIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC Ready Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rtcrdyie(&self) -> RTCRDYIE_R {
        RTCRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC Alarm Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rtcaie(&self) -> RTCAIE_R {
        RTCAIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTC Time Event Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rtctevie(&self) -> RTCTEVIE_R {
        RTCTEVIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC Time Event 1"]
    #[inline(always)]
    pub fn rtctev(&self) -> RTCTEV_R {
        RTCTEV_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - RTC Source Select 1"]
    #[inline(always)]
    pub fn rtcssel(&self) -> RTCSSEL_R {
        RTCSSEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - RTC Ready"]
    #[inline(always)]
    pub fn rtcrdy(&self) -> RTCRDY_R {
        RTCRDY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RTC Mode 0:Counter / 1: Calendar"]
    #[inline(always)]
    pub fn rtcmode(&self) -> RTCMODE_R {
        RTCMODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RTC Hold"]
    #[inline(always)]
    pub fn rtchold(&self) -> RTCHOLD_R {
        RTCHOLD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC BCD 0:Binary / 1:BCD"]
    #[inline(always)]
    pub fn rtcbcd(&self) -> RTCBCD_R {
        RTCBCD_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Ready Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtcrdyifg(&mut self) -> RTCRDYIFG_W<RTCCTL01_SPEC> {
        RTCRDYIFG_W::new(self, 0)
    }
    #[doc = "Bit 1 - RTC Alarm Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtcaifg(&mut self) -> RTCAIFG_W<RTCCTL01_SPEC> {
        RTCAIFG_W::new(self, 1)
    }
    #[doc = "Bit 2 - RTC Time Event Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtctevifg(&mut self) -> RTCTEVIFG_W<RTCCTL01_SPEC> {
        RTCTEVIFG_W::new(self, 2)
    }
    #[doc = "Bit 4 - RTC Ready Interrupt Enable Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtcrdyie(&mut self) -> RTCRDYIE_W<RTCCTL01_SPEC> {
        RTCRDYIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - RTC Alarm Interrupt Enable Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtcaie(&mut self) -> RTCAIE_W<RTCCTL01_SPEC> {
        RTCAIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - RTC Time Event Interrupt Enable Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtctevie(&mut self) -> RTCTEVIE_W<RTCCTL01_SPEC> {
        RTCTEVIE_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - RTC Time Event 1"]
    #[inline(always)]
    #[must_use]
    pub fn rtctev(&mut self) -> RTCTEV_W<RTCCTL01_SPEC> {
        RTCTEV_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - RTC Source Select 1"]
    #[inline(always)]
    #[must_use]
    pub fn rtcssel(&mut self) -> RTCSSEL_W<RTCCTL01_SPEC> {
        RTCSSEL_W::new(self, 10)
    }
    #[doc = "Bit 12 - RTC Ready"]
    #[inline(always)]
    #[must_use]
    pub fn rtcrdy(&mut self) -> RTCRDY_W<RTCCTL01_SPEC> {
        RTCRDY_W::new(self, 12)
    }
    #[doc = "Bit 13 - RTC Mode 0:Counter / 1: Calendar"]
    #[inline(always)]
    #[must_use]
    pub fn rtcmode(&mut self) -> RTCMODE_W<RTCCTL01_SPEC> {
        RTCMODE_W::new(self, 13)
    }
    #[doc = "Bit 14 - RTC Hold"]
    #[inline(always)]
    #[must_use]
    pub fn rtchold(&mut self) -> RTCHOLD_W<RTCCTL01_SPEC> {
        RTCHOLD_W::new(self, 14)
    }
    #[doc = "Bit 15 - RTC BCD 0:Binary / 1:BCD"]
    #[inline(always)]
    #[must_use]
    pub fn rtcbcd(&mut self) -> RTCBCD_W<RTCCTL01_SPEC> {
        RTCBCD_W::new(self, 15)
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
#[doc = "Real Timer Control 0/1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcctl01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcctl01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCCTL01_SPEC;
impl crate::RegisterSpec for RTCCTL01_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcctl01::R`](R) reader structure"]
impl crate::Readable for RTCCTL01_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtcctl01::W`](W) writer structure"]
impl crate::Writable for RTCCTL01_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCCTL01 to value 0"]
impl crate::Resettable for RTCCTL01_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
