#[doc = "Reader of register P7DIR"]
pub type R = crate::R<u8, super::P7DIR>;
#[doc = "Writer for register P7DIR"]
pub type W = crate::W<u8, super::P7DIR>;
#[doc = "Register P7DIR `reset()`'s with value 0"]
impl crate::ResetValue for super::P7DIR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P7DIR0`"]
pub type P7DIR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7DIR0`"]
pub struct P7DIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DIR0_W<'a> {
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
#[doc = "Reader of field `P7DIR1`"]
pub type P7DIR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7DIR1`"]
pub struct P7DIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DIR1_W<'a> {
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
#[doc = "Reader of field `P7DIR2`"]
pub type P7DIR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7DIR2`"]
pub struct P7DIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DIR2_W<'a> {
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
#[doc = "Reader of field `P7DIR3`"]
pub type P7DIR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7DIR3`"]
pub struct P7DIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DIR3_W<'a> {
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
#[doc = "Reader of field `P7DIR4`"]
pub type P7DIR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7DIR4`"]
pub struct P7DIR4_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DIR4_W<'a> {
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
#[doc = "Reader of field `P7DIR5`"]
pub type P7DIR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7DIR5`"]
pub struct P7DIR5_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DIR5_W<'a> {
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
#[doc = "Reader of field `P7DIR6`"]
pub type P7DIR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7DIR6`"]
pub struct P7DIR6_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DIR6_W<'a> {
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
#[doc = "Reader of field `P7DIR7`"]
pub type P7DIR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7DIR7`"]
pub struct P7DIR7_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DIR7_W<'a> {
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
    #[doc = "Bit 0 - P7DIR0"]
    #[inline(always)]
    pub fn p7dir0(&self) -> P7DIR0_R {
        P7DIR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P7DIR1"]
    #[inline(always)]
    pub fn p7dir1(&self) -> P7DIR1_R {
        P7DIR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P7DIR2"]
    #[inline(always)]
    pub fn p7dir2(&self) -> P7DIR2_R {
        P7DIR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P7DIR3"]
    #[inline(always)]
    pub fn p7dir3(&self) -> P7DIR3_R {
        P7DIR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P7DIR4"]
    #[inline(always)]
    pub fn p7dir4(&self) -> P7DIR4_R {
        P7DIR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P7DIR5"]
    #[inline(always)]
    pub fn p7dir5(&self) -> P7DIR5_R {
        P7DIR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P7DIR6"]
    #[inline(always)]
    pub fn p7dir6(&self) -> P7DIR6_R {
        P7DIR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P7DIR7"]
    #[inline(always)]
    pub fn p7dir7(&self) -> P7DIR7_R {
        P7DIR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P7DIR0"]
    #[inline(always)]
    pub fn p7dir0(&mut self) -> P7DIR0_W {
        P7DIR0_W { w: self }
    }
    #[doc = "Bit 1 - P7DIR1"]
    #[inline(always)]
    pub fn p7dir1(&mut self) -> P7DIR1_W {
        P7DIR1_W { w: self }
    }
    #[doc = "Bit 2 - P7DIR2"]
    #[inline(always)]
    pub fn p7dir2(&mut self) -> P7DIR2_W {
        P7DIR2_W { w: self }
    }
    #[doc = "Bit 3 - P7DIR3"]
    #[inline(always)]
    pub fn p7dir3(&mut self) -> P7DIR3_W {
        P7DIR3_W { w: self }
    }
    #[doc = "Bit 4 - P7DIR4"]
    #[inline(always)]
    pub fn p7dir4(&mut self) -> P7DIR4_W {
        P7DIR4_W { w: self }
    }
    #[doc = "Bit 5 - P7DIR5"]
    #[inline(always)]
    pub fn p7dir5(&mut self) -> P7DIR5_W {
        P7DIR5_W { w: self }
    }
    #[doc = "Bit 6 - P7DIR6"]
    #[inline(always)]
    pub fn p7dir6(&mut self) -> P7DIR6_W {
        P7DIR6_W { w: self }
    }
    #[doc = "Bit 7 - P7DIR7"]
    #[inline(always)]
    pub fn p7dir7(&mut self) -> P7DIR7_W {
        P7DIR7_W { w: self }
    }
}
