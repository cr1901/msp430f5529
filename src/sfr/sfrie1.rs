#[doc = "Reader of register SFRIE1"]
pub type R = crate::R<u16, super::SFRIE1>;
#[doc = "Writer for register SFRIE1"]
pub type W = crate::W<u16, super::SFRIE1>;
#[doc = "Register SFRIE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SFRIE1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDTIE`"]
pub type WDTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTIE`"]
pub struct WDTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTIE_W<'a> {
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
#[doc = "Reader of field `OFIE`"]
pub type OFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OFIE`"]
pub struct OFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OFIE_W<'a> {
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
#[doc = "Reader of field `VMAIE`"]
pub type VMAIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMAIE`"]
pub struct VMAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VMAIE_W<'a> {
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
#[doc = "Reader of field `NMIIE`"]
pub type NMIIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NMIIE`"]
pub struct NMIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIIE_W<'a> {
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
#[doc = "Reader of field `ACCVIE`"]
pub type ACCVIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACCVIE`"]
pub struct ACCVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCVIE_W<'a> {
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
#[doc = "Reader of field `JMBINIE`"]
pub type JMBINIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JMBINIE`"]
pub struct JMBINIE_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBINIE_W<'a> {
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
#[doc = "Reader of field `JMBOUTIE`"]
pub type JMBOUTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JMBOUTIE`"]
pub struct JMBOUTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBOUTIE_W<'a> {
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
    #[doc = "Bit 0 - WDT Interrupt Enable"]
    #[inline(always)]
    pub fn wdtie(&self) -> WDTIE_R {
        WDTIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Osc Fault Enable"]
    #[inline(always)]
    pub fn ofie(&self) -> OFIE_R {
        OFIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Vacant Memory Interrupt Enable"]
    #[inline(always)]
    pub fn vmaie(&self) -> VMAIE_R {
        VMAIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NMI Interrupt Enable"]
    #[inline(always)]
    pub fn nmiie(&self) -> NMIIE_R {
        NMIIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Flash Access Violation Interrupt Enable"]
    #[inline(always)]
    pub fn accvie(&self) -> ACCVIE_R {
        ACCVIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - JTAG Mail Box input Interrupt Enable"]
    #[inline(always)]
    pub fn jmbinie(&self) -> JMBINIE_R {
        JMBINIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - JTAG Mail Box output Interrupt Enable"]
    #[inline(always)]
    pub fn jmboutie(&self) -> JMBOUTIE_R {
        JMBOUTIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT Interrupt Enable"]
    #[inline(always)]
    pub fn wdtie(&mut self) -> WDTIE_W {
        WDTIE_W { w: self }
    }
    #[doc = "Bit 1 - Osc Fault Enable"]
    #[inline(always)]
    pub fn ofie(&mut self) -> OFIE_W {
        OFIE_W { w: self }
    }
    #[doc = "Bit 3 - Vacant Memory Interrupt Enable"]
    #[inline(always)]
    pub fn vmaie(&mut self) -> VMAIE_W {
        VMAIE_W { w: self }
    }
    #[doc = "Bit 4 - NMI Interrupt Enable"]
    #[inline(always)]
    pub fn nmiie(&mut self) -> NMIIE_W {
        NMIIE_W { w: self }
    }
    #[doc = "Bit 5 - Flash Access Violation Interrupt Enable"]
    #[inline(always)]
    pub fn accvie(&mut self) -> ACCVIE_W {
        ACCVIE_W { w: self }
    }
    #[doc = "Bit 6 - JTAG Mail Box input Interrupt Enable"]
    #[inline(always)]
    pub fn jmbinie(&mut self) -> JMBINIE_W {
        JMBINIE_W { w: self }
    }
    #[doc = "Bit 7 - JTAG Mail Box output Interrupt Enable"]
    #[inline(always)]
    pub fn jmboutie(&mut self) -> JMBOUTIE_W {
        JMBOUTIE_W { w: self }
    }
}
