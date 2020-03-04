#[doc = "Reader of register RTCMON"]
pub type R = crate::R<u8, super::RTCMON>;
#[doc = "Writer for register RTCMON"]
pub type W = crate::W<u8, super::RTCMON>;
#[doc = "Register RTCMON `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCMON {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MONTH0`"]
pub type MONTH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MONTH0`"]
pub struct MONTH0_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH0_W<'a> {
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
#[doc = "Reader of field `MONTH1`"]
pub type MONTH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MONTH1`"]
pub struct MONTH1_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH1_W<'a> {
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
#[doc = "Reader of field `MONTH2`"]
pub type MONTH2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MONTH2`"]
pub struct MONTH2_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH2_W<'a> {
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
#[doc = "Reader of field `MONTH3`"]
pub type MONTH3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MONTH3`"]
pub struct MONTH3_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH3_W<'a> {
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
#[doc = "Reader of field `MONTH4`"]
pub type MONTH4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MONTH4`"]
pub struct MONTH4_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH4_W<'a> {
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
#[doc = "Reader of field `MONTH5`"]
pub type MONTH5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MONTH5`"]
pub struct MONTH5_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH5_W<'a> {
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
#[doc = "Reader of field `MONTH6`"]
pub type MONTH6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MONTH6`"]
pub struct MONTH6_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH6_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Real Time Clock Month Bit: 0"]
    #[inline(always)]
    pub fn month0(&self) -> MONTH0_R {
        MONTH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock Month Bit: 1"]
    #[inline(always)]
    pub fn month1(&self) -> MONTH1_R {
        MONTH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock Month Bit: 2"]
    #[inline(always)]
    pub fn month2(&self) -> MONTH2_R {
        MONTH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Month Bit: 3"]
    #[inline(always)]
    pub fn month3(&self) -> MONTH3_R {
        MONTH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock Month Bit: 4"]
    #[inline(always)]
    pub fn month4(&self) -> MONTH4_R {
        MONTH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock Month Bit: 5"]
    #[inline(always)]
    pub fn month5(&self) -> MONTH5_R {
        MONTH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock Month Bit: 6"]
    #[inline(always)]
    pub fn month6(&self) -> MONTH6_R {
        MONTH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Month Bit: 0"]
    #[inline(always)]
    pub fn month0(&mut self) -> MONTH0_W {
        MONTH0_W { w: self }
    }
    #[doc = "Bit 1 - Real Time Clock Month Bit: 1"]
    #[inline(always)]
    pub fn month1(&mut self) -> MONTH1_W {
        MONTH1_W { w: self }
    }
    #[doc = "Bit 2 - Real Time Clock Month Bit: 2"]
    #[inline(always)]
    pub fn month2(&mut self) -> MONTH2_W {
        MONTH2_W { w: self }
    }
    #[doc = "Bit 3 - Real Time Clock Month Bit: 3"]
    #[inline(always)]
    pub fn month3(&mut self) -> MONTH3_W {
        MONTH3_W { w: self }
    }
    #[doc = "Bit 4 - Real Time Clock Month Bit: 4"]
    #[inline(always)]
    pub fn month4(&mut self) -> MONTH4_W {
        MONTH4_W { w: self }
    }
    #[doc = "Bit 5 - Real Time Clock Month Bit: 5"]
    #[inline(always)]
    pub fn month5(&mut self) -> MONTH5_W {
        MONTH5_W { w: self }
    }
    #[doc = "Bit 6 - Real Time Clock Month Bit: 6"]
    #[inline(always)]
    pub fn month6(&mut self) -> MONTH6_W {
        MONTH6_W { w: self }
    }
}
