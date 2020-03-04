#[doc = "Reader of register P7DS"]
pub type R = crate::R<u8, super::P7DS>;
#[doc = "Writer for register P7DS"]
pub type W = crate::W<u8, super::P7DS>;
#[doc = "Register P7DS `reset()`'s with value 0"]
impl crate::ResetValue for super::P7DS {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P7DS0`"]
pub type P7DS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7DS0`"]
pub struct P7DS0_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DS0_W<'a> {
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
#[doc = "Reader of field `P7DS1`"]
pub type P7DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7DS1`"]
pub struct P7DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DS1_W<'a> {
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
#[doc = "Reader of field `P7DS2`"]
pub type P7DS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7DS2`"]
pub struct P7DS2_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DS2_W<'a> {
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
#[doc = "Reader of field `P7DS3`"]
pub type P7DS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7DS3`"]
pub struct P7DS3_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DS3_W<'a> {
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
#[doc = "Reader of field `P7DS4`"]
pub type P7DS4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7DS4`"]
pub struct P7DS4_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DS4_W<'a> {
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
#[doc = "Reader of field `P7DS5`"]
pub type P7DS5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7DS5`"]
pub struct P7DS5_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DS5_W<'a> {
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
#[doc = "Reader of field `P7DS6`"]
pub type P7DS6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7DS6`"]
pub struct P7DS6_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DS6_W<'a> {
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
#[doc = "Reader of field `P7DS7`"]
pub type P7DS7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7DS7`"]
pub struct P7DS7_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DS7_W<'a> {
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
    #[doc = "Bit 0 - P7DS0"]
    #[inline(always)]
    pub fn p7ds0(&self) -> P7DS0_R {
        P7DS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P7DS1"]
    #[inline(always)]
    pub fn p7ds1(&self) -> P7DS1_R {
        P7DS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P7DS2"]
    #[inline(always)]
    pub fn p7ds2(&self) -> P7DS2_R {
        P7DS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P7DS3"]
    #[inline(always)]
    pub fn p7ds3(&self) -> P7DS3_R {
        P7DS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P7DS4"]
    #[inline(always)]
    pub fn p7ds4(&self) -> P7DS4_R {
        P7DS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P7DS5"]
    #[inline(always)]
    pub fn p7ds5(&self) -> P7DS5_R {
        P7DS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P7DS6"]
    #[inline(always)]
    pub fn p7ds6(&self) -> P7DS6_R {
        P7DS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P7DS7"]
    #[inline(always)]
    pub fn p7ds7(&self) -> P7DS7_R {
        P7DS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P7DS0"]
    #[inline(always)]
    pub fn p7ds0(&mut self) -> P7DS0_W {
        P7DS0_W { w: self }
    }
    #[doc = "Bit 1 - P7DS1"]
    #[inline(always)]
    pub fn p7ds1(&mut self) -> P7DS1_W {
        P7DS1_W { w: self }
    }
    #[doc = "Bit 2 - P7DS2"]
    #[inline(always)]
    pub fn p7ds2(&mut self) -> P7DS2_W {
        P7DS2_W { w: self }
    }
    #[doc = "Bit 3 - P7DS3"]
    #[inline(always)]
    pub fn p7ds3(&mut self) -> P7DS3_W {
        P7DS3_W { w: self }
    }
    #[doc = "Bit 4 - P7DS4"]
    #[inline(always)]
    pub fn p7ds4(&mut self) -> P7DS4_W {
        P7DS4_W { w: self }
    }
    #[doc = "Bit 5 - P7DS5"]
    #[inline(always)]
    pub fn p7ds5(&mut self) -> P7DS5_W {
        P7DS5_W { w: self }
    }
    #[doc = "Bit 6 - P7DS6"]
    #[inline(always)]
    pub fn p7ds6(&mut self) -> P7DS6_W {
        P7DS6_W { w: self }
    }
    #[doc = "Bit 7 - P7DS7"]
    #[inline(always)]
    pub fn p7ds7(&mut self) -> P7DS7_W {
        P7DS7_W { w: self }
    }
}
