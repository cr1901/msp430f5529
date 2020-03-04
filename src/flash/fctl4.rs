#[doc = "Reader of register FCTL4"]
pub type R = crate::R<u16, super::FCTL4>;
#[doc = "Writer for register FCTL4"]
pub type W = crate::W<u16, super::FCTL4>;
#[doc = "Register FCTL4 `reset()`'s with value 0"]
impl crate::ResetValue for super::FCTL4 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VPE`"]
pub type VPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VPE`"]
pub struct VPE_W<'a> {
    w: &'a mut W,
}
impl<'a> VPE_W<'a> {
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
#[doc = "Reader of field `MGR0`"]
pub type MGR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MGR0`"]
pub struct MGR0_W<'a> {
    w: &'a mut W,
}
impl<'a> MGR0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `MGR1`"]
pub type MGR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MGR1`"]
pub struct MGR1_W<'a> {
    w: &'a mut W,
}
impl<'a> MGR1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `LOCKINFO`"]
pub type LOCKINFO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCKINFO`"]
pub struct LOCKINFO_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKINFO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Voltage Changed during Program Error Flag"]
    #[inline(always)]
    pub fn vpe(&self) -> VPE_R {
        VPE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Marginal read 0 mode."]
    #[inline(always)]
    pub fn mgr0(&self) -> MGR0_R {
        MGR0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Marginal read 1 mode."]
    #[inline(always)]
    pub fn mgr1(&self) -> MGR1_R {
        MGR1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Lock INFO Memory bit: read = 1 - Segment is locked (read only)"]
    #[inline(always)]
    pub fn lockinfo(&self) -> LOCKINFO_R {
        LOCKINFO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Changed during Program Error Flag"]
    #[inline(always)]
    pub fn vpe(&mut self) -> VPE_W {
        VPE_W { w: self }
    }
    #[doc = "Bit 4 - Marginal read 0 mode."]
    #[inline(always)]
    pub fn mgr0(&mut self) -> MGR0_W {
        MGR0_W { w: self }
    }
    #[doc = "Bit 5 - Marginal read 1 mode."]
    #[inline(always)]
    pub fn mgr1(&mut self) -> MGR1_W {
        MGR1_W { w: self }
    }
    #[doc = "Bit 7 - Lock INFO Memory bit: read = 1 - Segment is locked (read only)"]
    #[inline(always)]
    pub fn lockinfo(&mut self) -> LOCKINFO_W {
        LOCKINFO_W { w: self }
    }
}
