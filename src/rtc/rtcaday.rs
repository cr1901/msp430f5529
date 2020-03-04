#[doc = "Reader of register RTCADAY"]
pub type R = crate::R<u8, super::RTCADAY>;
#[doc = "Writer for register RTCADAY"]
pub type W = crate::W<u8, super::RTCADAY>;
#[doc = "Register RTCADAY `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCADAY {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAY0`"]
pub type DAY0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAY0`"]
pub struct DAY0_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `DAY1`"]
pub type DAY1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAY1`"]
pub struct DAY1_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DAY2`"]
pub type DAY2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAY2`"]
pub struct DAY2_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `DAY3`"]
pub type DAY3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAY3`"]
pub struct DAY3_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DAY4`"]
pub type DAY4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAY4`"]
pub struct DAY4_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DAY5`"]
pub type DAY5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAY5`"]
pub struct DAY5_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DAY6`"]
pub type DAY6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAY6`"]
pub struct DAY6_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RTCAE`"]
pub type RTCAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCAE`"]
pub struct RTCAE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Real Time Clock Day Bit: 0"]
    #[inline(always)]
    pub fn day0(&self) -> DAY0_R {
        DAY0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock Day Bit: 1"]
    #[inline(always)]
    pub fn day1(&self) -> DAY1_R {
        DAY1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock Day Bit: 2"]
    #[inline(always)]
    pub fn day2(&self) -> DAY2_R {
        DAY2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Day Bit: 3"]
    #[inline(always)]
    pub fn day3(&self) -> DAY3_R {
        DAY3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock Day Bit: 4"]
    #[inline(always)]
    pub fn day4(&self) -> DAY4_R {
        DAY4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock Day Bit: 5"]
    #[inline(always)]
    pub fn day5(&self) -> DAY5_R {
        DAY5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock Day Bit: 6"]
    #[inline(always)]
    pub fn day6(&self) -> DAY6_R {
        DAY6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Real Time Clock Alarm enable"]
    #[inline(always)]
    pub fn rtcae(&self) -> RTCAE_R {
        RTCAE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Day Bit: 0"]
    #[inline(always)]
    pub fn day0(&mut self) -> DAY0_W {
        DAY0_W { w: self }
    }
    #[doc = "Bit 1 - Real Time Clock Day Bit: 1"]
    #[inline(always)]
    pub fn day1(&mut self) -> DAY1_W {
        DAY1_W { w: self }
    }
    #[doc = "Bit 2 - Real Time Clock Day Bit: 2"]
    #[inline(always)]
    pub fn day2(&mut self) -> DAY2_W {
        DAY2_W { w: self }
    }
    #[doc = "Bit 3 - Real Time Clock Day Bit: 3"]
    #[inline(always)]
    pub fn day3(&mut self) -> DAY3_W {
        DAY3_W { w: self }
    }
    #[doc = "Bit 4 - Real Time Clock Day Bit: 4"]
    #[inline(always)]
    pub fn day4(&mut self) -> DAY4_W {
        DAY4_W { w: self }
    }
    #[doc = "Bit 5 - Real Time Clock Day Bit: 5"]
    #[inline(always)]
    pub fn day5(&mut self) -> DAY5_W {
        DAY5_W { w: self }
    }
    #[doc = "Bit 6 - Real Time Clock Day Bit: 6"]
    #[inline(always)]
    pub fn day6(&mut self) -> DAY6_W {
        DAY6_W { w: self }
    }
    #[doc = "Bit 7 - Real Time Clock Alarm enable"]
    #[inline(always)]
    pub fn rtcae(&mut self) -> RTCAE_W {
        RTCAE_W { w: self }
    }
}
