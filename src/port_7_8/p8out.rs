#[doc = "Reader of register P8OUT"]
pub type R = crate::R<u8, super::P8OUT>;
#[doc = "Writer for register P8OUT"]
pub type W = crate::W<u8, super::P8OUT>;
#[doc = "Register P8OUT `reset()`'s with value 0"]
impl crate::ResetValue for super::P8OUT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P8OUT0`"]
pub type P8OUT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8OUT0`"]
pub struct P8OUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> P8OUT0_W<'a> {
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
#[doc = "Reader of field `P8OUT1`"]
pub type P8OUT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8OUT1`"]
pub struct P8OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> P8OUT1_W<'a> {
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
#[doc = "Reader of field `P8OUT2`"]
pub type P8OUT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8OUT2`"]
pub struct P8OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> P8OUT2_W<'a> {
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
#[doc = "Reader of field `P8OUT3`"]
pub type P8OUT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8OUT3`"]
pub struct P8OUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> P8OUT3_W<'a> {
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
#[doc = "Reader of field `P8OUT4`"]
pub type P8OUT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8OUT4`"]
pub struct P8OUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> P8OUT4_W<'a> {
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
#[doc = "Reader of field `P8OUT5`"]
pub type P8OUT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8OUT5`"]
pub struct P8OUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> P8OUT5_W<'a> {
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
#[doc = "Reader of field `P8OUT6`"]
pub type P8OUT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8OUT6`"]
pub struct P8OUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> P8OUT6_W<'a> {
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
#[doc = "Reader of field `P8OUT7`"]
pub type P8OUT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8OUT7`"]
pub struct P8OUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> P8OUT7_W<'a> {
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
    #[doc = "Bit 0 - P8OUT0"]
    #[inline(always)]
    pub fn p8out0(&self) -> P8OUT0_R {
        P8OUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P8OUT1"]
    #[inline(always)]
    pub fn p8out1(&self) -> P8OUT1_R {
        P8OUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P8OUT2"]
    #[inline(always)]
    pub fn p8out2(&self) -> P8OUT2_R {
        P8OUT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P8OUT3"]
    #[inline(always)]
    pub fn p8out3(&self) -> P8OUT3_R {
        P8OUT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P8OUT4"]
    #[inline(always)]
    pub fn p8out4(&self) -> P8OUT4_R {
        P8OUT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P8OUT5"]
    #[inline(always)]
    pub fn p8out5(&self) -> P8OUT5_R {
        P8OUT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P8OUT6"]
    #[inline(always)]
    pub fn p8out6(&self) -> P8OUT6_R {
        P8OUT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P8OUT7"]
    #[inline(always)]
    pub fn p8out7(&self) -> P8OUT7_R {
        P8OUT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P8OUT0"]
    #[inline(always)]
    pub fn p8out0(&mut self) -> P8OUT0_W {
        P8OUT0_W { w: self }
    }
    #[doc = "Bit 1 - P8OUT1"]
    #[inline(always)]
    pub fn p8out1(&mut self) -> P8OUT1_W {
        P8OUT1_W { w: self }
    }
    #[doc = "Bit 2 - P8OUT2"]
    #[inline(always)]
    pub fn p8out2(&mut self) -> P8OUT2_W {
        P8OUT2_W { w: self }
    }
    #[doc = "Bit 3 - P8OUT3"]
    #[inline(always)]
    pub fn p8out3(&mut self) -> P8OUT3_W {
        P8OUT3_W { w: self }
    }
    #[doc = "Bit 4 - P8OUT4"]
    #[inline(always)]
    pub fn p8out4(&mut self) -> P8OUT4_W {
        P8OUT4_W { w: self }
    }
    #[doc = "Bit 5 - P8OUT5"]
    #[inline(always)]
    pub fn p8out5(&mut self) -> P8OUT5_W {
        P8OUT5_W { w: self }
    }
    #[doc = "Bit 6 - P8OUT6"]
    #[inline(always)]
    pub fn p8out6(&mut self) -> P8OUT6_W {
        P8OUT6_W { w: self }
    }
    #[doc = "Bit 7 - P8OUT7"]
    #[inline(always)]
    pub fn p8out7(&mut self) -> P8OUT7_W {
        P8OUT7_W { w: self }
    }
}
