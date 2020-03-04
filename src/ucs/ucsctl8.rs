#[doc = "Reader of register UCSCTL8"]
pub type R = crate::R<u16, super::UCSCTL8>;
#[doc = "Writer for register UCSCTL8"]
pub type W = crate::W<u16, super::UCSCTL8>;
#[doc = "Register UCSCTL8 `reset()`'s with value 0"]
impl crate::ResetValue for super::UCSCTL8 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ACLKREQEN`"]
pub type ACLKREQEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACLKREQEN`"]
pub struct ACLKREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLKREQEN_W<'a> {
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
#[doc = "Reader of field `MCLKREQEN`"]
pub type MCLKREQEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCLKREQEN`"]
pub struct MCLKREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLKREQEN_W<'a> {
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
#[doc = "Reader of field `SMCLKREQEN`"]
pub type SMCLKREQEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMCLKREQEN`"]
pub struct SMCLKREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMCLKREQEN_W<'a> {
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
#[doc = "Reader of field `MODOSCREQEN`"]
pub type MODOSCREQEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODOSCREQEN`"]
pub struct MODOSCREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MODOSCREQEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - ACLK Clock Request Enable"]
    #[inline(always)]
    pub fn aclkreqen(&self) -> ACLKREQEN_R {
        ACLKREQEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MCLK Clock Request Enable"]
    #[inline(always)]
    pub fn mclkreqen(&self) -> MCLKREQEN_R {
        MCLKREQEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SMCLK Clock Request Enable"]
    #[inline(always)]
    pub fn smclkreqen(&self) -> SMCLKREQEN_R {
        SMCLKREQEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MODOSC Clock Request Enable"]
    #[inline(always)]
    pub fn modoscreqen(&self) -> MODOSCREQEN_R {
        MODOSCREQEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACLK Clock Request Enable"]
    #[inline(always)]
    pub fn aclkreqen(&mut self) -> ACLKREQEN_W {
        ACLKREQEN_W { w: self }
    }
    #[doc = "Bit 1 - MCLK Clock Request Enable"]
    #[inline(always)]
    pub fn mclkreqen(&mut self) -> MCLKREQEN_W {
        MCLKREQEN_W { w: self }
    }
    #[doc = "Bit 2 - SMCLK Clock Request Enable"]
    #[inline(always)]
    pub fn smclkreqen(&mut self) -> SMCLKREQEN_W {
        SMCLKREQEN_W { w: self }
    }
    #[doc = "Bit 3 - MODOSC Clock Request Enable"]
    #[inline(always)]
    pub fn modoscreqen(&mut self) -> MODOSCREQEN_W {
        MODOSCREQEN_W { w: self }
    }
}
