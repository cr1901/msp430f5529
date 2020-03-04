#[doc = "Reader of register USBIE"]
pub type R = crate::R<u8, super::USBIE>;
#[doc = "Writer for register USBIE"]
pub type W = crate::W<u8, super::USBIE>;
#[doc = "Register USBIE `reset()`'s with value 0"]
impl crate::ResetValue for super::USBIE {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STPOWIE`"]
pub type STPOWIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STPOWIE`"]
pub struct STPOWIE_W<'a> {
    w: &'a mut W,
}
impl<'a> STPOWIE_W<'a> {
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
#[doc = "Reader of field `SETUPIE`"]
pub type SETUPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETUPIE`"]
pub struct SETUPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUPIE_W<'a> {
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
#[doc = "Reader of field `RESRIE`"]
pub type RESRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESRIE`"]
pub struct RESRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESRIE_W<'a> {
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
#[doc = "Reader of field `SUSRIE`"]
pub type SUSRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSRIE`"]
pub struct SUSRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSRIE_W<'a> {
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
#[doc = "Reader of field `RSTRIE`"]
pub type RSTRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTRIE`"]
pub struct RSTRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTRIE_W<'a> {
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
    #[doc = "Bit 0 - USB - Setup Overwrite Interrupt Enable"]
    #[inline(always)]
    pub fn stpowie(&self) -> STPOWIE_R {
        STPOWIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB - Setup Interrupt Enable"]
    #[inline(always)]
    pub fn setupie(&self) -> SETUPIE_R {
        SETUPIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB - Function Resume Request Interrupt Enable"]
    #[inline(always)]
    pub fn resrie(&self) -> RESRIE_R {
        RESRIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USB - Function Suspend Request Interrupt Enable"]
    #[inline(always)]
    pub fn susrie(&self) -> SUSRIE_R {
        SUSRIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB - Function Reset Request Interrupt Enable"]
    #[inline(always)]
    pub fn rstrie(&self) -> RSTRIE_R {
        RSTRIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - Setup Overwrite Interrupt Enable"]
    #[inline(always)]
    pub fn stpowie(&mut self) -> STPOWIE_W {
        STPOWIE_W { w: self }
    }
    #[doc = "Bit 2 - USB - Setup Interrupt Enable"]
    #[inline(always)]
    pub fn setupie(&mut self) -> SETUPIE_W {
        SETUPIE_W { w: self }
    }
    #[doc = "Bit 5 - USB - Function Resume Request Interrupt Enable"]
    #[inline(always)]
    pub fn resrie(&mut self) -> RESRIE_W {
        RESRIE_W { w: self }
    }
    #[doc = "Bit 6 - USB - Function Suspend Request Interrupt Enable"]
    #[inline(always)]
    pub fn susrie(&mut self) -> SUSRIE_W {
        SUSRIE_W { w: self }
    }
    #[doc = "Bit 7 - USB - Function Reset Request Interrupt Enable"]
    #[inline(always)]
    pub fn rstrie(&mut self) -> RSTRIE_W {
        RSTRIE_W { w: self }
    }
}
