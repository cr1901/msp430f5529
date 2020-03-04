#[doc = "Reader of register P1IN"]
pub type R = crate::R<u8, super::P1IN>;
#[doc = "Writer for register P1IN"]
pub type W = crate::W<u8, super::P1IN>;
#[doc = "Register P1IN `reset()`'s with value 0"]
impl crate::ResetValue for super::P1IN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P1IN0`"]
pub type P1IN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IN0`"]
pub struct P1IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IN0_W<'a> {
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
#[doc = "Reader of field `P1IN1`"]
pub type P1IN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IN1`"]
pub struct P1IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IN1_W<'a> {
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
#[doc = "Reader of field `P1IN2`"]
pub type P1IN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IN2`"]
pub struct P1IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IN2_W<'a> {
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
#[doc = "Reader of field `P1IN3`"]
pub type P1IN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IN3`"]
pub struct P1IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IN3_W<'a> {
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
#[doc = "Reader of field `P1IN4`"]
pub type P1IN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IN4`"]
pub struct P1IN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IN4_W<'a> {
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
#[doc = "Reader of field `P1IN5`"]
pub type P1IN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IN5`"]
pub struct P1IN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IN5_W<'a> {
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
#[doc = "Reader of field `P1IN6`"]
pub type P1IN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IN6`"]
pub struct P1IN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IN6_W<'a> {
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
#[doc = "Reader of field `P1IN7`"]
pub type P1IN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IN7`"]
pub struct P1IN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IN7_W<'a> {
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
    #[doc = "Bit 0 - P1IN0"]
    #[inline(always)]
    pub fn p1in0(&self) -> P1IN0_R {
        P1IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P1IN1"]
    #[inline(always)]
    pub fn p1in1(&self) -> P1IN1_R {
        P1IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P1IN2"]
    #[inline(always)]
    pub fn p1in2(&self) -> P1IN2_R {
        P1IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P1IN3"]
    #[inline(always)]
    pub fn p1in3(&self) -> P1IN3_R {
        P1IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P1IN4"]
    #[inline(always)]
    pub fn p1in4(&self) -> P1IN4_R {
        P1IN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P1IN5"]
    #[inline(always)]
    pub fn p1in5(&self) -> P1IN5_R {
        P1IN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P1IN6"]
    #[inline(always)]
    pub fn p1in6(&self) -> P1IN6_R {
        P1IN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P1IN7"]
    #[inline(always)]
    pub fn p1in7(&self) -> P1IN7_R {
        P1IN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1IN0"]
    #[inline(always)]
    pub fn p1in0(&mut self) -> P1IN0_W {
        P1IN0_W { w: self }
    }
    #[doc = "Bit 1 - P1IN1"]
    #[inline(always)]
    pub fn p1in1(&mut self) -> P1IN1_W {
        P1IN1_W { w: self }
    }
    #[doc = "Bit 2 - P1IN2"]
    #[inline(always)]
    pub fn p1in2(&mut self) -> P1IN2_W {
        P1IN2_W { w: self }
    }
    #[doc = "Bit 3 - P1IN3"]
    #[inline(always)]
    pub fn p1in3(&mut self) -> P1IN3_W {
        P1IN3_W { w: self }
    }
    #[doc = "Bit 4 - P1IN4"]
    #[inline(always)]
    pub fn p1in4(&mut self) -> P1IN4_W {
        P1IN4_W { w: self }
    }
    #[doc = "Bit 5 - P1IN5"]
    #[inline(always)]
    pub fn p1in5(&mut self) -> P1IN5_W {
        P1IN5_W { w: self }
    }
    #[doc = "Bit 6 - P1IN6"]
    #[inline(always)]
    pub fn p1in6(&mut self) -> P1IN6_W {
        P1IN6_W { w: self }
    }
    #[doc = "Bit 7 - P1IN7"]
    #[inline(always)]
    pub fn p1in7(&mut self) -> P1IN7_W {
        P1IN7_W { w: self }
    }
}
