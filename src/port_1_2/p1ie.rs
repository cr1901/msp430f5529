#[doc = "Reader of register P1IE"]
pub type R = crate::R<u8, super::P1IE>;
#[doc = "Writer for register P1IE"]
pub type W = crate::W<u8, super::P1IE>;
#[doc = "Register P1IE `reset()`'s with value 0"]
impl crate::ResetValue for super::P1IE {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P1IE0`"]
pub type P1IE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IE0`"]
pub struct P1IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IE0_W<'a> {
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
#[doc = "Reader of field `P1IE1`"]
pub type P1IE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IE1`"]
pub struct P1IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IE1_W<'a> {
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
#[doc = "Reader of field `P1IE2`"]
pub type P1IE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IE2`"]
pub struct P1IE2_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IE2_W<'a> {
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
#[doc = "Reader of field `P1IE3`"]
pub type P1IE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IE3`"]
pub struct P1IE3_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IE3_W<'a> {
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
#[doc = "Reader of field `P1IE4`"]
pub type P1IE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IE4`"]
pub struct P1IE4_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IE4_W<'a> {
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
#[doc = "Reader of field `P1IE5`"]
pub type P1IE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IE5`"]
pub struct P1IE5_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IE5_W<'a> {
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
#[doc = "Reader of field `P1IE6`"]
pub type P1IE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IE6`"]
pub struct P1IE6_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IE6_W<'a> {
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
#[doc = "Reader of field `P1IE7`"]
pub type P1IE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IE7`"]
pub struct P1IE7_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IE7_W<'a> {
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
    #[doc = "Bit 0 - P1IE0"]
    #[inline(always)]
    pub fn p1ie0(&self) -> P1IE0_R {
        P1IE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P1IE1"]
    #[inline(always)]
    pub fn p1ie1(&self) -> P1IE1_R {
        P1IE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P1IE2"]
    #[inline(always)]
    pub fn p1ie2(&self) -> P1IE2_R {
        P1IE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P1IE3"]
    #[inline(always)]
    pub fn p1ie3(&self) -> P1IE3_R {
        P1IE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P1IE4"]
    #[inline(always)]
    pub fn p1ie4(&self) -> P1IE4_R {
        P1IE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P1IE5"]
    #[inline(always)]
    pub fn p1ie5(&self) -> P1IE5_R {
        P1IE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P1IE6"]
    #[inline(always)]
    pub fn p1ie6(&self) -> P1IE6_R {
        P1IE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P1IE7"]
    #[inline(always)]
    pub fn p1ie7(&self) -> P1IE7_R {
        P1IE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1IE0"]
    #[inline(always)]
    pub fn p1ie0(&mut self) -> P1IE0_W {
        P1IE0_W { w: self }
    }
    #[doc = "Bit 1 - P1IE1"]
    #[inline(always)]
    pub fn p1ie1(&mut self) -> P1IE1_W {
        P1IE1_W { w: self }
    }
    #[doc = "Bit 2 - P1IE2"]
    #[inline(always)]
    pub fn p1ie2(&mut self) -> P1IE2_W {
        P1IE2_W { w: self }
    }
    #[doc = "Bit 3 - P1IE3"]
    #[inline(always)]
    pub fn p1ie3(&mut self) -> P1IE3_W {
        P1IE3_W { w: self }
    }
    #[doc = "Bit 4 - P1IE4"]
    #[inline(always)]
    pub fn p1ie4(&mut self) -> P1IE4_W {
        P1IE4_W { w: self }
    }
    #[doc = "Bit 5 - P1IE5"]
    #[inline(always)]
    pub fn p1ie5(&mut self) -> P1IE5_W {
        P1IE5_W { w: self }
    }
    #[doc = "Bit 6 - P1IE6"]
    #[inline(always)]
    pub fn p1ie6(&mut self) -> P1IE6_W {
        P1IE6_W { w: self }
    }
    #[doc = "Bit 7 - P1IE7"]
    #[inline(always)]
    pub fn p1ie7(&mut self) -> P1IE7_W {
        P1IE7_W { w: self }
    }
}
