#[doc = "Reader of register P4DIR"]
pub type R = crate::R<u8, super::P4DIR>;
#[doc = "Writer for register P4DIR"]
pub type W = crate::W<u8, super::P4DIR>;
#[doc = "Register P4DIR `reset()`'s with value 0"]
impl crate::ResetValue for super::P4DIR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P4DIR0`"]
pub type P4DIR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4DIR0`"]
pub struct P4DIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DIR0_W<'a> {
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
#[doc = "Reader of field `P4DIR1`"]
pub type P4DIR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4DIR1`"]
pub struct P4DIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DIR1_W<'a> {
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
#[doc = "Reader of field `P4DIR2`"]
pub type P4DIR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4DIR2`"]
pub struct P4DIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DIR2_W<'a> {
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
#[doc = "Reader of field `P4DIR3`"]
pub type P4DIR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4DIR3`"]
pub struct P4DIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DIR3_W<'a> {
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
#[doc = "Reader of field `P4DIR4`"]
pub type P4DIR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4DIR4`"]
pub struct P4DIR4_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DIR4_W<'a> {
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
#[doc = "Reader of field `P4DIR5`"]
pub type P4DIR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4DIR5`"]
pub struct P4DIR5_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DIR5_W<'a> {
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
#[doc = "Reader of field `P4DIR6`"]
pub type P4DIR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4DIR6`"]
pub struct P4DIR6_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DIR6_W<'a> {
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
#[doc = "Reader of field `P4DIR7`"]
pub type P4DIR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4DIR7`"]
pub struct P4DIR7_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DIR7_W<'a> {
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
    #[doc = "Bit 0 - P4DIR0"]
    #[inline(always)]
    pub fn p4dir0(&self) -> P4DIR0_R {
        P4DIR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P4DIR1"]
    #[inline(always)]
    pub fn p4dir1(&self) -> P4DIR1_R {
        P4DIR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P4DIR2"]
    #[inline(always)]
    pub fn p4dir2(&self) -> P4DIR2_R {
        P4DIR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P4DIR3"]
    #[inline(always)]
    pub fn p4dir3(&self) -> P4DIR3_R {
        P4DIR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P4DIR4"]
    #[inline(always)]
    pub fn p4dir4(&self) -> P4DIR4_R {
        P4DIR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P4DIR5"]
    #[inline(always)]
    pub fn p4dir5(&self) -> P4DIR5_R {
        P4DIR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P4DIR6"]
    #[inline(always)]
    pub fn p4dir6(&self) -> P4DIR6_R {
        P4DIR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P4DIR7"]
    #[inline(always)]
    pub fn p4dir7(&self) -> P4DIR7_R {
        P4DIR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4DIR0"]
    #[inline(always)]
    pub fn p4dir0(&mut self) -> P4DIR0_W {
        P4DIR0_W { w: self }
    }
    #[doc = "Bit 1 - P4DIR1"]
    #[inline(always)]
    pub fn p4dir1(&mut self) -> P4DIR1_W {
        P4DIR1_W { w: self }
    }
    #[doc = "Bit 2 - P4DIR2"]
    #[inline(always)]
    pub fn p4dir2(&mut self) -> P4DIR2_W {
        P4DIR2_W { w: self }
    }
    #[doc = "Bit 3 - P4DIR3"]
    #[inline(always)]
    pub fn p4dir3(&mut self) -> P4DIR3_W {
        P4DIR3_W { w: self }
    }
    #[doc = "Bit 4 - P4DIR4"]
    #[inline(always)]
    pub fn p4dir4(&mut self) -> P4DIR4_W {
        P4DIR4_W { w: self }
    }
    #[doc = "Bit 5 - P4DIR5"]
    #[inline(always)]
    pub fn p4dir5(&mut self) -> P4DIR5_W {
        P4DIR5_W { w: self }
    }
    #[doc = "Bit 6 - P4DIR6"]
    #[inline(always)]
    pub fn p4dir6(&mut self) -> P4DIR6_W {
        P4DIR6_W { w: self }
    }
    #[doc = "Bit 7 - P4DIR7"]
    #[inline(always)]
    pub fn p4dir7(&mut self) -> P4DIR7_W {
        P4DIR7_W { w: self }
    }
}
