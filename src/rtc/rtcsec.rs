#[doc = "Reader of register RTCSEC"]
pub type R = crate::R<u8, super::RTCSEC>;
#[doc = "Writer for register RTCSEC"]
pub type W = crate::W<u8, super::RTCSEC>;
#[doc = "Register RTCSEC `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCSEC {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SECONDS0`"]
pub type SECONDS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECONDS0`"]
pub struct SECONDS0_W<'a> {
    w: &'a mut W,
}
impl<'a> SECONDS0_W<'a> {
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
#[doc = "Reader of field `SECONDS1`"]
pub type SECONDS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECONDS1`"]
pub struct SECONDS1_W<'a> {
    w: &'a mut W,
}
impl<'a> SECONDS1_W<'a> {
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
#[doc = "Reader of field `SECONDS2`"]
pub type SECONDS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECONDS2`"]
pub struct SECONDS2_W<'a> {
    w: &'a mut W,
}
impl<'a> SECONDS2_W<'a> {
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
#[doc = "Reader of field `SECONDS3`"]
pub type SECONDS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECONDS3`"]
pub struct SECONDS3_W<'a> {
    w: &'a mut W,
}
impl<'a> SECONDS3_W<'a> {
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
#[doc = "Reader of field `SECONDS4`"]
pub type SECONDS4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECONDS4`"]
pub struct SECONDS4_W<'a> {
    w: &'a mut W,
}
impl<'a> SECONDS4_W<'a> {
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
#[doc = "Reader of field `SECONDS5`"]
pub type SECONDS5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECONDS5`"]
pub struct SECONDS5_W<'a> {
    w: &'a mut W,
}
impl<'a> SECONDS5_W<'a> {
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
#[doc = "Reader of field `SECONDS6`"]
pub type SECONDS6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECONDS6`"]
pub struct SECONDS6_W<'a> {
    w: &'a mut W,
}
impl<'a> SECONDS6_W<'a> {
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
    #[doc = "Bit 0 - Real Time Clock Seconds Bit: 0"]
    #[inline(always)]
    pub fn seconds0(&self) -> SECONDS0_R {
        SECONDS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock Seconds Bit: 1"]
    #[inline(always)]
    pub fn seconds1(&self) -> SECONDS1_R {
        SECONDS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock Seconds Bit: 2"]
    #[inline(always)]
    pub fn seconds2(&self) -> SECONDS2_R {
        SECONDS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Seconds Bit: 3"]
    #[inline(always)]
    pub fn seconds3(&self) -> SECONDS3_R {
        SECONDS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock Seconds Bit: 4"]
    #[inline(always)]
    pub fn seconds4(&self) -> SECONDS4_R {
        SECONDS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock Seconds Bit: 5"]
    #[inline(always)]
    pub fn seconds5(&self) -> SECONDS5_R {
        SECONDS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock Seconds Bit: 6"]
    #[inline(always)]
    pub fn seconds6(&self) -> SECONDS6_R {
        SECONDS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Seconds Bit: 0"]
    #[inline(always)]
    pub fn seconds0(&mut self) -> SECONDS0_W {
        SECONDS0_W { w: self }
    }
    #[doc = "Bit 1 - Real Time Clock Seconds Bit: 1"]
    #[inline(always)]
    pub fn seconds1(&mut self) -> SECONDS1_W {
        SECONDS1_W { w: self }
    }
    #[doc = "Bit 2 - Real Time Clock Seconds Bit: 2"]
    #[inline(always)]
    pub fn seconds2(&mut self) -> SECONDS2_W {
        SECONDS2_W { w: self }
    }
    #[doc = "Bit 3 - Real Time Clock Seconds Bit: 3"]
    #[inline(always)]
    pub fn seconds3(&mut self) -> SECONDS3_W {
        SECONDS3_W { w: self }
    }
    #[doc = "Bit 4 - Real Time Clock Seconds Bit: 4"]
    #[inline(always)]
    pub fn seconds4(&mut self) -> SECONDS4_W {
        SECONDS4_W { w: self }
    }
    #[doc = "Bit 5 - Real Time Clock Seconds Bit: 5"]
    #[inline(always)]
    pub fn seconds5(&mut self) -> SECONDS5_W {
        SECONDS5_W { w: self }
    }
    #[doc = "Bit 6 - Real Time Clock Seconds Bit: 6"]
    #[inline(always)]
    pub fn seconds6(&mut self) -> SECONDS6_W {
        SECONDS6_W { w: self }
    }
}
