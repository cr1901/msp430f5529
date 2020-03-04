#[doc = "Reader of register CBCTL1"]
pub type R = crate::R<u16, super::CBCTL1>;
#[doc = "Writer for register CBCTL1"]
pub type W = crate::W<u16, super::CBCTL1>;
#[doc = "Register CBCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CBCTL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CBOUT`"]
pub type CBOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBOUT`"]
pub struct CBOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CBOUT_W<'a> {
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
#[doc = "Reader of field `CBOUTPOL`"]
pub type CBOUTPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBOUTPOL`"]
pub struct CBOUTPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CBOUTPOL_W<'a> {
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
#[doc = "Reader of field `CBF`"]
pub type CBF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBF`"]
pub struct CBF_W<'a> {
    w: &'a mut W,
}
impl<'a> CBF_W<'a> {
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
#[doc = "Reader of field `CBIES`"]
pub type CBIES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBIES`"]
pub struct CBIES_W<'a> {
    w: &'a mut W,
}
impl<'a> CBIES_W<'a> {
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
#[doc = "Reader of field `CBSHORT`"]
pub type CBSHORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBSHORT`"]
pub struct CBSHORT_W<'a> {
    w: &'a mut W,
}
impl<'a> CBSHORT_W<'a> {
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
#[doc = "Reader of field `CBEX`"]
pub type CBEX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBEX`"]
pub struct CBEX_W<'a> {
    w: &'a mut W,
}
impl<'a> CBEX_W<'a> {
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
#[doc = "Comp. B Filter delay Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CBFDLY_A {
    #[doc = "0: Comp. B Filter delay 0 : 450ns"]
    CBFDLY_0 = 0,
    #[doc = "1: Comp. B Filter delay 1 : 900ns"]
    CBFDLY_1 = 1,
    #[doc = "2: Comp. B Filter delay 2 : 1800ns"]
    CBFDLY_2 = 2,
    #[doc = "3: Comp. B Filter delay 3 : 3600ns"]
    CBFDLY_3 = 3,
}
impl From<CBFDLY_A> for u8 {
    #[inline(always)]
    fn from(variant: CBFDLY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CBFDLY`"]
pub type CBFDLY_R = crate::R<u8, CBFDLY_A>;
impl CBFDLY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBFDLY_A {
        match self.bits {
            0 => CBFDLY_A::CBFDLY_0,
            1 => CBFDLY_A::CBFDLY_1,
            2 => CBFDLY_A::CBFDLY_2,
            3 => CBFDLY_A::CBFDLY_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CBFDLY_0`"]
    #[inline(always)]
    pub fn is_cbfdly_0(&self) -> bool {
        *self == CBFDLY_A::CBFDLY_0
    }
    #[doc = "Checks if the value of the field is `CBFDLY_1`"]
    #[inline(always)]
    pub fn is_cbfdly_1(&self) -> bool {
        *self == CBFDLY_A::CBFDLY_1
    }
    #[doc = "Checks if the value of the field is `CBFDLY_2`"]
    #[inline(always)]
    pub fn is_cbfdly_2(&self) -> bool {
        *self == CBFDLY_A::CBFDLY_2
    }
    #[doc = "Checks if the value of the field is `CBFDLY_3`"]
    #[inline(always)]
    pub fn is_cbfdly_3(&self) -> bool {
        *self == CBFDLY_A::CBFDLY_3
    }
}
#[doc = "Write proxy for field `CBFDLY`"]
pub struct CBFDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CBFDLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBFDLY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Comp. B Filter delay 0 : 450ns"]
    #[inline(always)]
    pub fn cbfdly_0(self) -> &'a mut W {
        self.variant(CBFDLY_A::CBFDLY_0)
    }
    #[doc = "Comp. B Filter delay 1 : 900ns"]
    #[inline(always)]
    pub fn cbfdly_1(self) -> &'a mut W {
        self.variant(CBFDLY_A::CBFDLY_1)
    }
    #[doc = "Comp. B Filter delay 2 : 1800ns"]
    #[inline(always)]
    pub fn cbfdly_2(self) -> &'a mut W {
        self.variant(CBFDLY_A::CBFDLY_2)
    }
    #[doc = "Comp. B Filter delay 3 : 3600ns"]
    #[inline(always)]
    pub fn cbfdly_3(self) -> &'a mut W {
        self.variant(CBFDLY_A::CBFDLY_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Comp. B Power Mode Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CBPWRMD_A {
    #[doc = "0: Comp. B Power Mode 0 : High speed"]
    CBPWRMD_0 = 0,
    #[doc = "1: Comp. B Power Mode 1 : Normal"]
    CBPWRMD_1 = 1,
    #[doc = "2: Comp. B Power Mode 2 : Ultra-Low"]
    CBPWRMD_2 = 2,
    #[doc = "3: Comp. B Power Mode 3 : Reserved"]
    CBPWRMD_3 = 3,
}
impl From<CBPWRMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CBPWRMD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CBPWRMD`"]
pub type CBPWRMD_R = crate::R<u8, CBPWRMD_A>;
impl CBPWRMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBPWRMD_A {
        match self.bits {
            0 => CBPWRMD_A::CBPWRMD_0,
            1 => CBPWRMD_A::CBPWRMD_1,
            2 => CBPWRMD_A::CBPWRMD_2,
            3 => CBPWRMD_A::CBPWRMD_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CBPWRMD_0`"]
    #[inline(always)]
    pub fn is_cbpwrmd_0(&self) -> bool {
        *self == CBPWRMD_A::CBPWRMD_0
    }
    #[doc = "Checks if the value of the field is `CBPWRMD_1`"]
    #[inline(always)]
    pub fn is_cbpwrmd_1(&self) -> bool {
        *self == CBPWRMD_A::CBPWRMD_1
    }
    #[doc = "Checks if the value of the field is `CBPWRMD_2`"]
    #[inline(always)]
    pub fn is_cbpwrmd_2(&self) -> bool {
        *self == CBPWRMD_A::CBPWRMD_2
    }
    #[doc = "Checks if the value of the field is `CBPWRMD_3`"]
    #[inline(always)]
    pub fn is_cbpwrmd_3(&self) -> bool {
        *self == CBPWRMD_A::CBPWRMD_3
    }
}
#[doc = "Write proxy for field `CBPWRMD`"]
pub struct CBPWRMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CBPWRMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBPWRMD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Comp. B Power Mode 0 : High speed"]
    #[inline(always)]
    pub fn cbpwrmd_0(self) -> &'a mut W {
        self.variant(CBPWRMD_A::CBPWRMD_0)
    }
    #[doc = "Comp. B Power Mode 1 : Normal"]
    #[inline(always)]
    pub fn cbpwrmd_1(self) -> &'a mut W {
        self.variant(CBPWRMD_A::CBPWRMD_1)
    }
    #[doc = "Comp. B Power Mode 2 : Ultra-Low"]
    #[inline(always)]
    pub fn cbpwrmd_2(self) -> &'a mut W {
        self.variant(CBPWRMD_A::CBPWRMD_2)
    }
    #[doc = "Comp. B Power Mode 3 : Reserved"]
    #[inline(always)]
    pub fn cbpwrmd_3(self) -> &'a mut W {
        self.variant(CBPWRMD_A::CBPWRMD_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `CBON`"]
pub type CBON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBON`"]
pub struct CBON_W<'a> {
    w: &'a mut W,
}
impl<'a> CBON_W<'a> {
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
#[doc = "Reader of field `CBMRVL`"]
pub type CBMRVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBMRVL`"]
pub struct CBMRVL_W<'a> {
    w: &'a mut W,
}
impl<'a> CBMRVL_W<'a> {
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
#[doc = "Reader of field `CBMRVS`"]
pub type CBMRVS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBMRVS`"]
pub struct CBMRVS_W<'a> {
    w: &'a mut W,
}
impl<'a> CBMRVS_W<'a> {
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
    #[doc = "Bit 0 - Comp. B Output"]
    #[inline(always)]
    pub fn cbout(&self) -> CBOUT_R {
        CBOUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comp. B Output Polarity"]
    #[inline(always)]
    pub fn cboutpol(&self) -> CBOUTPOL_R {
        CBOUTPOL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comp. B Enable Output Filter"]
    #[inline(always)]
    pub fn cbf(&self) -> CBF_R {
        CBF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comp. B Interrupt Edge Select"]
    #[inline(always)]
    pub fn cbies(&self) -> CBIES_R {
        CBIES_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comp. B Input Short"]
    #[inline(always)]
    pub fn cbshort(&self) -> CBSHORT_R {
        CBSHORT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Comp. B Exchange Inputs"]
    #[inline(always)]
    pub fn cbex(&self) -> CBEX_R {
        CBEX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Comp. B Filter delay Bit 0"]
    #[inline(always)]
    pub fn cbfdly(&self) -> CBFDLY_R {
        CBFDLY_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Comp. B Power Mode Bit 0"]
    #[inline(always)]
    pub fn cbpwrmd(&self) -> CBPWRMD_R {
        CBPWRMD_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Comp. B enable"]
    #[inline(always)]
    pub fn cbon(&self) -> CBON_R {
        CBON_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Comp. B CBMRV Level"]
    #[inline(always)]
    pub fn cbmrvl(&self) -> CBMRVL_R {
        CBMRVL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Comp. B Output selects between VREF0 or VREF1"]
    #[inline(always)]
    pub fn cbmrvs(&self) -> CBMRVS_R {
        CBMRVS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. B Output"]
    #[inline(always)]
    pub fn cbout(&mut self) -> CBOUT_W {
        CBOUT_W { w: self }
    }
    #[doc = "Bit 1 - Comp. B Output Polarity"]
    #[inline(always)]
    pub fn cboutpol(&mut self) -> CBOUTPOL_W {
        CBOUTPOL_W { w: self }
    }
    #[doc = "Bit 2 - Comp. B Enable Output Filter"]
    #[inline(always)]
    pub fn cbf(&mut self) -> CBF_W {
        CBF_W { w: self }
    }
    #[doc = "Bit 3 - Comp. B Interrupt Edge Select"]
    #[inline(always)]
    pub fn cbies(&mut self) -> CBIES_W {
        CBIES_W { w: self }
    }
    #[doc = "Bit 4 - Comp. B Input Short"]
    #[inline(always)]
    pub fn cbshort(&mut self) -> CBSHORT_W {
        CBSHORT_W { w: self }
    }
    #[doc = "Bit 5 - Comp. B Exchange Inputs"]
    #[inline(always)]
    pub fn cbex(&mut self) -> CBEX_W {
        CBEX_W { w: self }
    }
    #[doc = "Bits 6:7 - Comp. B Filter delay Bit 0"]
    #[inline(always)]
    pub fn cbfdly(&mut self) -> CBFDLY_W {
        CBFDLY_W { w: self }
    }
    #[doc = "Bits 8:9 - Comp. B Power Mode Bit 0"]
    #[inline(always)]
    pub fn cbpwrmd(&mut self) -> CBPWRMD_W {
        CBPWRMD_W { w: self }
    }
    #[doc = "Bit 10 - Comp. B enable"]
    #[inline(always)]
    pub fn cbon(&mut self) -> CBON_W {
        CBON_W { w: self }
    }
    #[doc = "Bit 11 - Comp. B CBMRV Level"]
    #[inline(always)]
    pub fn cbmrvl(&mut self) -> CBMRVL_W {
        CBMRVL_W { w: self }
    }
    #[doc = "Bit 12 - Comp. B Output selects between VREF0 or VREF1"]
    #[inline(always)]
    pub fn cbmrvs(&mut self) -> CBMRVS_W {
        CBMRVS_W { w: self }
    }
}
