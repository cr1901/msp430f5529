#[doc = "Reader of register PMMCTL1"]
pub type R = crate::R<u16, super::PMMCTL1>;
#[doc = "Writer for register PMMCTL1"]
pub type W = crate::W<u16, super::PMMCTL1>;
#[doc = "Register PMMCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PMMCTL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PMMREFMD`"]
pub type PMMREFMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMMREFMD`"]
pub struct PMMREFMD_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMREFMD_W<'a> {
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
#[doc = "Reader of field `PMMCMD0`"]
pub type PMMCMD0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMMCMD0`"]
pub struct PMMCMD0_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMCMD0_W<'a> {
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
#[doc = "Reader of field `PMMCMD1`"]
pub type PMMCMD1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMMCMD1`"]
pub struct PMMCMD1_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMCMD1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - PMM Reference Mode"]
    #[inline(always)]
    pub fn pmmrefmd(&self) -> PMMREFMD_R {
        PMMREFMD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - PMM Voltage Regulator Current Mode Bit: 0"]
    #[inline(always)]
    pub fn pmmcmd0(&self) -> PMMCMD0_R {
        PMMCMD0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PMM Voltage Regulator Current Mode Bit: 1"]
    #[inline(always)]
    pub fn pmmcmd1(&self) -> PMMCMD1_R {
        PMMCMD1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PMM Reference Mode"]
    #[inline(always)]
    pub fn pmmrefmd(&mut self) -> PMMREFMD_W {
        PMMREFMD_W { w: self }
    }
    #[doc = "Bit 4 - PMM Voltage Regulator Current Mode Bit: 0"]
    #[inline(always)]
    pub fn pmmcmd0(&mut self) -> PMMCMD0_W {
        PMMCMD0_W { w: self }
    }
    #[doc = "Bit 5 - PMM Voltage Regulator Current Mode Bit: 1"]
    #[inline(always)]
    pub fn pmmcmd1(&mut self) -> PMMCMD1_W {
        PMMCMD1_W { w: self }
    }
}
