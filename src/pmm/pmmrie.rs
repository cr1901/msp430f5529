#[doc = "Reader of register PMMRIE"]
pub type R = crate::R<u16, super::PMMRIE>;
#[doc = "Writer for register PMMRIE"]
pub type W = crate::W<u16, super::PMMRIE>;
#[doc = "Register PMMRIE `reset()`'s with value 0"]
impl crate::ResetValue for super::PMMRIE {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SVSMLDLYIE`"]
pub type SVSMLDLYIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVSMLDLYIE`"]
pub struct SVSMLDLYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSMLDLYIE_W<'a> {
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
#[doc = "Reader of field `SVMLIE`"]
pub type SVMLIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVMLIE`"]
pub struct SVMLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMLIE_W<'a> {
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
#[doc = "Reader of field `SVMLVLRIE`"]
pub type SVMLVLRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVMLVLRIE`"]
pub struct SVMLVLRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMLVLRIE_W<'a> {
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
#[doc = "Reader of field `SVSMHDLYIE`"]
pub type SVSMHDLYIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVSMHDLYIE`"]
pub struct SVSMHDLYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSMHDLYIE_W<'a> {
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
#[doc = "Reader of field `SVMHIE`"]
pub type SVMHIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVMHIE`"]
pub struct SVMHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMHIE_W<'a> {
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
#[doc = "Reader of field `SVMHVLRIE`"]
pub type SVMHVLRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVMHVLRIE`"]
pub struct SVMHVLRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMHVLRIE_W<'a> {
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
#[doc = "Reader of field `SVSLPE`"]
pub type SVSLPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVSLPE`"]
pub struct SVSLPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSLPE_W<'a> {
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
#[doc = "Reader of field `SVMLVLRPE`"]
pub type SVMLVLRPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVMLVLRPE`"]
pub struct SVMLVLRPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMLVLRPE_W<'a> {
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
#[doc = "Reader of field `SVSHPE`"]
pub type SVSHPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVSHPE`"]
pub struct SVSHPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSHPE_W<'a> {
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
#[doc = "Reader of field `SVMHVLRPE`"]
pub type SVMHVLRPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVMHVLRPE`"]
pub struct SVMHVLRPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMHVLRPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SVS and SVM low side Delay expired interrupt enable"]
    #[inline(always)]
    pub fn svsmldlyie(&self) -> SVSMLDLYIE_R {
        SVSMLDLYIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SVM low side interrupt enable"]
    #[inline(always)]
    pub fn svmlie(&self) -> SVMLIE_R {
        SVMLIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SVM low side Voltage Level Reached interrupt enable"]
    #[inline(always)]
    pub fn svmlvlrie(&self) -> SVMLVLRIE_R {
        SVMLVLRIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SVS and SVM high side Delay expired interrupt enable"]
    #[inline(always)]
    pub fn svsmhdlyie(&self) -> SVSMHDLYIE_R {
        SVSMHDLYIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SVM high side interrupt enable"]
    #[inline(always)]
    pub fn svmhie(&self) -> SVMHIE_R {
        SVMHIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SVM high side Voltage Level Reached interrupt enable"]
    #[inline(always)]
    pub fn svmhvlrie(&self) -> SVMHVLRIE_R {
        SVMHVLRIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SVS low side POR enable"]
    #[inline(always)]
    pub fn svslpe(&self) -> SVSLPE_R {
        SVSLPE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SVM low side Voltage Level reached POR enable"]
    #[inline(always)]
    pub fn svmlvlrpe(&self) -> SVMLVLRPE_R {
        SVMLVLRPE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SVS high side POR enable"]
    #[inline(always)]
    pub fn svshpe(&self) -> SVSHPE_R {
        SVSHPE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SVM high side Voltage Level reached POR enable"]
    #[inline(always)]
    pub fn svmhvlrpe(&self) -> SVMHVLRPE_R {
        SVMHVLRPE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SVS and SVM low side Delay expired interrupt enable"]
    #[inline(always)]
    pub fn svsmldlyie(&mut self) -> SVSMLDLYIE_W {
        SVSMLDLYIE_W { w: self }
    }
    #[doc = "Bit 1 - SVM low side interrupt enable"]
    #[inline(always)]
    pub fn svmlie(&mut self) -> SVMLIE_W {
        SVMLIE_W { w: self }
    }
    #[doc = "Bit 2 - SVM low side Voltage Level Reached interrupt enable"]
    #[inline(always)]
    pub fn svmlvlrie(&mut self) -> SVMLVLRIE_W {
        SVMLVLRIE_W { w: self }
    }
    #[doc = "Bit 4 - SVS and SVM high side Delay expired interrupt enable"]
    #[inline(always)]
    pub fn svsmhdlyie(&mut self) -> SVSMHDLYIE_W {
        SVSMHDLYIE_W { w: self }
    }
    #[doc = "Bit 5 - SVM high side interrupt enable"]
    #[inline(always)]
    pub fn svmhie(&mut self) -> SVMHIE_W {
        SVMHIE_W { w: self }
    }
    #[doc = "Bit 6 - SVM high side Voltage Level Reached interrupt enable"]
    #[inline(always)]
    pub fn svmhvlrie(&mut self) -> SVMHVLRIE_W {
        SVMHVLRIE_W { w: self }
    }
    #[doc = "Bit 8 - SVS low side POR enable"]
    #[inline(always)]
    pub fn svslpe(&mut self) -> SVSLPE_W {
        SVSLPE_W { w: self }
    }
    #[doc = "Bit 9 - SVM low side Voltage Level reached POR enable"]
    #[inline(always)]
    pub fn svmlvlrpe(&mut self) -> SVMLVLRPE_W {
        SVMLVLRPE_W { w: self }
    }
    #[doc = "Bit 12 - SVS high side POR enable"]
    #[inline(always)]
    pub fn svshpe(&mut self) -> SVSHPE_W {
        SVSHPE_W { w: self }
    }
    #[doc = "Bit 13 - SVM high side Voltage Level reached POR enable"]
    #[inline(always)]
    pub fn svmhvlrpe(&mut self) -> SVMHVLRPE_W {
        SVMHVLRPE_W { w: self }
    }
}
