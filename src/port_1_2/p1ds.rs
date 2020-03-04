#[doc = "Reader of register P1DS"]
pub type R = crate::R<u8, super::P1DS>;
#[doc = "Writer for register P1DS"]
pub type W = crate::W<u8, super::P1DS>;
#[doc = "Register P1DS `reset()`'s with value 0"]
impl crate::ResetValue for super::P1DS {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P1DS0`"]
pub type P1DS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1DS0`"]
pub struct P1DS0_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DS0_W<'a> {
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
#[doc = "Reader of field `P1DS1`"]
pub type P1DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1DS1`"]
pub struct P1DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DS1_W<'a> {
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
#[doc = "Reader of field `P1DS2`"]
pub type P1DS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1DS2`"]
pub struct P1DS2_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DS2_W<'a> {
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
#[doc = "Reader of field `P1DS3`"]
pub type P1DS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1DS3`"]
pub struct P1DS3_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DS3_W<'a> {
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
#[doc = "Reader of field `P1DS4`"]
pub type P1DS4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1DS4`"]
pub struct P1DS4_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DS4_W<'a> {
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
#[doc = "Reader of field `P1DS5`"]
pub type P1DS5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1DS5`"]
pub struct P1DS5_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DS5_W<'a> {
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
#[doc = "Reader of field `P1DS6`"]
pub type P1DS6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1DS6`"]
pub struct P1DS6_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DS6_W<'a> {
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
#[doc = "Reader of field `P1DS7`"]
pub type P1DS7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1DS7`"]
pub struct P1DS7_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DS7_W<'a> {
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
    #[doc = "Bit 0 - P1DS0"]
    #[inline(always)]
    pub fn p1ds0(&self) -> P1DS0_R {
        P1DS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P1DS1"]
    #[inline(always)]
    pub fn p1ds1(&self) -> P1DS1_R {
        P1DS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P1DS2"]
    #[inline(always)]
    pub fn p1ds2(&self) -> P1DS2_R {
        P1DS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P1DS3"]
    #[inline(always)]
    pub fn p1ds3(&self) -> P1DS3_R {
        P1DS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P1DS4"]
    #[inline(always)]
    pub fn p1ds4(&self) -> P1DS4_R {
        P1DS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P1DS5"]
    #[inline(always)]
    pub fn p1ds5(&self) -> P1DS5_R {
        P1DS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P1DS6"]
    #[inline(always)]
    pub fn p1ds6(&self) -> P1DS6_R {
        P1DS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P1DS7"]
    #[inline(always)]
    pub fn p1ds7(&self) -> P1DS7_R {
        P1DS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1DS0"]
    #[inline(always)]
    pub fn p1ds0(&mut self) -> P1DS0_W {
        P1DS0_W { w: self }
    }
    #[doc = "Bit 1 - P1DS1"]
    #[inline(always)]
    pub fn p1ds1(&mut self) -> P1DS1_W {
        P1DS1_W { w: self }
    }
    #[doc = "Bit 2 - P1DS2"]
    #[inline(always)]
    pub fn p1ds2(&mut self) -> P1DS2_W {
        P1DS2_W { w: self }
    }
    #[doc = "Bit 3 - P1DS3"]
    #[inline(always)]
    pub fn p1ds3(&mut self) -> P1DS3_W {
        P1DS3_W { w: self }
    }
    #[doc = "Bit 4 - P1DS4"]
    #[inline(always)]
    pub fn p1ds4(&mut self) -> P1DS4_W {
        P1DS4_W { w: self }
    }
    #[doc = "Bit 5 - P1DS5"]
    #[inline(always)]
    pub fn p1ds5(&mut self) -> P1DS5_W {
        P1DS5_W { w: self }
    }
    #[doc = "Bit 6 - P1DS6"]
    #[inline(always)]
    pub fn p1ds6(&mut self) -> P1DS6_W {
        P1DS6_W { w: self }
    }
    #[doc = "Bit 7 - P1DS7"]
    #[inline(always)]
    pub fn p1ds7(&mut self) -> P1DS7_W {
        P1DS7_W { w: self }
    }
}
