#[doc = "Reader of register P5IN"]
pub type R = crate::R<u8, super::P5IN>;
#[doc = "Writer for register P5IN"]
pub type W = crate::W<u8, super::P5IN>;
#[doc = "Register P5IN `reset()`'s with value 0"]
impl crate::ResetValue for super::P5IN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P5IN0`"]
pub type P5IN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5IN0`"]
pub struct P5IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IN0_W<'a> {
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
#[doc = "Reader of field `P5IN1`"]
pub type P5IN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5IN1`"]
pub struct P5IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IN1_W<'a> {
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
#[doc = "Reader of field `P5IN2`"]
pub type P5IN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5IN2`"]
pub struct P5IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IN2_W<'a> {
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
#[doc = "Reader of field `P5IN3`"]
pub type P5IN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5IN3`"]
pub struct P5IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IN3_W<'a> {
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
#[doc = "Reader of field `P5IN4`"]
pub type P5IN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5IN4`"]
pub struct P5IN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IN4_W<'a> {
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
#[doc = "Reader of field `P5IN5`"]
pub type P5IN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5IN5`"]
pub struct P5IN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IN5_W<'a> {
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
#[doc = "Reader of field `P5IN6`"]
pub type P5IN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5IN6`"]
pub struct P5IN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IN6_W<'a> {
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
#[doc = "Reader of field `P5IN7`"]
pub type P5IN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5IN7`"]
pub struct P5IN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IN7_W<'a> {
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
    #[doc = "Bit 0 - P5IN0"]
    #[inline(always)]
    pub fn p5in0(&self) -> P5IN0_R {
        P5IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P5IN1"]
    #[inline(always)]
    pub fn p5in1(&self) -> P5IN1_R {
        P5IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P5IN2"]
    #[inline(always)]
    pub fn p5in2(&self) -> P5IN2_R {
        P5IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P5IN3"]
    #[inline(always)]
    pub fn p5in3(&self) -> P5IN3_R {
        P5IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P5IN4"]
    #[inline(always)]
    pub fn p5in4(&self) -> P5IN4_R {
        P5IN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P5IN5"]
    #[inline(always)]
    pub fn p5in5(&self) -> P5IN5_R {
        P5IN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P5IN6"]
    #[inline(always)]
    pub fn p5in6(&self) -> P5IN6_R {
        P5IN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P5IN7"]
    #[inline(always)]
    pub fn p5in7(&self) -> P5IN7_R {
        P5IN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P5IN0"]
    #[inline(always)]
    pub fn p5in0(&mut self) -> P5IN0_W {
        P5IN0_W { w: self }
    }
    #[doc = "Bit 1 - P5IN1"]
    #[inline(always)]
    pub fn p5in1(&mut self) -> P5IN1_W {
        P5IN1_W { w: self }
    }
    #[doc = "Bit 2 - P5IN2"]
    #[inline(always)]
    pub fn p5in2(&mut self) -> P5IN2_W {
        P5IN2_W { w: self }
    }
    #[doc = "Bit 3 - P5IN3"]
    #[inline(always)]
    pub fn p5in3(&mut self) -> P5IN3_W {
        P5IN3_W { w: self }
    }
    #[doc = "Bit 4 - P5IN4"]
    #[inline(always)]
    pub fn p5in4(&mut self) -> P5IN4_W {
        P5IN4_W { w: self }
    }
    #[doc = "Bit 5 - P5IN5"]
    #[inline(always)]
    pub fn p5in5(&mut self) -> P5IN5_W {
        P5IN5_W { w: self }
    }
    #[doc = "Bit 6 - P5IN6"]
    #[inline(always)]
    pub fn p5in6(&mut self) -> P5IN6_W {
        P5IN6_W { w: self }
    }
    #[doc = "Bit 7 - P5IN7"]
    #[inline(always)]
    pub fn p5in7(&mut self) -> P5IN7_W {
        P5IN7_W { w: self }
    }
}
