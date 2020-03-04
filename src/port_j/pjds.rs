#[doc = "Reader of register PJDS"]
pub type R = crate::R<u16, super::PJDS>;
#[doc = "Writer for register PJDS"]
pub type W = crate::W<u16, super::PJDS>;
#[doc = "Register PJDS `reset()`'s with value 0"]
impl crate::ResetValue for super::PJDS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PJDS0`"]
pub type PJDS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJDS0`"]
pub struct PJDS0_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDS0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `PJDS1`"]
pub type PJDS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJDS1`"]
pub struct PJDS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDS1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `PJDS2`"]
pub type PJDS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJDS2`"]
pub struct PJDS2_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDS2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PJDS3`"]
pub type PJDS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJDS3`"]
pub struct PJDS3_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDS3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PJDS0"]
    #[inline(always)]
    pub fn pjds0(&self) -> PJDS0_R {
        PJDS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PJDS1"]
    #[inline(always)]
    pub fn pjds1(&self) -> PJDS1_R {
        PJDS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PJDS2"]
    #[inline(always)]
    pub fn pjds2(&self) -> PJDS2_R {
        PJDS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PJDS3"]
    #[inline(always)]
    pub fn pjds3(&self) -> PJDS3_R {
        PJDS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJDS0"]
    #[inline(always)]
    pub fn pjds0(&mut self) -> PJDS0_W {
        PJDS0_W { w: self }
    }
    #[doc = "Bit 1 - PJDS1"]
    #[inline(always)]
    pub fn pjds1(&mut self) -> PJDS1_W {
        PJDS1_W { w: self }
    }
    #[doc = "Bit 2 - PJDS2"]
    #[inline(always)]
    pub fn pjds2(&mut self) -> PJDS2_W {
        PJDS2_W { w: self }
    }
    #[doc = "Bit 3 - PJDS3"]
    #[inline(always)]
    pub fn pjds3(&mut self) -> PJDS3_W {
        PJDS3_W { w: self }
    }
}
