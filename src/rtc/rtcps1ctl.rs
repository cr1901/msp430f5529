#[doc = "Register `RTCPS1CTL` reader"]
pub type R = crate::R<RTCPS1CTL_SPEC>;
#[doc = "Register `RTCPS1CTL` writer"]
pub type W = crate::W<RTCPS1CTL_SPEC>;
#[doc = "Field `RT1PSIFG` reader - RTC Prescale Timer 1 Interrupt Flag"]
pub type RT1PSIFG_R = crate::BitReader;
#[doc = "Field `RT1PSIFG` writer - RTC Prescale Timer 1 Interrupt Flag"]
pub type RT1PSIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT1PSIE` reader - RTC Prescale Timer 1 Interrupt Enable Flag"]
pub type RT1PSIE_R = crate::BitReader;
#[doc = "Field `RT1PSIE` writer - RTC Prescale Timer 1 Interrupt Enable Flag"]
pub type RT1PSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT1IP` reader - RTC Prescale Timer 1 Interrupt Interval Bit: 2"]
pub type RT1IP_R = crate::FieldReader<RT1IP_A>;
#[doc = "RTC Prescale Timer 1 Interrupt Interval Bit: 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RT1IP_A {
    #[doc = "0: RTC Prescale Timer 1 Interrupt Interval /2"]
    RT1IP_0 = 0,
    #[doc = "1: RTC Prescale Timer 1 Interrupt Interval /4"]
    RT1IP_1 = 1,
    #[doc = "2: RTC Prescale Timer 1 Interrupt Interval /8"]
    RT1IP_2 = 2,
    #[doc = "3: RTC Prescale Timer 1 Interrupt Interval /16"]
    RT1IP_3 = 3,
    #[doc = "4: RTC Prescale Timer 1 Interrupt Interval /32"]
    RT1IP_4 = 4,
    #[doc = "5: RTC Prescale Timer 1 Interrupt Interval /64"]
    RT1IP_5 = 5,
    #[doc = "6: RTC Prescale Timer 1 Interrupt Interval /128"]
    RT1IP_6 = 6,
    #[doc = "7: RTC Prescale Timer 1 Interrupt Interval /256"]
    RT1IP_7 = 7,
}
impl From<RT1IP_A> for u8 {
    #[inline(always)]
    fn from(variant: RT1IP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RT1IP_A {
    type Ux = u8;
}
impl RT1IP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT1IP_A {
        match self.bits {
            0 => RT1IP_A::RT1IP_0,
            1 => RT1IP_A::RT1IP_1,
            2 => RT1IP_A::RT1IP_2,
            3 => RT1IP_A::RT1IP_3,
            4 => RT1IP_A::RT1IP_4,
            5 => RT1IP_A::RT1IP_5,
            6 => RT1IP_A::RT1IP_6,
            7 => RT1IP_A::RT1IP_7,
            _ => unreachable!(),
        }
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /2"]
    #[inline(always)]
    pub fn is_rt1ip_0(&self) -> bool {
        *self == RT1IP_A::RT1IP_0
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /4"]
    #[inline(always)]
    pub fn is_rt1ip_1(&self) -> bool {
        *self == RT1IP_A::RT1IP_1
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /8"]
    #[inline(always)]
    pub fn is_rt1ip_2(&self) -> bool {
        *self == RT1IP_A::RT1IP_2
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /16"]
    #[inline(always)]
    pub fn is_rt1ip_3(&self) -> bool {
        *self == RT1IP_A::RT1IP_3
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /32"]
    #[inline(always)]
    pub fn is_rt1ip_4(&self) -> bool {
        *self == RT1IP_A::RT1IP_4
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /64"]
    #[inline(always)]
    pub fn is_rt1ip_5(&self) -> bool {
        *self == RT1IP_A::RT1IP_5
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /128"]
    #[inline(always)]
    pub fn is_rt1ip_6(&self) -> bool {
        *self == RT1IP_A::RT1IP_6
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /256"]
    #[inline(always)]
    pub fn is_rt1ip_7(&self) -> bool {
        *self == RT1IP_A::RT1IP_7
    }
}
#[doc = "Field `RT1IP` writer - RTC Prescale Timer 1 Interrupt Interval Bit: 2"]
pub type RT1IP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, RT1IP_A>;
impl<'a, REG> RT1IP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /2"]
    #[inline(always)]
    pub fn rt1ip_0(self) -> &'a mut crate::W<REG> {
        self.variant(RT1IP_A::RT1IP_0)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /4"]
    #[inline(always)]
    pub fn rt1ip_1(self) -> &'a mut crate::W<REG> {
        self.variant(RT1IP_A::RT1IP_1)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /8"]
    #[inline(always)]
    pub fn rt1ip_2(self) -> &'a mut crate::W<REG> {
        self.variant(RT1IP_A::RT1IP_2)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /16"]
    #[inline(always)]
    pub fn rt1ip_3(self) -> &'a mut crate::W<REG> {
        self.variant(RT1IP_A::RT1IP_3)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /32"]
    #[inline(always)]
    pub fn rt1ip_4(self) -> &'a mut crate::W<REG> {
        self.variant(RT1IP_A::RT1IP_4)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /64"]
    #[inline(always)]
    pub fn rt1ip_5(self) -> &'a mut crate::W<REG> {
        self.variant(RT1IP_A::RT1IP_5)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /128"]
    #[inline(always)]
    pub fn rt1ip_6(self) -> &'a mut crate::W<REG> {
        self.variant(RT1IP_A::RT1IP_6)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /256"]
    #[inline(always)]
    pub fn rt1ip_7(self) -> &'a mut crate::W<REG> {
        self.variant(RT1IP_A::RT1IP_7)
    }
}
#[doc = "Field `RT1PSHOLD` reader - RTC Prescale Timer 1 Hold"]
pub type RT1PSHOLD_R = crate::BitReader;
#[doc = "Field `RT1PSHOLD` writer - RTC Prescale Timer 1 Hold"]
pub type RT1PSHOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT1PSDIV` reader - RTC Prescale Timer 1 Clock Divide Bit: 2"]
pub type RT1PSDIV_R = crate::FieldReader<RT1PSDIV_A>;
#[doc = "RTC Prescale Timer 1 Clock Divide Bit: 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RT1PSDIV_A {
    #[doc = "0: RTC Prescale Timer 1 Clock Divide /2"]
    RT1PSDIV_0 = 0,
    #[doc = "1: RTC Prescale Timer 1 Clock Divide /4"]
    RT1PSDIV_1 = 1,
    #[doc = "2: RTC Prescale Timer 1 Clock Divide /8"]
    RT1PSDIV_2 = 2,
    #[doc = "3: RTC Prescale Timer 1 Clock Divide /16"]
    RT1PSDIV_3 = 3,
    #[doc = "4: RTC Prescale Timer 1 Clock Divide /32"]
    RT1PSDIV_4 = 4,
    #[doc = "5: RTC Prescale Timer 1 Clock Divide /64"]
    RT1PSDIV_5 = 5,
    #[doc = "6: RTC Prescale Timer 1 Clock Divide /128"]
    RT1PSDIV_6 = 6,
    #[doc = "7: RTC Prescale Timer 1 Clock Divide /256"]
    RT1PSDIV_7 = 7,
}
impl From<RT1PSDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: RT1PSDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RT1PSDIV_A {
    type Ux = u8;
}
impl RT1PSDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT1PSDIV_A {
        match self.bits {
            0 => RT1PSDIV_A::RT1PSDIV_0,
            1 => RT1PSDIV_A::RT1PSDIV_1,
            2 => RT1PSDIV_A::RT1PSDIV_2,
            3 => RT1PSDIV_A::RT1PSDIV_3,
            4 => RT1PSDIV_A::RT1PSDIV_4,
            5 => RT1PSDIV_A::RT1PSDIV_5,
            6 => RT1PSDIV_A::RT1PSDIV_6,
            7 => RT1PSDIV_A::RT1PSDIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /2"]
    #[inline(always)]
    pub fn is_rt1psdiv_0(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_0
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /4"]
    #[inline(always)]
    pub fn is_rt1psdiv_1(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_1
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /8"]
    #[inline(always)]
    pub fn is_rt1psdiv_2(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_2
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /16"]
    #[inline(always)]
    pub fn is_rt1psdiv_3(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_3
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /32"]
    #[inline(always)]
    pub fn is_rt1psdiv_4(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_4
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /64"]
    #[inline(always)]
    pub fn is_rt1psdiv_5(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_5
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /128"]
    #[inline(always)]
    pub fn is_rt1psdiv_6(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_6
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /256"]
    #[inline(always)]
    pub fn is_rt1psdiv_7(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_7
    }
}
#[doc = "Field `RT1PSDIV` writer - RTC Prescale Timer 1 Clock Divide Bit: 2"]
pub type RT1PSDIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, RT1PSDIV_A>;
impl<'a, REG> RT1PSDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RTC Prescale Timer 1 Clock Divide /2"]
    #[inline(always)]
    pub fn rt1psdiv_0(self) -> &'a mut crate::W<REG> {
        self.variant(RT1PSDIV_A::RT1PSDIV_0)
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /4"]
    #[inline(always)]
    pub fn rt1psdiv_1(self) -> &'a mut crate::W<REG> {
        self.variant(RT1PSDIV_A::RT1PSDIV_1)
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /8"]
    #[inline(always)]
    pub fn rt1psdiv_2(self) -> &'a mut crate::W<REG> {
        self.variant(RT1PSDIV_A::RT1PSDIV_2)
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /16"]
    #[inline(always)]
    pub fn rt1psdiv_3(self) -> &'a mut crate::W<REG> {
        self.variant(RT1PSDIV_A::RT1PSDIV_3)
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /32"]
    #[inline(always)]
    pub fn rt1psdiv_4(self) -> &'a mut crate::W<REG> {
        self.variant(RT1PSDIV_A::RT1PSDIV_4)
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /64"]
    #[inline(always)]
    pub fn rt1psdiv_5(self) -> &'a mut crate::W<REG> {
        self.variant(RT1PSDIV_A::RT1PSDIV_5)
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /128"]
    #[inline(always)]
    pub fn rt1psdiv_6(self) -> &'a mut crate::W<REG> {
        self.variant(RT1PSDIV_A::RT1PSDIV_6)
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /256"]
    #[inline(always)]
    pub fn rt1psdiv_7(self) -> &'a mut crate::W<REG> {
        self.variant(RT1PSDIV_A::RT1PSDIV_7)
    }
}
#[doc = "Field `RT1SSEL` reader - RTC Prescale Timer 1 Source Select Bit 1"]
pub type RT1SSEL_R = crate::FieldReader<RT1SSEL_A>;
#[doc = "RTC Prescale Timer 1 Source Select Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RT1SSEL_A {
    #[doc = "0: RTC Prescale Timer Source Select ACLK"]
    RT1SSEL_0 = 0,
    #[doc = "1: RTC Prescale Timer Source Select SMCLK"]
    RT1SSEL_1 = 1,
    #[doc = "2: RTC Prescale Timer Source Select RT0PS"]
    RT1SSEL_2 = 2,
    #[doc = "3: RTC Prescale Timer Source Select RT0PS"]
    RT1SSEL_3 = 3,
}
impl From<RT1SSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RT1SSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RT1SSEL_A {
    type Ux = u8;
}
impl RT1SSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT1SSEL_A {
        match self.bits {
            0 => RT1SSEL_A::RT1SSEL_0,
            1 => RT1SSEL_A::RT1SSEL_1,
            2 => RT1SSEL_A::RT1SSEL_2,
            3 => RT1SSEL_A::RT1SSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "RTC Prescale Timer Source Select ACLK"]
    #[inline(always)]
    pub fn is_rt1ssel_0(&self) -> bool {
        *self == RT1SSEL_A::RT1SSEL_0
    }
    #[doc = "RTC Prescale Timer Source Select SMCLK"]
    #[inline(always)]
    pub fn is_rt1ssel_1(&self) -> bool {
        *self == RT1SSEL_A::RT1SSEL_1
    }
    #[doc = "RTC Prescale Timer Source Select RT0PS"]
    #[inline(always)]
    pub fn is_rt1ssel_2(&self) -> bool {
        *self == RT1SSEL_A::RT1SSEL_2
    }
    #[doc = "RTC Prescale Timer Source Select RT0PS"]
    #[inline(always)]
    pub fn is_rt1ssel_3(&self) -> bool {
        *self == RT1SSEL_A::RT1SSEL_3
    }
}
#[doc = "Field `RT1SSEL` writer - RTC Prescale Timer 1 Source Select Bit 1"]
pub type RT1SSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RT1SSEL_A>;
impl<'a, REG> RT1SSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RTC Prescale Timer Source Select ACLK"]
    #[inline(always)]
    pub fn rt1ssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(RT1SSEL_A::RT1SSEL_0)
    }
    #[doc = "RTC Prescale Timer Source Select SMCLK"]
    #[inline(always)]
    pub fn rt1ssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(RT1SSEL_A::RT1SSEL_1)
    }
    #[doc = "RTC Prescale Timer Source Select RT0PS"]
    #[inline(always)]
    pub fn rt1ssel_2(self) -> &'a mut crate::W<REG> {
        self.variant(RT1SSEL_A::RT1SSEL_2)
    }
    #[doc = "RTC Prescale Timer Source Select RT0PS"]
    #[inline(always)]
    pub fn rt1ssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(RT1SSEL_A::RT1SSEL_3)
    }
}
impl R {
    #[doc = "Bit 0 - RTC Prescale Timer 1 Interrupt Flag"]
    #[inline(always)]
    pub fn rt1psifg(&self) -> RT1PSIFG_R {
        RT1PSIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Prescale Timer 1 Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rt1psie(&self) -> RT1PSIE_R {
        RT1PSIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - RTC Prescale Timer 1 Interrupt Interval Bit: 2"]
    #[inline(always)]
    pub fn rt1ip(&self) -> RT1IP_R {
        RT1IP_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 8 - RTC Prescale Timer 1 Hold"]
    #[inline(always)]
    pub fn rt1pshold(&self) -> RT1PSHOLD_R {
        RT1PSHOLD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 11:13 - RTC Prescale Timer 1 Clock Divide Bit: 2"]
    #[inline(always)]
    pub fn rt1psdiv(&self) -> RT1PSDIV_R {
        RT1PSDIV_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - RTC Prescale Timer 1 Source Select Bit 1"]
    #[inline(always)]
    pub fn rt1ssel(&self) -> RT1SSEL_R {
        RT1SSEL_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Prescale Timer 1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rt1psifg(&mut self) -> RT1PSIFG_W<RTCPS1CTL_SPEC> {
        RT1PSIFG_W::new(self, 0)
    }
    #[doc = "Bit 1 - RTC Prescale Timer 1 Interrupt Enable Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rt1psie(&mut self) -> RT1PSIE_W<RTCPS1CTL_SPEC> {
        RT1PSIE_W::new(self, 1)
    }
    #[doc = "Bits 2:4 - RTC Prescale Timer 1 Interrupt Interval Bit: 2"]
    #[inline(always)]
    #[must_use]
    pub fn rt1ip(&mut self) -> RT1IP_W<RTCPS1CTL_SPEC> {
        RT1IP_W::new(self, 2)
    }
    #[doc = "Bit 8 - RTC Prescale Timer 1 Hold"]
    #[inline(always)]
    #[must_use]
    pub fn rt1pshold(&mut self) -> RT1PSHOLD_W<RTCPS1CTL_SPEC> {
        RT1PSHOLD_W::new(self, 8)
    }
    #[doc = "Bits 11:13 - RTC Prescale Timer 1 Clock Divide Bit: 2"]
    #[inline(always)]
    #[must_use]
    pub fn rt1psdiv(&mut self) -> RT1PSDIV_W<RTCPS1CTL_SPEC> {
        RT1PSDIV_W::new(self, 11)
    }
    #[doc = "Bits 14:15 - RTC Prescale Timer 1 Source Select Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rt1ssel(&mut self) -> RT1SSEL_W<RTCPS1CTL_SPEC> {
        RT1SSEL_W::new(self, 14)
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
#[doc = "Real Timer Prescale Timer 1 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcps1ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcps1ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCPS1CTL_SPEC;
impl crate::RegisterSpec for RTCPS1CTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcps1ctl::R`](R) reader structure"]
impl crate::Readable for RTCPS1CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtcps1ctl::W`](W) writer structure"]
impl crate::Writable for RTCPS1CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCPS1CTL to value 0"]
impl crate::Resettable for RTCPS1CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
