#[doc = "Reader of register RTCPS1CTL"]
pub type R = crate::R<u16, super::RTCPS1CTL>;
#[doc = "Writer for register RTCPS1CTL"]
pub type W = crate::W<u16, super::RTCPS1CTL>;
#[doc = "Register RTCPS1CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCPS1CTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RT1PSIFG`"]
pub type RT1PSIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RT1PSIFG`"]
pub struct RT1PSIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RT1PSIFG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `RT1PSIE`"]
pub type RT1PSIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RT1PSIE`"]
pub struct RT1PSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RT1PSIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "RTC Prescale Timer 1 Interrupt Interval Bit: 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `RT1IP`"]
pub type RT1IP_R = crate::R<u8, RT1IP_A>;
impl RT1IP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT1IP_A {
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
    #[doc = "Checks if the value of the field is `RT1IP_0`"]
    #[inline(always)]
    pub fn is_rt1ip_0(&self) -> bool {
        *self == RT1IP_A::RT1IP_0
    }
    #[doc = "Checks if the value of the field is `RT1IP_1`"]
    #[inline(always)]
    pub fn is_rt1ip_1(&self) -> bool {
        *self == RT1IP_A::RT1IP_1
    }
    #[doc = "Checks if the value of the field is `RT1IP_2`"]
    #[inline(always)]
    pub fn is_rt1ip_2(&self) -> bool {
        *self == RT1IP_A::RT1IP_2
    }
    #[doc = "Checks if the value of the field is `RT1IP_3`"]
    #[inline(always)]
    pub fn is_rt1ip_3(&self) -> bool {
        *self == RT1IP_A::RT1IP_3
    }
    #[doc = "Checks if the value of the field is `RT1IP_4`"]
    #[inline(always)]
    pub fn is_rt1ip_4(&self) -> bool {
        *self == RT1IP_A::RT1IP_4
    }
    #[doc = "Checks if the value of the field is `RT1IP_5`"]
    #[inline(always)]
    pub fn is_rt1ip_5(&self) -> bool {
        *self == RT1IP_A::RT1IP_5
    }
    #[doc = "Checks if the value of the field is `RT1IP_6`"]
    #[inline(always)]
    pub fn is_rt1ip_6(&self) -> bool {
        *self == RT1IP_A::RT1IP_6
    }
    #[doc = "Checks if the value of the field is `RT1IP_7`"]
    #[inline(always)]
    pub fn is_rt1ip_7(&self) -> bool {
        *self == RT1IP_A::RT1IP_7
    }
}
#[doc = "Write proxy for field `RT1IP`"]
pub struct RT1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> RT1IP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT1IP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /2"]
    #[inline(always)]
    pub fn rt1ip_0(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_0)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /4"]
    #[inline(always)]
    pub fn rt1ip_1(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_1)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /8"]
    #[inline(always)]
    pub fn rt1ip_2(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_2)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /16"]
    #[inline(always)]
    pub fn rt1ip_3(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_3)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /32"]
    #[inline(always)]
    pub fn rt1ip_4(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_4)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /64"]
    #[inline(always)]
    pub fn rt1ip_5(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_5)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /128"]
    #[inline(always)]
    pub fn rt1ip_6(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_6)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /256"]
    #[inline(always)]
    pub fn rt1ip_7(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u16) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `RT1PSHOLD`"]
pub type RT1PSHOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RT1PSHOLD`"]
pub struct RT1PSHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RT1PSHOLD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "RTC Prescale Timer 1 Clock Divide Bit: 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `RT1PSDIV`"]
pub type RT1PSDIV_R = crate::R<u8, RT1PSDIV_A>;
impl RT1PSDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT1PSDIV_A {
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
    #[doc = "Checks if the value of the field is `RT1PSDIV_0`"]
    #[inline(always)]
    pub fn is_rt1psdiv_0(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_0
    }
    #[doc = "Checks if the value of the field is `RT1PSDIV_1`"]
    #[inline(always)]
    pub fn is_rt1psdiv_1(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_1
    }
    #[doc = "Checks if the value of the field is `RT1PSDIV_2`"]
    #[inline(always)]
    pub fn is_rt1psdiv_2(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_2
    }
    #[doc = "Checks if the value of the field is `RT1PSDIV_3`"]
    #[inline(always)]
    pub fn is_rt1psdiv_3(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_3
    }
    #[doc = "Checks if the value of the field is `RT1PSDIV_4`"]
    #[inline(always)]
    pub fn is_rt1psdiv_4(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_4
    }
    #[doc = "Checks if the value of the field is `RT1PSDIV_5`"]
    #[inline(always)]
    pub fn is_rt1psdiv_5(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_5
    }
    #[doc = "Checks if the value of the field is `RT1PSDIV_6`"]
    #[inline(always)]
    pub fn is_rt1psdiv_6(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_6
    }
    #[doc = "Checks if the value of the field is `RT1PSDIV_7`"]
    #[inline(always)]
    pub fn is_rt1psdiv_7(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_7
    }
}
#[doc = "Write proxy for field `RT1PSDIV`"]
pub struct RT1PSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RT1PSDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT1PSDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /2"]
    #[inline(always)]
    pub fn rt1psdiv_0(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::RT1PSDIV_0)
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /4"]
    #[inline(always)]
    pub fn rt1psdiv_1(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::RT1PSDIV_1)
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /8"]
    #[inline(always)]
    pub fn rt1psdiv_2(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::RT1PSDIV_2)
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /16"]
    #[inline(always)]
    pub fn rt1psdiv_3(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::RT1PSDIV_3)
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /32"]
    #[inline(always)]
    pub fn rt1psdiv_4(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::RT1PSDIV_4)
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /64"]
    #[inline(always)]
    pub fn rt1psdiv_5(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::RT1PSDIV_5)
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /128"]
    #[inline(always)]
    pub fn rt1psdiv_6(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::RT1PSDIV_6)
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /256"]
    #[inline(always)]
    pub fn rt1psdiv_7(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::RT1PSDIV_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u16) & 0x07) << 11);
        self.w
    }
}
#[doc = "RTC Prescale Timer 1 Source Select Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `RT1SSEL`"]
pub type RT1SSEL_R = crate::R<u8, RT1SSEL_A>;
impl RT1SSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT1SSEL_A {
        match self.bits {
            0 => RT1SSEL_A::RT1SSEL_0,
            1 => RT1SSEL_A::RT1SSEL_1,
            2 => RT1SSEL_A::RT1SSEL_2,
            3 => RT1SSEL_A::RT1SSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RT1SSEL_0`"]
    #[inline(always)]
    pub fn is_rt1ssel_0(&self) -> bool {
        *self == RT1SSEL_A::RT1SSEL_0
    }
    #[doc = "Checks if the value of the field is `RT1SSEL_1`"]
    #[inline(always)]
    pub fn is_rt1ssel_1(&self) -> bool {
        *self == RT1SSEL_A::RT1SSEL_1
    }
    #[doc = "Checks if the value of the field is `RT1SSEL_2`"]
    #[inline(always)]
    pub fn is_rt1ssel_2(&self) -> bool {
        *self == RT1SSEL_A::RT1SSEL_2
    }
    #[doc = "Checks if the value of the field is `RT1SSEL_3`"]
    #[inline(always)]
    pub fn is_rt1ssel_3(&self) -> bool {
        *self == RT1SSEL_A::RT1SSEL_3
    }
}
#[doc = "Write proxy for field `RT1SSEL`"]
pub struct RT1SSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RT1SSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT1SSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "RTC Prescale Timer Source Select ACLK"]
    #[inline(always)]
    pub fn rt1ssel_0(self) -> &'a mut W {
        self.variant(RT1SSEL_A::RT1SSEL_0)
    }
    #[doc = "RTC Prescale Timer Source Select SMCLK"]
    #[inline(always)]
    pub fn rt1ssel_1(self) -> &'a mut W {
        self.variant(RT1SSEL_A::RT1SSEL_1)
    }
    #[doc = "RTC Prescale Timer Source Select RT0PS"]
    #[inline(always)]
    pub fn rt1ssel_2(self) -> &'a mut W {
        self.variant(RT1SSEL_A::RT1SSEL_2)
    }
    #[doc = "RTC Prescale Timer Source Select RT0PS"]
    #[inline(always)]
    pub fn rt1ssel_3(self) -> &'a mut W {
        self.variant(RT1SSEL_A::RT1SSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u16) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RTC Prescale Timer 1 Interrupt Flag"]
    #[inline(always)]
    pub fn rt1psifg(&self) -> RT1PSIFG_R {
        RT1PSIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC Prescale Timer 1 Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rt1psie(&self) -> RT1PSIE_R {
        RT1PSIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - RTC Prescale Timer 1 Interrupt Interval Bit: 2"]
    #[inline(always)]
    pub fn rt1ip(&self) -> RT1IP_R {
        RT1IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 8 - RTC Prescale Timer 1 Hold"]
    #[inline(always)]
    pub fn rt1pshold(&self) -> RT1PSHOLD_R {
        RT1PSHOLD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - RTC Prescale Timer 1 Clock Divide Bit: 2"]
    #[inline(always)]
    pub fn rt1psdiv(&self) -> RT1PSDIV_R {
        RT1PSDIV_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 14:15 - RTC Prescale Timer 1 Source Select Bit 1"]
    #[inline(always)]
    pub fn rt1ssel(&self) -> RT1SSEL_R {
        RT1SSEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Prescale Timer 1 Interrupt Flag"]
    #[inline(always)]
    pub fn rt1psifg(&mut self) -> RT1PSIFG_W {
        RT1PSIFG_W { w: self }
    }
    #[doc = "Bit 1 - RTC Prescale Timer 1 Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rt1psie(&mut self) -> RT1PSIE_W {
        RT1PSIE_W { w: self }
    }
    #[doc = "Bits 2:4 - RTC Prescale Timer 1 Interrupt Interval Bit: 2"]
    #[inline(always)]
    pub fn rt1ip(&mut self) -> RT1IP_W {
        RT1IP_W { w: self }
    }
    #[doc = "Bit 8 - RTC Prescale Timer 1 Hold"]
    #[inline(always)]
    pub fn rt1pshold(&mut self) -> RT1PSHOLD_W {
        RT1PSHOLD_W { w: self }
    }
    #[doc = "Bits 11:13 - RTC Prescale Timer 1 Clock Divide Bit: 2"]
    #[inline(always)]
    pub fn rt1psdiv(&mut self) -> RT1PSDIV_W {
        RT1PSDIV_W { w: self }
    }
    #[doc = "Bits 14:15 - RTC Prescale Timer 1 Source Select Bit 1"]
    #[inline(always)]
    pub fn rt1ssel(&mut self) -> RT1SSEL_W {
        RT1SSEL_W { w: self }
    }
}
