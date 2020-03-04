#[doc = "Reader of register P2IN"]
pub type R = crate::R<u8, super::P2IN>;
#[doc = "Writer for register P2IN"]
pub type W = crate::W<u8, super::P2IN>;
#[doc = "Register P2IN `reset()`'s with value 0"]
impl crate::ResetValue for super::P2IN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P2IN0`"]
pub type P2IN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IN0`"]
pub struct P2IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IN0_W<'a> {
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
#[doc = "Reader of field `P2IN1`"]
pub type P2IN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IN1`"]
pub struct P2IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IN1_W<'a> {
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
#[doc = "Reader of field `P2IN2`"]
pub type P2IN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IN2`"]
pub struct P2IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IN2_W<'a> {
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
#[doc = "Reader of field `P2IN3`"]
pub type P2IN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IN3`"]
pub struct P2IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IN3_W<'a> {
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
#[doc = "Reader of field `P2IN4`"]
pub type P2IN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IN4`"]
pub struct P2IN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IN4_W<'a> {
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
#[doc = "Reader of field `P2IN5`"]
pub type P2IN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IN5`"]
pub struct P2IN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IN5_W<'a> {
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
#[doc = "Reader of field `P2IN6`"]
pub type P2IN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IN6`"]
pub struct P2IN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IN6_W<'a> {
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
#[doc = "Reader of field `P2IN7`"]
pub type P2IN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IN7`"]
pub struct P2IN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IN7_W<'a> {
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
    #[doc = "Bit 0 - P2IN0"]
    #[inline(always)]
    pub fn p2in0(&self) -> P2IN0_R {
        P2IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P2IN1"]
    #[inline(always)]
    pub fn p2in1(&self) -> P2IN1_R {
        P2IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P2IN2"]
    #[inline(always)]
    pub fn p2in2(&self) -> P2IN2_R {
        P2IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P2IN3"]
    #[inline(always)]
    pub fn p2in3(&self) -> P2IN3_R {
        P2IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P2IN4"]
    #[inline(always)]
    pub fn p2in4(&self) -> P2IN4_R {
        P2IN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P2IN5"]
    #[inline(always)]
    pub fn p2in5(&self) -> P2IN5_R {
        P2IN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P2IN6"]
    #[inline(always)]
    pub fn p2in6(&self) -> P2IN6_R {
        P2IN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P2IN7"]
    #[inline(always)]
    pub fn p2in7(&self) -> P2IN7_R {
        P2IN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2IN0"]
    #[inline(always)]
    pub fn p2in0(&mut self) -> P2IN0_W {
        P2IN0_W { w: self }
    }
    #[doc = "Bit 1 - P2IN1"]
    #[inline(always)]
    pub fn p2in1(&mut self) -> P2IN1_W {
        P2IN1_W { w: self }
    }
    #[doc = "Bit 2 - P2IN2"]
    #[inline(always)]
    pub fn p2in2(&mut self) -> P2IN2_W {
        P2IN2_W { w: self }
    }
    #[doc = "Bit 3 - P2IN3"]
    #[inline(always)]
    pub fn p2in3(&mut self) -> P2IN3_W {
        P2IN3_W { w: self }
    }
    #[doc = "Bit 4 - P2IN4"]
    #[inline(always)]
    pub fn p2in4(&mut self) -> P2IN4_W {
        P2IN4_W { w: self }
    }
    #[doc = "Bit 5 - P2IN5"]
    #[inline(always)]
    pub fn p2in5(&mut self) -> P2IN5_W {
        P2IN5_W { w: self }
    }
    #[doc = "Bit 6 - P2IN6"]
    #[inline(always)]
    pub fn p2in6(&mut self) -> P2IN6_W {
        P2IN6_W { w: self }
    }
    #[doc = "Bit 7 - P2IN7"]
    #[inline(always)]
    pub fn p2in7(&mut self) -> P2IN7_W {
        P2IN7_W { w: self }
    }
}
