#[doc = "Reader of register P2IE"]
pub type R = crate::R<u8, super::P2IE>;
#[doc = "Writer for register P2IE"]
pub type W = crate::W<u8, super::P2IE>;
#[doc = "Register P2IE `reset()`'s with value 0"]
impl crate::ResetValue for super::P2IE {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P2IE0`"]
pub type P2IE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IE0`"]
pub struct P2IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IE0_W<'a> {
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
#[doc = "Reader of field `P2IE1`"]
pub type P2IE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IE1`"]
pub struct P2IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IE1_W<'a> {
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
#[doc = "Reader of field `P2IE2`"]
pub type P2IE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IE2`"]
pub struct P2IE2_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IE2_W<'a> {
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
#[doc = "Reader of field `P2IE3`"]
pub type P2IE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IE3`"]
pub struct P2IE3_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IE3_W<'a> {
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
#[doc = "Reader of field `P2IE4`"]
pub type P2IE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IE4`"]
pub struct P2IE4_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IE4_W<'a> {
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
#[doc = "Reader of field `P2IE5`"]
pub type P2IE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IE5`"]
pub struct P2IE5_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IE5_W<'a> {
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
#[doc = "Reader of field `P2IE6`"]
pub type P2IE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IE6`"]
pub struct P2IE6_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IE6_W<'a> {
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
#[doc = "Reader of field `P2IE7`"]
pub type P2IE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IE7`"]
pub struct P2IE7_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IE7_W<'a> {
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
    #[doc = "Bit 0 - P2IE0"]
    #[inline(always)]
    pub fn p2ie0(&self) -> P2IE0_R {
        P2IE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P2IE1"]
    #[inline(always)]
    pub fn p2ie1(&self) -> P2IE1_R {
        P2IE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P2IE2"]
    #[inline(always)]
    pub fn p2ie2(&self) -> P2IE2_R {
        P2IE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P2IE3"]
    #[inline(always)]
    pub fn p2ie3(&self) -> P2IE3_R {
        P2IE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P2IE4"]
    #[inline(always)]
    pub fn p2ie4(&self) -> P2IE4_R {
        P2IE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P2IE5"]
    #[inline(always)]
    pub fn p2ie5(&self) -> P2IE5_R {
        P2IE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P2IE6"]
    #[inline(always)]
    pub fn p2ie6(&self) -> P2IE6_R {
        P2IE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P2IE7"]
    #[inline(always)]
    pub fn p2ie7(&self) -> P2IE7_R {
        P2IE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2IE0"]
    #[inline(always)]
    pub fn p2ie0(&mut self) -> P2IE0_W {
        P2IE0_W { w: self }
    }
    #[doc = "Bit 1 - P2IE1"]
    #[inline(always)]
    pub fn p2ie1(&mut self) -> P2IE1_W {
        P2IE1_W { w: self }
    }
    #[doc = "Bit 2 - P2IE2"]
    #[inline(always)]
    pub fn p2ie2(&mut self) -> P2IE2_W {
        P2IE2_W { w: self }
    }
    #[doc = "Bit 3 - P2IE3"]
    #[inline(always)]
    pub fn p2ie3(&mut self) -> P2IE3_W {
        P2IE3_W { w: self }
    }
    #[doc = "Bit 4 - P2IE4"]
    #[inline(always)]
    pub fn p2ie4(&mut self) -> P2IE4_W {
        P2IE4_W { w: self }
    }
    #[doc = "Bit 5 - P2IE5"]
    #[inline(always)]
    pub fn p2ie5(&mut self) -> P2IE5_W {
        P2IE5_W { w: self }
    }
    #[doc = "Bit 6 - P2IE6"]
    #[inline(always)]
    pub fn p2ie6(&mut self) -> P2IE6_W {
        P2IE6_W { w: self }
    }
    #[doc = "Bit 7 - P2IE7"]
    #[inline(always)]
    pub fn p2ie7(&mut self) -> P2IE7_W {
        P2IE7_W { w: self }
    }
}
