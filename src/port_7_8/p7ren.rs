#[doc = "Reader of register P7REN"]
pub type R = crate::R<u8, super::P7REN>;
#[doc = "Writer for register P7REN"]
pub type W = crate::W<u8, super::P7REN>;
#[doc = "Register P7REN `reset()`'s with value 0"]
impl crate::ResetValue for super::P7REN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P7REN0`"]
pub type P7REN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7REN0`"]
pub struct P7REN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P7REN0_W<'a> {
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
#[doc = "Reader of field `P7REN1`"]
pub type P7REN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7REN1`"]
pub struct P7REN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P7REN1_W<'a> {
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
#[doc = "Reader of field `P7REN2`"]
pub type P7REN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7REN2`"]
pub struct P7REN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P7REN2_W<'a> {
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
#[doc = "Reader of field `P7REN3`"]
pub type P7REN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7REN3`"]
pub struct P7REN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P7REN3_W<'a> {
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
#[doc = "Reader of field `P7REN4`"]
pub type P7REN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7REN4`"]
pub struct P7REN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P7REN4_W<'a> {
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
#[doc = "Reader of field `P7REN5`"]
pub type P7REN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7REN5`"]
pub struct P7REN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P7REN5_W<'a> {
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
#[doc = "Reader of field `P7REN6`"]
pub type P7REN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7REN6`"]
pub struct P7REN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P7REN6_W<'a> {
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
#[doc = "Reader of field `P7REN7`"]
pub type P7REN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7REN7`"]
pub struct P7REN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P7REN7_W<'a> {
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
    #[doc = "Bit 0 - P7REN0"]
    #[inline(always)]
    pub fn p7ren0(&self) -> P7REN0_R {
        P7REN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P7REN1"]
    #[inline(always)]
    pub fn p7ren1(&self) -> P7REN1_R {
        P7REN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P7REN2"]
    #[inline(always)]
    pub fn p7ren2(&self) -> P7REN2_R {
        P7REN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P7REN3"]
    #[inline(always)]
    pub fn p7ren3(&self) -> P7REN3_R {
        P7REN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P7REN4"]
    #[inline(always)]
    pub fn p7ren4(&self) -> P7REN4_R {
        P7REN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P7REN5"]
    #[inline(always)]
    pub fn p7ren5(&self) -> P7REN5_R {
        P7REN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P7REN6"]
    #[inline(always)]
    pub fn p7ren6(&self) -> P7REN6_R {
        P7REN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P7REN7"]
    #[inline(always)]
    pub fn p7ren7(&self) -> P7REN7_R {
        P7REN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P7REN0"]
    #[inline(always)]
    pub fn p7ren0(&mut self) -> P7REN0_W {
        P7REN0_W { w: self }
    }
    #[doc = "Bit 1 - P7REN1"]
    #[inline(always)]
    pub fn p7ren1(&mut self) -> P7REN1_W {
        P7REN1_W { w: self }
    }
    #[doc = "Bit 2 - P7REN2"]
    #[inline(always)]
    pub fn p7ren2(&mut self) -> P7REN2_W {
        P7REN2_W { w: self }
    }
    #[doc = "Bit 3 - P7REN3"]
    #[inline(always)]
    pub fn p7ren3(&mut self) -> P7REN3_W {
        P7REN3_W { w: self }
    }
    #[doc = "Bit 4 - P7REN4"]
    #[inline(always)]
    pub fn p7ren4(&mut self) -> P7REN4_W {
        P7REN4_W { w: self }
    }
    #[doc = "Bit 5 - P7REN5"]
    #[inline(always)]
    pub fn p7ren5(&mut self) -> P7REN5_W {
        P7REN5_W { w: self }
    }
    #[doc = "Bit 6 - P7REN6"]
    #[inline(always)]
    pub fn p7ren6(&mut self) -> P7REN6_W {
        P7REN6_W { w: self }
    }
    #[doc = "Bit 7 - P7REN7"]
    #[inline(always)]
    pub fn p7ren7(&mut self) -> P7REN7_W {
        P7REN7_W { w: self }
    }
}
