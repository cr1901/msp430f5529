#[doc = "Reader of register P8DS"]
pub type R = crate::R<u8, super::P8DS>;
#[doc = "Writer for register P8DS"]
pub type W = crate::W<u8, super::P8DS>;
#[doc = "Register P8DS `reset()`'s with value 0"]
impl crate::ResetValue for super::P8DS {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P8DS0`"]
pub type P8DS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8DS0`"]
pub struct P8DS0_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DS0_W<'a> {
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
#[doc = "Reader of field `P8DS1`"]
pub type P8DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8DS1`"]
pub struct P8DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DS1_W<'a> {
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
#[doc = "Reader of field `P8DS2`"]
pub type P8DS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8DS2`"]
pub struct P8DS2_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DS2_W<'a> {
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
#[doc = "Reader of field `P8DS3`"]
pub type P8DS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8DS3`"]
pub struct P8DS3_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DS3_W<'a> {
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
#[doc = "Reader of field `P8DS4`"]
pub type P8DS4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8DS4`"]
pub struct P8DS4_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DS4_W<'a> {
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
#[doc = "Reader of field `P8DS5`"]
pub type P8DS5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8DS5`"]
pub struct P8DS5_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DS5_W<'a> {
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
#[doc = "Reader of field `P8DS6`"]
pub type P8DS6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8DS6`"]
pub struct P8DS6_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DS6_W<'a> {
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
#[doc = "Reader of field `P8DS7`"]
pub type P8DS7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8DS7`"]
pub struct P8DS7_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DS7_W<'a> {
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
    #[doc = "Bit 0 - P8DS0"]
    #[inline(always)]
    pub fn p8ds0(&self) -> P8DS0_R {
        P8DS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P8DS1"]
    #[inline(always)]
    pub fn p8ds1(&self) -> P8DS1_R {
        P8DS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P8DS2"]
    #[inline(always)]
    pub fn p8ds2(&self) -> P8DS2_R {
        P8DS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P8DS3"]
    #[inline(always)]
    pub fn p8ds3(&self) -> P8DS3_R {
        P8DS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P8DS4"]
    #[inline(always)]
    pub fn p8ds4(&self) -> P8DS4_R {
        P8DS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P8DS5"]
    #[inline(always)]
    pub fn p8ds5(&self) -> P8DS5_R {
        P8DS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P8DS6"]
    #[inline(always)]
    pub fn p8ds6(&self) -> P8DS6_R {
        P8DS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P8DS7"]
    #[inline(always)]
    pub fn p8ds7(&self) -> P8DS7_R {
        P8DS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P8DS0"]
    #[inline(always)]
    pub fn p8ds0(&mut self) -> P8DS0_W {
        P8DS0_W { w: self }
    }
    #[doc = "Bit 1 - P8DS1"]
    #[inline(always)]
    pub fn p8ds1(&mut self) -> P8DS1_W {
        P8DS1_W { w: self }
    }
    #[doc = "Bit 2 - P8DS2"]
    #[inline(always)]
    pub fn p8ds2(&mut self) -> P8DS2_W {
        P8DS2_W { w: self }
    }
    #[doc = "Bit 3 - P8DS3"]
    #[inline(always)]
    pub fn p8ds3(&mut self) -> P8DS3_W {
        P8DS3_W { w: self }
    }
    #[doc = "Bit 4 - P8DS4"]
    #[inline(always)]
    pub fn p8ds4(&mut self) -> P8DS4_W {
        P8DS4_W { w: self }
    }
    #[doc = "Bit 5 - P8DS5"]
    #[inline(always)]
    pub fn p8ds5(&mut self) -> P8DS5_W {
        P8DS5_W { w: self }
    }
    #[doc = "Bit 6 - P8DS6"]
    #[inline(always)]
    pub fn p8ds6(&mut self) -> P8DS6_W {
        P8DS6_W { w: self }
    }
    #[doc = "Bit 7 - P8DS7"]
    #[inline(always)]
    pub fn p8ds7(&mut self) -> P8DS7_W {
        P8DS7_W { w: self }
    }
}
