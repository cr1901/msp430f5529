#[doc = "Reader of register P4MAP0"]
pub type R = crate::R<u8, super::P4MAP0>;
#[doc = "Writer for register P4MAP0"]
pub type W = crate::W<u8, super::P4MAP0>;
#[doc = "Register P4MAP0 `reset()`'s with value 0"]
impl crate::ResetValue for super::P4MAP0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PMAP0`"]
pub type PMAP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMAP0`"]
pub struct PMAP0_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAP0_W<'a> {
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
#[doc = "Reader of field `PMAP1`"]
pub type PMAP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMAP1`"]
pub struct PMAP1_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAP1_W<'a> {
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
#[doc = "Reader of field `PMAP2`"]
pub type PMAP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMAP2`"]
pub struct PMAP2_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAP2_W<'a> {
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
#[doc = "Reader of field `PMAP3`"]
pub type PMAP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMAP3`"]
pub struct PMAP3_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAP3_W<'a> {
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
#[doc = "Reader of field `PMAP4`"]
pub type PMAP4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMAP4`"]
pub struct PMAP4_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAP4_W<'a> {
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
#[doc = "Reader of field `PMAP5`"]
pub type PMAP5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMAP5`"]
pub struct PMAP5_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAP5_W<'a> {
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
#[doc = "Reader of field `PMAP6`"]
pub type PMAP6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMAP6`"]
pub struct PMAP6_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAP6_W<'a> {
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
#[doc = "Reader of field `PMAP7`"]
pub type PMAP7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMAP7`"]
pub struct PMAP7_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAP7_W<'a> {
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
    #[doc = "Bit 0 - PMAP0"]
    #[inline(always)]
    pub fn pmap0(&self) -> PMAP0_R {
        PMAP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PMAP1"]
    #[inline(always)]
    pub fn pmap1(&self) -> PMAP1_R {
        PMAP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PMAP2"]
    #[inline(always)]
    pub fn pmap2(&self) -> PMAP2_R {
        PMAP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PMAP3"]
    #[inline(always)]
    pub fn pmap3(&self) -> PMAP3_R {
        PMAP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PMAP4"]
    #[inline(always)]
    pub fn pmap4(&self) -> PMAP4_R {
        PMAP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PMAP5"]
    #[inline(always)]
    pub fn pmap5(&self) -> PMAP5_R {
        PMAP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PMAP6"]
    #[inline(always)]
    pub fn pmap6(&self) -> PMAP6_R {
        PMAP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PMAP7"]
    #[inline(always)]
    pub fn pmap7(&self) -> PMAP7_R {
        PMAP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PMAP0"]
    #[inline(always)]
    pub fn pmap0(&mut self) -> PMAP0_W {
        PMAP0_W { w: self }
    }
    #[doc = "Bit 1 - PMAP1"]
    #[inline(always)]
    pub fn pmap1(&mut self) -> PMAP1_W {
        PMAP1_W { w: self }
    }
    #[doc = "Bit 2 - PMAP2"]
    #[inline(always)]
    pub fn pmap2(&mut self) -> PMAP2_W {
        PMAP2_W { w: self }
    }
    #[doc = "Bit 3 - PMAP3"]
    #[inline(always)]
    pub fn pmap3(&mut self) -> PMAP3_W {
        PMAP3_W { w: self }
    }
    #[doc = "Bit 4 - PMAP4"]
    #[inline(always)]
    pub fn pmap4(&mut self) -> PMAP4_W {
        PMAP4_W { w: self }
    }
    #[doc = "Bit 5 - PMAP5"]
    #[inline(always)]
    pub fn pmap5(&mut self) -> PMAP5_W {
        PMAP5_W { w: self }
    }
    #[doc = "Bit 6 - PMAP6"]
    #[inline(always)]
    pub fn pmap6(&mut self) -> PMAP6_W {
        PMAP6_W { w: self }
    }
    #[doc = "Bit 7 - PMAP7"]
    #[inline(always)]
    pub fn pmap7(&mut self) -> PMAP7_W {
        PMAP7_W { w: self }
    }
}
