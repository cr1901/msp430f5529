#[doc = "Reader of register USBPHYCTL"]
pub type R = crate::R<u16, super::USBPHYCTL>;
#[doc = "Writer for register USBPHYCTL"]
pub type W = crate::W<u16, super::USBPHYCTL>;
#[doc = "Register USBPHYCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::USBPHYCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PUOUT0`"]
pub type PUOUT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUOUT0`"]
pub struct PUOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PUOUT0_W<'a> {
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
#[doc = "Reader of field `PUOUT1`"]
pub type PUOUT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUOUT1`"]
pub struct PUOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PUOUT1_W<'a> {
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
#[doc = "Reader of field `PUIN0`"]
pub type PUIN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUIN0`"]
pub struct PUIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PUIN0_W<'a> {
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
#[doc = "Reader of field `PUIN1`"]
pub type PUIN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUIN1`"]
pub struct PUIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PUIN1_W<'a> {
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
#[doc = "Reader of field `PUOPE`"]
pub type PUOPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUOPE`"]
pub struct PUOPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PUOPE_W<'a> {
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
#[doc = "Reader of field `PUSEL`"]
pub type PUSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUSEL`"]
pub struct PUSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PUSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PUIPE`"]
pub type PUIPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUIPE`"]
pub struct PUIPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PUIPE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - USB - USB Port Output Signal Bit 0"]
    #[inline(always)]
    pub fn puout0(&self) -> PUOUT0_R {
        PUOUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB - USB Port Output Signal Bit 1"]
    #[inline(always)]
    pub fn puout1(&self) -> PUOUT1_R {
        PUOUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB - PU0/DP Input Data"]
    #[inline(always)]
    pub fn puin0(&self) -> PUIN0_R {
        PUIN0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB - PU1/DM Input Data"]
    #[inline(always)]
    pub fn puin1(&self) -> PUIN1_R {
        PUIN1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB - USB Port Output Enable"]
    #[inline(always)]
    pub fn puope(&self) -> PUOPE_R {
        PUOPE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB - USB Port Function Select"]
    #[inline(always)]
    pub fn pusel(&self) -> PUSEL_R {
        PUSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USB - PHY Single Ended Input enable"]
    #[inline(always)]
    pub fn puipe(&self) -> PUIPE_R {
        PUIPE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - USB Port Output Signal Bit 0"]
    #[inline(always)]
    pub fn puout0(&mut self) -> PUOUT0_W {
        PUOUT0_W { w: self }
    }
    #[doc = "Bit 1 - USB - USB Port Output Signal Bit 1"]
    #[inline(always)]
    pub fn puout1(&mut self) -> PUOUT1_W {
        PUOUT1_W { w: self }
    }
    #[doc = "Bit 2 - USB - PU0/DP Input Data"]
    #[inline(always)]
    pub fn puin0(&mut self) -> PUIN0_W {
        PUIN0_W { w: self }
    }
    #[doc = "Bit 3 - USB - PU1/DM Input Data"]
    #[inline(always)]
    pub fn puin1(&mut self) -> PUIN1_W {
        PUIN1_W { w: self }
    }
    #[doc = "Bit 5 - USB - USB Port Output Enable"]
    #[inline(always)]
    pub fn puope(&mut self) -> PUOPE_W {
        PUOPE_W { w: self }
    }
    #[doc = "Bit 7 - USB - USB Port Function Select"]
    #[inline(always)]
    pub fn pusel(&mut self) -> PUSEL_W {
        PUSEL_W { w: self }
    }
    #[doc = "Bit 8 - USB - PHY Single Ended Input enable"]
    #[inline(always)]
    pub fn puipe(&mut self) -> PUIPE_W {
        PUIPE_W { w: self }
    }
}
