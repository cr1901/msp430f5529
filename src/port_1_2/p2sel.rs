#[doc = "Reader of register P2SEL"]
pub type R = crate::R<u8, super::P2SEL>;
#[doc = "Writer for register P2SEL"]
pub type W = crate::W<u8, super::P2SEL>;
#[doc = "Register P2SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::P2SEL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P2SEL0`"]
pub type P2SEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2SEL0`"]
pub struct P2SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> P2SEL0_W<'a> {
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
#[doc = "Reader of field `P2SEL1`"]
pub type P2SEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2SEL1`"]
pub struct P2SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> P2SEL1_W<'a> {
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
#[doc = "Reader of field `P2SEL2`"]
pub type P2SEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2SEL2`"]
pub struct P2SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> P2SEL2_W<'a> {
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
#[doc = "Reader of field `P2SEL3`"]
pub type P2SEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2SEL3`"]
pub struct P2SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> P2SEL3_W<'a> {
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
#[doc = "Reader of field `P2SEL4`"]
pub type P2SEL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2SEL4`"]
pub struct P2SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> P2SEL4_W<'a> {
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
#[doc = "Reader of field `P2SEL5`"]
pub type P2SEL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2SEL5`"]
pub struct P2SEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> P2SEL5_W<'a> {
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
#[doc = "Reader of field `P2SEL6`"]
pub type P2SEL6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2SEL6`"]
pub struct P2SEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> P2SEL6_W<'a> {
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
#[doc = "Reader of field `P2SEL7`"]
pub type P2SEL7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2SEL7`"]
pub struct P2SEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> P2SEL7_W<'a> {
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
    #[doc = "Bit 0 - P2SEL0"]
    #[inline(always)]
    pub fn p2sel0(&self) -> P2SEL0_R {
        P2SEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P2SEL1"]
    #[inline(always)]
    pub fn p2sel1(&self) -> P2SEL1_R {
        P2SEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P2SEL2"]
    #[inline(always)]
    pub fn p2sel2(&self) -> P2SEL2_R {
        P2SEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P2SEL3"]
    #[inline(always)]
    pub fn p2sel3(&self) -> P2SEL3_R {
        P2SEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P2SEL4"]
    #[inline(always)]
    pub fn p2sel4(&self) -> P2SEL4_R {
        P2SEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P2SEL5"]
    #[inline(always)]
    pub fn p2sel5(&self) -> P2SEL5_R {
        P2SEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P2SEL6"]
    #[inline(always)]
    pub fn p2sel6(&self) -> P2SEL6_R {
        P2SEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P2SEL7"]
    #[inline(always)]
    pub fn p2sel7(&self) -> P2SEL7_R {
        P2SEL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2SEL0"]
    #[inline(always)]
    pub fn p2sel0(&mut self) -> P2SEL0_W {
        P2SEL0_W { w: self }
    }
    #[doc = "Bit 1 - P2SEL1"]
    #[inline(always)]
    pub fn p2sel1(&mut self) -> P2SEL1_W {
        P2SEL1_W { w: self }
    }
    #[doc = "Bit 2 - P2SEL2"]
    #[inline(always)]
    pub fn p2sel2(&mut self) -> P2SEL2_W {
        P2SEL2_W { w: self }
    }
    #[doc = "Bit 3 - P2SEL3"]
    #[inline(always)]
    pub fn p2sel3(&mut self) -> P2SEL3_W {
        P2SEL3_W { w: self }
    }
    #[doc = "Bit 4 - P2SEL4"]
    #[inline(always)]
    pub fn p2sel4(&mut self) -> P2SEL4_W {
        P2SEL4_W { w: self }
    }
    #[doc = "Bit 5 - P2SEL5"]
    #[inline(always)]
    pub fn p2sel5(&mut self) -> P2SEL5_W {
        P2SEL5_W { w: self }
    }
    #[doc = "Bit 6 - P2SEL6"]
    #[inline(always)]
    pub fn p2sel6(&mut self) -> P2SEL6_W {
        P2SEL6_W { w: self }
    }
    #[doc = "Bit 7 - P2SEL7"]
    #[inline(always)]
    pub fn p2sel7(&mut self) -> P2SEL7_W {
        P2SEL7_W { w: self }
    }
}
