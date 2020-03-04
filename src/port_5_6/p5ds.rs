#[doc = "Reader of register P5DS"]
pub type R = crate::R<u8, super::P5DS>;
#[doc = "Writer for register P5DS"]
pub type W = crate::W<u8, super::P5DS>;
#[doc = "Register P5DS `reset()`'s with value 0"]
impl crate::ResetValue for super::P5DS {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P5DS0`"]
pub type P5DS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5DS0`"]
pub struct P5DS0_W<'a> {
    w: &'a mut W,
}
impl<'a> P5DS0_W<'a> {
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
#[doc = "Reader of field `P5DS1`"]
pub type P5DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5DS1`"]
pub struct P5DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> P5DS1_W<'a> {
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
#[doc = "Reader of field `P5DS2`"]
pub type P5DS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5DS2`"]
pub struct P5DS2_W<'a> {
    w: &'a mut W,
}
impl<'a> P5DS2_W<'a> {
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
#[doc = "Reader of field `P5DS3`"]
pub type P5DS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5DS3`"]
pub struct P5DS3_W<'a> {
    w: &'a mut W,
}
impl<'a> P5DS3_W<'a> {
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
#[doc = "Reader of field `P5DS4`"]
pub type P5DS4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5DS4`"]
pub struct P5DS4_W<'a> {
    w: &'a mut W,
}
impl<'a> P5DS4_W<'a> {
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
#[doc = "Reader of field `P5DS5`"]
pub type P5DS5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5DS5`"]
pub struct P5DS5_W<'a> {
    w: &'a mut W,
}
impl<'a> P5DS5_W<'a> {
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
#[doc = "Reader of field `P5DS6`"]
pub type P5DS6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5DS6`"]
pub struct P5DS6_W<'a> {
    w: &'a mut W,
}
impl<'a> P5DS6_W<'a> {
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
#[doc = "Reader of field `P5DS7`"]
pub type P5DS7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5DS7`"]
pub struct P5DS7_W<'a> {
    w: &'a mut W,
}
impl<'a> P5DS7_W<'a> {
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
    #[doc = "Bit 0 - P5DS0"]
    #[inline(always)]
    pub fn p5ds0(&self) -> P5DS0_R {
        P5DS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P5DS1"]
    #[inline(always)]
    pub fn p5ds1(&self) -> P5DS1_R {
        P5DS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P5DS2"]
    #[inline(always)]
    pub fn p5ds2(&self) -> P5DS2_R {
        P5DS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P5DS3"]
    #[inline(always)]
    pub fn p5ds3(&self) -> P5DS3_R {
        P5DS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P5DS4"]
    #[inline(always)]
    pub fn p5ds4(&self) -> P5DS4_R {
        P5DS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P5DS5"]
    #[inline(always)]
    pub fn p5ds5(&self) -> P5DS5_R {
        P5DS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P5DS6"]
    #[inline(always)]
    pub fn p5ds6(&self) -> P5DS6_R {
        P5DS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P5DS7"]
    #[inline(always)]
    pub fn p5ds7(&self) -> P5DS7_R {
        P5DS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P5DS0"]
    #[inline(always)]
    pub fn p5ds0(&mut self) -> P5DS0_W {
        P5DS0_W { w: self }
    }
    #[doc = "Bit 1 - P5DS1"]
    #[inline(always)]
    pub fn p5ds1(&mut self) -> P5DS1_W {
        P5DS1_W { w: self }
    }
    #[doc = "Bit 2 - P5DS2"]
    #[inline(always)]
    pub fn p5ds2(&mut self) -> P5DS2_W {
        P5DS2_W { w: self }
    }
    #[doc = "Bit 3 - P5DS3"]
    #[inline(always)]
    pub fn p5ds3(&mut self) -> P5DS3_W {
        P5DS3_W { w: self }
    }
    #[doc = "Bit 4 - P5DS4"]
    #[inline(always)]
    pub fn p5ds4(&mut self) -> P5DS4_W {
        P5DS4_W { w: self }
    }
    #[doc = "Bit 5 - P5DS5"]
    #[inline(always)]
    pub fn p5ds5(&mut self) -> P5DS5_W {
        P5DS5_W { w: self }
    }
    #[doc = "Bit 6 - P5DS6"]
    #[inline(always)]
    pub fn p5ds6(&mut self) -> P5DS6_W {
        P5DS6_W { w: self }
    }
    #[doc = "Bit 7 - P5DS7"]
    #[inline(always)]
    pub fn p5ds7(&mut self) -> P5DS7_W {
        P5DS7_W { w: self }
    }
}
