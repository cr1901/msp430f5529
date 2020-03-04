#[doc = "Reader of register PJREN"]
pub type R = crate::R<u16, super::PJREN>;
#[doc = "Writer for register PJREN"]
pub type W = crate::W<u16, super::PJREN>;
#[doc = "Register PJREN `reset()`'s with value 0"]
impl crate::ResetValue for super::PJREN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PJREN0`"]
pub type PJREN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJREN0`"]
pub struct PJREN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PJREN0_W<'a> {
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
#[doc = "Reader of field `PJREN1`"]
pub type PJREN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJREN1`"]
pub struct PJREN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PJREN1_W<'a> {
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
#[doc = "Reader of field `PJREN2`"]
pub type PJREN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJREN2`"]
pub struct PJREN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PJREN2_W<'a> {
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
#[doc = "Reader of field `PJREN3`"]
pub type PJREN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJREN3`"]
pub struct PJREN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PJREN3_W<'a> {
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
    #[doc = "Bit 0 - PJREN0"]
    #[inline(always)]
    pub fn pjren0(&self) -> PJREN0_R {
        PJREN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PJREN1"]
    #[inline(always)]
    pub fn pjren1(&self) -> PJREN1_R {
        PJREN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PJREN2"]
    #[inline(always)]
    pub fn pjren2(&self) -> PJREN2_R {
        PJREN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PJREN3"]
    #[inline(always)]
    pub fn pjren3(&self) -> PJREN3_R {
        PJREN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJREN0"]
    #[inline(always)]
    pub fn pjren0(&mut self) -> PJREN0_W {
        PJREN0_W { w: self }
    }
    #[doc = "Bit 1 - PJREN1"]
    #[inline(always)]
    pub fn pjren1(&mut self) -> PJREN1_W {
        PJREN1_W { w: self }
    }
    #[doc = "Bit 2 - PJREN2"]
    #[inline(always)]
    pub fn pjren2(&mut self) -> PJREN2_W {
        PJREN2_W { w: self }
    }
    #[doc = "Bit 3 - PJREN3"]
    #[inline(always)]
    pub fn pjren3(&mut self) -> PJREN3_W {
        PJREN3_W { w: self }
    }
}
