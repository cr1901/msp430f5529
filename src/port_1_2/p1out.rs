#[doc = "Reader of register P1OUT"]
pub type R = crate::R<u8, super::P1OUT>;
#[doc = "Writer for register P1OUT"]
pub type W = crate::W<u8, super::P1OUT>;
#[doc = "Register P1OUT `reset()`'s with value 0"]
impl crate::ResetValue for super::P1OUT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P1OUT0`"]
pub type P1OUT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1OUT0`"]
pub struct P1OUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> P1OUT0_W<'a> {
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
#[doc = "Reader of field `P1OUT1`"]
pub type P1OUT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1OUT1`"]
pub struct P1OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1OUT1_W<'a> {
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
#[doc = "Reader of field `P1OUT2`"]
pub type P1OUT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1OUT2`"]
pub struct P1OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> P1OUT2_W<'a> {
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
#[doc = "Reader of field `P1OUT3`"]
pub type P1OUT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1OUT3`"]
pub struct P1OUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> P1OUT3_W<'a> {
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
#[doc = "Reader of field `P1OUT4`"]
pub type P1OUT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1OUT4`"]
pub struct P1OUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> P1OUT4_W<'a> {
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
#[doc = "Reader of field `P1OUT5`"]
pub type P1OUT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1OUT5`"]
pub struct P1OUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> P1OUT5_W<'a> {
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
#[doc = "Reader of field `P1OUT6`"]
pub type P1OUT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1OUT6`"]
pub struct P1OUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> P1OUT6_W<'a> {
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
#[doc = "Reader of field `P1OUT7`"]
pub type P1OUT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1OUT7`"]
pub struct P1OUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> P1OUT7_W<'a> {
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
    #[doc = "Bit 0 - P1OUT0"]
    #[inline(always)]
    pub fn p1out0(&self) -> P1OUT0_R {
        P1OUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P1OUT1"]
    #[inline(always)]
    pub fn p1out1(&self) -> P1OUT1_R {
        P1OUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P1OUT2"]
    #[inline(always)]
    pub fn p1out2(&self) -> P1OUT2_R {
        P1OUT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P1OUT3"]
    #[inline(always)]
    pub fn p1out3(&self) -> P1OUT3_R {
        P1OUT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P1OUT4"]
    #[inline(always)]
    pub fn p1out4(&self) -> P1OUT4_R {
        P1OUT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P1OUT5"]
    #[inline(always)]
    pub fn p1out5(&self) -> P1OUT5_R {
        P1OUT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P1OUT6"]
    #[inline(always)]
    pub fn p1out6(&self) -> P1OUT6_R {
        P1OUT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P1OUT7"]
    #[inline(always)]
    pub fn p1out7(&self) -> P1OUT7_R {
        P1OUT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1OUT0"]
    #[inline(always)]
    pub fn p1out0(&mut self) -> P1OUT0_W {
        P1OUT0_W { w: self }
    }
    #[doc = "Bit 1 - P1OUT1"]
    #[inline(always)]
    pub fn p1out1(&mut self) -> P1OUT1_W {
        P1OUT1_W { w: self }
    }
    #[doc = "Bit 2 - P1OUT2"]
    #[inline(always)]
    pub fn p1out2(&mut self) -> P1OUT2_W {
        P1OUT2_W { w: self }
    }
    #[doc = "Bit 3 - P1OUT3"]
    #[inline(always)]
    pub fn p1out3(&mut self) -> P1OUT3_W {
        P1OUT3_W { w: self }
    }
    #[doc = "Bit 4 - P1OUT4"]
    #[inline(always)]
    pub fn p1out4(&mut self) -> P1OUT4_W {
        P1OUT4_W { w: self }
    }
    #[doc = "Bit 5 - P1OUT5"]
    #[inline(always)]
    pub fn p1out5(&mut self) -> P1OUT5_W {
        P1OUT5_W { w: self }
    }
    #[doc = "Bit 6 - P1OUT6"]
    #[inline(always)]
    pub fn p1out6(&mut self) -> P1OUT6_W {
        P1OUT6_W { w: self }
    }
    #[doc = "Bit 7 - P1OUT7"]
    #[inline(always)]
    pub fn p1out7(&mut self) -> P1OUT7_W {
        P1OUT7_W { w: self }
    }
}
