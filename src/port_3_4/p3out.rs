#[doc = "Reader of register P3OUT"]
pub type R = crate::R<u8, super::P3OUT>;
#[doc = "Writer for register P3OUT"]
pub type W = crate::W<u8, super::P3OUT>;
#[doc = "Register P3OUT `reset()`'s with value 0"]
impl crate::ResetValue for super::P3OUT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P3OUT0`"]
pub type P3OUT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3OUT0`"]
pub struct P3OUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> P3OUT0_W<'a> {
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
#[doc = "Reader of field `P3OUT1`"]
pub type P3OUT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3OUT1`"]
pub struct P3OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> P3OUT1_W<'a> {
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
#[doc = "Reader of field `P3OUT2`"]
pub type P3OUT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3OUT2`"]
pub struct P3OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> P3OUT2_W<'a> {
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
#[doc = "Reader of field `P3OUT3`"]
pub type P3OUT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3OUT3`"]
pub struct P3OUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> P3OUT3_W<'a> {
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
#[doc = "Reader of field `P3OUT4`"]
pub type P3OUT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3OUT4`"]
pub struct P3OUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> P3OUT4_W<'a> {
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
#[doc = "Reader of field `P3OUT5`"]
pub type P3OUT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3OUT5`"]
pub struct P3OUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> P3OUT5_W<'a> {
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
#[doc = "Reader of field `P3OUT6`"]
pub type P3OUT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3OUT6`"]
pub struct P3OUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> P3OUT6_W<'a> {
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
#[doc = "Reader of field `P3OUT7`"]
pub type P3OUT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3OUT7`"]
pub struct P3OUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> P3OUT7_W<'a> {
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
    #[doc = "Bit 0 - P3OUT0"]
    #[inline(always)]
    pub fn p3out0(&self) -> P3OUT0_R {
        P3OUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P3OUT1"]
    #[inline(always)]
    pub fn p3out1(&self) -> P3OUT1_R {
        P3OUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P3OUT2"]
    #[inline(always)]
    pub fn p3out2(&self) -> P3OUT2_R {
        P3OUT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P3OUT3"]
    #[inline(always)]
    pub fn p3out3(&self) -> P3OUT3_R {
        P3OUT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P3OUT4"]
    #[inline(always)]
    pub fn p3out4(&self) -> P3OUT4_R {
        P3OUT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P3OUT5"]
    #[inline(always)]
    pub fn p3out5(&self) -> P3OUT5_R {
        P3OUT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P3OUT6"]
    #[inline(always)]
    pub fn p3out6(&self) -> P3OUT6_R {
        P3OUT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P3OUT7"]
    #[inline(always)]
    pub fn p3out7(&self) -> P3OUT7_R {
        P3OUT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3OUT0"]
    #[inline(always)]
    pub fn p3out0(&mut self) -> P3OUT0_W {
        P3OUT0_W { w: self }
    }
    #[doc = "Bit 1 - P3OUT1"]
    #[inline(always)]
    pub fn p3out1(&mut self) -> P3OUT1_W {
        P3OUT1_W { w: self }
    }
    #[doc = "Bit 2 - P3OUT2"]
    #[inline(always)]
    pub fn p3out2(&mut self) -> P3OUT2_W {
        P3OUT2_W { w: self }
    }
    #[doc = "Bit 3 - P3OUT3"]
    #[inline(always)]
    pub fn p3out3(&mut self) -> P3OUT3_W {
        P3OUT3_W { w: self }
    }
    #[doc = "Bit 4 - P3OUT4"]
    #[inline(always)]
    pub fn p3out4(&mut self) -> P3OUT4_W {
        P3OUT4_W { w: self }
    }
    #[doc = "Bit 5 - P3OUT5"]
    #[inline(always)]
    pub fn p3out5(&mut self) -> P3OUT5_W {
        P3OUT5_W { w: self }
    }
    #[doc = "Bit 6 - P3OUT6"]
    #[inline(always)]
    pub fn p3out6(&mut self) -> P3OUT6_W {
        P3OUT6_W { w: self }
    }
    #[doc = "Bit 7 - P3OUT7"]
    #[inline(always)]
    pub fn p3out7(&mut self) -> P3OUT7_W {
        P3OUT7_W { w: self }
    }
}
