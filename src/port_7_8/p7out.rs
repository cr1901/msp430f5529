#[doc = "Reader of register P7OUT"]
pub type R = crate::R<u8, super::P7OUT>;
#[doc = "Writer for register P7OUT"]
pub type W = crate::W<u8, super::P7OUT>;
#[doc = "Register P7OUT `reset()`'s with value 0"]
impl crate::ResetValue for super::P7OUT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P7OUT0`"]
pub type P7OUT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7OUT0`"]
pub struct P7OUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> P7OUT0_W<'a> {
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
#[doc = "Reader of field `P7OUT1`"]
pub type P7OUT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7OUT1`"]
pub struct P7OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> P7OUT1_W<'a> {
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
#[doc = "Reader of field `P7OUT2`"]
pub type P7OUT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7OUT2`"]
pub struct P7OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> P7OUT2_W<'a> {
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
#[doc = "Reader of field `P7OUT3`"]
pub type P7OUT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7OUT3`"]
pub struct P7OUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> P7OUT3_W<'a> {
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
#[doc = "Reader of field `P7OUT4`"]
pub type P7OUT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7OUT4`"]
pub struct P7OUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> P7OUT4_W<'a> {
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
#[doc = "Reader of field `P7OUT5`"]
pub type P7OUT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7OUT5`"]
pub struct P7OUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> P7OUT5_W<'a> {
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
#[doc = "Reader of field `P7OUT6`"]
pub type P7OUT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7OUT6`"]
pub struct P7OUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> P7OUT6_W<'a> {
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
#[doc = "Reader of field `P7OUT7`"]
pub type P7OUT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7OUT7`"]
pub struct P7OUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> P7OUT7_W<'a> {
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
    #[doc = "Bit 0 - P7OUT0"]
    #[inline(always)]
    pub fn p7out0(&self) -> P7OUT0_R {
        P7OUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P7OUT1"]
    #[inline(always)]
    pub fn p7out1(&self) -> P7OUT1_R {
        P7OUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P7OUT2"]
    #[inline(always)]
    pub fn p7out2(&self) -> P7OUT2_R {
        P7OUT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P7OUT3"]
    #[inline(always)]
    pub fn p7out3(&self) -> P7OUT3_R {
        P7OUT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P7OUT4"]
    #[inline(always)]
    pub fn p7out4(&self) -> P7OUT4_R {
        P7OUT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P7OUT5"]
    #[inline(always)]
    pub fn p7out5(&self) -> P7OUT5_R {
        P7OUT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P7OUT6"]
    #[inline(always)]
    pub fn p7out6(&self) -> P7OUT6_R {
        P7OUT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P7OUT7"]
    #[inline(always)]
    pub fn p7out7(&self) -> P7OUT7_R {
        P7OUT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P7OUT0"]
    #[inline(always)]
    pub fn p7out0(&mut self) -> P7OUT0_W {
        P7OUT0_W { w: self }
    }
    #[doc = "Bit 1 - P7OUT1"]
    #[inline(always)]
    pub fn p7out1(&mut self) -> P7OUT1_W {
        P7OUT1_W { w: self }
    }
    #[doc = "Bit 2 - P7OUT2"]
    #[inline(always)]
    pub fn p7out2(&mut self) -> P7OUT2_W {
        P7OUT2_W { w: self }
    }
    #[doc = "Bit 3 - P7OUT3"]
    #[inline(always)]
    pub fn p7out3(&mut self) -> P7OUT3_W {
        P7OUT3_W { w: self }
    }
    #[doc = "Bit 4 - P7OUT4"]
    #[inline(always)]
    pub fn p7out4(&mut self) -> P7OUT4_W {
        P7OUT4_W { w: self }
    }
    #[doc = "Bit 5 - P7OUT5"]
    #[inline(always)]
    pub fn p7out5(&mut self) -> P7OUT5_W {
        P7OUT5_W { w: self }
    }
    #[doc = "Bit 6 - P7OUT6"]
    #[inline(always)]
    pub fn p7out6(&mut self) -> P7OUT6_W {
        P7OUT6_W { w: self }
    }
    #[doc = "Bit 7 - P7OUT7"]
    #[inline(always)]
    pub fn p7out7(&mut self) -> P7OUT7_W {
        P7OUT7_W { w: self }
    }
}
