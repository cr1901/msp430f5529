#[doc = "Reader of register P3DS"]
pub type R = crate::R<u8, super::P3DS>;
#[doc = "Writer for register P3DS"]
pub type W = crate::W<u8, super::P3DS>;
#[doc = "Register P3DS `reset()`'s with value 0"]
impl crate::ResetValue for super::P3DS {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P3DS0`"]
pub type P3DS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3DS0`"]
pub struct P3DS0_W<'a> {
    w: &'a mut W,
}
impl<'a> P3DS0_W<'a> {
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
#[doc = "Reader of field `P3DS1`"]
pub type P3DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3DS1`"]
pub struct P3DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> P3DS1_W<'a> {
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
#[doc = "Reader of field `P3DS2`"]
pub type P3DS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3DS2`"]
pub struct P3DS2_W<'a> {
    w: &'a mut W,
}
impl<'a> P3DS2_W<'a> {
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
#[doc = "Reader of field `P3DS3`"]
pub type P3DS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3DS3`"]
pub struct P3DS3_W<'a> {
    w: &'a mut W,
}
impl<'a> P3DS3_W<'a> {
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
#[doc = "Reader of field `P3DS4`"]
pub type P3DS4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3DS4`"]
pub struct P3DS4_W<'a> {
    w: &'a mut W,
}
impl<'a> P3DS4_W<'a> {
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
#[doc = "Reader of field `P3DS5`"]
pub type P3DS5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3DS5`"]
pub struct P3DS5_W<'a> {
    w: &'a mut W,
}
impl<'a> P3DS5_W<'a> {
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
#[doc = "Reader of field `P3DS6`"]
pub type P3DS6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3DS6`"]
pub struct P3DS6_W<'a> {
    w: &'a mut W,
}
impl<'a> P3DS6_W<'a> {
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
#[doc = "Reader of field `P3DS7`"]
pub type P3DS7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3DS7`"]
pub struct P3DS7_W<'a> {
    w: &'a mut W,
}
impl<'a> P3DS7_W<'a> {
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
    #[doc = "Bit 0 - P3DS0"]
    #[inline(always)]
    pub fn p3ds0(&self) -> P3DS0_R {
        P3DS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P3DS1"]
    #[inline(always)]
    pub fn p3ds1(&self) -> P3DS1_R {
        P3DS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P3DS2"]
    #[inline(always)]
    pub fn p3ds2(&self) -> P3DS2_R {
        P3DS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P3DS3"]
    #[inline(always)]
    pub fn p3ds3(&self) -> P3DS3_R {
        P3DS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P3DS4"]
    #[inline(always)]
    pub fn p3ds4(&self) -> P3DS4_R {
        P3DS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P3DS5"]
    #[inline(always)]
    pub fn p3ds5(&self) -> P3DS5_R {
        P3DS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P3DS6"]
    #[inline(always)]
    pub fn p3ds6(&self) -> P3DS6_R {
        P3DS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P3DS7"]
    #[inline(always)]
    pub fn p3ds7(&self) -> P3DS7_R {
        P3DS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3DS0"]
    #[inline(always)]
    pub fn p3ds0(&mut self) -> P3DS0_W {
        P3DS0_W { w: self }
    }
    #[doc = "Bit 1 - P3DS1"]
    #[inline(always)]
    pub fn p3ds1(&mut self) -> P3DS1_W {
        P3DS1_W { w: self }
    }
    #[doc = "Bit 2 - P3DS2"]
    #[inline(always)]
    pub fn p3ds2(&mut self) -> P3DS2_W {
        P3DS2_W { w: self }
    }
    #[doc = "Bit 3 - P3DS3"]
    #[inline(always)]
    pub fn p3ds3(&mut self) -> P3DS3_W {
        P3DS3_W { w: self }
    }
    #[doc = "Bit 4 - P3DS4"]
    #[inline(always)]
    pub fn p3ds4(&mut self) -> P3DS4_W {
        P3DS4_W { w: self }
    }
    #[doc = "Bit 5 - P3DS5"]
    #[inline(always)]
    pub fn p3ds5(&mut self) -> P3DS5_W {
        P3DS5_W { w: self }
    }
    #[doc = "Bit 6 - P3DS6"]
    #[inline(always)]
    pub fn p3ds6(&mut self) -> P3DS6_W {
        P3DS6_W { w: self }
    }
    #[doc = "Bit 7 - P3DS7"]
    #[inline(always)]
    pub fn p3ds7(&mut self) -> P3DS7_W {
        P3DS7_W { w: self }
    }
}
