#[doc = "Reader of register RTCHOUR"]
pub type R = crate::R<u8, super::RTCHOUR>;
#[doc = "Writer for register RTCHOUR"]
pub type W = crate::W<u8, super::RTCHOUR>;
#[doc = "Register RTCHOUR `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCHOUR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOUR0`"]
pub type HOUR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOUR0`"]
pub struct HOUR0_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR0_W<'a> {
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
#[doc = "Reader of field `HOUR1`"]
pub type HOUR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOUR1`"]
pub struct HOUR1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR1_W<'a> {
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
#[doc = "Reader of field `HOUR2`"]
pub type HOUR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOUR2`"]
pub struct HOUR2_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR2_W<'a> {
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
#[doc = "Reader of field `HOUR3`"]
pub type HOUR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOUR3`"]
pub struct HOUR3_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR3_W<'a> {
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
#[doc = "Reader of field `HOUR4`"]
pub type HOUR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOUR4`"]
pub struct HOUR4_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR4_W<'a> {
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
#[doc = "Reader of field `HOUR5`"]
pub type HOUR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOUR5`"]
pub struct HOUR5_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR5_W<'a> {
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
#[doc = "Reader of field `HOUR6`"]
pub type HOUR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOUR6`"]
pub struct HOUR6_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR6_W<'a> {
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
    #[doc = "Bit 0 - Real Time Clock Hour Bit: 0"]
    #[inline(always)]
    pub fn hour0(&self) -> HOUR0_R {
        HOUR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock Hour Bit: 1"]
    #[inline(always)]
    pub fn hour1(&self) -> HOUR1_R {
        HOUR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock Hour Bit: 2"]
    #[inline(always)]
    pub fn hour2(&self) -> HOUR2_R {
        HOUR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Hour Bit: 3"]
    #[inline(always)]
    pub fn hour3(&self) -> HOUR3_R {
        HOUR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock Hour Bit: 4"]
    #[inline(always)]
    pub fn hour4(&self) -> HOUR4_R {
        HOUR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock Hour Bit: 5"]
    #[inline(always)]
    pub fn hour5(&self) -> HOUR5_R {
        HOUR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock Hour Bit: 6"]
    #[inline(always)]
    pub fn hour6(&self) -> HOUR6_R {
        HOUR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Hour Bit: 0"]
    #[inline(always)]
    pub fn hour0(&mut self) -> HOUR0_W {
        HOUR0_W { w: self }
    }
    #[doc = "Bit 1 - Real Time Clock Hour Bit: 1"]
    #[inline(always)]
    pub fn hour1(&mut self) -> HOUR1_W {
        HOUR1_W { w: self }
    }
    #[doc = "Bit 2 - Real Time Clock Hour Bit: 2"]
    #[inline(always)]
    pub fn hour2(&mut self) -> HOUR2_W {
        HOUR2_W { w: self }
    }
    #[doc = "Bit 3 - Real Time Clock Hour Bit: 3"]
    #[inline(always)]
    pub fn hour3(&mut self) -> HOUR3_W {
        HOUR3_W { w: self }
    }
    #[doc = "Bit 4 - Real Time Clock Hour Bit: 4"]
    #[inline(always)]
    pub fn hour4(&mut self) -> HOUR4_W {
        HOUR4_W { w: self }
    }
    #[doc = "Bit 5 - Real Time Clock Hour Bit: 5"]
    #[inline(always)]
    pub fn hour5(&mut self) -> HOUR5_W {
        HOUR5_W { w: self }
    }
    #[doc = "Bit 6 - Real Time Clock Hour Bit: 6"]
    #[inline(always)]
    pub fn hour6(&mut self) -> HOUR6_W {
        HOUR6_W { w: self }
    }
}
