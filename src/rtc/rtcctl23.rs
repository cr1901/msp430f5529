#[doc = "Reader of register RTCCTL23"]
pub type R = crate::R<u16, super::RTCCTL23>;
#[doc = "Writer for register RTCCTL23"]
pub type W = crate::W<u16, super::RTCCTL23>;
#[doc = "Register RTCCTL23 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCCTL23 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTCCAL0`"]
pub type RTCCAL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCCAL0`"]
pub struct RTCCAL0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCAL0_W<'a> {
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
#[doc = "Reader of field `RTCCAL1`"]
pub type RTCCAL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCCAL1`"]
pub struct RTCCAL1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCAL1_W<'a> {
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
#[doc = "Reader of field `RTCCAL2`"]
pub type RTCCAL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCCAL2`"]
pub struct RTCCAL2_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCAL2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `RTCCAL3`"]
pub type RTCCAL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCCAL3`"]
pub struct RTCCAL3_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCAL3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RTCCAL4`"]
pub type RTCCAL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCCAL4`"]
pub struct RTCCAL4_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCAL4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RTCCAL5`"]
pub type RTCCAL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCCAL5`"]
pub struct RTCCAL5_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCAL5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RTCCALS`"]
pub type RTCCALS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCCALS`"]
pub struct RTCCALS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCALS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "RTC Calibration Frequency Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `RTCCALF`"]
pub type RTCCALF_R = crate::R<u8, RTCCALF_A>;
impl RTCCALF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCCALF_A {
        match self.bits {
            0 => RTCCALF_A::RTCCALF_0,
            1 => RTCCALF_A::RTCCALF_1,
            2 => RTCCALF_A::RTCCALF_2,
            3 => RTCCALF_A::RTCCALF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTCCALF_0`"]
    #[inline(always)]
    pub fn is_rtccalf_0(&self) -> bool {
        *self == RTCCALF_A::RTCCALF_0
    }
    #[doc = "Checks if the value of the field is `RTCCALF_1`"]
    #[inline(always)]
    pub fn is_rtccalf_1(&self) -> bool {
        *self == RTCCALF_A::RTCCALF_1
    }
    #[doc = "Checks if the value of the field is `RTCCALF_2`"]
    #[inline(always)]
    pub fn is_rtccalf_2(&self) -> bool {
        *self == RTCCALF_A::RTCCALF_2
    }
    #[doc = "Checks if the value of the field is `RTCCALF_3`"]
    #[inline(always)]
    pub fn is_rtccalf_3(&self) -> bool {
        *self == RTCCALF_A::RTCCALF_3
    }
}
#[doc = "Write proxy for field `RTCCALF`"]
pub struct RTCCALF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCALF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCCALF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "RTC Calibration Frequency: No Output"]
    #[inline(always)]
    pub fn rtccalf_0(self) -> &'a mut W {
        self.variant(RTCCALF_A::RTCCALF_0)
    }
    #[doc = "RTC Calibration Frequency: 512 Hz"]
    #[inline(always)]
    pub fn rtccalf_1(self) -> &'a mut W {
        self.variant(RTCCALF_A::RTCCALF_1)
    }
    #[doc = "RTC Calibration Frequency: 256 Hz"]
    #[inline(always)]
    pub fn rtccalf_2(self) -> &'a mut W {
        self.variant(RTCCALF_A::RTCCALF_2)
    }
    #[doc = "RTC Calibration Frequency: 1 Hz"]
    #[inline(always)]
    pub fn rtccalf_3(self) -> &'a mut W {
        self.variant(RTCCALF_A::RTCCALF_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RTC Calibration Bit 0"]
    #[inline(always)]
    pub fn rtccal0(&self) -> RTCCAL0_R {
        RTCCAL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC Calibration Bit 1"]
    #[inline(always)]
    pub fn rtccal1(&self) -> RTCCAL1_R {
        RTCCAL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTC Calibration Bit 2"]
    #[inline(always)]
    pub fn rtccal2(&self) -> RTCCAL2_R {
        RTCCAL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC Calibration Bit 3"]
    #[inline(always)]
    pub fn rtccal3(&self) -> RTCCAL3_R {
        RTCCAL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTC Calibration Bit 4"]
    #[inline(always)]
    pub fn rtccal4(&self) -> RTCCAL4_R {
        RTCCAL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RTC Calibration Bit 5"]
    #[inline(always)]
    pub fn rtccal5(&self) -> RTCCAL5_R {
        RTCCAL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RTC Calibration Sign"]
    #[inline(always)]
    pub fn rtccals(&self) -> RTCCALS_R {
        RTCCALS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - RTC Calibration Frequency Bit 1"]
    #[inline(always)]
    pub fn rtccalf(&self) -> RTCCALF_R {
        RTCCALF_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Calibration Bit 0"]
    #[inline(always)]
    pub fn rtccal0(&mut self) -> RTCCAL0_W {
        RTCCAL0_W { w: self }
    }
    #[doc = "Bit 1 - RTC Calibration Bit 1"]
    #[inline(always)]
    pub fn rtccal1(&mut self) -> RTCCAL1_W {
        RTCCAL1_W { w: self }
    }
    #[doc = "Bit 2 - RTC Calibration Bit 2"]
    #[inline(always)]
    pub fn rtccal2(&mut self) -> RTCCAL2_W {
        RTCCAL2_W { w: self }
    }
    #[doc = "Bit 3 - RTC Calibration Bit 3"]
    #[inline(always)]
    pub fn rtccal3(&mut self) -> RTCCAL3_W {
        RTCCAL3_W { w: self }
    }
    #[doc = "Bit 4 - RTC Calibration Bit 4"]
    #[inline(always)]
    pub fn rtccal4(&mut self) -> RTCCAL4_W {
        RTCCAL4_W { w: self }
    }
    #[doc = "Bit 5 - RTC Calibration Bit 5"]
    #[inline(always)]
    pub fn rtccal5(&mut self) -> RTCCAL5_W {
        RTCCAL5_W { w: self }
    }
    #[doc = "Bit 7 - RTC Calibration Sign"]
    #[inline(always)]
    pub fn rtccals(&mut self) -> RTCCALS_W {
        RTCCALS_W { w: self }
    }
    #[doc = "Bits 8:9 - RTC Calibration Frequency Bit 1"]
    #[inline(always)]
    pub fn rtccalf(&mut self) -> RTCCALF_W {
        RTCCALF_W { w: self }
    }
}
