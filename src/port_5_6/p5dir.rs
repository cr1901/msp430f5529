#[doc = "Reader of register P5DIR"]
pub type R = crate::R<u8, super::P5DIR>;
#[doc = "Writer for register P5DIR"]
pub type W = crate::W<u8, super::P5DIR>;
#[doc = "Register P5DIR `reset()`'s with value 0"]
impl crate::ResetValue for super::P5DIR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P5DIR0`"]
pub type P5DIR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5DIR0`"]
pub struct P5DIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> P5DIR0_W<'a> {
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
#[doc = "Reader of field `P5DIR1`"]
pub type P5DIR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5DIR1`"]
pub struct P5DIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> P5DIR1_W<'a> {
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
#[doc = "Reader of field `P5DIR2`"]
pub type P5DIR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5DIR2`"]
pub struct P5DIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> P5DIR2_W<'a> {
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
#[doc = "Reader of field `P5DIR3`"]
pub type P5DIR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5DIR3`"]
pub struct P5DIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> P5DIR3_W<'a> {
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
#[doc = "Reader of field `P5DIR4`"]
pub type P5DIR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5DIR4`"]
pub struct P5DIR4_W<'a> {
    w: &'a mut W,
}
impl<'a> P5DIR4_W<'a> {
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
#[doc = "Reader of field `P5DIR5`"]
pub type P5DIR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5DIR5`"]
pub struct P5DIR5_W<'a> {
    w: &'a mut W,
}
impl<'a> P5DIR5_W<'a> {
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
#[doc = "Reader of field `P5DIR6`"]
pub type P5DIR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5DIR6`"]
pub struct P5DIR6_W<'a> {
    w: &'a mut W,
}
impl<'a> P5DIR6_W<'a> {
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
#[doc = "Reader of field `P5DIR7`"]
pub type P5DIR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5DIR7`"]
pub struct P5DIR7_W<'a> {
    w: &'a mut W,
}
impl<'a> P5DIR7_W<'a> {
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
    #[doc = "Bit 0 - P5DIR0"]
    #[inline(always)]
    pub fn p5dir0(&self) -> P5DIR0_R {
        P5DIR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P5DIR1"]
    #[inline(always)]
    pub fn p5dir1(&self) -> P5DIR1_R {
        P5DIR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P5DIR2"]
    #[inline(always)]
    pub fn p5dir2(&self) -> P5DIR2_R {
        P5DIR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P5DIR3"]
    #[inline(always)]
    pub fn p5dir3(&self) -> P5DIR3_R {
        P5DIR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P5DIR4"]
    #[inline(always)]
    pub fn p5dir4(&self) -> P5DIR4_R {
        P5DIR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P5DIR5"]
    #[inline(always)]
    pub fn p5dir5(&self) -> P5DIR5_R {
        P5DIR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P5DIR6"]
    #[inline(always)]
    pub fn p5dir6(&self) -> P5DIR6_R {
        P5DIR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P5DIR7"]
    #[inline(always)]
    pub fn p5dir7(&self) -> P5DIR7_R {
        P5DIR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P5DIR0"]
    #[inline(always)]
    pub fn p5dir0(&mut self) -> P5DIR0_W {
        P5DIR0_W { w: self }
    }
    #[doc = "Bit 1 - P5DIR1"]
    #[inline(always)]
    pub fn p5dir1(&mut self) -> P5DIR1_W {
        P5DIR1_W { w: self }
    }
    #[doc = "Bit 2 - P5DIR2"]
    #[inline(always)]
    pub fn p5dir2(&mut self) -> P5DIR2_W {
        P5DIR2_W { w: self }
    }
    #[doc = "Bit 3 - P5DIR3"]
    #[inline(always)]
    pub fn p5dir3(&mut self) -> P5DIR3_W {
        P5DIR3_W { w: self }
    }
    #[doc = "Bit 4 - P5DIR4"]
    #[inline(always)]
    pub fn p5dir4(&mut self) -> P5DIR4_W {
        P5DIR4_W { w: self }
    }
    #[doc = "Bit 5 - P5DIR5"]
    #[inline(always)]
    pub fn p5dir5(&mut self) -> P5DIR5_W {
        P5DIR5_W { w: self }
    }
    #[doc = "Bit 6 - P5DIR6"]
    #[inline(always)]
    pub fn p5dir6(&mut self) -> P5DIR6_W {
        P5DIR6_W { w: self }
    }
    #[doc = "Bit 7 - P5DIR7"]
    #[inline(always)]
    pub fn p5dir7(&mut self) -> P5DIR7_W {
        P5DIR7_W { w: self }
    }
}
