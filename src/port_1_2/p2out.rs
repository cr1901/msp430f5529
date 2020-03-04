#[doc = "Reader of register P2OUT"]
pub type R = crate::R<u8, super::P2OUT>;
#[doc = "Writer for register P2OUT"]
pub type W = crate::W<u8, super::P2OUT>;
#[doc = "Register P2OUT `reset()`'s with value 0"]
impl crate::ResetValue for super::P2OUT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P2OUT0`"]
pub type P2OUT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2OUT0`"]
pub struct P2OUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> P2OUT0_W<'a> {
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
#[doc = "Reader of field `P2OUT1`"]
pub type P2OUT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2OUT1`"]
pub struct P2OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> P2OUT1_W<'a> {
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
#[doc = "Reader of field `P2OUT2`"]
pub type P2OUT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2OUT2`"]
pub struct P2OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> P2OUT2_W<'a> {
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
#[doc = "Reader of field `P2OUT3`"]
pub type P2OUT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2OUT3`"]
pub struct P2OUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> P2OUT3_W<'a> {
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
#[doc = "Reader of field `P2OUT4`"]
pub type P2OUT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2OUT4`"]
pub struct P2OUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> P2OUT4_W<'a> {
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
#[doc = "Reader of field `P2OUT5`"]
pub type P2OUT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2OUT5`"]
pub struct P2OUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> P2OUT5_W<'a> {
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
#[doc = "Reader of field `P2OUT6`"]
pub type P2OUT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2OUT6`"]
pub struct P2OUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> P2OUT6_W<'a> {
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
#[doc = "Reader of field `P2OUT7`"]
pub type P2OUT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2OUT7`"]
pub struct P2OUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> P2OUT7_W<'a> {
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
    #[doc = "Bit 0 - P2OUT0"]
    #[inline(always)]
    pub fn p2out0(&self) -> P2OUT0_R {
        P2OUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P2OUT1"]
    #[inline(always)]
    pub fn p2out1(&self) -> P2OUT1_R {
        P2OUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P2OUT2"]
    #[inline(always)]
    pub fn p2out2(&self) -> P2OUT2_R {
        P2OUT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P2OUT3"]
    #[inline(always)]
    pub fn p2out3(&self) -> P2OUT3_R {
        P2OUT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P2OUT4"]
    #[inline(always)]
    pub fn p2out4(&self) -> P2OUT4_R {
        P2OUT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P2OUT5"]
    #[inline(always)]
    pub fn p2out5(&self) -> P2OUT5_R {
        P2OUT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P2OUT6"]
    #[inline(always)]
    pub fn p2out6(&self) -> P2OUT6_R {
        P2OUT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P2OUT7"]
    #[inline(always)]
    pub fn p2out7(&self) -> P2OUT7_R {
        P2OUT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2OUT0"]
    #[inline(always)]
    pub fn p2out0(&mut self) -> P2OUT0_W {
        P2OUT0_W { w: self }
    }
    #[doc = "Bit 1 - P2OUT1"]
    #[inline(always)]
    pub fn p2out1(&mut self) -> P2OUT1_W {
        P2OUT1_W { w: self }
    }
    #[doc = "Bit 2 - P2OUT2"]
    #[inline(always)]
    pub fn p2out2(&mut self) -> P2OUT2_W {
        P2OUT2_W { w: self }
    }
    #[doc = "Bit 3 - P2OUT3"]
    #[inline(always)]
    pub fn p2out3(&mut self) -> P2OUT3_W {
        P2OUT3_W { w: self }
    }
    #[doc = "Bit 4 - P2OUT4"]
    #[inline(always)]
    pub fn p2out4(&mut self) -> P2OUT4_W {
        P2OUT4_W { w: self }
    }
    #[doc = "Bit 5 - P2OUT5"]
    #[inline(always)]
    pub fn p2out5(&mut self) -> P2OUT5_W {
        P2OUT5_W { w: self }
    }
    #[doc = "Bit 6 - P2OUT6"]
    #[inline(always)]
    pub fn p2out6(&mut self) -> P2OUT6_W {
        P2OUT6_W { w: self }
    }
    #[doc = "Bit 7 - P2OUT7"]
    #[inline(always)]
    pub fn p2out7(&mut self) -> P2OUT7_W {
        P2OUT7_W { w: self }
    }
}
