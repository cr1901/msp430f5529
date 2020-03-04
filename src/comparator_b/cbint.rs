#[doc = "Reader of register CBINT"]
pub type R = crate::R<u16, super::CBINT>;
#[doc = "Writer for register CBINT"]
pub type W = crate::W<u16, super::CBINT>;
#[doc = "Register CBINT `reset()`'s with value 0"]
impl crate::ResetValue for super::CBINT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CBIFG`"]
pub type CBIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBIFG`"]
pub struct CBIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CBIFG_W<'a> {
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
#[doc = "Reader of field `CBIIFG`"]
pub type CBIIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBIIFG`"]
pub struct CBIIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CBIIFG_W<'a> {
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
#[doc = "Reader of field `CBIE`"]
pub type CBIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBIE`"]
pub struct CBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CBIE_W<'a> {
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
#[doc = "Reader of field `CBIIE`"]
pub type CBIIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBIIE`"]
pub struct CBIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CBIIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Comp. B Interrupt Flag"]
    #[inline(always)]
    pub fn cbifg(&self) -> CBIFG_R {
        CBIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comp. B Interrupt Flag Inverted Polarity"]
    #[inline(always)]
    pub fn cbiifg(&self) -> CBIIFG_R {
        CBIIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Comp. B Interrupt Enable"]
    #[inline(always)]
    pub fn cbie(&self) -> CBIE_R {
        CBIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Comp. B Interrupt Enable Inverted Polarity"]
    #[inline(always)]
    pub fn cbiie(&self) -> CBIIE_R {
        CBIIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. B Interrupt Flag"]
    #[inline(always)]
    pub fn cbifg(&mut self) -> CBIFG_W {
        CBIFG_W { w: self }
    }
    #[doc = "Bit 1 - Comp. B Interrupt Flag Inverted Polarity"]
    #[inline(always)]
    pub fn cbiifg(&mut self) -> CBIIFG_W {
        CBIIFG_W { w: self }
    }
    #[doc = "Bit 8 - Comp. B Interrupt Enable"]
    #[inline(always)]
    pub fn cbie(&mut self) -> CBIE_W {
        CBIE_W { w: self }
    }
    #[doc = "Bit 9 - Comp. B Interrupt Enable Inverted Polarity"]
    #[inline(always)]
    pub fn cbiie(&mut self) -> CBIIE_W {
        CBIIE_W { w: self }
    }
}
