#[doc = "Reader of register SYSCTL"]
pub type R = crate::R<u16, super::SYSCTL>;
#[doc = "Writer for register SYSCTL"]
pub type W = crate::W<u16, super::SYSCTL>;
#[doc = "Register SYSCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSRIVECT`"]
pub type SYSRIVECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSRIVECT`"]
pub struct SYSRIVECT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRIVECT_W<'a> {
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
#[doc = "Reader of field `SYSPMMPE`"]
pub type SYSPMMPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSPMMPE`"]
pub struct SYSPMMPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSPMMPE_W<'a> {
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
#[doc = "Reader of field `SYSBSLIND`"]
pub type SYSBSLIND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSBSLIND`"]
pub struct SYSBSLIND_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSBSLIND_W<'a> {
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
#[doc = "Reader of field `SYSJTAGPIN`"]
pub type SYSJTAGPIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSJTAGPIN`"]
pub struct SYSJTAGPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSJTAGPIN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - SYS - RAM based interrupt vectors"]
    #[inline(always)]
    pub fn sysrivect(&self) -> SYSRIVECT_R {
        SYSRIVECT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - SYS - PMM access protect"]
    #[inline(always)]
    pub fn syspmmpe(&self) -> SYSPMMPE_R {
        SYSPMMPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SYS - TCK/RST indication detected"]
    #[inline(always)]
    pub fn sysbslind(&self) -> SYSBSLIND_R {
        SYSBSLIND_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SYS - Dedicated JTAG pins enabled"]
    #[inline(always)]
    pub fn sysjtagpin(&self) -> SYSJTAGPIN_R {
        SYSJTAGPIN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYS - RAM based interrupt vectors"]
    #[inline(always)]
    pub fn sysrivect(&mut self) -> SYSRIVECT_W {
        SYSRIVECT_W { w: self }
    }
    #[doc = "Bit 2 - SYS - PMM access protect"]
    #[inline(always)]
    pub fn syspmmpe(&mut self) -> SYSPMMPE_W {
        SYSPMMPE_W { w: self }
    }
    #[doc = "Bit 4 - SYS - TCK/RST indication detected"]
    #[inline(always)]
    pub fn sysbslind(&mut self) -> SYSBSLIND_W {
        SYSBSLIND_W { w: self }
    }
    #[doc = "Bit 5 - SYS - Dedicated JTAG pins enabled"]
    #[inline(always)]
    pub fn sysjtagpin(&mut self) -> SYSJTAGPIN_W {
        SYSJTAGPIN_W { w: self }
    }
}
