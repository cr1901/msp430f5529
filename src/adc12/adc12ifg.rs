#[doc = "Register `ADC12IFG` reader"]
pub struct R(crate::R<ADC12IFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC12IFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC12IFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC12IFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC12IFG` writer"]
pub struct W(crate::W<ADC12IFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC12IFG_SPEC>;
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
impl From<crate::W<ADC12IFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC12IFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC12IFG0` reader - ADC12 Memory 0 Interrupt Flag"]
pub struct ADC12IFG0_R(crate::FieldReader<bool, bool>);
impl ADC12IFG0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG0` writer - ADC12 Memory 0 Interrupt Flag"]
pub struct ADC12IFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG0_W<'a> {
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
#[doc = "Field `ADC12IFG1` reader - ADC12 Memory 1 Interrupt Flag"]
pub struct ADC12IFG1_R(crate::FieldReader<bool, bool>);
impl ADC12IFG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG1` writer - ADC12 Memory 1 Interrupt Flag"]
pub struct ADC12IFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG1_W<'a> {
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
#[doc = "Field `ADC12IFG2` reader - ADC12 Memory 2 Interrupt Flag"]
pub struct ADC12IFG2_R(crate::FieldReader<bool, bool>);
impl ADC12IFG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG2` writer - ADC12 Memory 2 Interrupt Flag"]
pub struct ADC12IFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG2_W<'a> {
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
#[doc = "Field `ADC12IFG3` reader - ADC12 Memory 3 Interrupt Flag"]
pub struct ADC12IFG3_R(crate::FieldReader<bool, bool>);
impl ADC12IFG3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG3` writer - ADC12 Memory 3 Interrupt Flag"]
pub struct ADC12IFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG3_W<'a> {
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
#[doc = "Field `ADC12IFG4` reader - ADC12 Memory 4 Interrupt Flag"]
pub struct ADC12IFG4_R(crate::FieldReader<bool, bool>);
impl ADC12IFG4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG4` writer - ADC12 Memory 4 Interrupt Flag"]
pub struct ADC12IFG4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG4_W<'a> {
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
#[doc = "Field `ADC12IFG5` reader - ADC12 Memory 5 Interrupt Flag"]
pub struct ADC12IFG5_R(crate::FieldReader<bool, bool>);
impl ADC12IFG5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG5` writer - ADC12 Memory 5 Interrupt Flag"]
pub struct ADC12IFG5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG5_W<'a> {
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
#[doc = "Field `ADC12IFG6` reader - ADC12 Memory 6 Interrupt Flag"]
pub struct ADC12IFG6_R(crate::FieldReader<bool, bool>);
impl ADC12IFG6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG6` writer - ADC12 Memory 6 Interrupt Flag"]
pub struct ADC12IFG6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG6_W<'a> {
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
#[doc = "Field `ADC12IFG7` reader - ADC12 Memory 7 Interrupt Flag"]
pub struct ADC12IFG7_R(crate::FieldReader<bool, bool>);
impl ADC12IFG7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG7` writer - ADC12 Memory 7 Interrupt Flag"]
pub struct ADC12IFG7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG7_W<'a> {
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
#[doc = "Field `ADC12IFG8` reader - ADC12 Memory 8 Interrupt Flag"]
pub struct ADC12IFG8_R(crate::FieldReader<bool, bool>);
impl ADC12IFG8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG8` writer - ADC12 Memory 8 Interrupt Flag"]
pub struct ADC12IFG8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG8_W<'a> {
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
#[doc = "Field `ADC12IFG9` reader - ADC12 Memory 9 Interrupt Flag"]
pub struct ADC12IFG9_R(crate::FieldReader<bool, bool>);
impl ADC12IFG9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG9` writer - ADC12 Memory 9 Interrupt Flag"]
pub struct ADC12IFG9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG9_W<'a> {
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
#[doc = "Field `ADC12IFG10` reader - ADC12 Memory 10 Interrupt Flag"]
pub struct ADC12IFG10_R(crate::FieldReader<bool, bool>);
impl ADC12IFG10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG10` writer - ADC12 Memory 10 Interrupt Flag"]
pub struct ADC12IFG10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG10_W<'a> {
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
#[doc = "Field `ADC12IFG11` reader - ADC12 Memory 11 Interrupt Flag"]
pub struct ADC12IFG11_R(crate::FieldReader<bool, bool>);
impl ADC12IFG11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG11` writer - ADC12 Memory 11 Interrupt Flag"]
pub struct ADC12IFG11_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG11_W<'a> {
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
#[doc = "Field `ADC12IFG12` reader - ADC12 Memory 12 Interrupt Flag"]
pub struct ADC12IFG12_R(crate::FieldReader<bool, bool>);
impl ADC12IFG12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG12` writer - ADC12 Memory 12 Interrupt Flag"]
pub struct ADC12IFG12_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG12_W<'a> {
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
#[doc = "Field `ADC12IFG13` reader - ADC12 Memory 13 Interrupt Flag"]
pub struct ADC12IFG13_R(crate::FieldReader<bool, bool>);
impl ADC12IFG13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG13` writer - ADC12 Memory 13 Interrupt Flag"]
pub struct ADC12IFG13_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG13_W<'a> {
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
#[doc = "Field `ADC12IFG14` reader - ADC12 Memory 14 Interrupt Flag"]
pub struct ADC12IFG14_R(crate::FieldReader<bool, bool>);
impl ADC12IFG14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG14` writer - ADC12 Memory 14 Interrupt Flag"]
pub struct ADC12IFG14_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG14_W<'a> {
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
#[doc = "Field `ADC12IFG15` reader - ADC12 Memory 15 Interrupt Flag"]
pub struct ADC12IFG15_R(crate::FieldReader<bool, bool>);
impl ADC12IFG15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG15` writer - ADC12 Memory 15 Interrupt Flag"]
pub struct ADC12IFG15_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG15_W<'a> {
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
    #[doc = "Bit 0 - ADC12 Memory 0 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg0(&self) -> ADC12IFG0_R {
        ADC12IFG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC12 Memory 1 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg1(&self) -> ADC12IFG1_R {
        ADC12IFG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC12 Memory 2 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg2(&self) -> ADC12IFG2_R {
        ADC12IFG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC12 Memory 3 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg3(&self) -> ADC12IFG3_R {
        ADC12IFG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC12 Memory 4 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg4(&self) -> ADC12IFG4_R {
        ADC12IFG4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC12 Memory 5 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg5(&self) -> ADC12IFG5_R {
        ADC12IFG5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC12 Memory 6 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg6(&self) -> ADC12IFG6_R {
        ADC12IFG6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC12 Memory 7 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg7(&self) -> ADC12IFG7_R {
        ADC12IFG7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC12 Memory 8 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg8(&self) -> ADC12IFG8_R {
        ADC12IFG8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC12 Memory 9 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg9(&self) -> ADC12IFG9_R {
        ADC12IFG9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ADC12 Memory 10 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg10(&self) -> ADC12IFG10_R {
        ADC12IFG10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADC12 Memory 11 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg11(&self) -> ADC12IFG11_R {
        ADC12IFG11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ADC12 Memory 12 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg12(&self) -> ADC12IFG12_R {
        ADC12IFG12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ADC12 Memory 13 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg13(&self) -> ADC12IFG13_R {
        ADC12IFG13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ADC12 Memory 14 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg14(&self) -> ADC12IFG14_R {
        ADC12IFG14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADC12 Memory 15 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg15(&self) -> ADC12IFG15_R {
        ADC12IFG15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12 Memory 0 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg0(&mut self) -> ADC12IFG0_W {
        ADC12IFG0_W { w: self }
    }
    #[doc = "Bit 1 - ADC12 Memory 1 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg1(&mut self) -> ADC12IFG1_W {
        ADC12IFG1_W { w: self }
    }
    #[doc = "Bit 2 - ADC12 Memory 2 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg2(&mut self) -> ADC12IFG2_W {
        ADC12IFG2_W { w: self }
    }
    #[doc = "Bit 3 - ADC12 Memory 3 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg3(&mut self) -> ADC12IFG3_W {
        ADC12IFG3_W { w: self }
    }
    #[doc = "Bit 4 - ADC12 Memory 4 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg4(&mut self) -> ADC12IFG4_W {
        ADC12IFG4_W { w: self }
    }
    #[doc = "Bit 5 - ADC12 Memory 5 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg5(&mut self) -> ADC12IFG5_W {
        ADC12IFG5_W { w: self }
    }
    #[doc = "Bit 6 - ADC12 Memory 6 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg6(&mut self) -> ADC12IFG6_W {
        ADC12IFG6_W { w: self }
    }
    #[doc = "Bit 7 - ADC12 Memory 7 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg7(&mut self) -> ADC12IFG7_W {
        ADC12IFG7_W { w: self }
    }
    #[doc = "Bit 8 - ADC12 Memory 8 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg8(&mut self) -> ADC12IFG8_W {
        ADC12IFG8_W { w: self }
    }
    #[doc = "Bit 9 - ADC12 Memory 9 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg9(&mut self) -> ADC12IFG9_W {
        ADC12IFG9_W { w: self }
    }
    #[doc = "Bit 10 - ADC12 Memory 10 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg10(&mut self) -> ADC12IFG10_W {
        ADC12IFG10_W { w: self }
    }
    #[doc = "Bit 11 - ADC12 Memory 11 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg11(&mut self) -> ADC12IFG11_W {
        ADC12IFG11_W { w: self }
    }
    #[doc = "Bit 12 - ADC12 Memory 12 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg12(&mut self) -> ADC12IFG12_W {
        ADC12IFG12_W { w: self }
    }
    #[doc = "Bit 13 - ADC12 Memory 13 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg13(&mut self) -> ADC12IFG13_W {
        ADC12IFG13_W { w: self }
    }
    #[doc = "Bit 14 - ADC12 Memory 14 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg14(&mut self) -> ADC12IFG14_W {
        ADC12IFG14_W { w: self }
    }
    #[doc = "Bit 15 - ADC12 Memory 15 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg15(&mut self) -> ADC12IFG15_W {
        ADC12IFG15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC12+ Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ifg](index.html) module"]
pub struct ADC12IFG_SPEC;
impl crate::RegisterSpec for ADC12IFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc12ifg::R](R) reader structure"]
impl crate::Readable for ADC12IFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc12ifg::W](W) writer structure"]
impl crate::Writable for ADC12IFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC12IFG to value 0"]
impl crate::Resettable for ADC12IFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
