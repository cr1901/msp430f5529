#[doc = "Reader of register P8REN"]
pub type R = crate::R<u8, super::P8REN>;
#[doc = "Writer for register P8REN"]
pub type W = crate::W<u8, super::P8REN>;
#[doc = "Register P8REN `reset()`'s with value 0"]
impl crate::ResetValue for super::P8REN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P8REN0`"]
pub type P8REN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8REN0`"]
pub struct P8REN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P8REN0_W<'a> {
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
#[doc = "Reader of field `P8REN1`"]
pub type P8REN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8REN1`"]
pub struct P8REN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P8REN1_W<'a> {
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
#[doc = "Reader of field `P8REN2`"]
pub type P8REN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8REN2`"]
pub struct P8REN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P8REN2_W<'a> {
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
#[doc = "Reader of field `P8REN3`"]
pub type P8REN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8REN3`"]
pub struct P8REN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P8REN3_W<'a> {
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
#[doc = "Reader of field `P8REN4`"]
pub type P8REN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8REN4`"]
pub struct P8REN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P8REN4_W<'a> {
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
#[doc = "Reader of field `P8REN5`"]
pub type P8REN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8REN5`"]
pub struct P8REN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P8REN5_W<'a> {
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
#[doc = "Reader of field `P8REN6`"]
pub type P8REN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8REN6`"]
pub struct P8REN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P8REN6_W<'a> {
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
#[doc = "Reader of field `P8REN7`"]
pub type P8REN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8REN7`"]
pub struct P8REN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P8REN7_W<'a> {
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
    #[doc = "Bit 0 - P8REN0"]
    #[inline(always)]
    pub fn p8ren0(&self) -> P8REN0_R {
        P8REN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P8REN1"]
    #[inline(always)]
    pub fn p8ren1(&self) -> P8REN1_R {
        P8REN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P8REN2"]
    #[inline(always)]
    pub fn p8ren2(&self) -> P8REN2_R {
        P8REN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P8REN3"]
    #[inline(always)]
    pub fn p8ren3(&self) -> P8REN3_R {
        P8REN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P8REN4"]
    #[inline(always)]
    pub fn p8ren4(&self) -> P8REN4_R {
        P8REN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P8REN5"]
    #[inline(always)]
    pub fn p8ren5(&self) -> P8REN5_R {
        P8REN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P8REN6"]
    #[inline(always)]
    pub fn p8ren6(&self) -> P8REN6_R {
        P8REN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P8REN7"]
    #[inline(always)]
    pub fn p8ren7(&self) -> P8REN7_R {
        P8REN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P8REN0"]
    #[inline(always)]
    pub fn p8ren0(&mut self) -> P8REN0_W {
        P8REN0_W { w: self }
    }
    #[doc = "Bit 1 - P8REN1"]
    #[inline(always)]
    pub fn p8ren1(&mut self) -> P8REN1_W {
        P8REN1_W { w: self }
    }
    #[doc = "Bit 2 - P8REN2"]
    #[inline(always)]
    pub fn p8ren2(&mut self) -> P8REN2_W {
        P8REN2_W { w: self }
    }
    #[doc = "Bit 3 - P8REN3"]
    #[inline(always)]
    pub fn p8ren3(&mut self) -> P8REN3_W {
        P8REN3_W { w: self }
    }
    #[doc = "Bit 4 - P8REN4"]
    #[inline(always)]
    pub fn p8ren4(&mut self) -> P8REN4_W {
        P8REN4_W { w: self }
    }
    #[doc = "Bit 5 - P8REN5"]
    #[inline(always)]
    pub fn p8ren5(&mut self) -> P8REN5_W {
        P8REN5_W { w: self }
    }
    #[doc = "Bit 6 - P8REN6"]
    #[inline(always)]
    pub fn p8ren6(&mut self) -> P8REN6_W {
        P8REN6_W { w: self }
    }
    #[doc = "Bit 7 - P8REN7"]
    #[inline(always)]
    pub fn p8ren7(&mut self) -> P8REN7_W {
        P8REN7_W { w: self }
    }
}
