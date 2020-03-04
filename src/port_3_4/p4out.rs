#[doc = "Reader of register P4OUT"]
pub type R = crate::R<u8, super::P4OUT>;
#[doc = "Writer for register P4OUT"]
pub type W = crate::W<u8, super::P4OUT>;
#[doc = "Register P4OUT `reset()`'s with value 0"]
impl crate::ResetValue for super::P4OUT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P4OUT0`"]
pub type P4OUT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4OUT0`"]
pub struct P4OUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> P4OUT0_W<'a> {
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
#[doc = "Reader of field `P4OUT1`"]
pub type P4OUT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4OUT1`"]
pub struct P4OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> P4OUT1_W<'a> {
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
#[doc = "Reader of field `P4OUT2`"]
pub type P4OUT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4OUT2`"]
pub struct P4OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> P4OUT2_W<'a> {
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
#[doc = "Reader of field `P4OUT3`"]
pub type P4OUT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4OUT3`"]
pub struct P4OUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> P4OUT3_W<'a> {
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
#[doc = "Reader of field `P4OUT4`"]
pub type P4OUT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4OUT4`"]
pub struct P4OUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> P4OUT4_W<'a> {
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
#[doc = "Reader of field `P4OUT5`"]
pub type P4OUT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4OUT5`"]
pub struct P4OUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> P4OUT5_W<'a> {
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
#[doc = "Reader of field `P4OUT6`"]
pub type P4OUT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4OUT6`"]
pub struct P4OUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> P4OUT6_W<'a> {
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
#[doc = "Reader of field `P4OUT7`"]
pub type P4OUT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4OUT7`"]
pub struct P4OUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> P4OUT7_W<'a> {
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
    #[doc = "Bit 0 - P4OUT0"]
    #[inline(always)]
    pub fn p4out0(&self) -> P4OUT0_R {
        P4OUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P4OUT1"]
    #[inline(always)]
    pub fn p4out1(&self) -> P4OUT1_R {
        P4OUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P4OUT2"]
    #[inline(always)]
    pub fn p4out2(&self) -> P4OUT2_R {
        P4OUT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P4OUT3"]
    #[inline(always)]
    pub fn p4out3(&self) -> P4OUT3_R {
        P4OUT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P4OUT4"]
    #[inline(always)]
    pub fn p4out4(&self) -> P4OUT4_R {
        P4OUT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P4OUT5"]
    #[inline(always)]
    pub fn p4out5(&self) -> P4OUT5_R {
        P4OUT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P4OUT6"]
    #[inline(always)]
    pub fn p4out6(&self) -> P4OUT6_R {
        P4OUT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P4OUT7"]
    #[inline(always)]
    pub fn p4out7(&self) -> P4OUT7_R {
        P4OUT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4OUT0"]
    #[inline(always)]
    pub fn p4out0(&mut self) -> P4OUT0_W {
        P4OUT0_W { w: self }
    }
    #[doc = "Bit 1 - P4OUT1"]
    #[inline(always)]
    pub fn p4out1(&mut self) -> P4OUT1_W {
        P4OUT1_W { w: self }
    }
    #[doc = "Bit 2 - P4OUT2"]
    #[inline(always)]
    pub fn p4out2(&mut self) -> P4OUT2_W {
        P4OUT2_W { w: self }
    }
    #[doc = "Bit 3 - P4OUT3"]
    #[inline(always)]
    pub fn p4out3(&mut self) -> P4OUT3_W {
        P4OUT3_W { w: self }
    }
    #[doc = "Bit 4 - P4OUT4"]
    #[inline(always)]
    pub fn p4out4(&mut self) -> P4OUT4_W {
        P4OUT4_W { w: self }
    }
    #[doc = "Bit 5 - P4OUT5"]
    #[inline(always)]
    pub fn p4out5(&mut self) -> P4OUT5_W {
        P4OUT5_W { w: self }
    }
    #[doc = "Bit 6 - P4OUT6"]
    #[inline(always)]
    pub fn p4out6(&mut self) -> P4OUT6_W {
        P4OUT6_W { w: self }
    }
    #[doc = "Bit 7 - P4OUT7"]
    #[inline(always)]
    pub fn p4out7(&mut self) -> P4OUT7_W {
        P4OUT7_W { w: self }
    }
}
