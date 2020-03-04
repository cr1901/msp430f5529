#[doc = "Reader of register P5REN"]
pub type R = crate::R<u8, super::P5REN>;
#[doc = "Writer for register P5REN"]
pub type W = crate::W<u8, super::P5REN>;
#[doc = "Register P5REN `reset()`'s with value 0"]
impl crate::ResetValue for super::P5REN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P5REN0`"]
pub type P5REN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5REN0`"]
pub struct P5REN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P5REN0_W<'a> {
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
#[doc = "Reader of field `P5REN1`"]
pub type P5REN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5REN1`"]
pub struct P5REN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P5REN1_W<'a> {
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
#[doc = "Reader of field `P5REN2`"]
pub type P5REN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5REN2`"]
pub struct P5REN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P5REN2_W<'a> {
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
#[doc = "Reader of field `P5REN3`"]
pub type P5REN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5REN3`"]
pub struct P5REN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P5REN3_W<'a> {
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
#[doc = "Reader of field `P5REN4`"]
pub type P5REN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5REN4`"]
pub struct P5REN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P5REN4_W<'a> {
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
#[doc = "Reader of field `P5REN5`"]
pub type P5REN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5REN5`"]
pub struct P5REN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P5REN5_W<'a> {
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
#[doc = "Reader of field `P5REN6`"]
pub type P5REN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5REN6`"]
pub struct P5REN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P5REN6_W<'a> {
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
#[doc = "Reader of field `P5REN7`"]
pub type P5REN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5REN7`"]
pub struct P5REN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P5REN7_W<'a> {
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
    #[doc = "Bit 0 - P5REN0"]
    #[inline(always)]
    pub fn p5ren0(&self) -> P5REN0_R {
        P5REN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P5REN1"]
    #[inline(always)]
    pub fn p5ren1(&self) -> P5REN1_R {
        P5REN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P5REN2"]
    #[inline(always)]
    pub fn p5ren2(&self) -> P5REN2_R {
        P5REN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P5REN3"]
    #[inline(always)]
    pub fn p5ren3(&self) -> P5REN3_R {
        P5REN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P5REN4"]
    #[inline(always)]
    pub fn p5ren4(&self) -> P5REN4_R {
        P5REN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P5REN5"]
    #[inline(always)]
    pub fn p5ren5(&self) -> P5REN5_R {
        P5REN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P5REN6"]
    #[inline(always)]
    pub fn p5ren6(&self) -> P5REN6_R {
        P5REN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P5REN7"]
    #[inline(always)]
    pub fn p5ren7(&self) -> P5REN7_R {
        P5REN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P5REN0"]
    #[inline(always)]
    pub fn p5ren0(&mut self) -> P5REN0_W {
        P5REN0_W { w: self }
    }
    #[doc = "Bit 1 - P5REN1"]
    #[inline(always)]
    pub fn p5ren1(&mut self) -> P5REN1_W {
        P5REN1_W { w: self }
    }
    #[doc = "Bit 2 - P5REN2"]
    #[inline(always)]
    pub fn p5ren2(&mut self) -> P5REN2_W {
        P5REN2_W { w: self }
    }
    #[doc = "Bit 3 - P5REN3"]
    #[inline(always)]
    pub fn p5ren3(&mut self) -> P5REN3_W {
        P5REN3_W { w: self }
    }
    #[doc = "Bit 4 - P5REN4"]
    #[inline(always)]
    pub fn p5ren4(&mut self) -> P5REN4_W {
        P5REN4_W { w: self }
    }
    #[doc = "Bit 5 - P5REN5"]
    #[inline(always)]
    pub fn p5ren5(&mut self) -> P5REN5_W {
        P5REN5_W { w: self }
    }
    #[doc = "Bit 6 - P5REN6"]
    #[inline(always)]
    pub fn p5ren6(&mut self) -> P5REN6_W {
        P5REN6_W { w: self }
    }
    #[doc = "Bit 7 - P5REN7"]
    #[inline(always)]
    pub fn p5ren7(&mut self) -> P5REN7_W {
        P5REN7_W { w: self }
    }
}
