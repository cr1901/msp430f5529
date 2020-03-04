#[doc = "Reader of register P3REN"]
pub type R = crate::R<u8, super::P3REN>;
#[doc = "Writer for register P3REN"]
pub type W = crate::W<u8, super::P3REN>;
#[doc = "Register P3REN `reset()`'s with value 0"]
impl crate::ResetValue for super::P3REN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P3REN0`"]
pub type P3REN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3REN0`"]
pub struct P3REN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P3REN0_W<'a> {
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
#[doc = "Reader of field `P3REN1`"]
pub type P3REN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3REN1`"]
pub struct P3REN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P3REN1_W<'a> {
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
#[doc = "Reader of field `P3REN2`"]
pub type P3REN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3REN2`"]
pub struct P3REN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P3REN2_W<'a> {
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
#[doc = "Reader of field `P3REN3`"]
pub type P3REN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3REN3`"]
pub struct P3REN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P3REN3_W<'a> {
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
#[doc = "Reader of field `P3REN4`"]
pub type P3REN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3REN4`"]
pub struct P3REN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P3REN4_W<'a> {
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
#[doc = "Reader of field `P3REN5`"]
pub type P3REN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3REN5`"]
pub struct P3REN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P3REN5_W<'a> {
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
#[doc = "Reader of field `P3REN6`"]
pub type P3REN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3REN6`"]
pub struct P3REN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P3REN6_W<'a> {
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
#[doc = "Reader of field `P3REN7`"]
pub type P3REN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3REN7`"]
pub struct P3REN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P3REN7_W<'a> {
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
    #[doc = "Bit 0 - P3REN0"]
    #[inline(always)]
    pub fn p3ren0(&self) -> P3REN0_R {
        P3REN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P3REN1"]
    #[inline(always)]
    pub fn p3ren1(&self) -> P3REN1_R {
        P3REN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P3REN2"]
    #[inline(always)]
    pub fn p3ren2(&self) -> P3REN2_R {
        P3REN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P3REN3"]
    #[inline(always)]
    pub fn p3ren3(&self) -> P3REN3_R {
        P3REN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P3REN4"]
    #[inline(always)]
    pub fn p3ren4(&self) -> P3REN4_R {
        P3REN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P3REN5"]
    #[inline(always)]
    pub fn p3ren5(&self) -> P3REN5_R {
        P3REN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P3REN6"]
    #[inline(always)]
    pub fn p3ren6(&self) -> P3REN6_R {
        P3REN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P3REN7"]
    #[inline(always)]
    pub fn p3ren7(&self) -> P3REN7_R {
        P3REN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3REN0"]
    #[inline(always)]
    pub fn p3ren0(&mut self) -> P3REN0_W {
        P3REN0_W { w: self }
    }
    #[doc = "Bit 1 - P3REN1"]
    #[inline(always)]
    pub fn p3ren1(&mut self) -> P3REN1_W {
        P3REN1_W { w: self }
    }
    #[doc = "Bit 2 - P3REN2"]
    #[inline(always)]
    pub fn p3ren2(&mut self) -> P3REN2_W {
        P3REN2_W { w: self }
    }
    #[doc = "Bit 3 - P3REN3"]
    #[inline(always)]
    pub fn p3ren3(&mut self) -> P3REN3_W {
        P3REN3_W { w: self }
    }
    #[doc = "Bit 4 - P3REN4"]
    #[inline(always)]
    pub fn p3ren4(&mut self) -> P3REN4_W {
        P3REN4_W { w: self }
    }
    #[doc = "Bit 5 - P3REN5"]
    #[inline(always)]
    pub fn p3ren5(&mut self) -> P3REN5_W {
        P3REN5_W { w: self }
    }
    #[doc = "Bit 6 - P3REN6"]
    #[inline(always)]
    pub fn p3ren6(&mut self) -> P3REN6_W {
        P3REN6_W { w: self }
    }
    #[doc = "Bit 7 - P3REN7"]
    #[inline(always)]
    pub fn p3ren7(&mut self) -> P3REN7_W {
        P3REN7_W { w: self }
    }
}
