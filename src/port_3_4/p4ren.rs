#[doc = "Reader of register P4REN"]
pub type R = crate::R<u8, super::P4REN>;
#[doc = "Writer for register P4REN"]
pub type W = crate::W<u8, super::P4REN>;
#[doc = "Register P4REN `reset()`'s with value 0"]
impl crate::ResetValue for super::P4REN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P4REN0`"]
pub type P4REN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4REN0`"]
pub struct P4REN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P4REN0_W<'a> {
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
#[doc = "Reader of field `P4REN1`"]
pub type P4REN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4REN1`"]
pub struct P4REN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P4REN1_W<'a> {
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
#[doc = "Reader of field `P4REN2`"]
pub type P4REN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4REN2`"]
pub struct P4REN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P4REN2_W<'a> {
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
#[doc = "Reader of field `P4REN3`"]
pub type P4REN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4REN3`"]
pub struct P4REN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P4REN3_W<'a> {
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
#[doc = "Reader of field `P4REN4`"]
pub type P4REN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4REN4`"]
pub struct P4REN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P4REN4_W<'a> {
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
#[doc = "Reader of field `P4REN5`"]
pub type P4REN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4REN5`"]
pub struct P4REN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P4REN5_W<'a> {
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
#[doc = "Reader of field `P4REN6`"]
pub type P4REN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4REN6`"]
pub struct P4REN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P4REN6_W<'a> {
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
#[doc = "Reader of field `P4REN7`"]
pub type P4REN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4REN7`"]
pub struct P4REN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P4REN7_W<'a> {
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
    #[doc = "Bit 0 - P4REN0"]
    #[inline(always)]
    pub fn p4ren0(&self) -> P4REN0_R {
        P4REN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P4REN1"]
    #[inline(always)]
    pub fn p4ren1(&self) -> P4REN1_R {
        P4REN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P4REN2"]
    #[inline(always)]
    pub fn p4ren2(&self) -> P4REN2_R {
        P4REN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P4REN3"]
    #[inline(always)]
    pub fn p4ren3(&self) -> P4REN3_R {
        P4REN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P4REN4"]
    #[inline(always)]
    pub fn p4ren4(&self) -> P4REN4_R {
        P4REN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P4REN5"]
    #[inline(always)]
    pub fn p4ren5(&self) -> P4REN5_R {
        P4REN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P4REN6"]
    #[inline(always)]
    pub fn p4ren6(&self) -> P4REN6_R {
        P4REN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P4REN7"]
    #[inline(always)]
    pub fn p4ren7(&self) -> P4REN7_R {
        P4REN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4REN0"]
    #[inline(always)]
    pub fn p4ren0(&mut self) -> P4REN0_W {
        P4REN0_W { w: self }
    }
    #[doc = "Bit 1 - P4REN1"]
    #[inline(always)]
    pub fn p4ren1(&mut self) -> P4REN1_W {
        P4REN1_W { w: self }
    }
    #[doc = "Bit 2 - P4REN2"]
    #[inline(always)]
    pub fn p4ren2(&mut self) -> P4REN2_W {
        P4REN2_W { w: self }
    }
    #[doc = "Bit 3 - P4REN3"]
    #[inline(always)]
    pub fn p4ren3(&mut self) -> P4REN3_W {
        P4REN3_W { w: self }
    }
    #[doc = "Bit 4 - P4REN4"]
    #[inline(always)]
    pub fn p4ren4(&mut self) -> P4REN4_W {
        P4REN4_W { w: self }
    }
    #[doc = "Bit 5 - P4REN5"]
    #[inline(always)]
    pub fn p4ren5(&mut self) -> P4REN5_W {
        P4REN5_W { w: self }
    }
    #[doc = "Bit 6 - P4REN6"]
    #[inline(always)]
    pub fn p4ren6(&mut self) -> P4REN6_W {
        P4REN6_W { w: self }
    }
    #[doc = "Bit 7 - P4REN7"]
    #[inline(always)]
    pub fn p4ren7(&mut self) -> P4REN7_W {
        P4REN7_W { w: self }
    }
}
