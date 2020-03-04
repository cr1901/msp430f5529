#[doc = "Reader of register SYSBSLC"]
pub type R = crate::R<u16, super::SYSBSLC>;
#[doc = "Writer for register SYSBSLC"]
pub type W = crate::W<u16, super::SYSBSLC>;
#[doc = "Register SYSBSLC `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSBSLC {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSBSLSIZE0`"]
pub type SYSBSLSIZE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSBSLSIZE0`"]
pub struct SYSBSLSIZE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSBSLSIZE0_W<'a> {
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
#[doc = "Reader of field `SYSBSLSIZE1`"]
pub type SYSBSLSIZE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSBSLSIZE1`"]
pub struct SYSBSLSIZE1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSBSLSIZE1_W<'a> {
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
#[doc = "Reader of field `SYSBSLR`"]
pub type SYSBSLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSBSLR`"]
pub struct SYSBSLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSBSLR_W<'a> {
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
#[doc = "Reader of field `SYSBSLOFF`"]
pub type SYSBSLOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSBSLOFF`"]
pub struct SYSBSLOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSBSLOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `SYSBSLPE`"]
pub type SYSBSLPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSBSLPE`"]
pub struct SYSBSLPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSBSLPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SYS - BSL Protection Size 0"]
    #[inline(always)]
    pub fn sysbslsize0(&self) -> SYSBSLSIZE0_R {
        SYSBSLSIZE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SYS - BSL Protection Size 1"]
    #[inline(always)]
    pub fn sysbslsize1(&self) -> SYSBSLSIZE1_R {
        SYSBSLSIZE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SYS - RAM assigned to BSL"]
    #[inline(always)]
    pub fn sysbslr(&self) -> SYSBSLR_R {
        SYSBSLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SYS - BSL Memory disabled"]
    #[inline(always)]
    pub fn sysbsloff(&self) -> SYSBSLOFF_R {
        SYSBSLOFF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SYS - BSL Memory protection enabled"]
    #[inline(always)]
    pub fn sysbslpe(&self) -> SYSBSLPE_R {
        SYSBSLPE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYS - BSL Protection Size 0"]
    #[inline(always)]
    pub fn sysbslsize0(&mut self) -> SYSBSLSIZE0_W {
        SYSBSLSIZE0_W { w: self }
    }
    #[doc = "Bit 1 - SYS - BSL Protection Size 1"]
    #[inline(always)]
    pub fn sysbslsize1(&mut self) -> SYSBSLSIZE1_W {
        SYSBSLSIZE1_W { w: self }
    }
    #[doc = "Bit 2 - SYS - RAM assigned to BSL"]
    #[inline(always)]
    pub fn sysbslr(&mut self) -> SYSBSLR_W {
        SYSBSLR_W { w: self }
    }
    #[doc = "Bit 14 - SYS - BSL Memory disabled"]
    #[inline(always)]
    pub fn sysbsloff(&mut self) -> SYSBSLOFF_W {
        SYSBSLOFF_W { w: self }
    }
    #[doc = "Bit 15 - SYS - BSL Memory protection enabled"]
    #[inline(always)]
    pub fn sysbslpe(&mut self) -> SYSBSLPE_W {
        SYSBSLPE_W { w: self }
    }
}
