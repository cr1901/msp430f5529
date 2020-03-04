#[doc = "Reader of register USBPLLIR"]
pub type R = crate::R<u16, super::USBPLLIR>;
#[doc = "Writer for register USBPLLIR"]
pub type W = crate::W<u16, super::USBPLLIR>;
#[doc = "Register USBPLLIR `reset()`'s with value 0"]
impl crate::ResetValue for super::USBPLLIR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBOOLIFG`"]
pub type USBOOLIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBOOLIFG`"]
pub struct USBOOLIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> USBOOLIFG_W<'a> {
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
#[doc = "Reader of field `USBLOSIFG`"]
pub type USBLOSIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBLOSIFG`"]
pub struct USBLOSIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> USBLOSIFG_W<'a> {
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
#[doc = "Reader of field `USBOORIFG`"]
pub type USBOORIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBOORIFG`"]
pub struct USBOORIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> USBOORIFG_W<'a> {
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
#[doc = "Reader of field `USBOOLIE`"]
pub type USBOOLIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBOOLIE`"]
pub struct USBOOLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBOOLIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `USBLOSIE`"]
pub type USBLOSIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBLOSIE`"]
pub struct USBLOSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBLOSIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `USBOORIE`"]
pub type USBOORIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBOORIE`"]
pub struct USBOORIE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBOORIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB - PLL out of lock Interrupt Flag"]
    #[inline(always)]
    pub fn usboolifg(&self) -> USBOOLIFG_R {
        USBOOLIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB - PLL loss of signal Interrupt Flag"]
    #[inline(always)]
    pub fn usblosifg(&self) -> USBLOSIFG_R {
        USBLOSIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB - PLL out of range Interrupt Flag"]
    #[inline(always)]
    pub fn usboorifg(&self) -> USBOORIFG_R {
        USBOORIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USB - PLL out of lock Interrupt enable"]
    #[inline(always)]
    pub fn usboolie(&self) -> USBOOLIE_R {
        USBOOLIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - USB - PLL loss of signal Interrupt enable"]
    #[inline(always)]
    pub fn usblosie(&self) -> USBLOSIE_R {
        USBLOSIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USB - PLL out of range Interrupt enable"]
    #[inline(always)]
    pub fn usboorie(&self) -> USBOORIE_R {
        USBOORIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - PLL out of lock Interrupt Flag"]
    #[inline(always)]
    pub fn usboolifg(&mut self) -> USBOOLIFG_W {
        USBOOLIFG_W { w: self }
    }
    #[doc = "Bit 1 - USB - PLL loss of signal Interrupt Flag"]
    #[inline(always)]
    pub fn usblosifg(&mut self) -> USBLOSIFG_W {
        USBLOSIFG_W { w: self }
    }
    #[doc = "Bit 2 - USB - PLL out of range Interrupt Flag"]
    #[inline(always)]
    pub fn usboorifg(&mut self) -> USBOORIFG_W {
        USBOORIFG_W { w: self }
    }
    #[doc = "Bit 8 - USB - PLL out of lock Interrupt enable"]
    #[inline(always)]
    pub fn usboolie(&mut self) -> USBOOLIE_W {
        USBOOLIE_W { w: self }
    }
    #[doc = "Bit 9 - USB - PLL loss of signal Interrupt enable"]
    #[inline(always)]
    pub fn usblosie(&mut self) -> USBLOSIE_W {
        USBLOSIE_W { w: self }
    }
    #[doc = "Bit 10 - USB - PLL out of range Interrupt enable"]
    #[inline(always)]
    pub fn usboorie(&mut self) -> USBOORIE_W {
        USBOORIE_W { w: self }
    }
}
