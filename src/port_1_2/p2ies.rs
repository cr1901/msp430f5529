#[doc = "Reader of register P2IES"]
pub type R = crate::R<u8, super::P2IES>;
#[doc = "Writer for register P2IES"]
pub type W = crate::W<u8, super::P2IES>;
#[doc = "Register P2IES `reset()`'s with value 0"]
impl crate::ResetValue for super::P2IES {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P2IES0`"]
pub type P2IES0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IES0`"]
pub struct P2IES0_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IES0_W<'a> {
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
#[doc = "Reader of field `P2IES1`"]
pub type P2IES1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IES1`"]
pub struct P2IES1_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IES1_W<'a> {
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
#[doc = "Reader of field `P2IES2`"]
pub type P2IES2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IES2`"]
pub struct P2IES2_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IES2_W<'a> {
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
#[doc = "Reader of field `P2IES3`"]
pub type P2IES3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IES3`"]
pub struct P2IES3_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IES3_W<'a> {
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
#[doc = "Reader of field `P2IES4`"]
pub type P2IES4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IES4`"]
pub struct P2IES4_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IES4_W<'a> {
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
#[doc = "Reader of field `P2IES5`"]
pub type P2IES5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IES5`"]
pub struct P2IES5_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IES5_W<'a> {
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
#[doc = "Reader of field `P2IES6`"]
pub type P2IES6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IES6`"]
pub struct P2IES6_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IES6_W<'a> {
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
#[doc = "Reader of field `P2IES7`"]
pub type P2IES7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2IES7`"]
pub struct P2IES7_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IES7_W<'a> {
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
    #[doc = "Bit 0 - P2IES0"]
    #[inline(always)]
    pub fn p2ies0(&self) -> P2IES0_R {
        P2IES0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P2IES1"]
    #[inline(always)]
    pub fn p2ies1(&self) -> P2IES1_R {
        P2IES1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P2IES2"]
    #[inline(always)]
    pub fn p2ies2(&self) -> P2IES2_R {
        P2IES2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P2IES3"]
    #[inline(always)]
    pub fn p2ies3(&self) -> P2IES3_R {
        P2IES3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P2IES4"]
    #[inline(always)]
    pub fn p2ies4(&self) -> P2IES4_R {
        P2IES4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P2IES5"]
    #[inline(always)]
    pub fn p2ies5(&self) -> P2IES5_R {
        P2IES5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P2IES6"]
    #[inline(always)]
    pub fn p2ies6(&self) -> P2IES6_R {
        P2IES6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P2IES7"]
    #[inline(always)]
    pub fn p2ies7(&self) -> P2IES7_R {
        P2IES7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2IES0"]
    #[inline(always)]
    pub fn p2ies0(&mut self) -> P2IES0_W {
        P2IES0_W { w: self }
    }
    #[doc = "Bit 1 - P2IES1"]
    #[inline(always)]
    pub fn p2ies1(&mut self) -> P2IES1_W {
        P2IES1_W { w: self }
    }
    #[doc = "Bit 2 - P2IES2"]
    #[inline(always)]
    pub fn p2ies2(&mut self) -> P2IES2_W {
        P2IES2_W { w: self }
    }
    #[doc = "Bit 3 - P2IES3"]
    #[inline(always)]
    pub fn p2ies3(&mut self) -> P2IES3_W {
        P2IES3_W { w: self }
    }
    #[doc = "Bit 4 - P2IES4"]
    #[inline(always)]
    pub fn p2ies4(&mut self) -> P2IES4_W {
        P2IES4_W { w: self }
    }
    #[doc = "Bit 5 - P2IES5"]
    #[inline(always)]
    pub fn p2ies5(&mut self) -> P2IES5_W {
        P2IES5_W { w: self }
    }
    #[doc = "Bit 6 - P2IES6"]
    #[inline(always)]
    pub fn p2ies6(&mut self) -> P2IES6_W {
        P2IES6_W { w: self }
    }
    #[doc = "Bit 7 - P2IES7"]
    #[inline(always)]
    pub fn p2ies7(&mut self) -> P2IES7_W {
        P2IES7_W { w: self }
    }
}
