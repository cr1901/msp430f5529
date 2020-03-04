#[doc = "Reader of register P3IN"]
pub type R = crate::R<u8, super::P3IN>;
#[doc = "Writer for register P3IN"]
pub type W = crate::W<u8, super::P3IN>;
#[doc = "Register P3IN `reset()`'s with value 0"]
impl crate::ResetValue for super::P3IN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P3IN0`"]
pub type P3IN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IN0`"]
pub struct P3IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IN0_W<'a> {
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
#[doc = "Reader of field `P3IN1`"]
pub type P3IN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IN1`"]
pub struct P3IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IN1_W<'a> {
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
#[doc = "Reader of field `P3IN2`"]
pub type P3IN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IN2`"]
pub struct P3IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IN2_W<'a> {
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
#[doc = "Reader of field `P3IN3`"]
pub type P3IN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IN3`"]
pub struct P3IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IN3_W<'a> {
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
#[doc = "Reader of field `P3IN4`"]
pub type P3IN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IN4`"]
pub struct P3IN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IN4_W<'a> {
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
#[doc = "Reader of field `P3IN5`"]
pub type P3IN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IN5`"]
pub struct P3IN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IN5_W<'a> {
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
#[doc = "Reader of field `P3IN6`"]
pub type P3IN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IN6`"]
pub struct P3IN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IN6_W<'a> {
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
#[doc = "Reader of field `P3IN7`"]
pub type P3IN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IN7`"]
pub struct P3IN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IN7_W<'a> {
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
    #[doc = "Bit 0 - P3IN0"]
    #[inline(always)]
    pub fn p3in0(&self) -> P3IN0_R {
        P3IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P3IN1"]
    #[inline(always)]
    pub fn p3in1(&self) -> P3IN1_R {
        P3IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P3IN2"]
    #[inline(always)]
    pub fn p3in2(&self) -> P3IN2_R {
        P3IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P3IN3"]
    #[inline(always)]
    pub fn p3in3(&self) -> P3IN3_R {
        P3IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P3IN4"]
    #[inline(always)]
    pub fn p3in4(&self) -> P3IN4_R {
        P3IN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P3IN5"]
    #[inline(always)]
    pub fn p3in5(&self) -> P3IN5_R {
        P3IN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P3IN6"]
    #[inline(always)]
    pub fn p3in6(&self) -> P3IN6_R {
        P3IN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P3IN7"]
    #[inline(always)]
    pub fn p3in7(&self) -> P3IN7_R {
        P3IN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3IN0"]
    #[inline(always)]
    pub fn p3in0(&mut self) -> P3IN0_W {
        P3IN0_W { w: self }
    }
    #[doc = "Bit 1 - P3IN1"]
    #[inline(always)]
    pub fn p3in1(&mut self) -> P3IN1_W {
        P3IN1_W { w: self }
    }
    #[doc = "Bit 2 - P3IN2"]
    #[inline(always)]
    pub fn p3in2(&mut self) -> P3IN2_W {
        P3IN2_W { w: self }
    }
    #[doc = "Bit 3 - P3IN3"]
    #[inline(always)]
    pub fn p3in3(&mut self) -> P3IN3_W {
        P3IN3_W { w: self }
    }
    #[doc = "Bit 4 - P3IN4"]
    #[inline(always)]
    pub fn p3in4(&mut self) -> P3IN4_W {
        P3IN4_W { w: self }
    }
    #[doc = "Bit 5 - P3IN5"]
    #[inline(always)]
    pub fn p3in5(&mut self) -> P3IN5_W {
        P3IN5_W { w: self }
    }
    #[doc = "Bit 6 - P3IN6"]
    #[inline(always)]
    pub fn p3in6(&mut self) -> P3IN6_W {
        P3IN6_W { w: self }
    }
    #[doc = "Bit 7 - P3IN7"]
    #[inline(always)]
    pub fn p3in7(&mut self) -> P3IN7_W {
        P3IN7_W { w: self }
    }
}
