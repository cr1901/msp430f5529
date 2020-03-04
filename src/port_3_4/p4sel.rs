#[doc = "Reader of register P4SEL"]
pub type R = crate::R<u8, super::P4SEL>;
#[doc = "Writer for register P4SEL"]
pub type W = crate::W<u8, super::P4SEL>;
#[doc = "Register P4SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::P4SEL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P4SEL0`"]
pub type P4SEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4SEL0`"]
pub struct P4SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> P4SEL0_W<'a> {
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
#[doc = "Reader of field `P4SEL1`"]
pub type P4SEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4SEL1`"]
pub struct P4SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> P4SEL1_W<'a> {
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
#[doc = "Reader of field `P4SEL2`"]
pub type P4SEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4SEL2`"]
pub struct P4SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> P4SEL2_W<'a> {
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
#[doc = "Reader of field `P4SEL3`"]
pub type P4SEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4SEL3`"]
pub struct P4SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> P4SEL3_W<'a> {
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
#[doc = "Reader of field `P4SEL4`"]
pub type P4SEL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4SEL4`"]
pub struct P4SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> P4SEL4_W<'a> {
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
#[doc = "Reader of field `P4SEL5`"]
pub type P4SEL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4SEL5`"]
pub struct P4SEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> P4SEL5_W<'a> {
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
#[doc = "Reader of field `P4SEL6`"]
pub type P4SEL6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4SEL6`"]
pub struct P4SEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> P4SEL6_W<'a> {
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
#[doc = "Reader of field `P4SEL7`"]
pub type P4SEL7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4SEL7`"]
pub struct P4SEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> P4SEL7_W<'a> {
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
    #[doc = "Bit 0 - P4SEL0"]
    #[inline(always)]
    pub fn p4sel0(&self) -> P4SEL0_R {
        P4SEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P4SEL1"]
    #[inline(always)]
    pub fn p4sel1(&self) -> P4SEL1_R {
        P4SEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P4SEL2"]
    #[inline(always)]
    pub fn p4sel2(&self) -> P4SEL2_R {
        P4SEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P4SEL3"]
    #[inline(always)]
    pub fn p4sel3(&self) -> P4SEL3_R {
        P4SEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P4SEL4"]
    #[inline(always)]
    pub fn p4sel4(&self) -> P4SEL4_R {
        P4SEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P4SEL5"]
    #[inline(always)]
    pub fn p4sel5(&self) -> P4SEL5_R {
        P4SEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P4SEL6"]
    #[inline(always)]
    pub fn p4sel6(&self) -> P4SEL6_R {
        P4SEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P4SEL7"]
    #[inline(always)]
    pub fn p4sel7(&self) -> P4SEL7_R {
        P4SEL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4SEL0"]
    #[inline(always)]
    pub fn p4sel0(&mut self) -> P4SEL0_W {
        P4SEL0_W { w: self }
    }
    #[doc = "Bit 1 - P4SEL1"]
    #[inline(always)]
    pub fn p4sel1(&mut self) -> P4SEL1_W {
        P4SEL1_W { w: self }
    }
    #[doc = "Bit 2 - P4SEL2"]
    #[inline(always)]
    pub fn p4sel2(&mut self) -> P4SEL2_W {
        P4SEL2_W { w: self }
    }
    #[doc = "Bit 3 - P4SEL3"]
    #[inline(always)]
    pub fn p4sel3(&mut self) -> P4SEL3_W {
        P4SEL3_W { w: self }
    }
    #[doc = "Bit 4 - P4SEL4"]
    #[inline(always)]
    pub fn p4sel4(&mut self) -> P4SEL4_W {
        P4SEL4_W { w: self }
    }
    #[doc = "Bit 5 - P4SEL5"]
    #[inline(always)]
    pub fn p4sel5(&mut self) -> P4SEL5_W {
        P4SEL5_W { w: self }
    }
    #[doc = "Bit 6 - P4SEL6"]
    #[inline(always)]
    pub fn p4sel6(&mut self) -> P4SEL6_W {
        P4SEL6_W { w: self }
    }
    #[doc = "Bit 7 - P4SEL7"]
    #[inline(always)]
    pub fn p4sel7(&mut self) -> P4SEL7_W {
        P4SEL7_W { w: self }
    }
}
