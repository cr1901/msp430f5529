#[doc = "Reader of register SFRRPCR"]
pub type R = crate::R<u16, super::SFRRPCR>;
#[doc = "Writer for register SFRRPCR"]
pub type W = crate::W<u16, super::SFRRPCR>;
#[doc = "Register SFRRPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SFRRPCR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSNMI`"]
pub type SYSNMI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSNMI`"]
pub struct SYSNMI_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSNMI_W<'a> {
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
#[doc = "Reader of field `SYSNMIIES`"]
pub type SYSNMIIES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSNMIIES`"]
pub struct SYSNMIIES_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSNMIIES_W<'a> {
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
#[doc = "Reader of field `SYSRSTUP`"]
pub type SYSRSTUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSRSTUP`"]
pub struct SYSRSTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRSTUP_W<'a> {
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
#[doc = "Reader of field `SYSRSTRE`"]
pub type SYSRSTRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSRSTRE`"]
pub struct SYSRSTRE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRSTRE_W<'a> {
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
    #[doc = "Bit 0 - NMI select"]
    #[inline(always)]
    pub fn sysnmi(&self) -> SYSNMI_R {
        SYSNMI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - NMI edge select"]
    #[inline(always)]
    pub fn sysnmiies(&self) -> SYSNMIIES_R {
        SYSNMIIES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RESET Pin pull down/up select"]
    #[inline(always)]
    pub fn sysrstup(&self) -> SYSRSTUP_R {
        SYSRSTUP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RESET Pin Resistor enable"]
    #[inline(always)]
    pub fn sysrstre(&self) -> SYSRSTRE_R {
        SYSRSTRE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NMI select"]
    #[inline(always)]
    pub fn sysnmi(&mut self) -> SYSNMI_W {
        SYSNMI_W { w: self }
    }
    #[doc = "Bit 1 - NMI edge select"]
    #[inline(always)]
    pub fn sysnmiies(&mut self) -> SYSNMIIES_W {
        SYSNMIIES_W { w: self }
    }
    #[doc = "Bit 2 - RESET Pin pull down/up select"]
    #[inline(always)]
    pub fn sysrstup(&mut self) -> SYSRSTUP_W {
        SYSRSTUP_W { w: self }
    }
    #[doc = "Bit 3 - RESET Pin Resistor enable"]
    #[inline(always)]
    pub fn sysrstre(&mut self) -> SYSRSTRE_W {
        SYSRSTRE_W { w: self }
    }
}
