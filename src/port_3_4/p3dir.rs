#[doc = "Reader of register P3DIR"]
pub type R = crate::R<u8, super::P3DIR>;
#[doc = "Writer for register P3DIR"]
pub type W = crate::W<u8, super::P3DIR>;
#[doc = "Register P3DIR `reset()`'s with value 0"]
impl crate::ResetValue for super::P3DIR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P3DIR0`"]
pub type P3DIR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3DIR0`"]
pub struct P3DIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> P3DIR0_W<'a> {
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
#[doc = "Reader of field `P3DIR1`"]
pub type P3DIR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3DIR1`"]
pub struct P3DIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> P3DIR1_W<'a> {
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
#[doc = "Reader of field `P3DIR2`"]
pub type P3DIR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3DIR2`"]
pub struct P3DIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> P3DIR2_W<'a> {
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
#[doc = "Reader of field `P3DIR3`"]
pub type P3DIR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3DIR3`"]
pub struct P3DIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> P3DIR3_W<'a> {
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
#[doc = "Reader of field `P3DIR4`"]
pub type P3DIR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3DIR4`"]
pub struct P3DIR4_W<'a> {
    w: &'a mut W,
}
impl<'a> P3DIR4_W<'a> {
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
#[doc = "Reader of field `P3DIR5`"]
pub type P3DIR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3DIR5`"]
pub struct P3DIR5_W<'a> {
    w: &'a mut W,
}
impl<'a> P3DIR5_W<'a> {
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
#[doc = "Reader of field `P3DIR6`"]
pub type P3DIR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3DIR6`"]
pub struct P3DIR6_W<'a> {
    w: &'a mut W,
}
impl<'a> P3DIR6_W<'a> {
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
#[doc = "Reader of field `P3DIR7`"]
pub type P3DIR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3DIR7`"]
pub struct P3DIR7_W<'a> {
    w: &'a mut W,
}
impl<'a> P3DIR7_W<'a> {
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
    #[doc = "Bit 0 - P3DIR0"]
    #[inline(always)]
    pub fn p3dir0(&self) -> P3DIR0_R {
        P3DIR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P3DIR1"]
    #[inline(always)]
    pub fn p3dir1(&self) -> P3DIR1_R {
        P3DIR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P3DIR2"]
    #[inline(always)]
    pub fn p3dir2(&self) -> P3DIR2_R {
        P3DIR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P3DIR3"]
    #[inline(always)]
    pub fn p3dir3(&self) -> P3DIR3_R {
        P3DIR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P3DIR4"]
    #[inline(always)]
    pub fn p3dir4(&self) -> P3DIR4_R {
        P3DIR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P3DIR5"]
    #[inline(always)]
    pub fn p3dir5(&self) -> P3DIR5_R {
        P3DIR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P3DIR6"]
    #[inline(always)]
    pub fn p3dir6(&self) -> P3DIR6_R {
        P3DIR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P3DIR7"]
    #[inline(always)]
    pub fn p3dir7(&self) -> P3DIR7_R {
        P3DIR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3DIR0"]
    #[inline(always)]
    pub fn p3dir0(&mut self) -> P3DIR0_W {
        P3DIR0_W { w: self }
    }
    #[doc = "Bit 1 - P3DIR1"]
    #[inline(always)]
    pub fn p3dir1(&mut self) -> P3DIR1_W {
        P3DIR1_W { w: self }
    }
    #[doc = "Bit 2 - P3DIR2"]
    #[inline(always)]
    pub fn p3dir2(&mut self) -> P3DIR2_W {
        P3DIR2_W { w: self }
    }
    #[doc = "Bit 3 - P3DIR3"]
    #[inline(always)]
    pub fn p3dir3(&mut self) -> P3DIR3_W {
        P3DIR3_W { w: self }
    }
    #[doc = "Bit 4 - P3DIR4"]
    #[inline(always)]
    pub fn p3dir4(&mut self) -> P3DIR4_W {
        P3DIR4_W { w: self }
    }
    #[doc = "Bit 5 - P3DIR5"]
    #[inline(always)]
    pub fn p3dir5(&mut self) -> P3DIR5_W {
        P3DIR5_W { w: self }
    }
    #[doc = "Bit 6 - P3DIR6"]
    #[inline(always)]
    pub fn p3dir6(&mut self) -> P3DIR6_W {
        P3DIR6_W { w: self }
    }
    #[doc = "Bit 7 - P3DIR7"]
    #[inline(always)]
    pub fn p3dir7(&mut self) -> P3DIR7_W {
        P3DIR7_W { w: self }
    }
}
