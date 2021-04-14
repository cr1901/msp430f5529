#[doc = "Register `CBCTL1` reader"]
pub struct R(crate::R<CBCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CBCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CBCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CBCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CBCTL1` writer"]
pub struct W(crate::W<CBCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CBCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CBCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CBCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CBOUT` reader - Comp. B Output"]
pub struct CBOUT_R(crate::FieldReader<bool, bool>);
impl CBOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBOUT` writer - Comp. B Output"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `CBOUTPOL` reader - Comp. B Output Polarity"]
pub struct CBOUTPOL_R(crate::FieldReader<bool, bool>);
impl CBOUTPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBOUTPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBOUTPOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBOUTPOL` writer - Comp. B Output Polarity"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CBF` reader - Comp. B Enable Output Filter"]
pub struct CBF_R(crate::FieldReader<bool, bool>);
impl CBF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBF` writer - Comp. B Enable Output Filter"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CBIES` reader - Comp. B Interrupt Edge Select"]
pub struct CBIES_R(crate::FieldReader<bool, bool>);
impl CBIES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBIES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBIES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBIES` writer - Comp. B Interrupt Edge Select"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CBSHORT` reader - Comp. B Input Short"]
pub struct CBSHORT_R(crate::FieldReader<bool, bool>);
impl CBSHORT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBSHORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBSHORT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBSHORT` writer - Comp. B Input Short"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CBEX` reader - Comp. B Exchange Inputs"]
pub struct CBEX_R(crate::FieldReader<bool, bool>);
impl CBEX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBEX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBEX` writer - Comp. B Exchange Inputs"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
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
#[doc = "Field `CBFDLY` reader - Comp. B Filter delay Bit 0"]
pub struct CBFDLY_R(crate::FieldReader<u8, CBFDLY_A>);
impl CBFDLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CBFDLY_R(crate::FieldReader::new(bits))
    }
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
        **self == CBFDLY_A::CBFDLY_0
    }
    #[doc = "Checks if the value of the field is `CBFDLY_1`"]
    #[inline(always)]
    pub fn is_cbfdly_1(&self) -> bool {
        **self == CBFDLY_A::CBFDLY_1
    }
    #[doc = "Checks if the value of the field is `CBFDLY_2`"]
    #[inline(always)]
    pub fn is_cbfdly_2(&self) -> bool {
        **self == CBFDLY_A::CBFDLY_2
    }
    #[doc = "Checks if the value of the field is `CBFDLY_3`"]
    #[inline(always)]
    pub fn is_cbfdly_3(&self) -> bool {
        **self == CBFDLY_A::CBFDLY_3
    }
}
impl core::ops::Deref for CBFDLY_R {
    type Target = crate::FieldReader<u8, CBFDLY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBFDLY` writer - Comp. B Filter delay Bit 0"]
pub struct CBFDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CBFDLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBFDLY_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u16 & 0x03) << 6);
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
#[doc = "Field `CBPWRMD` reader - Comp. B Power Mode Bit 0"]
pub struct CBPWRMD_R(crate::FieldReader<u8, CBPWRMD_A>);
impl CBPWRMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CBPWRMD_R(crate::FieldReader::new(bits))
    }
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
        **self == CBPWRMD_A::CBPWRMD_0
    }
    #[doc = "Checks if the value of the field is `CBPWRMD_1`"]
    #[inline(always)]
    pub fn is_cbpwrmd_1(&self) -> bool {
        **self == CBPWRMD_A::CBPWRMD_1
    }
    #[doc = "Checks if the value of the field is `CBPWRMD_2`"]
    #[inline(always)]
    pub fn is_cbpwrmd_2(&self) -> bool {
        **self == CBPWRMD_A::CBPWRMD_2
    }
    #[doc = "Checks if the value of the field is `CBPWRMD_3`"]
    #[inline(always)]
    pub fn is_cbpwrmd_3(&self) -> bool {
        **self == CBPWRMD_A::CBPWRMD_3
    }
}
impl core::ops::Deref for CBPWRMD_R {
    type Target = crate::FieldReader<u8, CBPWRMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBPWRMD` writer - Comp. B Power Mode Bit 0"]
pub struct CBPWRMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CBPWRMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBPWRMD_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u16 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `CBON` reader - Comp. B enable"]
pub struct CBON_R(crate::FieldReader<bool, bool>);
impl CBON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBON` writer - Comp. B enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CBMRVL` reader - Comp. B CBMRV Level"]
pub struct CBMRVL_R(crate::FieldReader<bool, bool>);
impl CBMRVL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBMRVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBMRVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBMRVL` writer - Comp. B CBMRV Level"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u16 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `CBMRVS` reader - Comp. B Output selects between VREF0 or VREF1"]
pub struct CBMRVS_R(crate::FieldReader<bool, bool>);
impl CBMRVS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBMRVS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBMRVS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBMRVS` writer - Comp. B Output selects between VREF0 or VREF1"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u16 & 0x01) << 12);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator B Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbctl1](index.html) module"]
pub struct CBCTL1_SPEC;
impl crate::RegisterSpec for CBCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cbctl1::R](R) reader structure"]
impl crate::Readable for CBCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cbctl1::W](W) writer structure"]
impl crate::Writable for CBCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CBCTL1 to value 0"]
impl crate::Resettable for CBCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
