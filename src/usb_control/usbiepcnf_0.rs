#[doc = "Reader of register USBIEPCNF_0"]
pub type R = crate::R<u8, super::USBIEPCNF_0>;
#[doc = "Writer for register USBIEPCNF_0"]
pub type W = crate::W<u8, super::USBIEPCNF_0>;
#[doc = "Register USBIEPCNF_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::USBIEPCNF_0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBIIE`"]
pub type USBIIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBIIE`"]
pub struct USBIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBIIE_W<'a> {
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
#[doc = "Reader of field `STALL`"]
pub type STALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STALL`"]
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TOGGLE`"]
pub type TOGGLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOGGLE`"]
pub struct TOGGLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOGGLE_W<'a> {
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
#[doc = "Reader of field `UBME`"]
pub type UBME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UBME`"]
pub struct UBME_W<'a> {
    w: &'a mut W,
}
impl<'a> UBME_W<'a> {
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
    #[doc = "Bit 2 - USB - Transaction Interrupt indication enable"]
    #[inline(always)]
    pub fn usbiie(&self) -> USBIIE_R {
        USBIIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB - Stall Condition"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB - Toggle Bit"]
    #[inline(always)]
    pub fn toggle(&self) -> TOGGLE_R {
        TOGGLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB - UBM In-Endpoint Enable"]
    #[inline(always)]
    pub fn ubme(&self) -> UBME_R {
        UBME_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - USB - Transaction Interrupt indication enable"]
    #[inline(always)]
    pub fn usbiie(&mut self) -> USBIIE_W {
        USBIIE_W { w: self }
    }
    #[doc = "Bit 3 - USB - Stall Condition"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    #[doc = "Bit 5 - USB - Toggle Bit"]
    #[inline(always)]
    pub fn toggle(&mut self) -> TOGGLE_W {
        TOGGLE_W { w: self }
    }
    #[doc = "Bit 7 - USB - UBM In-Endpoint Enable"]
    #[inline(always)]
    pub fn ubme(&mut self) -> UBME_W {
        UBME_W { w: self }
    }
}
