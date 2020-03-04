#[doc = "Reader of register P7IN"]
pub type R = crate::R<u8, super::P7IN>;
#[doc = "Writer for register P7IN"]
pub type W = crate::W<u8, super::P7IN>;
#[doc = "Register P7IN `reset()`'s with value 0"]
impl crate::ResetValue for super::P7IN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P7IN0`"]
pub type P7IN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7IN0`"]
pub struct P7IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P7IN0_W<'a> {
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
#[doc = "Reader of field `P7IN1`"]
pub type P7IN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7IN1`"]
pub struct P7IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P7IN1_W<'a> {
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
#[doc = "Reader of field `P7IN2`"]
pub type P7IN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7IN2`"]
pub struct P7IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P7IN2_W<'a> {
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
#[doc = "Reader of field `P7IN3`"]
pub type P7IN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7IN3`"]
pub struct P7IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P7IN3_W<'a> {
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
#[doc = "Reader of field `P7IN4`"]
pub type P7IN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7IN4`"]
pub struct P7IN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P7IN4_W<'a> {
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
#[doc = "Reader of field `P7IN5`"]
pub type P7IN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7IN5`"]
pub struct P7IN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P7IN5_W<'a> {
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
#[doc = "Reader of field `P7IN6`"]
pub type P7IN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7IN6`"]
pub struct P7IN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P7IN6_W<'a> {
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
#[doc = "Reader of field `P7IN7`"]
pub type P7IN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7IN7`"]
pub struct P7IN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P7IN7_W<'a> {
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
    #[doc = "Bit 0 - P7IN0"]
    #[inline(always)]
    pub fn p7in0(&self) -> P7IN0_R {
        P7IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P7IN1"]
    #[inline(always)]
    pub fn p7in1(&self) -> P7IN1_R {
        P7IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P7IN2"]
    #[inline(always)]
    pub fn p7in2(&self) -> P7IN2_R {
        P7IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P7IN3"]
    #[inline(always)]
    pub fn p7in3(&self) -> P7IN3_R {
        P7IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P7IN4"]
    #[inline(always)]
    pub fn p7in4(&self) -> P7IN4_R {
        P7IN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P7IN5"]
    #[inline(always)]
    pub fn p7in5(&self) -> P7IN5_R {
        P7IN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P7IN6"]
    #[inline(always)]
    pub fn p7in6(&self) -> P7IN6_R {
        P7IN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P7IN7"]
    #[inline(always)]
    pub fn p7in7(&self) -> P7IN7_R {
        P7IN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P7IN0"]
    #[inline(always)]
    pub fn p7in0(&mut self) -> P7IN0_W {
        P7IN0_W { w: self }
    }
    #[doc = "Bit 1 - P7IN1"]
    #[inline(always)]
    pub fn p7in1(&mut self) -> P7IN1_W {
        P7IN1_W { w: self }
    }
    #[doc = "Bit 2 - P7IN2"]
    #[inline(always)]
    pub fn p7in2(&mut self) -> P7IN2_W {
        P7IN2_W { w: self }
    }
    #[doc = "Bit 3 - P7IN3"]
    #[inline(always)]
    pub fn p7in3(&mut self) -> P7IN3_W {
        P7IN3_W { w: self }
    }
    #[doc = "Bit 4 - P7IN4"]
    #[inline(always)]
    pub fn p7in4(&mut self) -> P7IN4_W {
        P7IN4_W { w: self }
    }
    #[doc = "Bit 5 - P7IN5"]
    #[inline(always)]
    pub fn p7in5(&mut self) -> P7IN5_W {
        P7IN5_W { w: self }
    }
    #[doc = "Bit 6 - P7IN6"]
    #[inline(always)]
    pub fn p7in6(&mut self) -> P7IN6_W {
        P7IN6_W { w: self }
    }
    #[doc = "Bit 7 - P7IN7"]
    #[inline(always)]
    pub fn p7in7(&mut self) -> P7IN7_W {
        P7IN7_W { w: self }
    }
}
