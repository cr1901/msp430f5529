#[doc = "Reader of register P2REN"]
pub type R = crate::R<u8, super::P2REN>;
#[doc = "Writer for register P2REN"]
pub type W = crate::W<u8, super::P2REN>;
#[doc = "Register P2REN `reset()`'s with value 0"]
impl crate::ResetValue for super::P2REN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P2REN0`"]
pub type P2REN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2REN0`"]
pub struct P2REN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P2REN0_W<'a> {
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
#[doc = "Reader of field `P2REN1`"]
pub type P2REN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2REN1`"]
pub struct P2REN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P2REN1_W<'a> {
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
#[doc = "Reader of field `P2REN2`"]
pub type P2REN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2REN2`"]
pub struct P2REN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P2REN2_W<'a> {
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
#[doc = "Reader of field `P2REN3`"]
pub type P2REN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2REN3`"]
pub struct P2REN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P2REN3_W<'a> {
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
#[doc = "Reader of field `P2REN4`"]
pub type P2REN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2REN4`"]
pub struct P2REN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P2REN4_W<'a> {
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
#[doc = "Reader of field `P2REN5`"]
pub type P2REN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2REN5`"]
pub struct P2REN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P2REN5_W<'a> {
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
#[doc = "Reader of field `P2REN6`"]
pub type P2REN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2REN6`"]
pub struct P2REN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P2REN6_W<'a> {
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
#[doc = "Reader of field `P2REN7`"]
pub type P2REN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2REN7`"]
pub struct P2REN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P2REN7_W<'a> {
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
    #[doc = "Bit 0 - P2REN0"]
    #[inline(always)]
    pub fn p2ren0(&self) -> P2REN0_R {
        P2REN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P2REN1"]
    #[inline(always)]
    pub fn p2ren1(&self) -> P2REN1_R {
        P2REN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P2REN2"]
    #[inline(always)]
    pub fn p2ren2(&self) -> P2REN2_R {
        P2REN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P2REN3"]
    #[inline(always)]
    pub fn p2ren3(&self) -> P2REN3_R {
        P2REN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P2REN4"]
    #[inline(always)]
    pub fn p2ren4(&self) -> P2REN4_R {
        P2REN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P2REN5"]
    #[inline(always)]
    pub fn p2ren5(&self) -> P2REN5_R {
        P2REN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P2REN6"]
    #[inline(always)]
    pub fn p2ren6(&self) -> P2REN6_R {
        P2REN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P2REN7"]
    #[inline(always)]
    pub fn p2ren7(&self) -> P2REN7_R {
        P2REN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2REN0"]
    #[inline(always)]
    pub fn p2ren0(&mut self) -> P2REN0_W {
        P2REN0_W { w: self }
    }
    #[doc = "Bit 1 - P2REN1"]
    #[inline(always)]
    pub fn p2ren1(&mut self) -> P2REN1_W {
        P2REN1_W { w: self }
    }
    #[doc = "Bit 2 - P2REN2"]
    #[inline(always)]
    pub fn p2ren2(&mut self) -> P2REN2_W {
        P2REN2_W { w: self }
    }
    #[doc = "Bit 3 - P2REN3"]
    #[inline(always)]
    pub fn p2ren3(&mut self) -> P2REN3_W {
        P2REN3_W { w: self }
    }
    #[doc = "Bit 4 - P2REN4"]
    #[inline(always)]
    pub fn p2ren4(&mut self) -> P2REN4_W {
        P2REN4_W { w: self }
    }
    #[doc = "Bit 5 - P2REN5"]
    #[inline(always)]
    pub fn p2ren5(&mut self) -> P2REN5_W {
        P2REN5_W { w: self }
    }
    #[doc = "Bit 6 - P2REN6"]
    #[inline(always)]
    pub fn p2ren6(&mut self) -> P2REN6_W {
        P2REN6_W { w: self }
    }
    #[doc = "Bit 7 - P2REN7"]
    #[inline(always)]
    pub fn p2ren7(&mut self) -> P2REN7_W {
        P2REN7_W { w: self }
    }
}
