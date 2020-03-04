#[doc = "Reader of register SFRIFG1"]
pub type R = crate::R<u16, super::SFRIFG1>;
#[doc = "Writer for register SFRIFG1"]
pub type W = crate::W<u16, super::SFRIFG1>;
#[doc = "Register SFRIFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SFRIFG1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDTIFG`"]
pub type WDTIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTIFG`"]
pub struct WDTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTIFG_W<'a> {
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
#[doc = "Reader of field `OFIFG`"]
pub type OFIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OFIFG`"]
pub struct OFIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> OFIFG_W<'a> {
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
#[doc = "Reader of field `VMAIFG`"]
pub type VMAIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMAIFG`"]
pub struct VMAIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> VMAIFG_W<'a> {
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
#[doc = "Reader of field `NMIIFG`"]
pub type NMIIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NMIIFG`"]
pub struct NMIIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIIFG_W<'a> {
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
#[doc = "Reader of field `JMBINIFG`"]
pub type JMBINIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JMBINIFG`"]
pub struct JMBINIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBINIFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `JMBOUTIFG`"]
pub type JMBOUTIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JMBOUTIFG`"]
pub struct JMBOUTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBOUTIFG_W<'a> {
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
    #[doc = "Bit 0 - WDT Interrupt Flag"]
    #[inline(always)]
    pub fn wdtifg(&self) -> WDTIFG_R {
        WDTIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Osc Fault Flag"]
    #[inline(always)]
    pub fn ofifg(&self) -> OFIFG_R {
        OFIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Vacant Memory Interrupt Flag"]
    #[inline(always)]
    pub fn vmaifg(&self) -> VMAIFG_R {
        VMAIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NMI Interrupt Flag"]
    #[inline(always)]
    pub fn nmiifg(&self) -> NMIIFG_R {
        NMIIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - JTAG Mail Box input Interrupt Flag"]
    #[inline(always)]
    pub fn jmbinifg(&self) -> JMBINIFG_R {
        JMBINIFG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - JTAG Mail Box output Interrupt Flag"]
    #[inline(always)]
    pub fn jmboutifg(&self) -> JMBOUTIFG_R {
        JMBOUTIFG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT Interrupt Flag"]
    #[inline(always)]
    pub fn wdtifg(&mut self) -> WDTIFG_W {
        WDTIFG_W { w: self }
    }
    #[doc = "Bit 1 - Osc Fault Flag"]
    #[inline(always)]
    pub fn ofifg(&mut self) -> OFIFG_W {
        OFIFG_W { w: self }
    }
    #[doc = "Bit 3 - Vacant Memory Interrupt Flag"]
    #[inline(always)]
    pub fn vmaifg(&mut self) -> VMAIFG_W {
        VMAIFG_W { w: self }
    }
    #[doc = "Bit 4 - NMI Interrupt Flag"]
    #[inline(always)]
    pub fn nmiifg(&mut self) -> NMIIFG_W {
        NMIIFG_W { w: self }
    }
    #[doc = "Bit 6 - JTAG Mail Box input Interrupt Flag"]
    #[inline(always)]
    pub fn jmbinifg(&mut self) -> JMBINIFG_W {
        JMBINIFG_W { w: self }
    }
    #[doc = "Bit 7 - JTAG Mail Box output Interrupt Flag"]
    #[inline(always)]
    pub fn jmboutifg(&mut self) -> JMBOUTIFG_W {
        JMBOUTIFG_W { w: self }
    }
}
