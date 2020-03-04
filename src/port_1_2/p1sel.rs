#[doc = "Reader of register P1SEL"]
pub type R = crate::R<u8, super::P1SEL>;
#[doc = "Writer for register P1SEL"]
pub type W = crate::W<u8, super::P1SEL>;
#[doc = "Register P1SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::P1SEL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P1SEL0`"]
pub type P1SEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SEL0`"]
pub struct P1SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SEL0_W<'a> {
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
#[doc = "Reader of field `P1SEL1`"]
pub type P1SEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SEL1`"]
pub struct P1SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SEL1_W<'a> {
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
#[doc = "Reader of field `P1SEL2`"]
pub type P1SEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SEL2`"]
pub struct P1SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SEL2_W<'a> {
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
#[doc = "Reader of field `P1SEL3`"]
pub type P1SEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SEL3`"]
pub struct P1SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SEL3_W<'a> {
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
#[doc = "Reader of field `P1SEL4`"]
pub type P1SEL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SEL4`"]
pub struct P1SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SEL4_W<'a> {
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
#[doc = "Reader of field `P1SEL5`"]
pub type P1SEL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SEL5`"]
pub struct P1SEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SEL5_W<'a> {
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
#[doc = "Reader of field `P1SEL6`"]
pub type P1SEL6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SEL6`"]
pub struct P1SEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SEL6_W<'a> {
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
#[doc = "Reader of field `P1SEL7`"]
pub type P1SEL7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SEL7`"]
pub struct P1SEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SEL7_W<'a> {
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
    #[doc = "Bit 0 - P1SEL0"]
    #[inline(always)]
    pub fn p1sel0(&self) -> P1SEL0_R {
        P1SEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P1SEL1"]
    #[inline(always)]
    pub fn p1sel1(&self) -> P1SEL1_R {
        P1SEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P1SEL2"]
    #[inline(always)]
    pub fn p1sel2(&self) -> P1SEL2_R {
        P1SEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P1SEL3"]
    #[inline(always)]
    pub fn p1sel3(&self) -> P1SEL3_R {
        P1SEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P1SEL4"]
    #[inline(always)]
    pub fn p1sel4(&self) -> P1SEL4_R {
        P1SEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P1SEL5"]
    #[inline(always)]
    pub fn p1sel5(&self) -> P1SEL5_R {
        P1SEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P1SEL6"]
    #[inline(always)]
    pub fn p1sel6(&self) -> P1SEL6_R {
        P1SEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P1SEL7"]
    #[inline(always)]
    pub fn p1sel7(&self) -> P1SEL7_R {
        P1SEL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1SEL0"]
    #[inline(always)]
    pub fn p1sel0(&mut self) -> P1SEL0_W {
        P1SEL0_W { w: self }
    }
    #[doc = "Bit 1 - P1SEL1"]
    #[inline(always)]
    pub fn p1sel1(&mut self) -> P1SEL1_W {
        P1SEL1_W { w: self }
    }
    #[doc = "Bit 2 - P1SEL2"]
    #[inline(always)]
    pub fn p1sel2(&mut self) -> P1SEL2_W {
        P1SEL2_W { w: self }
    }
    #[doc = "Bit 3 - P1SEL3"]
    #[inline(always)]
    pub fn p1sel3(&mut self) -> P1SEL3_W {
        P1SEL3_W { w: self }
    }
    #[doc = "Bit 4 - P1SEL4"]
    #[inline(always)]
    pub fn p1sel4(&mut self) -> P1SEL4_W {
        P1SEL4_W { w: self }
    }
    #[doc = "Bit 5 - P1SEL5"]
    #[inline(always)]
    pub fn p1sel5(&mut self) -> P1SEL5_W {
        P1SEL5_W { w: self }
    }
    #[doc = "Bit 6 - P1SEL6"]
    #[inline(always)]
    pub fn p1sel6(&mut self) -> P1SEL6_W {
        P1SEL6_W { w: self }
    }
    #[doc = "Bit 7 - P1SEL7"]
    #[inline(always)]
    pub fn p1sel7(&mut self) -> P1SEL7_W {
        P1SEL7_W { w: self }
    }
}
