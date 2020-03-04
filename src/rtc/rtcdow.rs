#[doc = "Reader of register RTCDOW"]
pub type R = crate::R<u8, super::RTCDOW>;
#[doc = "Writer for register RTCDOW"]
pub type W = crate::W<u8, super::RTCDOW>;
#[doc = "Register RTCDOW `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCDOW {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOW0`"]
pub type DOW0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOW0`"]
pub struct DOW0_W<'a> {
    w: &'a mut W,
}
impl<'a> DOW0_W<'a> {
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
#[doc = "Reader of field `DOW1`"]
pub type DOW1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOW1`"]
pub struct DOW1_W<'a> {
    w: &'a mut W,
}
impl<'a> DOW1_W<'a> {
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
#[doc = "Reader of field `DOW2`"]
pub type DOW2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOW2`"]
pub struct DOW2_W<'a> {
    w: &'a mut W,
}
impl<'a> DOW2_W<'a> {
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
#[doc = "Reader of field `DOW3`"]
pub type DOW3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOW3`"]
pub struct DOW3_W<'a> {
    w: &'a mut W,
}
impl<'a> DOW3_W<'a> {
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
#[doc = "Reader of field `DOW4`"]
pub type DOW4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOW4`"]
pub struct DOW4_W<'a> {
    w: &'a mut W,
}
impl<'a> DOW4_W<'a> {
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
#[doc = "Reader of field `DOW5`"]
pub type DOW5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOW5`"]
pub struct DOW5_W<'a> {
    w: &'a mut W,
}
impl<'a> DOW5_W<'a> {
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
#[doc = "Reader of field `DOW6`"]
pub type DOW6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOW6`"]
pub struct DOW6_W<'a> {
    w: &'a mut W,
}
impl<'a> DOW6_W<'a> {
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
    #[doc = "Bit 0 - Real Time Clock DOW Bit: 0"]
    #[inline(always)]
    pub fn dow0(&self) -> DOW0_R {
        DOW0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock DOW Bit: 1"]
    #[inline(always)]
    pub fn dow1(&self) -> DOW1_R {
        DOW1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock DOW Bit: 2"]
    #[inline(always)]
    pub fn dow2(&self) -> DOW2_R {
        DOW2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock DOW Bit: 3"]
    #[inline(always)]
    pub fn dow3(&self) -> DOW3_R {
        DOW3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock DOW Bit: 4"]
    #[inline(always)]
    pub fn dow4(&self) -> DOW4_R {
        DOW4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock DOW Bit: 5"]
    #[inline(always)]
    pub fn dow5(&self) -> DOW5_R {
        DOW5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock DOW Bit: 6"]
    #[inline(always)]
    pub fn dow6(&self) -> DOW6_R {
        DOW6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock DOW Bit: 0"]
    #[inline(always)]
    pub fn dow0(&mut self) -> DOW0_W {
        DOW0_W { w: self }
    }
    #[doc = "Bit 1 - Real Time Clock DOW Bit: 1"]
    #[inline(always)]
    pub fn dow1(&mut self) -> DOW1_W {
        DOW1_W { w: self }
    }
    #[doc = "Bit 2 - Real Time Clock DOW Bit: 2"]
    #[inline(always)]
    pub fn dow2(&mut self) -> DOW2_W {
        DOW2_W { w: self }
    }
    #[doc = "Bit 3 - Real Time Clock DOW Bit: 3"]
    #[inline(always)]
    pub fn dow3(&mut self) -> DOW3_W {
        DOW3_W { w: self }
    }
    #[doc = "Bit 4 - Real Time Clock DOW Bit: 4"]
    #[inline(always)]
    pub fn dow4(&mut self) -> DOW4_W {
        DOW4_W { w: self }
    }
    #[doc = "Bit 5 - Real Time Clock DOW Bit: 5"]
    #[inline(always)]
    pub fn dow5(&mut self) -> DOW5_W {
        DOW5_W { w: self }
    }
    #[doc = "Bit 6 - Real Time Clock DOW Bit: 6"]
    #[inline(always)]
    pub fn dow6(&mut self) -> DOW6_W {
        DOW6_W { w: self }
    }
}
