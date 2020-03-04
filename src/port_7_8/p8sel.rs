#[doc = "Reader of register P8SEL"]
pub type R = crate::R<u8, super::P8SEL>;
#[doc = "Writer for register P8SEL"]
pub type W = crate::W<u8, super::P8SEL>;
#[doc = "Register P8SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::P8SEL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P8SEL0`"]
pub type P8SEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8SEL0`"]
pub struct P8SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> P8SEL0_W<'a> {
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
#[doc = "Reader of field `P8SEL1`"]
pub type P8SEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8SEL1`"]
pub struct P8SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> P8SEL1_W<'a> {
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
#[doc = "Reader of field `P8SEL2`"]
pub type P8SEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8SEL2`"]
pub struct P8SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> P8SEL2_W<'a> {
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
#[doc = "Reader of field `P8SEL3`"]
pub type P8SEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8SEL3`"]
pub struct P8SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> P8SEL3_W<'a> {
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
#[doc = "Reader of field `P8SEL4`"]
pub type P8SEL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8SEL4`"]
pub struct P8SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> P8SEL4_W<'a> {
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
#[doc = "Reader of field `P8SEL5`"]
pub type P8SEL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8SEL5`"]
pub struct P8SEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> P8SEL5_W<'a> {
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
#[doc = "Reader of field `P8SEL6`"]
pub type P8SEL6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8SEL6`"]
pub struct P8SEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> P8SEL6_W<'a> {
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
#[doc = "Reader of field `P8SEL7`"]
pub type P8SEL7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8SEL7`"]
pub struct P8SEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> P8SEL7_W<'a> {
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
    #[doc = "Bit 0 - P8SEL0"]
    #[inline(always)]
    pub fn p8sel0(&self) -> P8SEL0_R {
        P8SEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P8SEL1"]
    #[inline(always)]
    pub fn p8sel1(&self) -> P8SEL1_R {
        P8SEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P8SEL2"]
    #[inline(always)]
    pub fn p8sel2(&self) -> P8SEL2_R {
        P8SEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P8SEL3"]
    #[inline(always)]
    pub fn p8sel3(&self) -> P8SEL3_R {
        P8SEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P8SEL4"]
    #[inline(always)]
    pub fn p8sel4(&self) -> P8SEL4_R {
        P8SEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P8SEL5"]
    #[inline(always)]
    pub fn p8sel5(&self) -> P8SEL5_R {
        P8SEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P8SEL6"]
    #[inline(always)]
    pub fn p8sel6(&self) -> P8SEL6_R {
        P8SEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P8SEL7"]
    #[inline(always)]
    pub fn p8sel7(&self) -> P8SEL7_R {
        P8SEL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P8SEL0"]
    #[inline(always)]
    pub fn p8sel0(&mut self) -> P8SEL0_W {
        P8SEL0_W { w: self }
    }
    #[doc = "Bit 1 - P8SEL1"]
    #[inline(always)]
    pub fn p8sel1(&mut self) -> P8SEL1_W {
        P8SEL1_W { w: self }
    }
    #[doc = "Bit 2 - P8SEL2"]
    #[inline(always)]
    pub fn p8sel2(&mut self) -> P8SEL2_W {
        P8SEL2_W { w: self }
    }
    #[doc = "Bit 3 - P8SEL3"]
    #[inline(always)]
    pub fn p8sel3(&mut self) -> P8SEL3_W {
        P8SEL3_W { w: self }
    }
    #[doc = "Bit 4 - P8SEL4"]
    #[inline(always)]
    pub fn p8sel4(&mut self) -> P8SEL4_W {
        P8SEL4_W { w: self }
    }
    #[doc = "Bit 5 - P8SEL5"]
    #[inline(always)]
    pub fn p8sel5(&mut self) -> P8SEL5_W {
        P8SEL5_W { w: self }
    }
    #[doc = "Bit 6 - P8SEL6"]
    #[inline(always)]
    pub fn p8sel6(&mut self) -> P8SEL6_W {
        P8SEL6_W { w: self }
    }
    #[doc = "Bit 7 - P8SEL7"]
    #[inline(always)]
    pub fn p8sel7(&mut self) -> P8SEL7_W {
        P8SEL7_W { w: self }
    }
}
