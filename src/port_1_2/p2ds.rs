#[doc = "Reader of register P2DS"]
pub type R = crate::R<u8, super::P2DS>;
#[doc = "Writer for register P2DS"]
pub type W = crate::W<u8, super::P2DS>;
#[doc = "Register P2DS `reset()`'s with value 0"]
impl crate::ResetValue for super::P2DS {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P2DS0`"]
pub type P2DS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2DS0`"]
pub struct P2DS0_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DS0_W<'a> {
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
#[doc = "Reader of field `P2DS1`"]
pub type P2DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2DS1`"]
pub struct P2DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DS1_W<'a> {
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
#[doc = "Reader of field `P2DS2`"]
pub type P2DS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2DS2`"]
pub struct P2DS2_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DS2_W<'a> {
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
#[doc = "Reader of field `P2DS3`"]
pub type P2DS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2DS3`"]
pub struct P2DS3_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DS3_W<'a> {
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
#[doc = "Reader of field `P2DS4`"]
pub type P2DS4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2DS4`"]
pub struct P2DS4_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DS4_W<'a> {
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
#[doc = "Reader of field `P2DS5`"]
pub type P2DS5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2DS5`"]
pub struct P2DS5_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DS5_W<'a> {
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
#[doc = "Reader of field `P2DS6`"]
pub type P2DS6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2DS6`"]
pub struct P2DS6_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DS6_W<'a> {
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
#[doc = "Reader of field `P2DS7`"]
pub type P2DS7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2DS7`"]
pub struct P2DS7_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DS7_W<'a> {
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
    #[doc = "Bit 0 - P2DS0"]
    #[inline(always)]
    pub fn p2ds0(&self) -> P2DS0_R {
        P2DS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P2DS1"]
    #[inline(always)]
    pub fn p2ds1(&self) -> P2DS1_R {
        P2DS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P2DS2"]
    #[inline(always)]
    pub fn p2ds2(&self) -> P2DS2_R {
        P2DS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P2DS3"]
    #[inline(always)]
    pub fn p2ds3(&self) -> P2DS3_R {
        P2DS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P2DS4"]
    #[inline(always)]
    pub fn p2ds4(&self) -> P2DS4_R {
        P2DS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P2DS5"]
    #[inline(always)]
    pub fn p2ds5(&self) -> P2DS5_R {
        P2DS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P2DS6"]
    #[inline(always)]
    pub fn p2ds6(&self) -> P2DS6_R {
        P2DS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P2DS7"]
    #[inline(always)]
    pub fn p2ds7(&self) -> P2DS7_R {
        P2DS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2DS0"]
    #[inline(always)]
    pub fn p2ds0(&mut self) -> P2DS0_W {
        P2DS0_W { w: self }
    }
    #[doc = "Bit 1 - P2DS1"]
    #[inline(always)]
    pub fn p2ds1(&mut self) -> P2DS1_W {
        P2DS1_W { w: self }
    }
    #[doc = "Bit 2 - P2DS2"]
    #[inline(always)]
    pub fn p2ds2(&mut self) -> P2DS2_W {
        P2DS2_W { w: self }
    }
    #[doc = "Bit 3 - P2DS3"]
    #[inline(always)]
    pub fn p2ds3(&mut self) -> P2DS3_W {
        P2DS3_W { w: self }
    }
    #[doc = "Bit 4 - P2DS4"]
    #[inline(always)]
    pub fn p2ds4(&mut self) -> P2DS4_W {
        P2DS4_W { w: self }
    }
    #[doc = "Bit 5 - P2DS5"]
    #[inline(always)]
    pub fn p2ds5(&mut self) -> P2DS5_W {
        P2DS5_W { w: self }
    }
    #[doc = "Bit 6 - P2DS6"]
    #[inline(always)]
    pub fn p2ds6(&mut self) -> P2DS6_W {
        P2DS6_W { w: self }
    }
    #[doc = "Bit 7 - P2DS7"]
    #[inline(always)]
    pub fn p2ds7(&mut self) -> P2DS7_W {
        P2DS7_W { w: self }
    }
}
