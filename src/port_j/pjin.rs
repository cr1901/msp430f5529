#[doc = "Reader of register PJIN"]
pub type R = crate::R<u16, super::PJIN>;
#[doc = "Writer for register PJIN"]
pub type W = crate::W<u16, super::PJIN>;
#[doc = "Register PJIN `reset()`'s with value 0"]
impl crate::ResetValue for super::PJIN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PJIN0`"]
pub type PJIN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJIN0`"]
pub struct PJIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PJIN0_W<'a> {
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
#[doc = "Reader of field `PJIN1`"]
pub type PJIN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJIN1`"]
pub struct PJIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PJIN1_W<'a> {
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
#[doc = "Reader of field `PJIN2`"]
pub type PJIN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJIN2`"]
pub struct PJIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PJIN2_W<'a> {
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
#[doc = "Reader of field `PJIN3`"]
pub type PJIN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJIN3`"]
pub struct PJIN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PJIN3_W<'a> {
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
    #[doc = "Bit 0 - PJIN0"]
    #[inline(always)]
    pub fn pjin0(&self) -> PJIN0_R {
        PJIN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PJIN1"]
    #[inline(always)]
    pub fn pjin1(&self) -> PJIN1_R {
        PJIN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PJIN2"]
    #[inline(always)]
    pub fn pjin2(&self) -> PJIN2_R {
        PJIN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PJIN3"]
    #[inline(always)]
    pub fn pjin3(&self) -> PJIN3_R {
        PJIN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJIN0"]
    #[inline(always)]
    pub fn pjin0(&mut self) -> PJIN0_W {
        PJIN0_W { w: self }
    }
    #[doc = "Bit 1 - PJIN1"]
    #[inline(always)]
    pub fn pjin1(&mut self) -> PJIN1_W {
        PJIN1_W { w: self }
    }
    #[doc = "Bit 2 - PJIN2"]
    #[inline(always)]
    pub fn pjin2(&mut self) -> PJIN2_W {
        PJIN2_W { w: self }
    }
    #[doc = "Bit 3 - PJIN3"]
    #[inline(always)]
    pub fn pjin3(&mut self) -> PJIN3_W {
        PJIN3_W { w: self }
    }
}
