#[doc = "Reader of register P4IN"]
pub type R = crate::R<u8, super::P4IN>;
#[doc = "Writer for register P4IN"]
pub type W = crate::W<u8, super::P4IN>;
#[doc = "Register P4IN `reset()`'s with value 0"]
impl crate::ResetValue for super::P4IN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P4IN0`"]
pub type P4IN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IN0`"]
pub struct P4IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IN0_W<'a> {
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
#[doc = "Reader of field `P4IN1`"]
pub type P4IN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IN1`"]
pub struct P4IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IN1_W<'a> {
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
#[doc = "Reader of field `P4IN2`"]
pub type P4IN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IN2`"]
pub struct P4IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IN2_W<'a> {
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
#[doc = "Reader of field `P4IN3`"]
pub type P4IN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IN3`"]
pub struct P4IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IN3_W<'a> {
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
#[doc = "Reader of field `P4IN4`"]
pub type P4IN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IN4`"]
pub struct P4IN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IN4_W<'a> {
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
#[doc = "Reader of field `P4IN5`"]
pub type P4IN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IN5`"]
pub struct P4IN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IN5_W<'a> {
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
#[doc = "Reader of field `P4IN6`"]
pub type P4IN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IN6`"]
pub struct P4IN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IN6_W<'a> {
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
#[doc = "Reader of field `P4IN7`"]
pub type P4IN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IN7`"]
pub struct P4IN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IN7_W<'a> {
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
    #[doc = "Bit 0 - P4IN0"]
    #[inline(always)]
    pub fn p4in0(&self) -> P4IN0_R {
        P4IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P4IN1"]
    #[inline(always)]
    pub fn p4in1(&self) -> P4IN1_R {
        P4IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P4IN2"]
    #[inline(always)]
    pub fn p4in2(&self) -> P4IN2_R {
        P4IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P4IN3"]
    #[inline(always)]
    pub fn p4in3(&self) -> P4IN3_R {
        P4IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P4IN4"]
    #[inline(always)]
    pub fn p4in4(&self) -> P4IN4_R {
        P4IN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P4IN5"]
    #[inline(always)]
    pub fn p4in5(&self) -> P4IN5_R {
        P4IN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P4IN6"]
    #[inline(always)]
    pub fn p4in6(&self) -> P4IN6_R {
        P4IN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P4IN7"]
    #[inline(always)]
    pub fn p4in7(&self) -> P4IN7_R {
        P4IN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4IN0"]
    #[inline(always)]
    pub fn p4in0(&mut self) -> P4IN0_W {
        P4IN0_W { w: self }
    }
    #[doc = "Bit 1 - P4IN1"]
    #[inline(always)]
    pub fn p4in1(&mut self) -> P4IN1_W {
        P4IN1_W { w: self }
    }
    #[doc = "Bit 2 - P4IN2"]
    #[inline(always)]
    pub fn p4in2(&mut self) -> P4IN2_W {
        P4IN2_W { w: self }
    }
    #[doc = "Bit 3 - P4IN3"]
    #[inline(always)]
    pub fn p4in3(&mut self) -> P4IN3_W {
        P4IN3_W { w: self }
    }
    #[doc = "Bit 4 - P4IN4"]
    #[inline(always)]
    pub fn p4in4(&mut self) -> P4IN4_W {
        P4IN4_W { w: self }
    }
    #[doc = "Bit 5 - P4IN5"]
    #[inline(always)]
    pub fn p4in5(&mut self) -> P4IN5_W {
        P4IN5_W { w: self }
    }
    #[doc = "Bit 6 - P4IN6"]
    #[inline(always)]
    pub fn p4in6(&mut self) -> P4IN6_W {
        P4IN6_W { w: self }
    }
    #[doc = "Bit 7 - P4IN7"]
    #[inline(always)]
    pub fn p4in7(&mut self) -> P4IN7_W {
        P4IN7_W { w: self }
    }
}
