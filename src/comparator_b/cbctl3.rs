#[doc = "Register `CBCTL3` reader"]
pub struct R(crate::R<CBCTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CBCTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CBCTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CBCTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CBCTL3` writer"]
pub struct W(crate::W<CBCTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CBCTL3_SPEC>;
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
impl From<crate::W<CBCTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CBCTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CBPD0` reader - Comp. B Disable Input Buffer of Port Register .0"]
pub struct CBPD0_R(crate::FieldReader<bool, bool>);
impl CBPD0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBPD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBPD0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBPD0` writer - Comp. B Disable Input Buffer of Port Register .0"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `CBPD1` reader - Comp. B Disable Input Buffer of Port Register .1"]
pub struct CBPD1_R(crate::FieldReader<bool, bool>);
impl CBPD1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBPD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBPD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBPD1` writer - Comp. B Disable Input Buffer of Port Register .1"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CBPD2` reader - Comp. B Disable Input Buffer of Port Register .2"]
pub struct CBPD2_R(crate::FieldReader<bool, bool>);
impl CBPD2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBPD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBPD2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBPD2` writer - Comp. B Disable Input Buffer of Port Register .2"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CBPD3` reader - Comp. B Disable Input Buffer of Port Register .3"]
pub struct CBPD3_R(crate::FieldReader<bool, bool>);
impl CBPD3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBPD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBPD3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBPD3` writer - Comp. B Disable Input Buffer of Port Register .3"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CBPD4` reader - Comp. B Disable Input Buffer of Port Register .4"]
pub struct CBPD4_R(crate::FieldReader<bool, bool>);
impl CBPD4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBPD4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBPD4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBPD4` writer - Comp. B Disable Input Buffer of Port Register .4"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CBPD5` reader - Comp. B Disable Input Buffer of Port Register .5"]
pub struct CBPD5_R(crate::FieldReader<bool, bool>);
impl CBPD5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBPD5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBPD5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBPD5` writer - Comp. B Disable Input Buffer of Port Register .5"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CBPD6` reader - Comp. B Disable Input Buffer of Port Register .6"]
pub struct CBPD6_R(crate::FieldReader<bool, bool>);
impl CBPD6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBPD6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBPD6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBPD6` writer - Comp. B Disable Input Buffer of Port Register .6"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CBPD7` reader - Comp. B Disable Input Buffer of Port Register .7"]
pub struct CBPD7_R(crate::FieldReader<bool, bool>);
impl CBPD7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBPD7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBPD7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBPD7` writer - Comp. B Disable Input Buffer of Port Register .7"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CBPD8` reader - Comp. B Disable Input Buffer of Port Register .8"]
pub struct CBPD8_R(crate::FieldReader<bool, bool>);
impl CBPD8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBPD8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBPD8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBPD8` writer - Comp. B Disable Input Buffer of Port Register .8"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CBPD9` reader - Comp. B Disable Input Buffer of Port Register .9"]
pub struct CBPD9_R(crate::FieldReader<bool, bool>);
impl CBPD9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBPD9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBPD9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBPD9` writer - Comp. B Disable Input Buffer of Port Register .9"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CBPD10` reader - Comp. B Disable Input Buffer of Port Register .10"]
pub struct CBPD10_R(crate::FieldReader<bool, bool>);
impl CBPD10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBPD10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBPD10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBPD10` writer - Comp. B Disable Input Buffer of Port Register .10"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CBPD11` reader - Comp. B Disable Input Buffer of Port Register .11"]
pub struct CBPD11_R(crate::FieldReader<bool, bool>);
impl CBPD11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBPD11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBPD11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBPD11` writer - Comp. B Disable Input Buffer of Port Register .11"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u16 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `CBPD12` reader - Comp. B Disable Input Buffer of Port Register .12"]
pub struct CBPD12_R(crate::FieldReader<bool, bool>);
impl CBPD12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBPD12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBPD12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBPD12` writer - Comp. B Disable Input Buffer of Port Register .12"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u16 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CBPD13` reader - Comp. B Disable Input Buffer of Port Register .13"]
pub struct CBPD13_R(crate::FieldReader<bool, bool>);
impl CBPD13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBPD13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBPD13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBPD13` writer - Comp. B Disable Input Buffer of Port Register .13"]
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u16 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `CBPD14` reader - Comp. B Disable Input Buffer of Port Register .14"]
pub struct CBPD14_R(crate::FieldReader<bool, bool>);
impl CBPD14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBPD14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBPD14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBPD14` writer - Comp. B Disable Input Buffer of Port Register .14"]
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u16 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `CBPD15` reader - Comp. B Disable Input Buffer of Port Register .15"]
pub struct CBPD15_R(crate::FieldReader<bool, bool>);
impl CBPD15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBPD15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBPD15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBPD15` writer - Comp. B Disable Input Buffer of Port Register .15"]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u16 & 0x01) << 15);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator B Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbctl3](index.html) module"]
pub struct CBCTL3_SPEC;
impl crate::RegisterSpec for CBCTL3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cbctl3::R](R) reader structure"]
impl crate::Readable for CBCTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cbctl3::W](W) writer structure"]
impl crate::Writable for CBCTL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CBCTL3 to value 0"]
impl crate::Resettable for CBCTL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
