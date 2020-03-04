#[doc = "Reader of register P3SEL"]
pub type R = crate::R<u8, super::P3SEL>;
#[doc = "Writer for register P3SEL"]
pub type W = crate::W<u8, super::P3SEL>;
#[doc = "Register P3SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::P3SEL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P3SEL0`"]
pub type P3SEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3SEL0`"]
pub struct P3SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL0_W<'a> {
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
#[doc = "Reader of field `P3SEL1`"]
pub type P3SEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3SEL1`"]
pub struct P3SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL1_W<'a> {
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
#[doc = "Reader of field `P3SEL2`"]
pub type P3SEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3SEL2`"]
pub struct P3SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL2_W<'a> {
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
#[doc = "Reader of field `P3SEL3`"]
pub type P3SEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3SEL3`"]
pub struct P3SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL3_W<'a> {
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
#[doc = "Reader of field `P3SEL4`"]
pub type P3SEL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3SEL4`"]
pub struct P3SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL4_W<'a> {
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
#[doc = "Reader of field `P3SEL5`"]
pub type P3SEL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3SEL5`"]
pub struct P3SEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL5_W<'a> {
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
#[doc = "Reader of field `P3SEL6`"]
pub type P3SEL6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3SEL6`"]
pub struct P3SEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL6_W<'a> {
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
#[doc = "Reader of field `P3SEL7`"]
pub type P3SEL7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3SEL7`"]
pub struct P3SEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL7_W<'a> {
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
    #[doc = "Bit 0 - P3SEL0"]
    #[inline(always)]
    pub fn p3sel0(&self) -> P3SEL0_R {
        P3SEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P3SEL1"]
    #[inline(always)]
    pub fn p3sel1(&self) -> P3SEL1_R {
        P3SEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P3SEL2"]
    #[inline(always)]
    pub fn p3sel2(&self) -> P3SEL2_R {
        P3SEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P3SEL3"]
    #[inline(always)]
    pub fn p3sel3(&self) -> P3SEL3_R {
        P3SEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P3SEL4"]
    #[inline(always)]
    pub fn p3sel4(&self) -> P3SEL4_R {
        P3SEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P3SEL5"]
    #[inline(always)]
    pub fn p3sel5(&self) -> P3SEL5_R {
        P3SEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P3SEL6"]
    #[inline(always)]
    pub fn p3sel6(&self) -> P3SEL6_R {
        P3SEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P3SEL7"]
    #[inline(always)]
    pub fn p3sel7(&self) -> P3SEL7_R {
        P3SEL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3SEL0"]
    #[inline(always)]
    pub fn p3sel0(&mut self) -> P3SEL0_W {
        P3SEL0_W { w: self }
    }
    #[doc = "Bit 1 - P3SEL1"]
    #[inline(always)]
    pub fn p3sel1(&mut self) -> P3SEL1_W {
        P3SEL1_W { w: self }
    }
    #[doc = "Bit 2 - P3SEL2"]
    #[inline(always)]
    pub fn p3sel2(&mut self) -> P3SEL2_W {
        P3SEL2_W { w: self }
    }
    #[doc = "Bit 3 - P3SEL3"]
    #[inline(always)]
    pub fn p3sel3(&mut self) -> P3SEL3_W {
        P3SEL3_W { w: self }
    }
    #[doc = "Bit 4 - P3SEL4"]
    #[inline(always)]
    pub fn p3sel4(&mut self) -> P3SEL4_W {
        P3SEL4_W { w: self }
    }
    #[doc = "Bit 5 - P3SEL5"]
    #[inline(always)]
    pub fn p3sel5(&mut self) -> P3SEL5_W {
        P3SEL5_W { w: self }
    }
    #[doc = "Bit 6 - P3SEL6"]
    #[inline(always)]
    pub fn p3sel6(&mut self) -> P3SEL6_W {
        P3SEL6_W { w: self }
    }
    #[doc = "Bit 7 - P3SEL7"]
    #[inline(always)]
    pub fn p3sel7(&mut self) -> P3SEL7_W {
        P3SEL7_W { w: self }
    }
}
