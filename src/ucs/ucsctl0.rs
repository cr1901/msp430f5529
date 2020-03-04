#[doc = "Reader of register UCSCTL0"]
pub type R = crate::R<u16, super::UCSCTL0>;
#[doc = "Writer for register UCSCTL0"]
pub type W = crate::W<u16, super::UCSCTL0>;
#[doc = "Register UCSCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::UCSCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MOD0`"]
pub type MOD0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOD0`"]
pub struct MOD0_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD0_W<'a> {
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
#[doc = "Reader of field `MOD1`"]
pub type MOD1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOD1`"]
pub struct MOD1_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD1_W<'a> {
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
#[doc = "Reader of field `MOD2`"]
pub type MOD2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOD2`"]
pub struct MOD2_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD2_W<'a> {
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
#[doc = "Reader of field `MOD3`"]
pub type MOD3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOD3`"]
pub struct MOD3_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD3_W<'a> {
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
#[doc = "Reader of field `MOD4`"]
pub type MOD4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOD4`"]
pub struct MOD4_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD4_W<'a> {
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
#[doc = "Reader of field `DCO0`"]
pub type DCO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCO0`"]
pub struct DCO0_W<'a> {
    w: &'a mut W,
}
impl<'a> DCO0_W<'a> {
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
#[doc = "Reader of field `DCO1`"]
pub type DCO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCO1`"]
pub struct DCO1_W<'a> {
    w: &'a mut W,
}
impl<'a> DCO1_W<'a> {
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
#[doc = "Reader of field `DCO2`"]
pub type DCO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCO2`"]
pub struct DCO2_W<'a> {
    w: &'a mut W,
}
impl<'a> DCO2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `DCO3`"]
pub type DCO3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCO3`"]
pub struct DCO3_W<'a> {
    w: &'a mut W,
}
impl<'a> DCO3_W<'a> {
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
#[doc = "Reader of field `DCO4`"]
pub type DCO4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCO4`"]
pub struct DCO4_W<'a> {
    w: &'a mut W,
}
impl<'a> DCO4_W<'a> {
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
    #[doc = "Bit 3 - Modulation Bit Counter Bit : 0"]
    #[inline(always)]
    pub fn mod0(&self) -> MOD0_R {
        MOD0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Modulation Bit Counter Bit : 1"]
    #[inline(always)]
    pub fn mod1(&self) -> MOD1_R {
        MOD1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Modulation Bit Counter Bit : 2"]
    #[inline(always)]
    pub fn mod2(&self) -> MOD2_R {
        MOD2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Modulation Bit Counter Bit : 3"]
    #[inline(always)]
    pub fn mod3(&self) -> MOD3_R {
        MOD3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Modulation Bit Counter Bit : 4"]
    #[inline(always)]
    pub fn mod4(&self) -> MOD4_R {
        MOD4_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DCO TAP Bit : 0"]
    #[inline(always)]
    pub fn dco0(&self) -> DCO0_R {
        DCO0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DCO TAP Bit : 1"]
    #[inline(always)]
    pub fn dco1(&self) -> DCO1_R {
        DCO1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DCO TAP Bit : 2"]
    #[inline(always)]
    pub fn dco2(&self) -> DCO2_R {
        DCO2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DCO TAP Bit : 3"]
    #[inline(always)]
    pub fn dco3(&self) -> DCO3_R {
        DCO3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DCO TAP Bit : 4"]
    #[inline(always)]
    pub fn dco4(&self) -> DCO4_R {
        DCO4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Modulation Bit Counter Bit : 0"]
    #[inline(always)]
    pub fn mod0(&mut self) -> MOD0_W {
        MOD0_W { w: self }
    }
    #[doc = "Bit 4 - Modulation Bit Counter Bit : 1"]
    #[inline(always)]
    pub fn mod1(&mut self) -> MOD1_W {
        MOD1_W { w: self }
    }
    #[doc = "Bit 5 - Modulation Bit Counter Bit : 2"]
    #[inline(always)]
    pub fn mod2(&mut self) -> MOD2_W {
        MOD2_W { w: self }
    }
    #[doc = "Bit 6 - Modulation Bit Counter Bit : 3"]
    #[inline(always)]
    pub fn mod3(&mut self) -> MOD3_W {
        MOD3_W { w: self }
    }
    #[doc = "Bit 7 - Modulation Bit Counter Bit : 4"]
    #[inline(always)]
    pub fn mod4(&mut self) -> MOD4_W {
        MOD4_W { w: self }
    }
    #[doc = "Bit 8 - DCO TAP Bit : 0"]
    #[inline(always)]
    pub fn dco0(&mut self) -> DCO0_W {
        DCO0_W { w: self }
    }
    #[doc = "Bit 9 - DCO TAP Bit : 1"]
    #[inline(always)]
    pub fn dco1(&mut self) -> DCO1_W {
        DCO1_W { w: self }
    }
    #[doc = "Bit 10 - DCO TAP Bit : 2"]
    #[inline(always)]
    pub fn dco2(&mut self) -> DCO2_W {
        DCO2_W { w: self }
    }
    #[doc = "Bit 11 - DCO TAP Bit : 3"]
    #[inline(always)]
    pub fn dco3(&mut self) -> DCO3_W {
        DCO3_W { w: self }
    }
    #[doc = "Bit 12 - DCO TAP Bit : 4"]
    #[inline(always)]
    pub fn dco4(&mut self) -> DCO4_W {
        DCO4_W { w: self }
    }
}
