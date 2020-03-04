#[doc = "Reader of register CBCTL3"]
pub type R = crate::R<u16, super::CBCTL3>;
#[doc = "Writer for register CBCTL3"]
pub type W = crate::W<u16, super::CBCTL3>;
#[doc = "Register CBCTL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CBCTL3 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CBPD0`"]
pub type CBPD0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBPD0`"]
pub struct CBPD0_W<'a> {
    w: &'a mut W,
}
impl<'a> CBPD0_W<'a> {
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
#[doc = "Reader of field `CBPD1`"]
pub type CBPD1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBPD1`"]
pub struct CBPD1_W<'a> {
    w: &'a mut W,
}
impl<'a> CBPD1_W<'a> {
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
#[doc = "Reader of field `CBPD2`"]
pub type CBPD2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBPD2`"]
pub struct CBPD2_W<'a> {
    w: &'a mut W,
}
impl<'a> CBPD2_W<'a> {
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
#[doc = "Reader of field `CBPD3`"]
pub type CBPD3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBPD3`"]
pub struct CBPD3_W<'a> {
    w: &'a mut W,
}
impl<'a> CBPD3_W<'a> {
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
#[doc = "Reader of field `CBPD4`"]
pub type CBPD4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBPD4`"]
pub struct CBPD4_W<'a> {
    w: &'a mut W,
}
impl<'a> CBPD4_W<'a> {
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
#[doc = "Reader of field `CBPD5`"]
pub type CBPD5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBPD5`"]
pub struct CBPD5_W<'a> {
    w: &'a mut W,
}
impl<'a> CBPD5_W<'a> {
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
#[doc = "Reader of field `CBPD6`"]
pub type CBPD6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBPD6`"]
pub struct CBPD6_W<'a> {
    w: &'a mut W,
}
impl<'a> CBPD6_W<'a> {
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
#[doc = "Reader of field `CBPD7`"]
pub type CBPD7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBPD7`"]
pub struct CBPD7_W<'a> {
    w: &'a mut W,
}
impl<'a> CBPD7_W<'a> {
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
#[doc = "Reader of field `CBPD8`"]
pub type CBPD8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBPD8`"]
pub struct CBPD8_W<'a> {
    w: &'a mut W,
}
impl<'a> CBPD8_W<'a> {
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
#[doc = "Reader of field `CBPD9`"]
pub type CBPD9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBPD9`"]
pub struct CBPD9_W<'a> {
    w: &'a mut W,
}
impl<'a> CBPD9_W<'a> {
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
#[doc = "Reader of field `CBPD10`"]
pub type CBPD10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBPD10`"]
pub struct CBPD10_W<'a> {
    w: &'a mut W,
}
impl<'a> CBPD10_W<'a> {
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
#[doc = "Reader of field `CBPD11`"]
pub type CBPD11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBPD11`"]
pub struct CBPD11_W<'a> {
    w: &'a mut W,
}
impl<'a> CBPD11_W<'a> {
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
#[doc = "Reader of field `CBPD12`"]
pub type CBPD12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBPD12`"]
pub struct CBPD12_W<'a> {
    w: &'a mut W,
}
impl<'a> CBPD12_W<'a> {
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
#[doc = "Reader of field `CBPD13`"]
pub type CBPD13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBPD13`"]
pub struct CBPD13_W<'a> {
    w: &'a mut W,
}
impl<'a> CBPD13_W<'a> {
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
#[doc = "Reader of field `CBPD14`"]
pub type CBPD14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBPD14`"]
pub struct CBPD14_W<'a> {
    w: &'a mut W,
}
impl<'a> CBPD14_W<'a> {
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
#[doc = "Reader of field `CBPD15`"]
pub type CBPD15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBPD15`"]
pub struct CBPD15_W<'a> {
    w: &'a mut W,
}
impl<'a> CBPD15_W<'a> {
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
    #[doc = "Bit 0 - Comp. B Disable Input Buffer of Port Register .0"]
    #[inline(always)]
    pub fn cbpd0(&self) -> CBPD0_R {
        CBPD0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comp. B Disable Input Buffer of Port Register .1"]
    #[inline(always)]
    pub fn cbpd1(&self) -> CBPD1_R {
        CBPD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comp. B Disable Input Buffer of Port Register .2"]
    #[inline(always)]
    pub fn cbpd2(&self) -> CBPD2_R {
        CBPD2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comp. B Disable Input Buffer of Port Register .3"]
    #[inline(always)]
    pub fn cbpd3(&self) -> CBPD3_R {
        CBPD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comp. B Disable Input Buffer of Port Register .4"]
    #[inline(always)]
    pub fn cbpd4(&self) -> CBPD4_R {
        CBPD4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Comp. B Disable Input Buffer of Port Register .5"]
    #[inline(always)]
    pub fn cbpd5(&self) -> CBPD5_R {
        CBPD5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Comp. B Disable Input Buffer of Port Register .6"]
    #[inline(always)]
    pub fn cbpd6(&self) -> CBPD6_R {
        CBPD6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Comp. B Disable Input Buffer of Port Register .7"]
    #[inline(always)]
    pub fn cbpd7(&self) -> CBPD7_R {
        CBPD7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Comp. B Disable Input Buffer of Port Register .8"]
    #[inline(always)]
    pub fn cbpd8(&self) -> CBPD8_R {
        CBPD8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Comp. B Disable Input Buffer of Port Register .9"]
    #[inline(always)]
    pub fn cbpd9(&self) -> CBPD9_R {
        CBPD9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Comp. B Disable Input Buffer of Port Register .10"]
    #[inline(always)]
    pub fn cbpd10(&self) -> CBPD10_R {
        CBPD10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Comp. B Disable Input Buffer of Port Register .11"]
    #[inline(always)]
    pub fn cbpd11(&self) -> CBPD11_R {
        CBPD11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Comp. B Disable Input Buffer of Port Register .12"]
    #[inline(always)]
    pub fn cbpd12(&self) -> CBPD12_R {
        CBPD12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Comp. B Disable Input Buffer of Port Register .13"]
    #[inline(always)]
    pub fn cbpd13(&self) -> CBPD13_R {
        CBPD13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Comp. B Disable Input Buffer of Port Register .14"]
    #[inline(always)]
    pub fn cbpd14(&self) -> CBPD14_R {
        CBPD14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Comp. B Disable Input Buffer of Port Register .15"]
    #[inline(always)]
    pub fn cbpd15(&self) -> CBPD15_R {
        CBPD15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. B Disable Input Buffer of Port Register .0"]
    #[inline(always)]
    pub fn cbpd0(&mut self) -> CBPD0_W {
        CBPD0_W { w: self }
    }
    #[doc = "Bit 1 - Comp. B Disable Input Buffer of Port Register .1"]
    #[inline(always)]
    pub fn cbpd1(&mut self) -> CBPD1_W {
        CBPD1_W { w: self }
    }
    #[doc = "Bit 2 - Comp. B Disable Input Buffer of Port Register .2"]
    #[inline(always)]
    pub fn cbpd2(&mut self) -> CBPD2_W {
        CBPD2_W { w: self }
    }
    #[doc = "Bit 3 - Comp. B Disable Input Buffer of Port Register .3"]
    #[inline(always)]
    pub fn cbpd3(&mut self) -> CBPD3_W {
        CBPD3_W { w: self }
    }
    #[doc = "Bit 4 - Comp. B Disable Input Buffer of Port Register .4"]
    #[inline(always)]
    pub fn cbpd4(&mut self) -> CBPD4_W {
        CBPD4_W { w: self }
    }
    #[doc = "Bit 5 - Comp. B Disable Input Buffer of Port Register .5"]
    #[inline(always)]
    pub fn cbpd5(&mut self) -> CBPD5_W {
        CBPD5_W { w: self }
    }
    #[doc = "Bit 6 - Comp. B Disable Input Buffer of Port Register .6"]
    #[inline(always)]
    pub fn cbpd6(&mut self) -> CBPD6_W {
        CBPD6_W { w: self }
    }
    #[doc = "Bit 7 - Comp. B Disable Input Buffer of Port Register .7"]
    #[inline(always)]
    pub fn cbpd7(&mut self) -> CBPD7_W {
        CBPD7_W { w: self }
    }
    #[doc = "Bit 8 - Comp. B Disable Input Buffer of Port Register .8"]
    #[inline(always)]
    pub fn cbpd8(&mut self) -> CBPD8_W {
        CBPD8_W { w: self }
    }
    #[doc = "Bit 9 - Comp. B Disable Input Buffer of Port Register .9"]
    #[inline(always)]
    pub fn cbpd9(&mut self) -> CBPD9_W {
        CBPD9_W { w: self }
    }
    #[doc = "Bit 10 - Comp. B Disable Input Buffer of Port Register .10"]
    #[inline(always)]
    pub fn cbpd10(&mut self) -> CBPD10_W {
        CBPD10_W { w: self }
    }
    #[doc = "Bit 11 - Comp. B Disable Input Buffer of Port Register .11"]
    #[inline(always)]
    pub fn cbpd11(&mut self) -> CBPD11_W {
        CBPD11_W { w: self }
    }
    #[doc = "Bit 12 - Comp. B Disable Input Buffer of Port Register .12"]
    #[inline(always)]
    pub fn cbpd12(&mut self) -> CBPD12_W {
        CBPD12_W { w: self }
    }
    #[doc = "Bit 13 - Comp. B Disable Input Buffer of Port Register .13"]
    #[inline(always)]
    pub fn cbpd13(&mut self) -> CBPD13_W {
        CBPD13_W { w: self }
    }
    #[doc = "Bit 14 - Comp. B Disable Input Buffer of Port Register .14"]
    #[inline(always)]
    pub fn cbpd14(&mut self) -> CBPD14_W {
        CBPD14_W { w: self }
    }
    #[doc = "Bit 15 - Comp. B Disable Input Buffer of Port Register .15"]
    #[inline(always)]
    pub fn cbpd15(&mut self) -> CBPD15_W {
        CBPD15_W { w: self }
    }
}
