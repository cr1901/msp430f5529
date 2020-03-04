#[doc = "Reader of register P8DIR"]
pub type R = crate::R<u8, super::P8DIR>;
#[doc = "Writer for register P8DIR"]
pub type W = crate::W<u8, super::P8DIR>;
#[doc = "Register P8DIR `reset()`'s with value 0"]
impl crate::ResetValue for super::P8DIR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P8DIR0`"]
pub type P8DIR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8DIR0`"]
pub struct P8DIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DIR0_W<'a> {
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
#[doc = "Reader of field `P8DIR1`"]
pub type P8DIR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8DIR1`"]
pub struct P8DIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DIR1_W<'a> {
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
#[doc = "Reader of field `P8DIR2`"]
pub type P8DIR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8DIR2`"]
pub struct P8DIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DIR2_W<'a> {
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
#[doc = "Reader of field `P8DIR3`"]
pub type P8DIR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8DIR3`"]
pub struct P8DIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DIR3_W<'a> {
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
#[doc = "Reader of field `P8DIR4`"]
pub type P8DIR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8DIR4`"]
pub struct P8DIR4_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DIR4_W<'a> {
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
#[doc = "Reader of field `P8DIR5`"]
pub type P8DIR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8DIR5`"]
pub struct P8DIR5_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DIR5_W<'a> {
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
#[doc = "Reader of field `P8DIR6`"]
pub type P8DIR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8DIR6`"]
pub struct P8DIR6_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DIR6_W<'a> {
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
#[doc = "Reader of field `P8DIR7`"]
pub type P8DIR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8DIR7`"]
pub struct P8DIR7_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DIR7_W<'a> {
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
    #[doc = "Bit 0 - P8DIR0"]
    #[inline(always)]
    pub fn p8dir0(&self) -> P8DIR0_R {
        P8DIR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P8DIR1"]
    #[inline(always)]
    pub fn p8dir1(&self) -> P8DIR1_R {
        P8DIR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P8DIR2"]
    #[inline(always)]
    pub fn p8dir2(&self) -> P8DIR2_R {
        P8DIR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P8DIR3"]
    #[inline(always)]
    pub fn p8dir3(&self) -> P8DIR3_R {
        P8DIR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P8DIR4"]
    #[inline(always)]
    pub fn p8dir4(&self) -> P8DIR4_R {
        P8DIR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P8DIR5"]
    #[inline(always)]
    pub fn p8dir5(&self) -> P8DIR5_R {
        P8DIR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P8DIR6"]
    #[inline(always)]
    pub fn p8dir6(&self) -> P8DIR6_R {
        P8DIR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P8DIR7"]
    #[inline(always)]
    pub fn p8dir7(&self) -> P8DIR7_R {
        P8DIR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P8DIR0"]
    #[inline(always)]
    pub fn p8dir0(&mut self) -> P8DIR0_W {
        P8DIR0_W { w: self }
    }
    #[doc = "Bit 1 - P8DIR1"]
    #[inline(always)]
    pub fn p8dir1(&mut self) -> P8DIR1_W {
        P8DIR1_W { w: self }
    }
    #[doc = "Bit 2 - P8DIR2"]
    #[inline(always)]
    pub fn p8dir2(&mut self) -> P8DIR2_W {
        P8DIR2_W { w: self }
    }
    #[doc = "Bit 3 - P8DIR3"]
    #[inline(always)]
    pub fn p8dir3(&mut self) -> P8DIR3_W {
        P8DIR3_W { w: self }
    }
    #[doc = "Bit 4 - P8DIR4"]
    #[inline(always)]
    pub fn p8dir4(&mut self) -> P8DIR4_W {
        P8DIR4_W { w: self }
    }
    #[doc = "Bit 5 - P8DIR5"]
    #[inline(always)]
    pub fn p8dir5(&mut self) -> P8DIR5_W {
        P8DIR5_W { w: self }
    }
    #[doc = "Bit 6 - P8DIR6"]
    #[inline(always)]
    pub fn p8dir6(&mut self) -> P8DIR6_W {
        P8DIR6_W { w: self }
    }
    #[doc = "Bit 7 - P8DIR7"]
    #[inline(always)]
    pub fn p8dir7(&mut self) -> P8DIR7_W {
        P8DIR7_W { w: self }
    }
}
