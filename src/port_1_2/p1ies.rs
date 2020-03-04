#[doc = "Reader of register P1IES"]
pub type R = crate::R<u8, super::P1IES>;
#[doc = "Writer for register P1IES"]
pub type W = crate::W<u8, super::P1IES>;
#[doc = "Register P1IES `reset()`'s with value 0"]
impl crate::ResetValue for super::P1IES {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P1IES0`"]
pub type P1IES0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IES0`"]
pub struct P1IES0_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IES0_W<'a> {
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
#[doc = "Reader of field `P1IES1`"]
pub type P1IES1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IES1`"]
pub struct P1IES1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IES1_W<'a> {
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
#[doc = "Reader of field `P1IES2`"]
pub type P1IES2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IES2`"]
pub struct P1IES2_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IES2_W<'a> {
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
#[doc = "Reader of field `P1IES3`"]
pub type P1IES3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IES3`"]
pub struct P1IES3_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IES3_W<'a> {
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
#[doc = "Reader of field `P1IES4`"]
pub type P1IES4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IES4`"]
pub struct P1IES4_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IES4_W<'a> {
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
#[doc = "Reader of field `P1IES5`"]
pub type P1IES5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IES5`"]
pub struct P1IES5_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IES5_W<'a> {
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
#[doc = "Reader of field `P1IES6`"]
pub type P1IES6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IES6`"]
pub struct P1IES6_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IES6_W<'a> {
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
#[doc = "Reader of field `P1IES7`"]
pub type P1IES7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1IES7`"]
pub struct P1IES7_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IES7_W<'a> {
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
    #[doc = "Bit 0 - P1IES0"]
    #[inline(always)]
    pub fn p1ies0(&self) -> P1IES0_R {
        P1IES0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P1IES1"]
    #[inline(always)]
    pub fn p1ies1(&self) -> P1IES1_R {
        P1IES1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P1IES2"]
    #[inline(always)]
    pub fn p1ies2(&self) -> P1IES2_R {
        P1IES2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P1IES3"]
    #[inline(always)]
    pub fn p1ies3(&self) -> P1IES3_R {
        P1IES3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P1IES4"]
    #[inline(always)]
    pub fn p1ies4(&self) -> P1IES4_R {
        P1IES4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P1IES5"]
    #[inline(always)]
    pub fn p1ies5(&self) -> P1IES5_R {
        P1IES5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P1IES6"]
    #[inline(always)]
    pub fn p1ies6(&self) -> P1IES6_R {
        P1IES6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P1IES7"]
    #[inline(always)]
    pub fn p1ies7(&self) -> P1IES7_R {
        P1IES7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1IES0"]
    #[inline(always)]
    pub fn p1ies0(&mut self) -> P1IES0_W {
        P1IES0_W { w: self }
    }
    #[doc = "Bit 1 - P1IES1"]
    #[inline(always)]
    pub fn p1ies1(&mut self) -> P1IES1_W {
        P1IES1_W { w: self }
    }
    #[doc = "Bit 2 - P1IES2"]
    #[inline(always)]
    pub fn p1ies2(&mut self) -> P1IES2_W {
        P1IES2_W { w: self }
    }
    #[doc = "Bit 3 - P1IES3"]
    #[inline(always)]
    pub fn p1ies3(&mut self) -> P1IES3_W {
        P1IES3_W { w: self }
    }
    #[doc = "Bit 4 - P1IES4"]
    #[inline(always)]
    pub fn p1ies4(&mut self) -> P1IES4_W {
        P1IES4_W { w: self }
    }
    #[doc = "Bit 5 - P1IES5"]
    #[inline(always)]
    pub fn p1ies5(&mut self) -> P1IES5_W {
        P1IES5_W { w: self }
    }
    #[doc = "Bit 6 - P1IES6"]
    #[inline(always)]
    pub fn p1ies6(&mut self) -> P1IES6_W {
        P1IES6_W { w: self }
    }
    #[doc = "Bit 7 - P1IES7"]
    #[inline(always)]
    pub fn p1ies7(&mut self) -> P1IES7_W {
        P1IES7_W { w: self }
    }
}
