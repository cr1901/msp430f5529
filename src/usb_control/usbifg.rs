#[doc = "Reader of register USBIFG"]
pub type R = crate::R<u8, super::USBIFG>;
#[doc = "Writer for register USBIFG"]
pub type W = crate::W<u8, super::USBIFG>;
#[doc = "Register USBIFG `reset()`'s with value 0"]
impl crate::ResetValue for super::USBIFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STPOWIFG`"]
pub type STPOWIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STPOWIFG`"]
pub struct STPOWIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> STPOWIFG_W<'a> {
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
#[doc = "Reader of field `SETUPIFG`"]
pub type SETUPIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETUPIFG`"]
pub struct SETUPIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUPIFG_W<'a> {
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
#[doc = "Reader of field `RESRIFG`"]
pub type RESRIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESRIFG`"]
pub struct RESRIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RESRIFG_W<'a> {
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
#[doc = "Reader of field `SUSRIFG`"]
pub type SUSRIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSRIFG`"]
pub struct SUSRIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSRIFG_W<'a> {
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
#[doc = "Reader of field `RSTRIFG`"]
pub type RSTRIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTRIFG`"]
pub struct RSTRIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTRIFG_W<'a> {
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
    #[doc = "Bit 0 - USB - Setup Overwrite Interrupt Flag"]
    #[inline(always)]
    pub fn stpowifg(&self) -> STPOWIFG_R {
        STPOWIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB - Setup Interrupt Flag"]
    #[inline(always)]
    pub fn setupifg(&self) -> SETUPIFG_R {
        SETUPIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB - Function Resume Request Interrupt Flag"]
    #[inline(always)]
    pub fn resrifg(&self) -> RESRIFG_R {
        RESRIFG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USB - Function Suspend Request Interrupt Flag"]
    #[inline(always)]
    pub fn susrifg(&self) -> SUSRIFG_R {
        SUSRIFG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB - Function Reset Request Interrupt Flag"]
    #[inline(always)]
    pub fn rstrifg(&self) -> RSTRIFG_R {
        RSTRIFG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - Setup Overwrite Interrupt Flag"]
    #[inline(always)]
    pub fn stpowifg(&mut self) -> STPOWIFG_W {
        STPOWIFG_W { w: self }
    }
    #[doc = "Bit 2 - USB - Setup Interrupt Flag"]
    #[inline(always)]
    pub fn setupifg(&mut self) -> SETUPIFG_W {
        SETUPIFG_W { w: self }
    }
    #[doc = "Bit 5 - USB - Function Resume Request Interrupt Flag"]
    #[inline(always)]
    pub fn resrifg(&mut self) -> RESRIFG_W {
        RESRIFG_W { w: self }
    }
    #[doc = "Bit 6 - USB - Function Suspend Request Interrupt Flag"]
    #[inline(always)]
    pub fn susrifg(&mut self) -> SUSRIFG_W {
        SUSRIFG_W { w: self }
    }
    #[doc = "Bit 7 - USB - Function Reset Request Interrupt Flag"]
    #[inline(always)]
    pub fn rstrifg(&mut self) -> RSTRIFG_W {
        RSTRIFG_W { w: self }
    }
}
