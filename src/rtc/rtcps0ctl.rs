#[doc = "Reader of register RTCPS0CTL"]
pub type R = crate::R<u16, super::RTCPS0CTL>;
#[doc = "Writer for register RTCPS0CTL"]
pub type W = crate::W<u16, super::RTCPS0CTL>;
#[doc = "Register RTCPS0CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCPS0CTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RT0PSIFG`"]
pub type RT0PSIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RT0PSIFG`"]
pub struct RT0PSIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0PSIFG_W<'a> {
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
#[doc = "Reader of field `RT0PSIE`"]
pub type RT0PSIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RT0PSIE`"]
pub struct RT0PSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0PSIE_W<'a> {
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
#[doc = "RTC Prescale Timer 0 Interrupt Interval Bit: 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RT0IP_A {
    #[doc = "0: RTC Prescale Timer 0 Interrupt Interval /2"]
    RT0IP_0 = 0,
    #[doc = "1: RTC Prescale Timer 0 Interrupt Interval /4"]
    RT0IP_1 = 1,
    #[doc = "2: RTC Prescale Timer 0 Interrupt Interval /8"]
    RT0IP_2 = 2,
    #[doc = "3: RTC Prescale Timer 0 Interrupt Interval /16"]
    RT0IP_3 = 3,
    #[doc = "4: RTC Prescale Timer 0 Interrupt Interval /32"]
    RT0IP_4 = 4,
    #[doc = "5: RTC Prescale Timer 0 Interrupt Interval /64"]
    RT0IP_5 = 5,
    #[doc = "6: RTC Prescale Timer 0 Interrupt Interval /128"]
    RT0IP_6 = 6,
    #[doc = "7: RTC Prescale Timer 0 Interrupt Interval /256"]
    RT0IP_7 = 7,
}
impl From<RT0IP_A> for u8 {
    #[inline(always)]
    fn from(variant: RT0IP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RT0IP`"]
pub type RT0IP_R = crate::R<u8, RT0IP_A>;
impl RT0IP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT0IP_A {
        match self.bits {
            0 => RT0IP_A::RT0IP_0,
            1 => RT0IP_A::RT0IP_1,
            2 => RT0IP_A::RT0IP_2,
            3 => RT0IP_A::RT0IP_3,
            4 => RT0IP_A::RT0IP_4,
            5 => RT0IP_A::RT0IP_5,
            6 => RT0IP_A::RT0IP_6,
            7 => RT0IP_A::RT0IP_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RT0IP_0`"]
    #[inline(always)]
    pub fn is_rt0ip_0(&self) -> bool {
        *self == RT0IP_A::RT0IP_0
    }
    #[doc = "Checks if the value of the field is `RT0IP_1`"]
    #[inline(always)]
    pub fn is_rt0ip_1(&self) -> bool {
        *self == RT0IP_A::RT0IP_1
    }
    #[doc = "Checks if the value of the field is `RT0IP_2`"]
    #[inline(always)]
    pub fn is_rt0ip_2(&self) -> bool {
        *self == RT0IP_A::RT0IP_2
    }
    #[doc = "Checks if the value of the field is `RT0IP_3`"]
    #[inline(always)]
    pub fn is_rt0ip_3(&self) -> bool {
        *self == RT0IP_A::RT0IP_3
    }
    #[doc = "Checks if the value of the field is `RT0IP_4`"]
    #[inline(always)]
    pub fn is_rt0ip_4(&self) -> bool {
        *self == RT0IP_A::RT0IP_4
    }
    #[doc = "Checks if the value of the field is `RT0IP_5`"]
    #[inline(always)]
    pub fn is_rt0ip_5(&self) -> bool {
        *self == RT0IP_A::RT0IP_5
    }
    #[doc = "Checks if the value of the field is `RT0IP_6`"]
    #[inline(always)]
    pub fn is_rt0ip_6(&self) -> bool {
        *self == RT0IP_A::RT0IP_6
    }
    #[doc = "Checks if the value of the field is `RT0IP_7`"]
    #[inline(always)]
    pub fn is_rt0ip_7(&self) -> bool {
        *self == RT0IP_A::RT0IP_7
    }
}
#[doc = "Write proxy for field `RT0IP`"]
pub struct RT0IP_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0IP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT0IP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "RTC Prescale Timer 0 Interrupt Interval /2"]
    #[inline(always)]
    pub fn rt0ip_0(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_0)
    }
    #[doc = "RTC Prescale Timer 0 Interrupt Interval /4"]
    #[inline(always)]
    pub fn rt0ip_1(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_1)
    }
    #[doc = "RTC Prescale Timer 0 Interrupt Interval /8"]
    #[inline(always)]
    pub fn rt0ip_2(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_2)
    }
    #[doc = "RTC Prescale Timer 0 Interrupt Interval /16"]
    #[inline(always)]
    pub fn rt0ip_3(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_3)
    }
    #[doc = "RTC Prescale Timer 0 Interrupt Interval /32"]
    #[inline(always)]
    pub fn rt0ip_4(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_4)
    }
    #[doc = "RTC Prescale Timer 0 Interrupt Interval /64"]
    #[inline(always)]
    pub fn rt0ip_5(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_5)
    }
    #[doc = "RTC Prescale Timer 0 Interrupt Interval /128"]
    #[inline(always)]
    pub fn rt0ip_6(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_6)
    }
    #[doc = "RTC Prescale Timer 0 Interrupt Interval /256"]
    #[inline(always)]
    pub fn rt0ip_7(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u16) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `RT0PSHOLD`"]
pub type RT0PSHOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RT0PSHOLD`"]
pub struct RT0PSHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0PSHOLD_W<'a> {
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
#[doc = "RTC Prescale Timer 0 Clock Divide Bit: 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RT0PSDIV_A {
    #[doc = "0: RTC Prescale Timer 0 Clock Divide /2"]
    RT0PSDIV_0 = 0,
    #[doc = "1: RTC Prescale Timer 0 Clock Divide /4"]
    RT0PSDIV_1 = 1,
    #[doc = "2: RTC Prescale Timer 0 Clock Divide /8"]
    RT0PSDIV_2 = 2,
    #[doc = "3: RTC Prescale Timer 0 Clock Divide /16"]
    RT0PSDIV_3 = 3,
    #[doc = "4: RTC Prescale Timer 0 Clock Divide /32"]
    RT0PSDIV_4 = 4,
    #[doc = "5: RTC Prescale Timer 0 Clock Divide /64"]
    RT0PSDIV_5 = 5,
    #[doc = "6: RTC Prescale Timer 0 Clock Divide /128"]
    RT0PSDIV_6 = 6,
    #[doc = "7: RTC Prescale Timer 0 Clock Divide /256"]
    RT0PSDIV_7 = 7,
}
impl From<RT0PSDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: RT0PSDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RT0PSDIV`"]
pub type RT0PSDIV_R = crate::R<u8, RT0PSDIV_A>;
impl RT0PSDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT0PSDIV_A {
        match self.bits {
            0 => RT0PSDIV_A::RT0PSDIV_0,
            1 => RT0PSDIV_A::RT0PSDIV_1,
            2 => RT0PSDIV_A::RT0PSDIV_2,
            3 => RT0PSDIV_A::RT0PSDIV_3,
            4 => RT0PSDIV_A::RT0PSDIV_4,
            5 => RT0PSDIV_A::RT0PSDIV_5,
            6 => RT0PSDIV_A::RT0PSDIV_6,
            7 => RT0PSDIV_A::RT0PSDIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RT0PSDIV_0`"]
    #[inline(always)]
    pub fn is_rt0psdiv_0(&self) -> bool {
        *self == RT0PSDIV_A::RT0PSDIV_0
    }
    #[doc = "Checks if the value of the field is `RT0PSDIV_1`"]
    #[inline(always)]
    pub fn is_rt0psdiv_1(&self) -> bool {
        *self == RT0PSDIV_A::RT0PSDIV_1
    }
    #[doc = "Checks if the value of the field is `RT0PSDIV_2`"]
    #[inline(always)]
    pub fn is_rt0psdiv_2(&self) -> bool {
        *self == RT0PSDIV_A::RT0PSDIV_2
    }
    #[doc = "Checks if the value of the field is `RT0PSDIV_3`"]
    #[inline(always)]
    pub fn is_rt0psdiv_3(&self) -> bool {
        *self == RT0PSDIV_A::RT0PSDIV_3
    }
    #[doc = "Checks if the value of the field is `RT0PSDIV_4`"]
    #[inline(always)]
    pub fn is_rt0psdiv_4(&self) -> bool {
        *self == RT0PSDIV_A::RT0PSDIV_4
    }
    #[doc = "Checks if the value of the field is `RT0PSDIV_5`"]
    #[inline(always)]
    pub fn is_rt0psdiv_5(&self) -> bool {
        *self == RT0PSDIV_A::RT0PSDIV_5
    }
    #[doc = "Checks if the value of the field is `RT0PSDIV_6`"]
    #[inline(always)]
    pub fn is_rt0psdiv_6(&self) -> bool {
        *self == RT0PSDIV_A::RT0PSDIV_6
    }
    #[doc = "Checks if the value of the field is `RT0PSDIV_7`"]
    #[inline(always)]
    pub fn is_rt0psdiv_7(&self) -> bool {
        *self == RT0PSDIV_A::RT0PSDIV_7
    }
}
#[doc = "Write proxy for field `RT0PSDIV`"]
pub struct RT0PSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0PSDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT0PSDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "RTC Prescale Timer 0 Clock Divide /2"]
    #[inline(always)]
    pub fn rt0psdiv_0(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::RT0PSDIV_0)
    }
    #[doc = "RTC Prescale Timer 0 Clock Divide /4"]
    #[inline(always)]
    pub fn rt0psdiv_1(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::RT0PSDIV_1)
    }
    #[doc = "RTC Prescale Timer 0 Clock Divide /8"]
    #[inline(always)]
    pub fn rt0psdiv_2(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::RT0PSDIV_2)
    }
    #[doc = "RTC Prescale Timer 0 Clock Divide /16"]
    #[inline(always)]
    pub fn rt0psdiv_3(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::RT0PSDIV_3)
    }
    #[doc = "RTC Prescale Timer 0 Clock Divide /32"]
    #[inline(always)]
    pub fn rt0psdiv_4(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::RT0PSDIV_4)
    }
    #[doc = "RTC Prescale Timer 0 Clock Divide /64"]
    #[inline(always)]
    pub fn rt0psdiv_5(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::RT0PSDIV_5)
    }
    #[doc = "RTC Prescale Timer 0 Clock Divide /128"]
    #[inline(always)]
    pub fn rt0psdiv_6(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::RT0PSDIV_6)
    }
    #[doc = "RTC Prescale Timer 0 Clock Divide /256"]
    #[inline(always)]
    pub fn rt0psdiv_7(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::RT0PSDIV_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u16) & 0x07) << 11);
        self.w
    }
}
#[doc = "Reader of field `RT0SSEL`"]
pub type RT0SSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RT0SSEL`"]
pub struct RT0SSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0SSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RTC Prescale Timer 0 Interrupt Flag"]
    #[inline(always)]
    pub fn rt0psifg(&self) -> RT0PSIFG_R {
        RT0PSIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC Prescale Timer 0 Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rt0psie(&self) -> RT0PSIE_R {
        RT0PSIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - RTC Prescale Timer 0 Interrupt Interval Bit: 2"]
    #[inline(always)]
    pub fn rt0ip(&self) -> RT0IP_R {
        RT0IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 8 - RTC Prescale Timer 0 Hold"]
    #[inline(always)]
    pub fn rt0pshold(&self) -> RT0PSHOLD_R {
        RT0PSHOLD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - RTC Prescale Timer 0 Clock Divide Bit: 2"]
    #[inline(always)]
    pub fn rt0psdiv(&self) -> RT0PSDIV_R {
        RT0PSDIV_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 14 - RTC Prescale Timer 0 Source Select 0:ACLK / 1:SMCLK"]
    #[inline(always)]
    pub fn rt0ssel(&self) -> RT0SSEL_R {
        RT0SSEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Prescale Timer 0 Interrupt Flag"]
    #[inline(always)]
    pub fn rt0psifg(&mut self) -> RT0PSIFG_W {
        RT0PSIFG_W { w: self }
    }
    #[doc = "Bit 1 - RTC Prescale Timer 0 Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rt0psie(&mut self) -> RT0PSIE_W {
        RT0PSIE_W { w: self }
    }
    #[doc = "Bits 2:4 - RTC Prescale Timer 0 Interrupt Interval Bit: 2"]
    #[inline(always)]
    pub fn rt0ip(&mut self) -> RT0IP_W {
        RT0IP_W { w: self }
    }
    #[doc = "Bit 8 - RTC Prescale Timer 0 Hold"]
    #[inline(always)]
    pub fn rt0pshold(&mut self) -> RT0PSHOLD_W {
        RT0PSHOLD_W { w: self }
    }
    #[doc = "Bits 11:13 - RTC Prescale Timer 0 Clock Divide Bit: 2"]
    #[inline(always)]
    pub fn rt0psdiv(&mut self) -> RT0PSDIV_W {
        RT0PSDIV_W { w: self }
    }
    #[doc = "Bit 14 - RTC Prescale Timer 0 Source Select 0:ACLK / 1:SMCLK"]
    #[inline(always)]
    pub fn rt0ssel(&mut self) -> RT0SSEL_W {
        RT0SSEL_W { w: self }
    }
}
