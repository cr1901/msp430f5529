#[doc = "Reader of register UCSCTL7"]
pub type R = crate::R<u16, super::UCSCTL7>;
#[doc = "Writer for register UCSCTL7"]
pub type W = crate::W<u16, super::UCSCTL7>;
#[doc = "Register UCSCTL7 `reset()`'s with value 0"]
impl crate::ResetValue for super::UCSCTL7 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCOFFG`"]
pub type DCOFFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCOFFG`"]
pub struct DCOFFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOFFG_W<'a> {
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
#[doc = "Reader of field `XT1LFOFFG`"]
pub type XT1LFOFFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XT1LFOFFG`"]
pub struct XT1LFOFFG_W<'a> {
    w: &'a mut W,
}
impl<'a> XT1LFOFFG_W<'a> {
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
#[doc = "Reader of field `XT2OFFG`"]
pub type XT2OFFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XT2OFFG`"]
pub struct XT2OFFG_W<'a> {
    w: &'a mut W,
}
impl<'a> XT2OFFG_W<'a> {
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
    #[doc = "Bit 0 - DCO Fault Flag"]
    #[inline(always)]
    pub fn dcoffg(&self) -> DCOFFG_R {
        DCOFFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XT1 Low Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn xt1lfoffg(&self) -> XT1LFOFFG_R {
        XT1LFOFFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - High Frequency Oscillator 2 Fault Flag"]
    #[inline(always)]
    pub fn xt2offg(&self) -> XT2OFFG_R {
        XT2OFFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCO Fault Flag"]
    #[inline(always)]
    pub fn dcoffg(&mut self) -> DCOFFG_W {
        DCOFFG_W { w: self }
    }
    #[doc = "Bit 1 - XT1 Low Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn xt1lfoffg(&mut self) -> XT1LFOFFG_W {
        XT1LFOFFG_W { w: self }
    }
    #[doc = "Bit 3 - High Frequency Oscillator 2 Fault Flag"]
    #[inline(always)]
    pub fn xt2offg(&mut self) -> XT2OFFG_W {
        XT2OFFG_W { w: self }
    }
}
