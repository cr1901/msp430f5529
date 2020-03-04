#[doc = "Reader of register USBCTL"]
pub type R = crate::R<u8, super::USBCTL>;
#[doc = "Writer for register USBCTL"]
pub type W = crate::W<u8, super::USBCTL>;
#[doc = "Register USBCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::USBCTL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
#[doc = "Reader of field `FRSTE`"]
pub type FRSTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRSTE`"]
pub struct FRSTE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRSTE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RWUP`"]
pub type RWUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWUP`"]
pub struct RWUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RWUP_W<'a> {
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
#[doc = "Reader of field `FEN`"]
pub type FEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEN`"]
pub struct FEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - USB - Data Response Bit"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - USB - Function Reset Connection Enable"]
    #[inline(always)]
    pub fn frste(&self) -> FRSTE_R {
        FRSTE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB - Device Remote Wakeup Request"]
    #[inline(always)]
    pub fn rwup(&self) -> RWUP_R {
        RWUP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USB - Function Enable Bit"]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - Data Response Bit"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 4 - USB - Function Reset Connection Enable"]
    #[inline(always)]
    pub fn frste(&mut self) -> FRSTE_W {
        FRSTE_W { w: self }
    }
    #[doc = "Bit 5 - USB - Device Remote Wakeup Request"]
    #[inline(always)]
    pub fn rwup(&mut self) -> RWUP_W {
        RWUP_W { w: self }
    }
    #[doc = "Bit 6 - USB - Function Enable Bit"]
    #[inline(always)]
    pub fn fen(&mut self) -> FEN_W {
        FEN_W { w: self }
    }
}
