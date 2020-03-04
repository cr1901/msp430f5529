#[doc = "Reader of register P6OUT"]
pub type R = crate::R<u8, super::P6OUT>;
#[doc = "Writer for register P6OUT"]
pub type W = crate::W<u8, super::P6OUT>;
#[doc = "Register P6OUT `reset()`'s with value 0"]
impl crate::ResetValue for super::P6OUT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P6OUT0`"]
pub type P6OUT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6OUT0`"]
pub struct P6OUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> P6OUT0_W<'a> {
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
#[doc = "Reader of field `P6OUT1`"]
pub type P6OUT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6OUT1`"]
pub struct P6OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> P6OUT1_W<'a> {
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
#[doc = "Reader of field `P6OUT2`"]
pub type P6OUT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6OUT2`"]
pub struct P6OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> P6OUT2_W<'a> {
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
#[doc = "Reader of field `P6OUT3`"]
pub type P6OUT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6OUT3`"]
pub struct P6OUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> P6OUT3_W<'a> {
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
#[doc = "Reader of field `P6OUT4`"]
pub type P6OUT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6OUT4`"]
pub struct P6OUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> P6OUT4_W<'a> {
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
#[doc = "Reader of field `P6OUT5`"]
pub type P6OUT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6OUT5`"]
pub struct P6OUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> P6OUT5_W<'a> {
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
#[doc = "Reader of field `P6OUT6`"]
pub type P6OUT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6OUT6`"]
pub struct P6OUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> P6OUT6_W<'a> {
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
#[doc = "Reader of field `P6OUT7`"]
pub type P6OUT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6OUT7`"]
pub struct P6OUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> P6OUT7_W<'a> {
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
    #[doc = "Bit 0 - P6OUT0"]
    #[inline(always)]
    pub fn p6out0(&self) -> P6OUT0_R {
        P6OUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P6OUT1"]
    #[inline(always)]
    pub fn p6out1(&self) -> P6OUT1_R {
        P6OUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P6OUT2"]
    #[inline(always)]
    pub fn p6out2(&self) -> P6OUT2_R {
        P6OUT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P6OUT3"]
    #[inline(always)]
    pub fn p6out3(&self) -> P6OUT3_R {
        P6OUT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P6OUT4"]
    #[inline(always)]
    pub fn p6out4(&self) -> P6OUT4_R {
        P6OUT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P6OUT5"]
    #[inline(always)]
    pub fn p6out5(&self) -> P6OUT5_R {
        P6OUT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P6OUT6"]
    #[inline(always)]
    pub fn p6out6(&self) -> P6OUT6_R {
        P6OUT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P6OUT7"]
    #[inline(always)]
    pub fn p6out7(&self) -> P6OUT7_R {
        P6OUT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P6OUT0"]
    #[inline(always)]
    pub fn p6out0(&mut self) -> P6OUT0_W {
        P6OUT0_W { w: self }
    }
    #[doc = "Bit 1 - P6OUT1"]
    #[inline(always)]
    pub fn p6out1(&mut self) -> P6OUT1_W {
        P6OUT1_W { w: self }
    }
    #[doc = "Bit 2 - P6OUT2"]
    #[inline(always)]
    pub fn p6out2(&mut self) -> P6OUT2_W {
        P6OUT2_W { w: self }
    }
    #[doc = "Bit 3 - P6OUT3"]
    #[inline(always)]
    pub fn p6out3(&mut self) -> P6OUT3_W {
        P6OUT3_W { w: self }
    }
    #[doc = "Bit 4 - P6OUT4"]
    #[inline(always)]
    pub fn p6out4(&mut self) -> P6OUT4_W {
        P6OUT4_W { w: self }
    }
    #[doc = "Bit 5 - P6OUT5"]
    #[inline(always)]
    pub fn p6out5(&mut self) -> P6OUT5_W {
        P6OUT5_W { w: self }
    }
    #[doc = "Bit 6 - P6OUT6"]
    #[inline(always)]
    pub fn p6out6(&mut self) -> P6OUT6_W {
        P6OUT6_W { w: self }
    }
    #[doc = "Bit 7 - P6OUT7"]
    #[inline(always)]
    pub fn p6out7(&mut self) -> P6OUT7_W {
        P6OUT7_W { w: self }
    }
}
