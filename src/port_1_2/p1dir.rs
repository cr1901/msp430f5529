#[doc = "Reader of register P1DIR"]
pub type R = crate::R<u8, super::P1DIR>;
#[doc = "Writer for register P1DIR"]
pub type W = crate::W<u8, super::P1DIR>;
#[doc = "Register P1DIR `reset()`'s with value 0"]
impl crate::ResetValue for super::P1DIR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P1DIR0`"]
pub type P1DIR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1DIR0`"]
pub struct P1DIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DIR0_W<'a> {
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
#[doc = "Reader of field `P1DIR1`"]
pub type P1DIR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1DIR1`"]
pub struct P1DIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DIR1_W<'a> {
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
#[doc = "Reader of field `P1DIR2`"]
pub type P1DIR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1DIR2`"]
pub struct P1DIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DIR2_W<'a> {
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
#[doc = "Reader of field `P1DIR3`"]
pub type P1DIR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1DIR3`"]
pub struct P1DIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DIR3_W<'a> {
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
#[doc = "Reader of field `P1DIR4`"]
pub type P1DIR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1DIR4`"]
pub struct P1DIR4_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DIR4_W<'a> {
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
#[doc = "Reader of field `P1DIR5`"]
pub type P1DIR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1DIR5`"]
pub struct P1DIR5_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DIR5_W<'a> {
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
#[doc = "Reader of field `P1DIR6`"]
pub type P1DIR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1DIR6`"]
pub struct P1DIR6_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DIR6_W<'a> {
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
#[doc = "Reader of field `P1DIR7`"]
pub type P1DIR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1DIR7`"]
pub struct P1DIR7_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DIR7_W<'a> {
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
    #[doc = "Bit 0 - P1DIR0"]
    #[inline(always)]
    pub fn p1dir0(&self) -> P1DIR0_R {
        P1DIR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P1DIR1"]
    #[inline(always)]
    pub fn p1dir1(&self) -> P1DIR1_R {
        P1DIR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P1DIR2"]
    #[inline(always)]
    pub fn p1dir2(&self) -> P1DIR2_R {
        P1DIR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P1DIR3"]
    #[inline(always)]
    pub fn p1dir3(&self) -> P1DIR3_R {
        P1DIR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P1DIR4"]
    #[inline(always)]
    pub fn p1dir4(&self) -> P1DIR4_R {
        P1DIR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P1DIR5"]
    #[inline(always)]
    pub fn p1dir5(&self) -> P1DIR5_R {
        P1DIR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P1DIR6"]
    #[inline(always)]
    pub fn p1dir6(&self) -> P1DIR6_R {
        P1DIR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P1DIR7"]
    #[inline(always)]
    pub fn p1dir7(&self) -> P1DIR7_R {
        P1DIR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1DIR0"]
    #[inline(always)]
    pub fn p1dir0(&mut self) -> P1DIR0_W {
        P1DIR0_W { w: self }
    }
    #[doc = "Bit 1 - P1DIR1"]
    #[inline(always)]
    pub fn p1dir1(&mut self) -> P1DIR1_W {
        P1DIR1_W { w: self }
    }
    #[doc = "Bit 2 - P1DIR2"]
    #[inline(always)]
    pub fn p1dir2(&mut self) -> P1DIR2_W {
        P1DIR2_W { w: self }
    }
    #[doc = "Bit 3 - P1DIR3"]
    #[inline(always)]
    pub fn p1dir3(&mut self) -> P1DIR3_W {
        P1DIR3_W { w: self }
    }
    #[doc = "Bit 4 - P1DIR4"]
    #[inline(always)]
    pub fn p1dir4(&mut self) -> P1DIR4_W {
        P1DIR4_W { w: self }
    }
    #[doc = "Bit 5 - P1DIR5"]
    #[inline(always)]
    pub fn p1dir5(&mut self) -> P1DIR5_W {
        P1DIR5_W { w: self }
    }
    #[doc = "Bit 6 - P1DIR6"]
    #[inline(always)]
    pub fn p1dir6(&mut self) -> P1DIR6_W {
        P1DIR6_W { w: self }
    }
    #[doc = "Bit 7 - P1DIR7"]
    #[inline(always)]
    pub fn p1dir7(&mut self) -> P1DIR7_W {
        P1DIR7_W { w: self }
    }
}
