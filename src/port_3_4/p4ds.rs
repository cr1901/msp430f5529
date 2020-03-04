#[doc = "Reader of register P4DS"]
pub type R = crate::R<u8, super::P4DS>;
#[doc = "Writer for register P4DS"]
pub type W = crate::W<u8, super::P4DS>;
#[doc = "Register P4DS `reset()`'s with value 0"]
impl crate::ResetValue for super::P4DS {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P4DS0`"]
pub type P4DS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4DS0`"]
pub struct P4DS0_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DS0_W<'a> {
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
#[doc = "Reader of field `P4DS1`"]
pub type P4DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4DS1`"]
pub struct P4DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DS1_W<'a> {
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
#[doc = "Reader of field `P4DS2`"]
pub type P4DS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4DS2`"]
pub struct P4DS2_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DS2_W<'a> {
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
#[doc = "Reader of field `P4DS3`"]
pub type P4DS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4DS3`"]
pub struct P4DS3_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DS3_W<'a> {
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
#[doc = "Reader of field `P4DS4`"]
pub type P4DS4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4DS4`"]
pub struct P4DS4_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DS4_W<'a> {
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
#[doc = "Reader of field `P4DS5`"]
pub type P4DS5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4DS5`"]
pub struct P4DS5_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DS5_W<'a> {
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
#[doc = "Reader of field `P4DS6`"]
pub type P4DS6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4DS6`"]
pub struct P4DS6_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DS6_W<'a> {
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
#[doc = "Reader of field `P4DS7`"]
pub type P4DS7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4DS7`"]
pub struct P4DS7_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DS7_W<'a> {
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
    #[doc = "Bit 0 - P4DS0"]
    #[inline(always)]
    pub fn p4ds0(&self) -> P4DS0_R {
        P4DS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P4DS1"]
    #[inline(always)]
    pub fn p4ds1(&self) -> P4DS1_R {
        P4DS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P4DS2"]
    #[inline(always)]
    pub fn p4ds2(&self) -> P4DS2_R {
        P4DS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P4DS3"]
    #[inline(always)]
    pub fn p4ds3(&self) -> P4DS3_R {
        P4DS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P4DS4"]
    #[inline(always)]
    pub fn p4ds4(&self) -> P4DS4_R {
        P4DS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P4DS5"]
    #[inline(always)]
    pub fn p4ds5(&self) -> P4DS5_R {
        P4DS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P4DS6"]
    #[inline(always)]
    pub fn p4ds6(&self) -> P4DS6_R {
        P4DS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P4DS7"]
    #[inline(always)]
    pub fn p4ds7(&self) -> P4DS7_R {
        P4DS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4DS0"]
    #[inline(always)]
    pub fn p4ds0(&mut self) -> P4DS0_W {
        P4DS0_W { w: self }
    }
    #[doc = "Bit 1 - P4DS1"]
    #[inline(always)]
    pub fn p4ds1(&mut self) -> P4DS1_W {
        P4DS1_W { w: self }
    }
    #[doc = "Bit 2 - P4DS2"]
    #[inline(always)]
    pub fn p4ds2(&mut self) -> P4DS2_W {
        P4DS2_W { w: self }
    }
    #[doc = "Bit 3 - P4DS3"]
    #[inline(always)]
    pub fn p4ds3(&mut self) -> P4DS3_W {
        P4DS3_W { w: self }
    }
    #[doc = "Bit 4 - P4DS4"]
    #[inline(always)]
    pub fn p4ds4(&mut self) -> P4DS4_W {
        P4DS4_W { w: self }
    }
    #[doc = "Bit 5 - P4DS5"]
    #[inline(always)]
    pub fn p4ds5(&mut self) -> P4DS5_W {
        P4DS5_W { w: self }
    }
    #[doc = "Bit 6 - P4DS6"]
    #[inline(always)]
    pub fn p4ds6(&mut self) -> P4DS6_W {
        P4DS6_W { w: self }
    }
    #[doc = "Bit 7 - P4DS7"]
    #[inline(always)]
    pub fn p4ds7(&mut self) -> P4DS7_W {
        P4DS7_W { w: self }
    }
}
