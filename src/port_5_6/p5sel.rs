#[doc = "Reader of register P5SEL"]
pub type R = crate::R<u8, super::P5SEL>;
#[doc = "Writer for register P5SEL"]
pub type W = crate::W<u8, super::P5SEL>;
#[doc = "Register P5SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::P5SEL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P5SEL0`"]
pub type P5SEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5SEL0`"]
pub struct P5SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> P5SEL0_W<'a> {
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
#[doc = "Reader of field `P5SEL1`"]
pub type P5SEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5SEL1`"]
pub struct P5SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> P5SEL1_W<'a> {
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
#[doc = "Reader of field `P5SEL2`"]
pub type P5SEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5SEL2`"]
pub struct P5SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> P5SEL2_W<'a> {
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
#[doc = "Reader of field `P5SEL3`"]
pub type P5SEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5SEL3`"]
pub struct P5SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> P5SEL3_W<'a> {
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
#[doc = "Reader of field `P5SEL4`"]
pub type P5SEL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5SEL4`"]
pub struct P5SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> P5SEL4_W<'a> {
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
#[doc = "Reader of field `P5SEL5`"]
pub type P5SEL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5SEL5`"]
pub struct P5SEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> P5SEL5_W<'a> {
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
#[doc = "Reader of field `P5SEL6`"]
pub type P5SEL6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5SEL6`"]
pub struct P5SEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> P5SEL6_W<'a> {
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
#[doc = "Reader of field `P5SEL7`"]
pub type P5SEL7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5SEL7`"]
pub struct P5SEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> P5SEL7_W<'a> {
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
    #[doc = "Bit 0 - P5SEL0"]
    #[inline(always)]
    pub fn p5sel0(&self) -> P5SEL0_R {
        P5SEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P5SEL1"]
    #[inline(always)]
    pub fn p5sel1(&self) -> P5SEL1_R {
        P5SEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P5SEL2"]
    #[inline(always)]
    pub fn p5sel2(&self) -> P5SEL2_R {
        P5SEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P5SEL3"]
    #[inline(always)]
    pub fn p5sel3(&self) -> P5SEL3_R {
        P5SEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P5SEL4"]
    #[inline(always)]
    pub fn p5sel4(&self) -> P5SEL4_R {
        P5SEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P5SEL5"]
    #[inline(always)]
    pub fn p5sel5(&self) -> P5SEL5_R {
        P5SEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P5SEL6"]
    #[inline(always)]
    pub fn p5sel6(&self) -> P5SEL6_R {
        P5SEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P5SEL7"]
    #[inline(always)]
    pub fn p5sel7(&self) -> P5SEL7_R {
        P5SEL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P5SEL0"]
    #[inline(always)]
    pub fn p5sel0(&mut self) -> P5SEL0_W {
        P5SEL0_W { w: self }
    }
    #[doc = "Bit 1 - P5SEL1"]
    #[inline(always)]
    pub fn p5sel1(&mut self) -> P5SEL1_W {
        P5SEL1_W { w: self }
    }
    #[doc = "Bit 2 - P5SEL2"]
    #[inline(always)]
    pub fn p5sel2(&mut self) -> P5SEL2_W {
        P5SEL2_W { w: self }
    }
    #[doc = "Bit 3 - P5SEL3"]
    #[inline(always)]
    pub fn p5sel3(&mut self) -> P5SEL3_W {
        P5SEL3_W { w: self }
    }
    #[doc = "Bit 4 - P5SEL4"]
    #[inline(always)]
    pub fn p5sel4(&mut self) -> P5SEL4_W {
        P5SEL4_W { w: self }
    }
    #[doc = "Bit 5 - P5SEL5"]
    #[inline(always)]
    pub fn p5sel5(&mut self) -> P5SEL5_W {
        P5SEL5_W { w: self }
    }
    #[doc = "Bit 6 - P5SEL6"]
    #[inline(always)]
    pub fn p5sel6(&mut self) -> P5SEL6_W {
        P5SEL6_W { w: self }
    }
    #[doc = "Bit 7 - P5SEL7"]
    #[inline(always)]
    pub fn p5sel7(&mut self) -> P5SEL7_W {
        P5SEL7_W { w: self }
    }
}
