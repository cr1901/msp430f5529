#[doc = "Reader of register P6DS"]
pub type R = crate::R<u8, super::P6DS>;
#[doc = "Writer for register P6DS"]
pub type W = crate::W<u8, super::P6DS>;
#[doc = "Register P6DS `reset()`'s with value 0"]
impl crate::ResetValue for super::P6DS {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P6DS0`"]
pub type P6DS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6DS0`"]
pub struct P6DS0_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DS0_W<'a> {
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
#[doc = "Reader of field `P6DS1`"]
pub type P6DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6DS1`"]
pub struct P6DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DS1_W<'a> {
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
#[doc = "Reader of field `P6DS2`"]
pub type P6DS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6DS2`"]
pub struct P6DS2_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DS2_W<'a> {
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
#[doc = "Reader of field `P6DS3`"]
pub type P6DS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6DS3`"]
pub struct P6DS3_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DS3_W<'a> {
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
#[doc = "Reader of field `P6DS4`"]
pub type P6DS4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6DS4`"]
pub struct P6DS4_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DS4_W<'a> {
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
#[doc = "Reader of field `P6DS5`"]
pub type P6DS5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6DS5`"]
pub struct P6DS5_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DS5_W<'a> {
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
#[doc = "Reader of field `P6DS6`"]
pub type P6DS6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6DS6`"]
pub struct P6DS6_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DS6_W<'a> {
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
#[doc = "Reader of field `P6DS7`"]
pub type P6DS7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6DS7`"]
pub struct P6DS7_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DS7_W<'a> {
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
    #[doc = "Bit 0 - P6DS0"]
    #[inline(always)]
    pub fn p6ds0(&self) -> P6DS0_R {
        P6DS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P6DS1"]
    #[inline(always)]
    pub fn p6ds1(&self) -> P6DS1_R {
        P6DS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P6DS2"]
    #[inline(always)]
    pub fn p6ds2(&self) -> P6DS2_R {
        P6DS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P6DS3"]
    #[inline(always)]
    pub fn p6ds3(&self) -> P6DS3_R {
        P6DS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P6DS4"]
    #[inline(always)]
    pub fn p6ds4(&self) -> P6DS4_R {
        P6DS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P6DS5"]
    #[inline(always)]
    pub fn p6ds5(&self) -> P6DS5_R {
        P6DS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P6DS6"]
    #[inline(always)]
    pub fn p6ds6(&self) -> P6DS6_R {
        P6DS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P6DS7"]
    #[inline(always)]
    pub fn p6ds7(&self) -> P6DS7_R {
        P6DS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P6DS0"]
    #[inline(always)]
    pub fn p6ds0(&mut self) -> P6DS0_W {
        P6DS0_W { w: self }
    }
    #[doc = "Bit 1 - P6DS1"]
    #[inline(always)]
    pub fn p6ds1(&mut self) -> P6DS1_W {
        P6DS1_W { w: self }
    }
    #[doc = "Bit 2 - P6DS2"]
    #[inline(always)]
    pub fn p6ds2(&mut self) -> P6DS2_W {
        P6DS2_W { w: self }
    }
    #[doc = "Bit 3 - P6DS3"]
    #[inline(always)]
    pub fn p6ds3(&mut self) -> P6DS3_W {
        P6DS3_W { w: self }
    }
    #[doc = "Bit 4 - P6DS4"]
    #[inline(always)]
    pub fn p6ds4(&mut self) -> P6DS4_W {
        P6DS4_W { w: self }
    }
    #[doc = "Bit 5 - P6DS5"]
    #[inline(always)]
    pub fn p6ds5(&mut self) -> P6DS5_W {
        P6DS5_W { w: self }
    }
    #[doc = "Bit 6 - P6DS6"]
    #[inline(always)]
    pub fn p6ds6(&mut self) -> P6DS6_W {
        P6DS6_W { w: self }
    }
    #[doc = "Bit 7 - P6DS7"]
    #[inline(always)]
    pub fn p6ds7(&mut self) -> P6DS7_W {
        P6DS7_W { w: self }
    }
}
