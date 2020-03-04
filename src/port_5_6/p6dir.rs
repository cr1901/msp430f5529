#[doc = "Reader of register P6DIR"]
pub type R = crate::R<u8, super::P6DIR>;
#[doc = "Writer for register P6DIR"]
pub type W = crate::W<u8, super::P6DIR>;
#[doc = "Register P6DIR `reset()`'s with value 0"]
impl crate::ResetValue for super::P6DIR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P6DIR0`"]
pub type P6DIR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6DIR0`"]
pub struct P6DIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DIR0_W<'a> {
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
#[doc = "Reader of field `P6DIR1`"]
pub type P6DIR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6DIR1`"]
pub struct P6DIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DIR1_W<'a> {
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
#[doc = "Reader of field `P6DIR2`"]
pub type P6DIR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6DIR2`"]
pub struct P6DIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DIR2_W<'a> {
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
#[doc = "Reader of field `P6DIR3`"]
pub type P6DIR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6DIR3`"]
pub struct P6DIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DIR3_W<'a> {
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
#[doc = "Reader of field `P6DIR4`"]
pub type P6DIR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6DIR4`"]
pub struct P6DIR4_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DIR4_W<'a> {
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
#[doc = "Reader of field `P6DIR5`"]
pub type P6DIR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6DIR5`"]
pub struct P6DIR5_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DIR5_W<'a> {
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
#[doc = "Reader of field `P6DIR6`"]
pub type P6DIR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6DIR6`"]
pub struct P6DIR6_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DIR6_W<'a> {
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
#[doc = "Reader of field `P6DIR7`"]
pub type P6DIR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6DIR7`"]
pub struct P6DIR7_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DIR7_W<'a> {
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
    #[doc = "Bit 0 - P6DIR0"]
    #[inline(always)]
    pub fn p6dir0(&self) -> P6DIR0_R {
        P6DIR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P6DIR1"]
    #[inline(always)]
    pub fn p6dir1(&self) -> P6DIR1_R {
        P6DIR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P6DIR2"]
    #[inline(always)]
    pub fn p6dir2(&self) -> P6DIR2_R {
        P6DIR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P6DIR3"]
    #[inline(always)]
    pub fn p6dir3(&self) -> P6DIR3_R {
        P6DIR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P6DIR4"]
    #[inline(always)]
    pub fn p6dir4(&self) -> P6DIR4_R {
        P6DIR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P6DIR5"]
    #[inline(always)]
    pub fn p6dir5(&self) -> P6DIR5_R {
        P6DIR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P6DIR6"]
    #[inline(always)]
    pub fn p6dir6(&self) -> P6DIR6_R {
        P6DIR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P6DIR7"]
    #[inline(always)]
    pub fn p6dir7(&self) -> P6DIR7_R {
        P6DIR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P6DIR0"]
    #[inline(always)]
    pub fn p6dir0(&mut self) -> P6DIR0_W {
        P6DIR0_W { w: self }
    }
    #[doc = "Bit 1 - P6DIR1"]
    #[inline(always)]
    pub fn p6dir1(&mut self) -> P6DIR1_W {
        P6DIR1_W { w: self }
    }
    #[doc = "Bit 2 - P6DIR2"]
    #[inline(always)]
    pub fn p6dir2(&mut self) -> P6DIR2_W {
        P6DIR2_W { w: self }
    }
    #[doc = "Bit 3 - P6DIR3"]
    #[inline(always)]
    pub fn p6dir3(&mut self) -> P6DIR3_W {
        P6DIR3_W { w: self }
    }
    #[doc = "Bit 4 - P6DIR4"]
    #[inline(always)]
    pub fn p6dir4(&mut self) -> P6DIR4_W {
        P6DIR4_W { w: self }
    }
    #[doc = "Bit 5 - P6DIR5"]
    #[inline(always)]
    pub fn p6dir5(&mut self) -> P6DIR5_W {
        P6DIR5_W { w: self }
    }
    #[doc = "Bit 6 - P6DIR6"]
    #[inline(always)]
    pub fn p6dir6(&mut self) -> P6DIR6_W {
        P6DIR6_W { w: self }
    }
    #[doc = "Bit 7 - P6DIR7"]
    #[inline(always)]
    pub fn p6dir7(&mut self) -> P6DIR7_W {
        P6DIR7_W { w: self }
    }
}
