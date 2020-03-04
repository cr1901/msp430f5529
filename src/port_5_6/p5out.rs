#[doc = "Reader of register P5OUT"]
pub type R = crate::R<u8, super::P5OUT>;
#[doc = "Writer for register P5OUT"]
pub type W = crate::W<u8, super::P5OUT>;
#[doc = "Register P5OUT `reset()`'s with value 0"]
impl crate::ResetValue for super::P5OUT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P5OUT0`"]
pub type P5OUT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5OUT0`"]
pub struct P5OUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> P5OUT0_W<'a> {
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
#[doc = "Reader of field `P5OUT1`"]
pub type P5OUT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5OUT1`"]
pub struct P5OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> P5OUT1_W<'a> {
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
#[doc = "Reader of field `P5OUT2`"]
pub type P5OUT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5OUT2`"]
pub struct P5OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> P5OUT2_W<'a> {
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
#[doc = "Reader of field `P5OUT3`"]
pub type P5OUT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5OUT3`"]
pub struct P5OUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> P5OUT3_W<'a> {
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
#[doc = "Reader of field `P5OUT4`"]
pub type P5OUT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5OUT4`"]
pub struct P5OUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> P5OUT4_W<'a> {
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
#[doc = "Reader of field `P5OUT5`"]
pub type P5OUT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5OUT5`"]
pub struct P5OUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> P5OUT5_W<'a> {
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
#[doc = "Reader of field `P5OUT6`"]
pub type P5OUT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5OUT6`"]
pub struct P5OUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> P5OUT6_W<'a> {
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
#[doc = "Reader of field `P5OUT7`"]
pub type P5OUT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5OUT7`"]
pub struct P5OUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> P5OUT7_W<'a> {
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
    #[doc = "Bit 0 - P5OUT0"]
    #[inline(always)]
    pub fn p5out0(&self) -> P5OUT0_R {
        P5OUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P5OUT1"]
    #[inline(always)]
    pub fn p5out1(&self) -> P5OUT1_R {
        P5OUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P5OUT2"]
    #[inline(always)]
    pub fn p5out2(&self) -> P5OUT2_R {
        P5OUT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P5OUT3"]
    #[inline(always)]
    pub fn p5out3(&self) -> P5OUT3_R {
        P5OUT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P5OUT4"]
    #[inline(always)]
    pub fn p5out4(&self) -> P5OUT4_R {
        P5OUT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P5OUT5"]
    #[inline(always)]
    pub fn p5out5(&self) -> P5OUT5_R {
        P5OUT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P5OUT6"]
    #[inline(always)]
    pub fn p5out6(&self) -> P5OUT6_R {
        P5OUT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P5OUT7"]
    #[inline(always)]
    pub fn p5out7(&self) -> P5OUT7_R {
        P5OUT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P5OUT0"]
    #[inline(always)]
    pub fn p5out0(&mut self) -> P5OUT0_W {
        P5OUT0_W { w: self }
    }
    #[doc = "Bit 1 - P5OUT1"]
    #[inline(always)]
    pub fn p5out1(&mut self) -> P5OUT1_W {
        P5OUT1_W { w: self }
    }
    #[doc = "Bit 2 - P5OUT2"]
    #[inline(always)]
    pub fn p5out2(&mut self) -> P5OUT2_W {
        P5OUT2_W { w: self }
    }
    #[doc = "Bit 3 - P5OUT3"]
    #[inline(always)]
    pub fn p5out3(&mut self) -> P5OUT3_W {
        P5OUT3_W { w: self }
    }
    #[doc = "Bit 4 - P5OUT4"]
    #[inline(always)]
    pub fn p5out4(&mut self) -> P5OUT4_W {
        P5OUT4_W { w: self }
    }
    #[doc = "Bit 5 - P5OUT5"]
    #[inline(always)]
    pub fn p5out5(&mut self) -> P5OUT5_W {
        P5OUT5_W { w: self }
    }
    #[doc = "Bit 6 - P5OUT6"]
    #[inline(always)]
    pub fn p5out6(&mut self) -> P5OUT6_W {
        P5OUT6_W { w: self }
    }
    #[doc = "Bit 7 - P5OUT7"]
    #[inline(always)]
    pub fn p5out7(&mut self) -> P5OUT7_W {
        P5OUT7_W { w: self }
    }
}
