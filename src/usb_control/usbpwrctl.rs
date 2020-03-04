#[doc = "Reader of register USBPWRCTL"]
pub type R = crate::R<u16, super::USBPWRCTL>;
#[doc = "Writer for register USBPWRCTL"]
pub type W = crate::W<u16, super::USBPWRCTL>;
#[doc = "Register USBPWRCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::USBPWRCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VUOVLIFG`"]
pub type VUOVLIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VUOVLIFG`"]
pub struct VUOVLIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> VUOVLIFG_W<'a> {
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
#[doc = "Reader of field `VBONIFG`"]
pub type VBONIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBONIFG`"]
pub struct VBONIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> VBONIFG_W<'a> {
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
#[doc = "Reader of field `VBOFFIFG`"]
pub type VBOFFIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBOFFIFG`"]
pub struct VBOFFIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> VBOFFIFG_W<'a> {
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
#[doc = "Reader of field `USBBGVBV`"]
pub type USBBGVBV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBBGVBV`"]
pub struct USBBGVBV_W<'a> {
    w: &'a mut W,
}
impl<'a> USBBGVBV_W<'a> {
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
#[doc = "Reader of field `USBDETEN`"]
pub type USBDETEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBDETEN`"]
pub struct USBDETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDETEN_W<'a> {
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
#[doc = "Reader of field `OVLAOFF`"]
pub type OVLAOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVLAOFF`"]
pub struct OVLAOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVLAOFF_W<'a> {
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
#[doc = "Reader of field `SLDOAON`"]
pub type SLDOAON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLDOAON`"]
pub struct SLDOAON_W<'a> {
    w: &'a mut W,
}
impl<'a> SLDOAON_W<'a> {
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
#[doc = "Reader of field `VUOVLIE`"]
pub type VUOVLIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VUOVLIE`"]
pub struct VUOVLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VUOVLIE_W<'a> {
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
#[doc = "Reader of field `VBONIE`"]
pub type VBONIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBONIE`"]
pub struct VBONIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VBONIE_W<'a> {
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
#[doc = "Reader of field `VBOFFIE`"]
pub type VBOFFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBOFFIE`"]
pub struct VBOFFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VBOFFIE_W<'a> {
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
#[doc = "Reader of field `VUSBEN`"]
pub type VUSBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VUSBEN`"]
pub struct VUSBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VUSBEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SLDOEN`"]
pub type SLDOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLDOEN`"]
pub struct SLDOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLDOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB - VUSB Overload Interrupt Flag"]
    #[inline(always)]
    pub fn vuovlifg(&self) -> VUOVLIFG_R {
        VUOVLIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB - VBUS \"Coming ON\" Interrupt Flag"]
    #[inline(always)]
    pub fn vbonifg(&self) -> VBONIFG_R {
        VBONIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB - VBUS \"Going OFF\" Interrupt Flag"]
    #[inline(always)]
    pub fn vboffifg(&self) -> VBOFFIFG_R {
        VBOFFIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB - USB Bandgap and VBUS valid"]
    #[inline(always)]
    pub fn usbbgvbv(&self) -> USBBGVBV_R {
        USBBGVBV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USB - VBUS on/off events enable"]
    #[inline(always)]
    pub fn usbdeten(&self) -> USBDETEN_R {
        USBDETEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB - LDO overload auto off enable"]
    #[inline(always)]
    pub fn ovlaoff(&self) -> OVLAOFF_R {
        OVLAOFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USB - Secondary LDO auto on enable"]
    #[inline(always)]
    pub fn sldoaon(&self) -> SLDOAON_R {
        SLDOAON_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USB - Overload indication Interrupt Enable"]
    #[inline(always)]
    pub fn vuovlie(&self) -> VUOVLIE_R {
        VUOVLIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - USB - VBUS \"Coming ON\" Interrupt Enable"]
    #[inline(always)]
    pub fn vbonie(&self) -> VBONIE_R {
        VBONIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USB - VBUS \"Going OFF\" Interrupt Enable"]
    #[inline(always)]
    pub fn vboffie(&self) -> VBOFFIE_R {
        VBOFFIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USB - LDO Enable (3.3V)"]
    #[inline(always)]
    pub fn vusben(&self) -> VUSBEN_R {
        VUSBEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - USB - Secondary LDO Enable (1.8V)"]
    #[inline(always)]
    pub fn sldoen(&self) -> SLDOEN_R {
        SLDOEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - VUSB Overload Interrupt Flag"]
    #[inline(always)]
    pub fn vuovlifg(&mut self) -> VUOVLIFG_W {
        VUOVLIFG_W { w: self }
    }
    #[doc = "Bit 1 - USB - VBUS \"Coming ON\" Interrupt Flag"]
    #[inline(always)]
    pub fn vbonifg(&mut self) -> VBONIFG_W {
        VBONIFG_W { w: self }
    }
    #[doc = "Bit 2 - USB - VBUS \"Going OFF\" Interrupt Flag"]
    #[inline(always)]
    pub fn vboffifg(&mut self) -> VBOFFIFG_W {
        VBOFFIFG_W { w: self }
    }
    #[doc = "Bit 3 - USB - USB Bandgap and VBUS valid"]
    #[inline(always)]
    pub fn usbbgvbv(&mut self) -> USBBGVBV_W {
        USBBGVBV_W { w: self }
    }
    #[doc = "Bit 4 - USB - VBUS on/off events enable"]
    #[inline(always)]
    pub fn usbdeten(&mut self) -> USBDETEN_W {
        USBDETEN_W { w: self }
    }
    #[doc = "Bit 5 - USB - LDO overload auto off enable"]
    #[inline(always)]
    pub fn ovlaoff(&mut self) -> OVLAOFF_W {
        OVLAOFF_W { w: self }
    }
    #[doc = "Bit 6 - USB - Secondary LDO auto on enable"]
    #[inline(always)]
    pub fn sldoaon(&mut self) -> SLDOAON_W {
        SLDOAON_W { w: self }
    }
    #[doc = "Bit 8 - USB - Overload indication Interrupt Enable"]
    #[inline(always)]
    pub fn vuovlie(&mut self) -> VUOVLIE_W {
        VUOVLIE_W { w: self }
    }
    #[doc = "Bit 9 - USB - VBUS \"Coming ON\" Interrupt Enable"]
    #[inline(always)]
    pub fn vbonie(&mut self) -> VBONIE_W {
        VBONIE_W { w: self }
    }
    #[doc = "Bit 10 - USB - VBUS \"Going OFF\" Interrupt Enable"]
    #[inline(always)]
    pub fn vboffie(&mut self) -> VBOFFIE_W {
        VBOFFIE_W { w: self }
    }
    #[doc = "Bit 11 - USB - LDO Enable (3.3V)"]
    #[inline(always)]
    pub fn vusben(&mut self) -> VUSBEN_W {
        VUSBEN_W { w: self }
    }
    #[doc = "Bit 12 - USB - Secondary LDO Enable (1.8V)"]
    #[inline(always)]
    pub fn sldoen(&mut self) -> SLDOEN_W {
        SLDOEN_W { w: self }
    }
}
