#[doc = "Reader of register P6SEL"]
pub type R = crate::R<u8, super::P6SEL>;
#[doc = "Writer for register P6SEL"]
pub type W = crate::W<u8, super::P6SEL>;
#[doc = "Register P6SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::P6SEL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P6SEL0`"]
pub type P6SEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6SEL0`"]
pub struct P6SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> P6SEL0_W<'a> {
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
#[doc = "Reader of field `P6SEL1`"]
pub type P6SEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6SEL1`"]
pub struct P6SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> P6SEL1_W<'a> {
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
#[doc = "Reader of field `P6SEL2`"]
pub type P6SEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6SEL2`"]
pub struct P6SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> P6SEL2_W<'a> {
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
#[doc = "Reader of field `P6SEL3`"]
pub type P6SEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6SEL3`"]
pub struct P6SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> P6SEL3_W<'a> {
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
#[doc = "Reader of field `P6SEL4`"]
pub type P6SEL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6SEL4`"]
pub struct P6SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> P6SEL4_W<'a> {
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
#[doc = "Reader of field `P6SEL5`"]
pub type P6SEL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6SEL5`"]
pub struct P6SEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> P6SEL5_W<'a> {
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
#[doc = "Reader of field `P6SEL6`"]
pub type P6SEL6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6SEL6`"]
pub struct P6SEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> P6SEL6_W<'a> {
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
#[doc = "Reader of field `P6SEL7`"]
pub type P6SEL7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6SEL7`"]
pub struct P6SEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> P6SEL7_W<'a> {
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
    #[doc = "Bit 0 - P6SEL0"]
    #[inline(always)]
    pub fn p6sel0(&self) -> P6SEL0_R {
        P6SEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P6SEL1"]
    #[inline(always)]
    pub fn p6sel1(&self) -> P6SEL1_R {
        P6SEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P6SEL2"]
    #[inline(always)]
    pub fn p6sel2(&self) -> P6SEL2_R {
        P6SEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P6SEL3"]
    #[inline(always)]
    pub fn p6sel3(&self) -> P6SEL3_R {
        P6SEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P6SEL4"]
    #[inline(always)]
    pub fn p6sel4(&self) -> P6SEL4_R {
        P6SEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P6SEL5"]
    #[inline(always)]
    pub fn p6sel5(&self) -> P6SEL5_R {
        P6SEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P6SEL6"]
    #[inline(always)]
    pub fn p6sel6(&self) -> P6SEL6_R {
        P6SEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P6SEL7"]
    #[inline(always)]
    pub fn p6sel7(&self) -> P6SEL7_R {
        P6SEL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P6SEL0"]
    #[inline(always)]
    pub fn p6sel0(&mut self) -> P6SEL0_W {
        P6SEL0_W { w: self }
    }
    #[doc = "Bit 1 - P6SEL1"]
    #[inline(always)]
    pub fn p6sel1(&mut self) -> P6SEL1_W {
        P6SEL1_W { w: self }
    }
    #[doc = "Bit 2 - P6SEL2"]
    #[inline(always)]
    pub fn p6sel2(&mut self) -> P6SEL2_W {
        P6SEL2_W { w: self }
    }
    #[doc = "Bit 3 - P6SEL3"]
    #[inline(always)]
    pub fn p6sel3(&mut self) -> P6SEL3_W {
        P6SEL3_W { w: self }
    }
    #[doc = "Bit 4 - P6SEL4"]
    #[inline(always)]
    pub fn p6sel4(&mut self) -> P6SEL4_W {
        P6SEL4_W { w: self }
    }
    #[doc = "Bit 5 - P6SEL5"]
    #[inline(always)]
    pub fn p6sel5(&mut self) -> P6SEL5_W {
        P6SEL5_W { w: self }
    }
    #[doc = "Bit 6 - P6SEL6"]
    #[inline(always)]
    pub fn p6sel6(&mut self) -> P6SEL6_W {
        P6SEL6_W { w: self }
    }
    #[doc = "Bit 7 - P6SEL7"]
    #[inline(always)]
    pub fn p6sel7(&mut self) -> P6SEL7_W {
        P6SEL7_W { w: self }
    }
}
