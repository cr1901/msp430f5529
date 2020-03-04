#[doc = "Reader of register P2DIR"]
pub type R = crate::R<u8, super::P2DIR>;
#[doc = "Writer for register P2DIR"]
pub type W = crate::W<u8, super::P2DIR>;
#[doc = "Register P2DIR `reset()`'s with value 0"]
impl crate::ResetValue for super::P2DIR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P2DIR0`"]
pub type P2DIR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2DIR0`"]
pub struct P2DIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DIR0_W<'a> {
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
#[doc = "Reader of field `P2DIR1`"]
pub type P2DIR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2DIR1`"]
pub struct P2DIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DIR1_W<'a> {
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
#[doc = "Reader of field `P2DIR2`"]
pub type P2DIR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2DIR2`"]
pub struct P2DIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DIR2_W<'a> {
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
#[doc = "Reader of field `P2DIR3`"]
pub type P2DIR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2DIR3`"]
pub struct P2DIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DIR3_W<'a> {
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
#[doc = "Reader of field `P2DIR4`"]
pub type P2DIR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2DIR4`"]
pub struct P2DIR4_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DIR4_W<'a> {
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
#[doc = "Reader of field `P2DIR5`"]
pub type P2DIR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2DIR5`"]
pub struct P2DIR5_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DIR5_W<'a> {
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
#[doc = "Reader of field `P2DIR6`"]
pub type P2DIR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2DIR6`"]
pub struct P2DIR6_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DIR6_W<'a> {
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
#[doc = "Reader of field `P2DIR7`"]
pub type P2DIR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2DIR7`"]
pub struct P2DIR7_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DIR7_W<'a> {
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
    #[doc = "Bit 0 - P2DIR0"]
    #[inline(always)]
    pub fn p2dir0(&self) -> P2DIR0_R {
        P2DIR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P2DIR1"]
    #[inline(always)]
    pub fn p2dir1(&self) -> P2DIR1_R {
        P2DIR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P2DIR2"]
    #[inline(always)]
    pub fn p2dir2(&self) -> P2DIR2_R {
        P2DIR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P2DIR3"]
    #[inline(always)]
    pub fn p2dir3(&self) -> P2DIR3_R {
        P2DIR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P2DIR4"]
    #[inline(always)]
    pub fn p2dir4(&self) -> P2DIR4_R {
        P2DIR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P2DIR5"]
    #[inline(always)]
    pub fn p2dir5(&self) -> P2DIR5_R {
        P2DIR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P2DIR6"]
    #[inline(always)]
    pub fn p2dir6(&self) -> P2DIR6_R {
        P2DIR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P2DIR7"]
    #[inline(always)]
    pub fn p2dir7(&self) -> P2DIR7_R {
        P2DIR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2DIR0"]
    #[inline(always)]
    pub fn p2dir0(&mut self) -> P2DIR0_W {
        P2DIR0_W { w: self }
    }
    #[doc = "Bit 1 - P2DIR1"]
    #[inline(always)]
    pub fn p2dir1(&mut self) -> P2DIR1_W {
        P2DIR1_W { w: self }
    }
    #[doc = "Bit 2 - P2DIR2"]
    #[inline(always)]
    pub fn p2dir2(&mut self) -> P2DIR2_W {
        P2DIR2_W { w: self }
    }
    #[doc = "Bit 3 - P2DIR3"]
    #[inline(always)]
    pub fn p2dir3(&mut self) -> P2DIR3_W {
        P2DIR3_W { w: self }
    }
    #[doc = "Bit 4 - P2DIR4"]
    #[inline(always)]
    pub fn p2dir4(&mut self) -> P2DIR4_W {
        P2DIR4_W { w: self }
    }
    #[doc = "Bit 5 - P2DIR5"]
    #[inline(always)]
    pub fn p2dir5(&mut self) -> P2DIR5_W {
        P2DIR5_W { w: self }
    }
    #[doc = "Bit 6 - P2DIR6"]
    #[inline(always)]
    pub fn p2dir6(&mut self) -> P2DIR6_W {
        P2DIR6_W { w: self }
    }
    #[doc = "Bit 7 - P2DIR7"]
    #[inline(always)]
    pub fn p2dir7(&mut self) -> P2DIR7_W {
        P2DIR7_W { w: self }
    }
}
