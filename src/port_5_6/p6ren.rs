#[doc = "Reader of register P6REN"]
pub type R = crate::R<u8, super::P6REN>;
#[doc = "Writer for register P6REN"]
pub type W = crate::W<u8, super::P6REN>;
#[doc = "Register P6REN `reset()`'s with value 0"]
impl crate::ResetValue for super::P6REN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P6REN0`"]
pub type P6REN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6REN0`"]
pub struct P6REN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P6REN0_W<'a> {
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
#[doc = "Reader of field `P6REN1`"]
pub type P6REN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6REN1`"]
pub struct P6REN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P6REN1_W<'a> {
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
#[doc = "Reader of field `P6REN2`"]
pub type P6REN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6REN2`"]
pub struct P6REN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P6REN2_W<'a> {
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
#[doc = "Reader of field `P6REN3`"]
pub type P6REN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6REN3`"]
pub struct P6REN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P6REN3_W<'a> {
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
#[doc = "Reader of field `P6REN4`"]
pub type P6REN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6REN4`"]
pub struct P6REN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P6REN4_W<'a> {
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
#[doc = "Reader of field `P6REN5`"]
pub type P6REN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6REN5`"]
pub struct P6REN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P6REN5_W<'a> {
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
#[doc = "Reader of field `P6REN6`"]
pub type P6REN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6REN6`"]
pub struct P6REN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P6REN6_W<'a> {
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
#[doc = "Reader of field `P6REN7`"]
pub type P6REN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6REN7`"]
pub struct P6REN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P6REN7_W<'a> {
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
    #[doc = "Bit 0 - P6REN0"]
    #[inline(always)]
    pub fn p6ren0(&self) -> P6REN0_R {
        P6REN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P6REN1"]
    #[inline(always)]
    pub fn p6ren1(&self) -> P6REN1_R {
        P6REN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P6REN2"]
    #[inline(always)]
    pub fn p6ren2(&self) -> P6REN2_R {
        P6REN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P6REN3"]
    #[inline(always)]
    pub fn p6ren3(&self) -> P6REN3_R {
        P6REN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P6REN4"]
    #[inline(always)]
    pub fn p6ren4(&self) -> P6REN4_R {
        P6REN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P6REN5"]
    #[inline(always)]
    pub fn p6ren5(&self) -> P6REN5_R {
        P6REN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P6REN6"]
    #[inline(always)]
    pub fn p6ren6(&self) -> P6REN6_R {
        P6REN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P6REN7"]
    #[inline(always)]
    pub fn p6ren7(&self) -> P6REN7_R {
        P6REN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P6REN0"]
    #[inline(always)]
    pub fn p6ren0(&mut self) -> P6REN0_W {
        P6REN0_W { w: self }
    }
    #[doc = "Bit 1 - P6REN1"]
    #[inline(always)]
    pub fn p6ren1(&mut self) -> P6REN1_W {
        P6REN1_W { w: self }
    }
    #[doc = "Bit 2 - P6REN2"]
    #[inline(always)]
    pub fn p6ren2(&mut self) -> P6REN2_W {
        P6REN2_W { w: self }
    }
    #[doc = "Bit 3 - P6REN3"]
    #[inline(always)]
    pub fn p6ren3(&mut self) -> P6REN3_W {
        P6REN3_W { w: self }
    }
    #[doc = "Bit 4 - P6REN4"]
    #[inline(always)]
    pub fn p6ren4(&mut self) -> P6REN4_W {
        P6REN4_W { w: self }
    }
    #[doc = "Bit 5 - P6REN5"]
    #[inline(always)]
    pub fn p6ren5(&mut self) -> P6REN5_W {
        P6REN5_W { w: self }
    }
    #[doc = "Bit 6 - P6REN6"]
    #[inline(always)]
    pub fn p6ren6(&mut self) -> P6REN6_W {
        P6REN6_W { w: self }
    }
    #[doc = "Bit 7 - P6REN7"]
    #[inline(always)]
    pub fn p6ren7(&mut self) -> P6REN7_W {
        P6REN7_W { w: self }
    }
}
