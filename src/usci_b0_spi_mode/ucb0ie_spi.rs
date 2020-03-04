#[doc = "Reader of register UCB0IE_SPI"]
pub type R = crate::R<u8, super::UCB0IE_SPI>;
#[doc = "Writer for register UCB0IE_SPI"]
pub type W = crate::W<u8, super::UCB0IE_SPI>;
#[doc = "Register UCB0IE_SPI `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB0IE_SPI {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCRXIE`"]
pub type UCRXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCRXIE`"]
pub struct UCRXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIE_W<'a> {
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
#[doc = "Reader of field `UCTXIE`"]
pub type UCTXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCTXIE`"]
pub struct UCTXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `UCSTTIE`"]
pub type UCSTTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCSTTIE`"]
pub struct UCSTTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTTIE_W<'a> {
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
#[doc = "Reader of field `UCSTPIE`"]
pub type UCSTPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCSTPIE`"]
pub struct UCSTPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTPIE_W<'a> {
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
#[doc = "Reader of field `UCALIE`"]
pub type UCALIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCALIE`"]
pub struct UCALIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCALIE_W<'a> {
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
#[doc = "Reader of field `UCNACKIE`"]
pub type UCNACKIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCNACKIE`"]
pub struct UCNACKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCNACKIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - USCI Receive Interrupt Enable"]
    #[inline(always)]
    pub fn ucrxie(&self) -> UCRXIE_R {
        UCRXIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn uctxie(&self) -> UCTXIE_R {
        UCTXIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - START Condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&self) -> UCSTTIE_R {
        UCSTTIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - STOP Condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&self) -> UCSTPIE_R {
        UCSTPIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Arbitration Lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&self) -> UCALIE_R {
        UCALIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - NACK Condition interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&self) -> UCNACKIE_R {
        UCNACKIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Receive Interrupt Enable"]
    #[inline(always)]
    pub fn ucrxie(&mut self) -> UCRXIE_W {
        UCRXIE_W { w: self }
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn uctxie(&mut self) -> UCTXIE_W {
        UCTXIE_W { w: self }
    }
    #[doc = "Bit 2 - START Condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&mut self) -> UCSTTIE_W {
        UCSTTIE_W { w: self }
    }
    #[doc = "Bit 3 - STOP Condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&mut self) -> UCSTPIE_W {
        UCSTPIE_W { w: self }
    }
    #[doc = "Bit 4 - Arbitration Lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&mut self) -> UCALIE_W {
        UCALIE_W { w: self }
    }
    #[doc = "Bit 5 - NACK Condition interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&mut self) -> UCNACKIE_W {
        UCNACKIE_W { w: self }
    }
}
