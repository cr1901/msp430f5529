#[doc = "Reader of register PMMIFG"]
pub type R = crate::R<u16, super::PMMIFG>;
#[doc = "Writer for register PMMIFG"]
pub type W = crate::W<u16, super::PMMIFG>;
#[doc = "Register PMMIFG `reset()`'s with value 0"]
impl crate::ResetValue for super::PMMIFG {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SVSMLDLYIFG`"]
pub type SVSMLDLYIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVSMLDLYIFG`"]
pub struct SVSMLDLYIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSMLDLYIFG_W<'a> {
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
#[doc = "Reader of field `SVMLIFG`"]
pub type SVMLIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVMLIFG`"]
pub struct SVMLIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMLIFG_W<'a> {
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
#[doc = "Reader of field `SVMLVLRIFG`"]
pub type SVMLVLRIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVMLVLRIFG`"]
pub struct SVMLVLRIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMLVLRIFG_W<'a> {
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
#[doc = "Reader of field `SVSMHDLYIFG`"]
pub type SVSMHDLYIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVSMHDLYIFG`"]
pub struct SVSMHDLYIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSMHDLYIFG_W<'a> {
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
#[doc = "Reader of field `SVMHIFG`"]
pub type SVMHIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVMHIFG`"]
pub struct SVMHIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMHIFG_W<'a> {
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
#[doc = "Reader of field `SVMHVLRIFG`"]
pub type SVMHVLRIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVMHVLRIFG`"]
pub struct SVMHVLRIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMHVLRIFG_W<'a> {
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
#[doc = "Reader of field `PMMBORIFG`"]
pub type PMMBORIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMMBORIFG`"]
pub struct PMMBORIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMBORIFG_W<'a> {
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
#[doc = "Reader of field `PMMRSTIFG`"]
pub type PMMRSTIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMMRSTIFG`"]
pub struct PMMRSTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMRSTIFG_W<'a> {
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
#[doc = "Reader of field `PMMPORIFG`"]
pub type PMMPORIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMMPORIFG`"]
pub struct PMMPORIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMPORIFG_W<'a> {
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
#[doc = "Reader of field `SVSHIFG`"]
pub type SVSHIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVSHIFG`"]
pub struct SVSHIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSHIFG_W<'a> {
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
#[doc = "Reader of field `SVSLIFG`"]
pub type SVSLIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVSLIFG`"]
pub struct SVSLIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSLIFG_W<'a> {
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
#[doc = "Reader of field `PMMLPM5IFG`"]
pub type PMMLPM5IFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMMLPM5IFG`"]
pub struct PMMLPM5IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMLPM5IFG_W<'a> {
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
    #[doc = "Bit 0 - SVS and SVM low side Delay expired interrupt flag"]
    #[inline(always)]
    pub fn svsmldlyifg(&self) -> SVSMLDLYIFG_R {
        SVSMLDLYIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SVM low side interrupt flag"]
    #[inline(always)]
    pub fn svmlifg(&self) -> SVMLIFG_R {
        SVMLIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SVM low side Voltage Level Reached interrupt flag"]
    #[inline(always)]
    pub fn svmlvlrifg(&self) -> SVMLVLRIFG_R {
        SVMLVLRIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SVS and SVM high side Delay expired interrupt flag"]
    #[inline(always)]
    pub fn svsmhdlyifg(&self) -> SVSMHDLYIFG_R {
        SVSMHDLYIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SVM high side interrupt flag"]
    #[inline(always)]
    pub fn svmhifg(&self) -> SVMHIFG_R {
        SVMHIFG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SVM high side Voltage Level Reached interrupt flag"]
    #[inline(always)]
    pub fn svmhvlrifg(&self) -> SVMHVLRIFG_R {
        SVMHVLRIFG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PMM Software BOR interrupt flag"]
    #[inline(always)]
    pub fn pmmborifg(&self) -> PMMBORIFG_R {
        PMMBORIFG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PMM RESET pin interrupt flag"]
    #[inline(always)]
    pub fn pmmrstifg(&self) -> PMMRSTIFG_R {
        PMMRSTIFG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PMM Software POR interrupt flag"]
    #[inline(always)]
    pub fn pmmporifg(&self) -> PMMPORIFG_R {
        PMMPORIFG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SVS low side interrupt flag"]
    #[inline(always)]
    pub fn svshifg(&self) -> SVSHIFG_R {
        SVSHIFG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SVS high side interrupt flag"]
    #[inline(always)]
    pub fn svslifg(&self) -> SVSLIFG_R {
        SVSLIFG_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LPM5 indication Flag"]
    #[inline(always)]
    pub fn pmmlpm5ifg(&self) -> PMMLPM5IFG_R {
        PMMLPM5IFG_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SVS and SVM low side Delay expired interrupt flag"]
    #[inline(always)]
    pub fn svsmldlyifg(&mut self) -> SVSMLDLYIFG_W {
        SVSMLDLYIFG_W { w: self }
    }
    #[doc = "Bit 1 - SVM low side interrupt flag"]
    #[inline(always)]
    pub fn svmlifg(&mut self) -> SVMLIFG_W {
        SVMLIFG_W { w: self }
    }
    #[doc = "Bit 2 - SVM low side Voltage Level Reached interrupt flag"]
    #[inline(always)]
    pub fn svmlvlrifg(&mut self) -> SVMLVLRIFG_W {
        SVMLVLRIFG_W { w: self }
    }
    #[doc = "Bit 4 - SVS and SVM high side Delay expired interrupt flag"]
    #[inline(always)]
    pub fn svsmhdlyifg(&mut self) -> SVSMHDLYIFG_W {
        SVSMHDLYIFG_W { w: self }
    }
    #[doc = "Bit 5 - SVM high side interrupt flag"]
    #[inline(always)]
    pub fn svmhifg(&mut self) -> SVMHIFG_W {
        SVMHIFG_W { w: self }
    }
    #[doc = "Bit 6 - SVM high side Voltage Level Reached interrupt flag"]
    #[inline(always)]
    pub fn svmhvlrifg(&mut self) -> SVMHVLRIFG_W {
        SVMHVLRIFG_W { w: self }
    }
    #[doc = "Bit 8 - PMM Software BOR interrupt flag"]
    #[inline(always)]
    pub fn pmmborifg(&mut self) -> PMMBORIFG_W {
        PMMBORIFG_W { w: self }
    }
    #[doc = "Bit 9 - PMM RESET pin interrupt flag"]
    #[inline(always)]
    pub fn pmmrstifg(&mut self) -> PMMRSTIFG_W {
        PMMRSTIFG_W { w: self }
    }
    #[doc = "Bit 10 - PMM Software POR interrupt flag"]
    #[inline(always)]
    pub fn pmmporifg(&mut self) -> PMMPORIFG_W {
        PMMPORIFG_W { w: self }
    }
    #[doc = "Bit 12 - SVS low side interrupt flag"]
    #[inline(always)]
    pub fn svshifg(&mut self) -> SVSHIFG_W {
        SVSHIFG_W { w: self }
    }
    #[doc = "Bit 13 - SVS high side interrupt flag"]
    #[inline(always)]
    pub fn svslifg(&mut self) -> SVSLIFG_W {
        SVSLIFG_W { w: self }
    }
    #[doc = "Bit 15 - LPM5 indication Flag"]
    #[inline(always)]
    pub fn pmmlpm5ifg(&mut self) -> PMMLPM5IFG_W {
        PMMLPM5IFG_W { w: self }
    }
}
