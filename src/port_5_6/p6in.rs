#[doc = "Reader of register P6IN"]
pub type R = crate::R<u8, super::P6IN>;
#[doc = "Writer for register P6IN"]
pub type W = crate::W<u8, super::P6IN>;
#[doc = "Register P6IN `reset()`'s with value 0"]
impl crate::ResetValue for super::P6IN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P6IN0`"]
pub type P6IN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6IN0`"]
pub struct P6IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P6IN0_W<'a> {
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
#[doc = "Reader of field `P6IN1`"]
pub type P6IN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6IN1`"]
pub struct P6IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P6IN1_W<'a> {
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
#[doc = "Reader of field `P6IN2`"]
pub type P6IN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6IN2`"]
pub struct P6IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P6IN2_W<'a> {
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
#[doc = "Reader of field `P6IN3`"]
pub type P6IN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6IN3`"]
pub struct P6IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P6IN3_W<'a> {
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
#[doc = "Reader of field `P6IN4`"]
pub type P6IN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6IN4`"]
pub struct P6IN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P6IN4_W<'a> {
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
#[doc = "Reader of field `P6IN5`"]
pub type P6IN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6IN5`"]
pub struct P6IN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P6IN5_W<'a> {
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
#[doc = "Reader of field `P6IN6`"]
pub type P6IN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6IN6`"]
pub struct P6IN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P6IN6_W<'a> {
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
#[doc = "Reader of field `P6IN7`"]
pub type P6IN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6IN7`"]
pub struct P6IN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P6IN7_W<'a> {
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
    #[doc = "Bit 0 - P6IN0"]
    #[inline(always)]
    pub fn p6in0(&self) -> P6IN0_R {
        P6IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P6IN1"]
    #[inline(always)]
    pub fn p6in1(&self) -> P6IN1_R {
        P6IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P6IN2"]
    #[inline(always)]
    pub fn p6in2(&self) -> P6IN2_R {
        P6IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P6IN3"]
    #[inline(always)]
    pub fn p6in3(&self) -> P6IN3_R {
        P6IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P6IN4"]
    #[inline(always)]
    pub fn p6in4(&self) -> P6IN4_R {
        P6IN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P6IN5"]
    #[inline(always)]
    pub fn p6in5(&self) -> P6IN5_R {
        P6IN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P6IN6"]
    #[inline(always)]
    pub fn p6in6(&self) -> P6IN6_R {
        P6IN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P6IN7"]
    #[inline(always)]
    pub fn p6in7(&self) -> P6IN7_R {
        P6IN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P6IN0"]
    #[inline(always)]
    pub fn p6in0(&mut self) -> P6IN0_W {
        P6IN0_W { w: self }
    }
    #[doc = "Bit 1 - P6IN1"]
    #[inline(always)]
    pub fn p6in1(&mut self) -> P6IN1_W {
        P6IN1_W { w: self }
    }
    #[doc = "Bit 2 - P6IN2"]
    #[inline(always)]
    pub fn p6in2(&mut self) -> P6IN2_W {
        P6IN2_W { w: self }
    }
    #[doc = "Bit 3 - P6IN3"]
    #[inline(always)]
    pub fn p6in3(&mut self) -> P6IN3_W {
        P6IN3_W { w: self }
    }
    #[doc = "Bit 4 - P6IN4"]
    #[inline(always)]
    pub fn p6in4(&mut self) -> P6IN4_W {
        P6IN4_W { w: self }
    }
    #[doc = "Bit 5 - P6IN5"]
    #[inline(always)]
    pub fn p6in5(&mut self) -> P6IN5_W {
        P6IN5_W { w: self }
    }
    #[doc = "Bit 6 - P6IN6"]
    #[inline(always)]
    pub fn p6in6(&mut self) -> P6IN6_W {
        P6IN6_W { w: self }
    }
    #[doc = "Bit 7 - P6IN7"]
    #[inline(always)]
    pub fn p6in7(&mut self) -> P6IN7_W {
        P6IN7_W { w: self }
    }
}
