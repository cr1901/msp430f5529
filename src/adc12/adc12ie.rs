#[doc = "Register `ADC12IE` reader"]
pub struct R(crate::R<ADC12IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC12IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC12IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC12IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC12IE` writer"]
pub struct W(crate::W<ADC12IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC12IE_SPEC>;
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
impl From<crate::W<ADC12IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC12IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC12IE0` reader - ADC12 Memory 0 Interrupt Enable"]
pub struct ADC12IE0_R(crate::FieldReader<bool, bool>);
impl ADC12IE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE0` writer - ADC12 Memory 0 Interrupt Enable"]
pub struct ADC12IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE0_W<'a> {
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
#[doc = "Field `ADC12IE1` reader - ADC12 Memory 1 Interrupt Enable"]
pub struct ADC12IE1_R(crate::FieldReader<bool, bool>);
impl ADC12IE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE1` writer - ADC12 Memory 1 Interrupt Enable"]
pub struct ADC12IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE1_W<'a> {
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
#[doc = "Field `ADC12IE2` reader - ADC12 Memory 2 Interrupt Enable"]
pub struct ADC12IE2_R(crate::FieldReader<bool, bool>);
impl ADC12IE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE2` writer - ADC12 Memory 2 Interrupt Enable"]
pub struct ADC12IE2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE2_W<'a> {
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
#[doc = "Field `ADC12IE3` reader - ADC12 Memory 3 Interrupt Enable"]
pub struct ADC12IE3_R(crate::FieldReader<bool, bool>);
impl ADC12IE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE3` writer - ADC12 Memory 3 Interrupt Enable"]
pub struct ADC12IE3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE3_W<'a> {
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
#[doc = "Field `ADC12IE4` reader - ADC12 Memory 4 Interrupt Enable"]
pub struct ADC12IE4_R(crate::FieldReader<bool, bool>);
impl ADC12IE4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE4` writer - ADC12 Memory 4 Interrupt Enable"]
pub struct ADC12IE4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE4_W<'a> {
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
#[doc = "Field `ADC12IE5` reader - ADC12 Memory 5 Interrupt Enable"]
pub struct ADC12IE5_R(crate::FieldReader<bool, bool>);
impl ADC12IE5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE5` writer - ADC12 Memory 5 Interrupt Enable"]
pub struct ADC12IE5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE5_W<'a> {
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
#[doc = "Field `ADC12IE6` reader - ADC12 Memory 6 Interrupt Enable"]
pub struct ADC12IE6_R(crate::FieldReader<bool, bool>);
impl ADC12IE6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE6` writer - ADC12 Memory 6 Interrupt Enable"]
pub struct ADC12IE6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE6_W<'a> {
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
#[doc = "Field `ADC12IE7` reader - ADC12 Memory 7 Interrupt Enable"]
pub struct ADC12IE7_R(crate::FieldReader<bool, bool>);
impl ADC12IE7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE7` writer - ADC12 Memory 7 Interrupt Enable"]
pub struct ADC12IE7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE7_W<'a> {
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
#[doc = "Field `ADC12IE8` reader - ADC12 Memory 8 Interrupt Enable"]
pub struct ADC12IE8_R(crate::FieldReader<bool, bool>);
impl ADC12IE8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE8` writer - ADC12 Memory 8 Interrupt Enable"]
pub struct ADC12IE8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE8_W<'a> {
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
#[doc = "Field `ADC12IE9` reader - ADC12 Memory 9 Interrupt Enable"]
pub struct ADC12IE9_R(crate::FieldReader<bool, bool>);
impl ADC12IE9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE9` writer - ADC12 Memory 9 Interrupt Enable"]
pub struct ADC12IE9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE9_W<'a> {
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
#[doc = "Field `ADC12IE10` reader - ADC12 Memory 10 Interrupt Enable"]
pub struct ADC12IE10_R(crate::FieldReader<bool, bool>);
impl ADC12IE10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE10` writer - ADC12 Memory 10 Interrupt Enable"]
pub struct ADC12IE10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE10_W<'a> {
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
#[doc = "Field `ADC12IE11` reader - ADC12 Memory 11 Interrupt Enable"]
pub struct ADC12IE11_R(crate::FieldReader<bool, bool>);
impl ADC12IE11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE11` writer - ADC12 Memory 11 Interrupt Enable"]
pub struct ADC12IE11_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE11_W<'a> {
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
#[doc = "Field `ADC12IE12` reader - ADC12 Memory 12 Interrupt Enable"]
pub struct ADC12IE12_R(crate::FieldReader<bool, bool>);
impl ADC12IE12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE12` writer - ADC12 Memory 12 Interrupt Enable"]
pub struct ADC12IE12_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE12_W<'a> {
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
#[doc = "Field `ADC12IE13` reader - ADC12 Memory 13 Interrupt Enable"]
pub struct ADC12IE13_R(crate::FieldReader<bool, bool>);
impl ADC12IE13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE13` writer - ADC12 Memory 13 Interrupt Enable"]
pub struct ADC12IE13_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE13_W<'a> {
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
#[doc = "Field `ADC12IE14` reader - ADC12 Memory 14 Interrupt Enable"]
pub struct ADC12IE14_R(crate::FieldReader<bool, bool>);
impl ADC12IE14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE14` writer - ADC12 Memory 14 Interrupt Enable"]
pub struct ADC12IE14_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE14_W<'a> {
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
#[doc = "Field `ADC12IE15` reader - ADC12 Memory 15 Interrupt Enable"]
pub struct ADC12IE15_R(crate::FieldReader<bool, bool>);
impl ADC12IE15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE15` writer - ADC12 Memory 15 Interrupt Enable"]
pub struct ADC12IE15_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE15_W<'a> {
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
    #[doc = "Bit 0 - ADC12 Memory 0 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie0(&self) -> ADC12IE0_R {
        ADC12IE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC12 Memory 1 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie1(&self) -> ADC12IE1_R {
        ADC12IE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC12 Memory 2 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie2(&self) -> ADC12IE2_R {
        ADC12IE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC12 Memory 3 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie3(&self) -> ADC12IE3_R {
        ADC12IE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC12 Memory 4 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie4(&self) -> ADC12IE4_R {
        ADC12IE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC12 Memory 5 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie5(&self) -> ADC12IE5_R {
        ADC12IE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC12 Memory 6 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie6(&self) -> ADC12IE6_R {
        ADC12IE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC12 Memory 7 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie7(&self) -> ADC12IE7_R {
        ADC12IE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC12 Memory 8 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie8(&self) -> ADC12IE8_R {
        ADC12IE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC12 Memory 9 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie9(&self) -> ADC12IE9_R {
        ADC12IE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ADC12 Memory 10 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie10(&self) -> ADC12IE10_R {
        ADC12IE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADC12 Memory 11 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie11(&self) -> ADC12IE11_R {
        ADC12IE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ADC12 Memory 12 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie12(&self) -> ADC12IE12_R {
        ADC12IE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ADC12 Memory 13 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie13(&self) -> ADC12IE13_R {
        ADC12IE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ADC12 Memory 14 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie14(&self) -> ADC12IE14_R {
        ADC12IE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADC12 Memory 15 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie15(&self) -> ADC12IE15_R {
        ADC12IE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12 Memory 0 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie0(&mut self) -> ADC12IE0_W {
        ADC12IE0_W { w: self }
    }
    #[doc = "Bit 1 - ADC12 Memory 1 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie1(&mut self) -> ADC12IE1_W {
        ADC12IE1_W { w: self }
    }
    #[doc = "Bit 2 - ADC12 Memory 2 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie2(&mut self) -> ADC12IE2_W {
        ADC12IE2_W { w: self }
    }
    #[doc = "Bit 3 - ADC12 Memory 3 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie3(&mut self) -> ADC12IE3_W {
        ADC12IE3_W { w: self }
    }
    #[doc = "Bit 4 - ADC12 Memory 4 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie4(&mut self) -> ADC12IE4_W {
        ADC12IE4_W { w: self }
    }
    #[doc = "Bit 5 - ADC12 Memory 5 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie5(&mut self) -> ADC12IE5_W {
        ADC12IE5_W { w: self }
    }
    #[doc = "Bit 6 - ADC12 Memory 6 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie6(&mut self) -> ADC12IE6_W {
        ADC12IE6_W { w: self }
    }
    #[doc = "Bit 7 - ADC12 Memory 7 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie7(&mut self) -> ADC12IE7_W {
        ADC12IE7_W { w: self }
    }
    #[doc = "Bit 8 - ADC12 Memory 8 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie8(&mut self) -> ADC12IE8_W {
        ADC12IE8_W { w: self }
    }
    #[doc = "Bit 9 - ADC12 Memory 9 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie9(&mut self) -> ADC12IE9_W {
        ADC12IE9_W { w: self }
    }
    #[doc = "Bit 10 - ADC12 Memory 10 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie10(&mut self) -> ADC12IE10_W {
        ADC12IE10_W { w: self }
    }
    #[doc = "Bit 11 - ADC12 Memory 11 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie11(&mut self) -> ADC12IE11_W {
        ADC12IE11_W { w: self }
    }
    #[doc = "Bit 12 - ADC12 Memory 12 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie12(&mut self) -> ADC12IE12_W {
        ADC12IE12_W { w: self }
    }
    #[doc = "Bit 13 - ADC12 Memory 13 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie13(&mut self) -> ADC12IE13_W {
        ADC12IE13_W { w: self }
    }
    #[doc = "Bit 14 - ADC12 Memory 14 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie14(&mut self) -> ADC12IE14_W {
        ADC12IE14_W { w: self }
    }
    #[doc = "Bit 15 - ADC12 Memory 15 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie15(&mut self) -> ADC12IE15_W {
        ADC12IE15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC12+ Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ie](index.html) module"]
pub struct ADC12IE_SPEC;
impl crate::RegisterSpec for ADC12IE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc12ie::R](R) reader structure"]
impl crate::Readable for ADC12IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc12ie::W](W) writer structure"]
impl crate::Writable for ADC12IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC12IE to value 0"]
impl crate::Resettable for ADC12IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
