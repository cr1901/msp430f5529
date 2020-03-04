#[doc = "Reader of register SVSMIO"]
pub type R = crate::R<u16, super::SVSMIO>;
#[doc = "Writer for register SVSMIO"]
pub type W = crate::W<u16, super::SVSMIO>;
#[doc = "Register SVSMIO `reset()`'s with value 0"]
impl crate::ResetValue for super::SVSMIO {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SVMLOE`"]
pub type SVMLOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVMLOE`"]
pub struct SVMLOE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMLOE_W<'a> {
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
#[doc = "Reader of field `SVMLVLROE`"]
pub type SVMLVLROE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVMLVLROE`"]
pub struct SVMLVLROE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMLVLROE_W<'a> {
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
#[doc = "Reader of field `SVMOUTPOL`"]
pub type SVMOUTPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVMOUTPOL`"]
pub struct SVMOUTPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMOUTPOL_W<'a> {
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
#[doc = "Reader of field `SVMHOE`"]
pub type SVMHOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVMHOE`"]
pub struct SVMHOE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMHOE_W<'a> {
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
#[doc = "Reader of field `SVMHVLROE`"]
pub type SVMHVLROE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVMHVLROE`"]
pub struct SVMHVLROE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMHVLROE_W<'a> {
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
    #[doc = "Bit 3 - SVM low side output enable"]
    #[inline(always)]
    pub fn svmloe(&self) -> SVMLOE_R {
        SVMLOE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SVM low side voltage level reached output enable"]
    #[inline(always)]
    pub fn svmlvlroe(&self) -> SVMLVLROE_R {
        SVMLVLROE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SVMOUT pin polarity"]
    #[inline(always)]
    pub fn svmoutpol(&self) -> SVMOUTPOL_R {
        SVMOUTPOL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SVM high side output enable"]
    #[inline(always)]
    pub fn svmhoe(&self) -> SVMHOE_R {
        SVMHOE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SVM high side voltage level reached output enable"]
    #[inline(always)]
    pub fn svmhvlroe(&self) -> SVMHVLROE_R {
        SVMHVLROE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - SVM low side output enable"]
    #[inline(always)]
    pub fn svmloe(&mut self) -> SVMLOE_W {
        SVMLOE_W { w: self }
    }
    #[doc = "Bit 4 - SVM low side voltage level reached output enable"]
    #[inline(always)]
    pub fn svmlvlroe(&mut self) -> SVMLVLROE_W {
        SVMLVLROE_W { w: self }
    }
    #[doc = "Bit 5 - SVMOUT pin polarity"]
    #[inline(always)]
    pub fn svmoutpol(&mut self) -> SVMOUTPOL_W {
        SVMOUTPOL_W { w: self }
    }
    #[doc = "Bit 11 - SVM high side output enable"]
    #[inline(always)]
    pub fn svmhoe(&mut self) -> SVMHOE_W {
        SVMHOE_W { w: self }
    }
    #[doc = "Bit 12 - SVM high side voltage level reached output enable"]
    #[inline(always)]
    pub fn svmhvlroe(&mut self) -> SVMHVLROE_W {
        SVMHVLROE_W { w: self }
    }
}
