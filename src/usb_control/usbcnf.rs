#[doc = "Reader of register USBCNF"]
pub type R = crate::R<u16, super::USBCNF>;
#[doc = "Writer for register USBCNF"]
pub type W = crate::W<u16, super::USBCNF>;
#[doc = "Register USBCNF `reset()`'s with value 0"]
impl crate::ResetValue for super::USBCNF {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USB_EN`"]
pub type USB_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB_EN`"]
pub struct USB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_EN_W<'a> {
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
#[doc = "Reader of field `PUR_EN`"]
pub type PUR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUR_EN`"]
pub struct PUR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PUR_EN_W<'a> {
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
#[doc = "Reader of field `PUR_IN`"]
pub type PUR_IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUR_IN`"]
pub struct PUR_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> PUR_IN_W<'a> {
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
#[doc = "Reader of field `BLKRDY`"]
pub type BLKRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLKRDY`"]
pub struct BLKRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKRDY_W<'a> {
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
#[doc = "Reader of field `FNTEN`"]
pub type FNTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FNTEN`"]
pub struct FNTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FNTEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - USB - Module enable"]
    #[inline(always)]
    pub fn usb_en(&self) -> USB_EN_R {
        USB_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB - PUR pin enable"]
    #[inline(always)]
    pub fn pur_en(&self) -> PUR_EN_R {
        PUR_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB - PUR pin input value"]
    #[inline(always)]
    pub fn pur_in(&self) -> PUR_IN_R {
        PUR_IN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB - Block ready signal for DMA"]
    #[inline(always)]
    pub fn blkrdy(&self) -> BLKRDY_R {
        BLKRDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USB - Frame Number receive Trigger enable for DMA"]
    #[inline(always)]
    pub fn fnten(&self) -> FNTEN_R {
        FNTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - Module enable"]
    #[inline(always)]
    pub fn usb_en(&mut self) -> USB_EN_W {
        USB_EN_W { w: self }
    }
    #[doc = "Bit 1 - USB - PUR pin enable"]
    #[inline(always)]
    pub fn pur_en(&mut self) -> PUR_EN_W {
        PUR_EN_W { w: self }
    }
    #[doc = "Bit 2 - USB - PUR pin input value"]
    #[inline(always)]
    pub fn pur_in(&mut self) -> PUR_IN_W {
        PUR_IN_W { w: self }
    }
    #[doc = "Bit 3 - USB - Block ready signal for DMA"]
    #[inline(always)]
    pub fn blkrdy(&mut self) -> BLKRDY_W {
        BLKRDY_W { w: self }
    }
    #[doc = "Bit 4 - USB - Frame Number receive Trigger enable for DMA"]
    #[inline(always)]
    pub fn fnten(&mut self) -> FNTEN_W {
        FNTEN_W { w: self }
    }
}
