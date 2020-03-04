#[doc = "Reader of register P8IN"]
pub type R = crate::R<u8, super::P8IN>;
#[doc = "Writer for register P8IN"]
pub type W = crate::W<u8, super::P8IN>;
#[doc = "Register P8IN `reset()`'s with value 0"]
impl crate::ResetValue for super::P8IN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P8IN0`"]
pub type P8IN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8IN0`"]
pub struct P8IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P8IN0_W<'a> {
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
#[doc = "Reader of field `P8IN1`"]
pub type P8IN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8IN1`"]
pub struct P8IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P8IN1_W<'a> {
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
#[doc = "Reader of field `P8IN2`"]
pub type P8IN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8IN2`"]
pub struct P8IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P8IN2_W<'a> {
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
#[doc = "Reader of field `P8IN3`"]
pub type P8IN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8IN3`"]
pub struct P8IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P8IN3_W<'a> {
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
#[doc = "Reader of field `P8IN4`"]
pub type P8IN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8IN4`"]
pub struct P8IN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P8IN4_W<'a> {
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
#[doc = "Reader of field `P8IN5`"]
pub type P8IN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8IN5`"]
pub struct P8IN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P8IN5_W<'a> {
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
#[doc = "Reader of field `P8IN6`"]
pub type P8IN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8IN6`"]
pub struct P8IN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P8IN6_W<'a> {
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
#[doc = "Reader of field `P8IN7`"]
pub type P8IN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8IN7`"]
pub struct P8IN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P8IN7_W<'a> {
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
    #[doc = "Bit 0 - P8IN0"]
    #[inline(always)]
    pub fn p8in0(&self) -> P8IN0_R {
        P8IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P8IN1"]
    #[inline(always)]
    pub fn p8in1(&self) -> P8IN1_R {
        P8IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P8IN2"]
    #[inline(always)]
    pub fn p8in2(&self) -> P8IN2_R {
        P8IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P8IN3"]
    #[inline(always)]
    pub fn p8in3(&self) -> P8IN3_R {
        P8IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P8IN4"]
    #[inline(always)]
    pub fn p8in4(&self) -> P8IN4_R {
        P8IN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P8IN5"]
    #[inline(always)]
    pub fn p8in5(&self) -> P8IN5_R {
        P8IN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P8IN6"]
    #[inline(always)]
    pub fn p8in6(&self) -> P8IN6_R {
        P8IN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P8IN7"]
    #[inline(always)]
    pub fn p8in7(&self) -> P8IN7_R {
        P8IN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P8IN0"]
    #[inline(always)]
    pub fn p8in0(&mut self) -> P8IN0_W {
        P8IN0_W { w: self }
    }
    #[doc = "Bit 1 - P8IN1"]
    #[inline(always)]
    pub fn p8in1(&mut self) -> P8IN1_W {
        P8IN1_W { w: self }
    }
    #[doc = "Bit 2 - P8IN2"]
    #[inline(always)]
    pub fn p8in2(&mut self) -> P8IN2_W {
        P8IN2_W { w: self }
    }
    #[doc = "Bit 3 - P8IN3"]
    #[inline(always)]
    pub fn p8in3(&mut self) -> P8IN3_W {
        P8IN3_W { w: self }
    }
    #[doc = "Bit 4 - P8IN4"]
    #[inline(always)]
    pub fn p8in4(&mut self) -> P8IN4_W {
        P8IN4_W { w: self }
    }
    #[doc = "Bit 5 - P8IN5"]
    #[inline(always)]
    pub fn p8in5(&mut self) -> P8IN5_W {
        P8IN5_W { w: self }
    }
    #[doc = "Bit 6 - P8IN6"]
    #[inline(always)]
    pub fn p8in6(&mut self) -> P8IN6_W {
        P8IN6_W { w: self }
    }
    #[doc = "Bit 7 - P8IN7"]
    #[inline(always)]
    pub fn p8in7(&mut self) -> P8IN7_W {
        P8IN7_W { w: self }
    }
}
