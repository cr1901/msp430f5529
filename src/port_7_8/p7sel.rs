#[doc = "Reader of register P7SEL"]
pub type R = crate::R<u8, super::P7SEL>;
#[doc = "Writer for register P7SEL"]
pub type W = crate::W<u8, super::P7SEL>;
#[doc = "Register P7SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::P7SEL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P7SEL0`"]
pub type P7SEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7SEL0`"]
pub struct P7SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> P7SEL0_W<'a> {
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
#[doc = "Reader of field `P7SEL1`"]
pub type P7SEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7SEL1`"]
pub struct P7SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> P7SEL1_W<'a> {
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
#[doc = "Reader of field `P7SEL2`"]
pub type P7SEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7SEL2`"]
pub struct P7SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> P7SEL2_W<'a> {
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
#[doc = "Reader of field `P7SEL3`"]
pub type P7SEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7SEL3`"]
pub struct P7SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> P7SEL3_W<'a> {
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
#[doc = "Reader of field `P7SEL4`"]
pub type P7SEL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7SEL4`"]
pub struct P7SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> P7SEL4_W<'a> {
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
#[doc = "Reader of field `P7SEL5`"]
pub type P7SEL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7SEL5`"]
pub struct P7SEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> P7SEL5_W<'a> {
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
#[doc = "Reader of field `P7SEL6`"]
pub type P7SEL6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7SEL6`"]
pub struct P7SEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> P7SEL6_W<'a> {
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
#[doc = "Reader of field `P7SEL7`"]
pub type P7SEL7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7SEL7`"]
pub struct P7SEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> P7SEL7_W<'a> {
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
    #[doc = "Bit 0 - P7SEL0"]
    #[inline(always)]
    pub fn p7sel0(&self) -> P7SEL0_R {
        P7SEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P7SEL1"]
    #[inline(always)]
    pub fn p7sel1(&self) -> P7SEL1_R {
        P7SEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P7SEL2"]
    #[inline(always)]
    pub fn p7sel2(&self) -> P7SEL2_R {
        P7SEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P7SEL3"]
    #[inline(always)]
    pub fn p7sel3(&self) -> P7SEL3_R {
        P7SEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P7SEL4"]
    #[inline(always)]
    pub fn p7sel4(&self) -> P7SEL4_R {
        P7SEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P7SEL5"]
    #[inline(always)]
    pub fn p7sel5(&self) -> P7SEL5_R {
        P7SEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P7SEL6"]
    #[inline(always)]
    pub fn p7sel6(&self) -> P7SEL6_R {
        P7SEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P7SEL7"]
    #[inline(always)]
    pub fn p7sel7(&self) -> P7SEL7_R {
        P7SEL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P7SEL0"]
    #[inline(always)]
    pub fn p7sel0(&mut self) -> P7SEL0_W {
        P7SEL0_W { w: self }
    }
    #[doc = "Bit 1 - P7SEL1"]
    #[inline(always)]
    pub fn p7sel1(&mut self) -> P7SEL1_W {
        P7SEL1_W { w: self }
    }
    #[doc = "Bit 2 - P7SEL2"]
    #[inline(always)]
    pub fn p7sel2(&mut self) -> P7SEL2_W {
        P7SEL2_W { w: self }
    }
    #[doc = "Bit 3 - P7SEL3"]
    #[inline(always)]
    pub fn p7sel3(&mut self) -> P7SEL3_W {
        P7SEL3_W { w: self }
    }
    #[doc = "Bit 4 - P7SEL4"]
    #[inline(always)]
    pub fn p7sel4(&mut self) -> P7SEL4_W {
        P7SEL4_W { w: self }
    }
    #[doc = "Bit 5 - P7SEL5"]
    #[inline(always)]
    pub fn p7sel5(&mut self) -> P7SEL5_W {
        P7SEL5_W { w: self }
    }
    #[doc = "Bit 6 - P7SEL6"]
    #[inline(always)]
    pub fn p7sel6(&mut self) -> P7SEL6_W {
        P7SEL6_W { w: self }
    }
    #[doc = "Bit 7 - P7SEL7"]
    #[inline(always)]
    pub fn p7sel7(&mut self) -> P7SEL7_W {
        P7SEL7_W { w: self }
    }
}
