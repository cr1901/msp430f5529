#[doc = "Reader of register RTCMIN"]
pub type R = crate::R<u8, super::RTCMIN>;
#[doc = "Writer for register RTCMIN"]
pub type W = crate::W<u8, super::RTCMIN>;
#[doc = "Register RTCMIN `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCMIN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MINUTES0`"]
pub type MINUTES0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MINUTES0`"]
pub struct MINUTES0_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTES0_W<'a> {
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
#[doc = "Reader of field `MINUTES1`"]
pub type MINUTES1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MINUTES1`"]
pub struct MINUTES1_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTES1_W<'a> {
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
#[doc = "Reader of field `MINUTES2`"]
pub type MINUTES2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MINUTES2`"]
pub struct MINUTES2_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTES2_W<'a> {
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
#[doc = "Reader of field `MINUTES3`"]
pub type MINUTES3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MINUTES3`"]
pub struct MINUTES3_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTES3_W<'a> {
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
#[doc = "Reader of field `MINUTES4`"]
pub type MINUTES4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MINUTES4`"]
pub struct MINUTES4_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTES4_W<'a> {
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
#[doc = "Reader of field `MINUTES5`"]
pub type MINUTES5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MINUTES5`"]
pub struct MINUTES5_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTES5_W<'a> {
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
#[doc = "Reader of field `MINUTES6`"]
pub type MINUTES6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MINUTES6`"]
pub struct MINUTES6_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTES6_W<'a> {
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
    #[doc = "Bit 0 - Real Time Clock Minutes Bit: 0"]
    #[inline(always)]
    pub fn minutes0(&self) -> MINUTES0_R {
        MINUTES0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock Minutes Bit: 1"]
    #[inline(always)]
    pub fn minutes1(&self) -> MINUTES1_R {
        MINUTES1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock Minutes Bit: 2"]
    #[inline(always)]
    pub fn minutes2(&self) -> MINUTES2_R {
        MINUTES2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Minutes Bit: 3"]
    #[inline(always)]
    pub fn minutes3(&self) -> MINUTES3_R {
        MINUTES3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock Minutes Bit: 4"]
    #[inline(always)]
    pub fn minutes4(&self) -> MINUTES4_R {
        MINUTES4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock Minutes Bit: 5"]
    #[inline(always)]
    pub fn minutes5(&self) -> MINUTES5_R {
        MINUTES5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock Minutes Bit: 6"]
    #[inline(always)]
    pub fn minutes6(&self) -> MINUTES6_R {
        MINUTES6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Minutes Bit: 0"]
    #[inline(always)]
    pub fn minutes0(&mut self) -> MINUTES0_W {
        MINUTES0_W { w: self }
    }
    #[doc = "Bit 1 - Real Time Clock Minutes Bit: 1"]
    #[inline(always)]
    pub fn minutes1(&mut self) -> MINUTES1_W {
        MINUTES1_W { w: self }
    }
    #[doc = "Bit 2 - Real Time Clock Minutes Bit: 2"]
    #[inline(always)]
    pub fn minutes2(&mut self) -> MINUTES2_W {
        MINUTES2_W { w: self }
    }
    #[doc = "Bit 3 - Real Time Clock Minutes Bit: 3"]
    #[inline(always)]
    pub fn minutes3(&mut self) -> MINUTES3_W {
        MINUTES3_W { w: self }
    }
    #[doc = "Bit 4 - Real Time Clock Minutes Bit: 4"]
    #[inline(always)]
    pub fn minutes4(&mut self) -> MINUTES4_W {
        MINUTES4_W { w: self }
    }
    #[doc = "Bit 5 - Real Time Clock Minutes Bit: 5"]
    #[inline(always)]
    pub fn minutes5(&mut self) -> MINUTES5_W {
        MINUTES5_W { w: self }
    }
    #[doc = "Bit 6 - Real Time Clock Minutes Bit: 6"]
    #[inline(always)]
    pub fn minutes6(&mut self) -> MINUTES6_W {
        MINUTES6_W { w: self }
    }
}
