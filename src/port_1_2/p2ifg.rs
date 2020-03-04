#[doc = "Reader of register P2IFG"]
pub type R = crate::R<u8, super::P2IFG>;
#[doc = "Writer for register P2IFG"]
pub type W = crate::W<u8, super::P2IFG>;
#[doc = "Register P2IFG `reset()`'s with value 0"]
impl crate::ResetValue for super::P2IFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P2IFG0`"]
pub type P2IFG0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IFG0`"]
pub struct P2IFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IFG0_W<'a> {
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
#[doc = "Reader of field `P2IFG1`"]
pub type P2IFG1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IFG1`"]
pub struct P2IFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IFG1_W<'a> {
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
#[doc = "Reader of field `P2IFG2`"]
pub type P2IFG2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IFG2`"]
pub struct P2IFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IFG2_W<'a> {
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
#[doc = "Reader of field `P2IFG3`"]
pub type P2IFG3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IFG3`"]
pub struct P2IFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IFG3_W<'a> {
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
#[doc = "Reader of field `P2IFG4`"]
pub type P2IFG4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IFG4`"]
pub struct P2IFG4_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IFG4_W<'a> {
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
#[doc = "Reader of field `P2IFG5`"]
pub type P2IFG5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IFG5`"]
pub struct P2IFG5_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IFG5_W<'a> {
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
#[doc = "Reader of field `P2IFG6`"]
pub type P2IFG6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IFG6`"]
pub struct P2IFG6_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IFG6_W<'a> {
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
#[doc = "Reader of field `P2IFG7`"]
pub type P2IFG7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IFG7`"]
pub struct P2IFG7_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IFG7_W<'a> {
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
    #[doc = "Bit 0 - P2IFG0"]
    #[inline(always)]
    pub fn p2ifg0(&self) -> P2IFG0_R {
        P2IFG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P2IFG1"]
    #[inline(always)]
    pub fn p2ifg1(&self) -> P2IFG1_R {
        P2IFG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P2IFG2"]
    #[inline(always)]
    pub fn p2ifg2(&self) -> P2IFG2_R {
        P2IFG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P2IFG3"]
    #[inline(always)]
    pub fn p2ifg3(&self) -> P2IFG3_R {
        P2IFG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P2IFG4"]
    #[inline(always)]
    pub fn p2ifg4(&self) -> P2IFG4_R {
        P2IFG4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P2IFG5"]
    #[inline(always)]
    pub fn p2ifg5(&self) -> P2IFG5_R {
        P2IFG5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P2IFG6"]
    #[inline(always)]
    pub fn p2ifg6(&self) -> P2IFG6_R {
        P2IFG6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P2IFG7"]
    #[inline(always)]
    pub fn p2ifg7(&self) -> P2IFG7_R {
        P2IFG7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2IFG0"]
    #[inline(always)]
    pub fn p2ifg0(&mut self) -> P2IFG0_W {
        P2IFG0_W { w: self }
    }
    #[doc = "Bit 1 - P2IFG1"]
    #[inline(always)]
    pub fn p2ifg1(&mut self) -> P2IFG1_W {
        P2IFG1_W { w: self }
    }
    #[doc = "Bit 2 - P2IFG2"]
    #[inline(always)]
    pub fn p2ifg2(&mut self) -> P2IFG2_W {
        P2IFG2_W { w: self }
    }
    #[doc = "Bit 3 - P2IFG3"]
    #[inline(always)]
    pub fn p2ifg3(&mut self) -> P2IFG3_W {
        P2IFG3_W { w: self }
    }
    #[doc = "Bit 4 - P2IFG4"]
    #[inline(always)]
    pub fn p2ifg4(&mut self) -> P2IFG4_W {
        P2IFG4_W { w: self }
    }
    #[doc = "Bit 5 - P2IFG5"]
    #[inline(always)]
    pub fn p2ifg5(&mut self) -> P2IFG5_W {
        P2IFG5_W { w: self }
    }
    #[doc = "Bit 6 - P2IFG6"]
    #[inline(always)]
    pub fn p2ifg6(&mut self) -> P2IFG6_W {
        P2IFG6_W { w: self }
    }
    #[doc = "Bit 7 - P2IFG7"]
    #[inline(always)]
    pub fn p2ifg7(&mut self) -> P2IFG7_W {
        P2IFG7_W { w: self }
    }
}
