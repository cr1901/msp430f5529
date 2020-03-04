#[doc = "Reader of register P1REN"]
pub type R = crate::R<u8, super::P1REN>;
#[doc = "Writer for register P1REN"]
pub type W = crate::W<u8, super::P1REN>;
#[doc = "Register P1REN `reset()`'s with value 0"]
impl crate::ResetValue for super::P1REN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P1REN0`"]
pub type P1REN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1REN0`"]
pub struct P1REN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P1REN0_W<'a> {
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
#[doc = "Reader of field `P1REN1`"]
pub type P1REN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1REN1`"]
pub struct P1REN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1REN1_W<'a> {
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
#[doc = "Reader of field `P1REN2`"]
pub type P1REN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1REN2`"]
pub struct P1REN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P1REN2_W<'a> {
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
#[doc = "Reader of field `P1REN3`"]
pub type P1REN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1REN3`"]
pub struct P1REN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P1REN3_W<'a> {
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
#[doc = "Reader of field `P1REN4`"]
pub type P1REN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1REN4`"]
pub struct P1REN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P1REN4_W<'a> {
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
#[doc = "Reader of field `P1REN5`"]
pub type P1REN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1REN5`"]
pub struct P1REN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P1REN5_W<'a> {
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
#[doc = "Reader of field `P1REN6`"]
pub type P1REN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1REN6`"]
pub struct P1REN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P1REN6_W<'a> {
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
#[doc = "Reader of field `P1REN7`"]
pub type P1REN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1REN7`"]
pub struct P1REN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P1REN7_W<'a> {
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
    #[doc = "Bit 0 - P1REN0"]
    #[inline(always)]
    pub fn p1ren0(&self) -> P1REN0_R {
        P1REN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P1REN1"]
    #[inline(always)]
    pub fn p1ren1(&self) -> P1REN1_R {
        P1REN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P1REN2"]
    #[inline(always)]
    pub fn p1ren2(&self) -> P1REN2_R {
        P1REN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P1REN3"]
    #[inline(always)]
    pub fn p1ren3(&self) -> P1REN3_R {
        P1REN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P1REN4"]
    #[inline(always)]
    pub fn p1ren4(&self) -> P1REN4_R {
        P1REN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P1REN5"]
    #[inline(always)]
    pub fn p1ren5(&self) -> P1REN5_R {
        P1REN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P1REN6"]
    #[inline(always)]
    pub fn p1ren6(&self) -> P1REN6_R {
        P1REN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P1REN7"]
    #[inline(always)]
    pub fn p1ren7(&self) -> P1REN7_R {
        P1REN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1REN0"]
    #[inline(always)]
    pub fn p1ren0(&mut self) -> P1REN0_W {
        P1REN0_W { w: self }
    }
    #[doc = "Bit 1 - P1REN1"]
    #[inline(always)]
    pub fn p1ren1(&mut self) -> P1REN1_W {
        P1REN1_W { w: self }
    }
    #[doc = "Bit 2 - P1REN2"]
    #[inline(always)]
    pub fn p1ren2(&mut self) -> P1REN2_W {
        P1REN2_W { w: self }
    }
    #[doc = "Bit 3 - P1REN3"]
    #[inline(always)]
    pub fn p1ren3(&mut self) -> P1REN3_W {
        P1REN3_W { w: self }
    }
    #[doc = "Bit 4 - P1REN4"]
    #[inline(always)]
    pub fn p1ren4(&mut self) -> P1REN4_W {
        P1REN4_W { w: self }
    }
    #[doc = "Bit 5 - P1REN5"]
    #[inline(always)]
    pub fn p1ren5(&mut self) -> P1REN5_W {
        P1REN5_W { w: self }
    }
    #[doc = "Bit 6 - P1REN6"]
    #[inline(always)]
    pub fn p1ren6(&mut self) -> P1REN6_W {
        P1REN6_W { w: self }
    }
    #[doc = "Bit 7 - P1REN7"]
    #[inline(always)]
    pub fn p1ren7(&mut self) -> P1REN7_W {
        P1REN7_W { w: self }
    }
}
