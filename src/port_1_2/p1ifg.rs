#[doc = "Reader of register P1IFG"]
pub type R = crate::R<u8, super::P1IFG>;
#[doc = "Writer for register P1IFG"]
pub type W = crate::W<u8, super::P1IFG>;
#[doc = "Register P1IFG `reset()`'s with value 0"]
impl crate::ResetValue for super::P1IFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P1IFG0`"]
pub type P1IFG0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IFG0`"]
pub struct P1IFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IFG0_W<'a> {
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
#[doc = "Reader of field `P1IFG1`"]
pub type P1IFG1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IFG1`"]
pub struct P1IFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IFG1_W<'a> {
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
#[doc = "Reader of field `P1IFG2`"]
pub type P1IFG2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IFG2`"]
pub struct P1IFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IFG2_W<'a> {
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
#[doc = "Reader of field `P1IFG3`"]
pub type P1IFG3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IFG3`"]
pub struct P1IFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IFG3_W<'a> {
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
#[doc = "Reader of field `P1IFG4`"]
pub type P1IFG4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IFG4`"]
pub struct P1IFG4_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IFG4_W<'a> {
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
#[doc = "Reader of field `P1IFG5`"]
pub type P1IFG5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IFG5`"]
pub struct P1IFG5_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IFG5_W<'a> {
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
#[doc = "Reader of field `P1IFG6`"]
pub type P1IFG6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IFG6`"]
pub struct P1IFG6_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IFG6_W<'a> {
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
#[doc = "Reader of field `P1IFG7`"]
pub type P1IFG7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IFG7`"]
pub struct P1IFG7_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IFG7_W<'a> {
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
    #[doc = "Bit 0 - P1IFG0"]
    #[inline(always)]
    pub fn p1ifg0(&self) -> P1IFG0_R {
        P1IFG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P1IFG1"]
    #[inline(always)]
    pub fn p1ifg1(&self) -> P1IFG1_R {
        P1IFG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P1IFG2"]
    #[inline(always)]
    pub fn p1ifg2(&self) -> P1IFG2_R {
        P1IFG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P1IFG3"]
    #[inline(always)]
    pub fn p1ifg3(&self) -> P1IFG3_R {
        P1IFG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P1IFG4"]
    #[inline(always)]
    pub fn p1ifg4(&self) -> P1IFG4_R {
        P1IFG4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P1IFG5"]
    #[inline(always)]
    pub fn p1ifg5(&self) -> P1IFG5_R {
        P1IFG5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P1IFG6"]
    #[inline(always)]
    pub fn p1ifg6(&self) -> P1IFG6_R {
        P1IFG6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P1IFG7"]
    #[inline(always)]
    pub fn p1ifg7(&self) -> P1IFG7_R {
        P1IFG7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1IFG0"]
    #[inline(always)]
    pub fn p1ifg0(&mut self) -> P1IFG0_W {
        P1IFG0_W { w: self }
    }
    #[doc = "Bit 1 - P1IFG1"]
    #[inline(always)]
    pub fn p1ifg1(&mut self) -> P1IFG1_W {
        P1IFG1_W { w: self }
    }
    #[doc = "Bit 2 - P1IFG2"]
    #[inline(always)]
    pub fn p1ifg2(&mut self) -> P1IFG2_W {
        P1IFG2_W { w: self }
    }
    #[doc = "Bit 3 - P1IFG3"]
    #[inline(always)]
    pub fn p1ifg3(&mut self) -> P1IFG3_W {
        P1IFG3_W { w: self }
    }
    #[doc = "Bit 4 - P1IFG4"]
    #[inline(always)]
    pub fn p1ifg4(&mut self) -> P1IFG4_W {
        P1IFG4_W { w: self }
    }
    #[doc = "Bit 5 - P1IFG5"]
    #[inline(always)]
    pub fn p1ifg5(&mut self) -> P1IFG5_W {
        P1IFG5_W { w: self }
    }
    #[doc = "Bit 6 - P1IFG6"]
    #[inline(always)]
    pub fn p1ifg6(&mut self) -> P1IFG6_W {
        P1IFG6_W { w: self }
    }
    #[doc = "Bit 7 - P1IFG7"]
    #[inline(always)]
    pub fn p1ifg7(&mut self) -> P1IFG7_W {
        P1IFG7_W { w: self }
    }
}
