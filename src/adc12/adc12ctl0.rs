#[doc = "Register `ADC12CTL0` reader"]
pub struct R(crate::R<ADC12CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC12CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC12CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC12CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC12CTL0` writer"]
pub struct W(crate::W<ADC12CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC12CTL0_SPEC>;
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
impl From<crate::W<ADC12CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC12CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC12SC` reader - ADC12 Start Conversion"]
pub struct ADC12SC_R(crate::FieldReader<bool, bool>);
impl ADC12SC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12SC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12SC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12SC` writer - ADC12 Start Conversion"]
pub struct ADC12SC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12SC_W<'a> {
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
#[doc = "Field `ADC12ENC` reader - ADC12 Enable Conversion"]
pub struct ADC12ENC_R(crate::FieldReader<bool, bool>);
impl ADC12ENC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12ENC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12ENC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12ENC` writer - ADC12 Enable Conversion"]
pub struct ADC12ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12ENC_W<'a> {
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
#[doc = "Field `ADC12TOVIE` reader - ADC12 Timer Overflow interrupt enable"]
pub struct ADC12TOVIE_R(crate::FieldReader<bool, bool>);
impl ADC12TOVIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12TOVIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12TOVIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12TOVIE` writer - ADC12 Timer Overflow interrupt enable"]
pub struct ADC12TOVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12TOVIE_W<'a> {
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
#[doc = "Field `ADC12OVIE` reader - ADC12 Overflow interrupt enable"]
pub struct ADC12OVIE_R(crate::FieldReader<bool, bool>);
impl ADC12OVIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12OVIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12OVIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12OVIE` writer - ADC12 Overflow interrupt enable"]
pub struct ADC12OVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12OVIE_W<'a> {
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
#[doc = "Field `ADC12ON` reader - ADC12 On/enable"]
pub struct ADC12ON_R(crate::FieldReader<bool, bool>);
impl ADC12ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12ON` writer - ADC12 On/enable"]
pub struct ADC12ON_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12ON_W<'a> {
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
#[doc = "Field `ADC12REFON` reader - ADC12 Reference on"]
pub struct ADC12REFON_R(crate::FieldReader<bool, bool>);
impl ADC12REFON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12REFON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12REFON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12REFON` writer - ADC12 Reference on"]
pub struct ADC12REFON_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12REFON_W<'a> {
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
#[doc = "Field `ADC12REF2_5V` reader - ADC12 Ref 0:1.5V / 1:2.5V"]
pub struct ADC12REF2_5V_R(crate::FieldReader<bool, bool>);
impl ADC12REF2_5V_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12REF2_5V_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12REF2_5V_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12REF2_5V` writer - ADC12 Ref 0:1.5V / 1:2.5V"]
pub struct ADC12REF2_5V_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12REF2_5V_W<'a> {
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
#[doc = "Field `ADC12MSC` reader - ADC12 Multiple SampleConversion"]
pub struct ADC12MSC_R(crate::FieldReader<bool, bool>);
impl ADC12MSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12MSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12MSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12MSC` writer - ADC12 Multiple SampleConversion"]
pub struct ADC12MSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12MSC_W<'a> {
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
#[doc = "ADC12 Sample Hold 0 Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12SHT0_A {
    #[doc = "0: ADC12 Sample Hold 0 Select Bit: 0"]
    ADC12SHT0_0 = 0,
    #[doc = "1: ADC12 Sample Hold 0 Select Bit: 1"]
    ADC12SHT0_1 = 1,
    #[doc = "2: ADC12 Sample Hold 0 Select Bit: 2"]
    ADC12SHT0_2 = 2,
    #[doc = "3: ADC12 Sample Hold 0 Select Bit: 3"]
    ADC12SHT0_3 = 3,
    #[doc = "4: ADC12 Sample Hold 0 Select Bit: 4"]
    ADC12SHT0_4 = 4,
    #[doc = "5: ADC12 Sample Hold 0 Select Bit: 5"]
    ADC12SHT0_5 = 5,
    #[doc = "6: ADC12 Sample Hold 0 Select Bit: 6"]
    ADC12SHT0_6 = 6,
    #[doc = "7: ADC12 Sample Hold 0 Select Bit: 7"]
    ADC12SHT0_7 = 7,
    #[doc = "8: ADC12 Sample Hold 0 Select Bit: 8"]
    ADC12SHT0_8 = 8,
    #[doc = "9: ADC12 Sample Hold 0 Select Bit: 9"]
    ADC12SHT0_9 = 9,
    #[doc = "10: ADC12 Sample Hold 0 Select Bit: 10"]
    ADC12SHT0_10 = 10,
    #[doc = "11: ADC12 Sample Hold 0 Select Bit: 11"]
    ADC12SHT0_11 = 11,
    #[doc = "12: ADC12 Sample Hold 0 Select Bit: 12"]
    ADC12SHT0_12 = 12,
    #[doc = "13: ADC12 Sample Hold 0 Select Bit: 13"]
    ADC12SHT0_13 = 13,
    #[doc = "14: ADC12 Sample Hold 0 Select Bit: 14"]
    ADC12SHT0_14 = 14,
    #[doc = "15: ADC12 Sample Hold 0 Select Bit: 15"]
    ADC12SHT0_15 = 15,
}
impl From<ADC12SHT0_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12SHT0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC12SHT0` reader - ADC12 Sample Hold 0 Select Bit: 0"]
pub struct ADC12SHT0_R(crate::FieldReader<u8, ADC12SHT0_A>);
impl ADC12SHT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC12SHT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12SHT0_A {
        match self.bits {
            0 => ADC12SHT0_A::ADC12SHT0_0,
            1 => ADC12SHT0_A::ADC12SHT0_1,
            2 => ADC12SHT0_A::ADC12SHT0_2,
            3 => ADC12SHT0_A::ADC12SHT0_3,
            4 => ADC12SHT0_A::ADC12SHT0_4,
            5 => ADC12SHT0_A::ADC12SHT0_5,
            6 => ADC12SHT0_A::ADC12SHT0_6,
            7 => ADC12SHT0_A::ADC12SHT0_7,
            8 => ADC12SHT0_A::ADC12SHT0_8,
            9 => ADC12SHT0_A::ADC12SHT0_9,
            10 => ADC12SHT0_A::ADC12SHT0_10,
            11 => ADC12SHT0_A::ADC12SHT0_11,
            12 => ADC12SHT0_A::ADC12SHT0_12,
            13 => ADC12SHT0_A::ADC12SHT0_13,
            14 => ADC12SHT0_A::ADC12SHT0_14,
            15 => ADC12SHT0_A::ADC12SHT0_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_0`"]
    #[inline(always)]
    pub fn is_adc12sht0_0(&self) -> bool {
        **self == ADC12SHT0_A::ADC12SHT0_0
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_1`"]
    #[inline(always)]
    pub fn is_adc12sht0_1(&self) -> bool {
        **self == ADC12SHT0_A::ADC12SHT0_1
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_2`"]
    #[inline(always)]
    pub fn is_adc12sht0_2(&self) -> bool {
        **self == ADC12SHT0_A::ADC12SHT0_2
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_3`"]
    #[inline(always)]
    pub fn is_adc12sht0_3(&self) -> bool {
        **self == ADC12SHT0_A::ADC12SHT0_3
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_4`"]
    #[inline(always)]
    pub fn is_adc12sht0_4(&self) -> bool {
        **self == ADC12SHT0_A::ADC12SHT0_4
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_5`"]
    #[inline(always)]
    pub fn is_adc12sht0_5(&self) -> bool {
        **self == ADC12SHT0_A::ADC12SHT0_5
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_6`"]
    #[inline(always)]
    pub fn is_adc12sht0_6(&self) -> bool {
        **self == ADC12SHT0_A::ADC12SHT0_6
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_7`"]
    #[inline(always)]
    pub fn is_adc12sht0_7(&self) -> bool {
        **self == ADC12SHT0_A::ADC12SHT0_7
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_8`"]
    #[inline(always)]
    pub fn is_adc12sht0_8(&self) -> bool {
        **self == ADC12SHT0_A::ADC12SHT0_8
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_9`"]
    #[inline(always)]
    pub fn is_adc12sht0_9(&self) -> bool {
        **self == ADC12SHT0_A::ADC12SHT0_9
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_10`"]
    #[inline(always)]
    pub fn is_adc12sht0_10(&self) -> bool {
        **self == ADC12SHT0_A::ADC12SHT0_10
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_11`"]
    #[inline(always)]
    pub fn is_adc12sht0_11(&self) -> bool {
        **self == ADC12SHT0_A::ADC12SHT0_11
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_12`"]
    #[inline(always)]
    pub fn is_adc12sht0_12(&self) -> bool {
        **self == ADC12SHT0_A::ADC12SHT0_12
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_13`"]
    #[inline(always)]
    pub fn is_adc12sht0_13(&self) -> bool {
        **self == ADC12SHT0_A::ADC12SHT0_13
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_14`"]
    #[inline(always)]
    pub fn is_adc12sht0_14(&self) -> bool {
        **self == ADC12SHT0_A::ADC12SHT0_14
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_15`"]
    #[inline(always)]
    pub fn is_adc12sht0_15(&self) -> bool {
        **self == ADC12SHT0_A::ADC12SHT0_15
    }
}
impl core::ops::Deref for ADC12SHT0_R {
    type Target = crate::FieldReader<u8, ADC12SHT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12SHT0` writer - ADC12 Sample Hold 0 Select Bit: 0"]
pub struct ADC12SHT0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12SHT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12SHT0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 0"]
    #[inline(always)]
    pub fn adc12sht0_0(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_0)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 1"]
    #[inline(always)]
    pub fn adc12sht0_1(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_1)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 2"]
    #[inline(always)]
    pub fn adc12sht0_2(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_2)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 3"]
    #[inline(always)]
    pub fn adc12sht0_3(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_3)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 4"]
    #[inline(always)]
    pub fn adc12sht0_4(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_4)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 5"]
    #[inline(always)]
    pub fn adc12sht0_5(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_5)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 6"]
    #[inline(always)]
    pub fn adc12sht0_6(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_6)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 7"]
    #[inline(always)]
    pub fn adc12sht0_7(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_7)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 8"]
    #[inline(always)]
    pub fn adc12sht0_8(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_8)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 9"]
    #[inline(always)]
    pub fn adc12sht0_9(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_9)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 10"]
    #[inline(always)]
    pub fn adc12sht0_10(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_10)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 11"]
    #[inline(always)]
    pub fn adc12sht0_11(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_11)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 12"]
    #[inline(always)]
    pub fn adc12sht0_12(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_12)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 13"]
    #[inline(always)]
    pub fn adc12sht0_13(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_13)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 14"]
    #[inline(always)]
    pub fn adc12sht0_14(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_14)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 15"]
    #[inline(always)]
    pub fn adc12sht0_15(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u16 & 0x0f) << 8);
        self.w
    }
}
#[doc = "ADC12 Sample Hold 1 Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12SHT1_A {
    #[doc = "0: ADC12 Sample Hold 1 Select Bit: 0"]
    ADC12SHT1_0 = 0,
    #[doc = "1: ADC12 Sample Hold 1 Select Bit: 1"]
    ADC12SHT1_1 = 1,
    #[doc = "2: ADC12 Sample Hold 1 Select Bit: 2"]
    ADC12SHT1_2 = 2,
    #[doc = "3: ADC12 Sample Hold 1 Select Bit: 3"]
    ADC12SHT1_3 = 3,
    #[doc = "4: ADC12 Sample Hold 1 Select Bit: 4"]
    ADC12SHT1_4 = 4,
    #[doc = "5: ADC12 Sample Hold 1 Select Bit: 5"]
    ADC12SHT1_5 = 5,
    #[doc = "6: ADC12 Sample Hold 1 Select Bit: 6"]
    ADC12SHT1_6 = 6,
    #[doc = "7: ADC12 Sample Hold 1 Select Bit: 7"]
    ADC12SHT1_7 = 7,
    #[doc = "8: ADC12 Sample Hold 1 Select Bit: 8"]
    ADC12SHT1_8 = 8,
    #[doc = "9: ADC12 Sample Hold 1 Select Bit: 9"]
    ADC12SHT1_9 = 9,
    #[doc = "10: ADC12 Sample Hold 1 Select Bit: 10"]
    ADC12SHT1_10 = 10,
    #[doc = "11: ADC12 Sample Hold 1 Select Bit: 11"]
    ADC12SHT1_11 = 11,
    #[doc = "12: ADC12 Sample Hold 1 Select Bit: 12"]
    ADC12SHT1_12 = 12,
    #[doc = "13: ADC12 Sample Hold 1 Select Bit: 13"]
    ADC12SHT1_13 = 13,
    #[doc = "14: ADC12 Sample Hold 1 Select Bit: 14"]
    ADC12SHT1_14 = 14,
    #[doc = "15: ADC12 Sample Hold 1 Select Bit: 15"]
    ADC12SHT1_15 = 15,
}
impl From<ADC12SHT1_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12SHT1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC12SHT1` reader - ADC12 Sample Hold 1 Select Bit: 0"]
pub struct ADC12SHT1_R(crate::FieldReader<u8, ADC12SHT1_A>);
impl ADC12SHT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC12SHT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12SHT1_A {
        match self.bits {
            0 => ADC12SHT1_A::ADC12SHT1_0,
            1 => ADC12SHT1_A::ADC12SHT1_1,
            2 => ADC12SHT1_A::ADC12SHT1_2,
            3 => ADC12SHT1_A::ADC12SHT1_3,
            4 => ADC12SHT1_A::ADC12SHT1_4,
            5 => ADC12SHT1_A::ADC12SHT1_5,
            6 => ADC12SHT1_A::ADC12SHT1_6,
            7 => ADC12SHT1_A::ADC12SHT1_7,
            8 => ADC12SHT1_A::ADC12SHT1_8,
            9 => ADC12SHT1_A::ADC12SHT1_9,
            10 => ADC12SHT1_A::ADC12SHT1_10,
            11 => ADC12SHT1_A::ADC12SHT1_11,
            12 => ADC12SHT1_A::ADC12SHT1_12,
            13 => ADC12SHT1_A::ADC12SHT1_13,
            14 => ADC12SHT1_A::ADC12SHT1_14,
            15 => ADC12SHT1_A::ADC12SHT1_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_0`"]
    #[inline(always)]
    pub fn is_adc12sht1_0(&self) -> bool {
        **self == ADC12SHT1_A::ADC12SHT1_0
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_1`"]
    #[inline(always)]
    pub fn is_adc12sht1_1(&self) -> bool {
        **self == ADC12SHT1_A::ADC12SHT1_1
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_2`"]
    #[inline(always)]
    pub fn is_adc12sht1_2(&self) -> bool {
        **self == ADC12SHT1_A::ADC12SHT1_2
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_3`"]
    #[inline(always)]
    pub fn is_adc12sht1_3(&self) -> bool {
        **self == ADC12SHT1_A::ADC12SHT1_3
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_4`"]
    #[inline(always)]
    pub fn is_adc12sht1_4(&self) -> bool {
        **self == ADC12SHT1_A::ADC12SHT1_4
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_5`"]
    #[inline(always)]
    pub fn is_adc12sht1_5(&self) -> bool {
        **self == ADC12SHT1_A::ADC12SHT1_5
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_6`"]
    #[inline(always)]
    pub fn is_adc12sht1_6(&self) -> bool {
        **self == ADC12SHT1_A::ADC12SHT1_6
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_7`"]
    #[inline(always)]
    pub fn is_adc12sht1_7(&self) -> bool {
        **self == ADC12SHT1_A::ADC12SHT1_7
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_8`"]
    #[inline(always)]
    pub fn is_adc12sht1_8(&self) -> bool {
        **self == ADC12SHT1_A::ADC12SHT1_8
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_9`"]
    #[inline(always)]
    pub fn is_adc12sht1_9(&self) -> bool {
        **self == ADC12SHT1_A::ADC12SHT1_9
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_10`"]
    #[inline(always)]
    pub fn is_adc12sht1_10(&self) -> bool {
        **self == ADC12SHT1_A::ADC12SHT1_10
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_11`"]
    #[inline(always)]
    pub fn is_adc12sht1_11(&self) -> bool {
        **self == ADC12SHT1_A::ADC12SHT1_11
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_12`"]
    #[inline(always)]
    pub fn is_adc12sht1_12(&self) -> bool {
        **self == ADC12SHT1_A::ADC12SHT1_12
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_13`"]
    #[inline(always)]
    pub fn is_adc12sht1_13(&self) -> bool {
        **self == ADC12SHT1_A::ADC12SHT1_13
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_14`"]
    #[inline(always)]
    pub fn is_adc12sht1_14(&self) -> bool {
        **self == ADC12SHT1_A::ADC12SHT1_14
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_15`"]
    #[inline(always)]
    pub fn is_adc12sht1_15(&self) -> bool {
        **self == ADC12SHT1_A::ADC12SHT1_15
    }
}
impl core::ops::Deref for ADC12SHT1_R {
    type Target = crate::FieldReader<u8, ADC12SHT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12SHT1` writer - ADC12 Sample Hold 1 Select Bit: 0"]
pub struct ADC12SHT1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12SHT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12SHT1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 0"]
    #[inline(always)]
    pub fn adc12sht1_0(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_0)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 1"]
    #[inline(always)]
    pub fn adc12sht1_1(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_1)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 2"]
    #[inline(always)]
    pub fn adc12sht1_2(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_2)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 3"]
    #[inline(always)]
    pub fn adc12sht1_3(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_3)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 4"]
    #[inline(always)]
    pub fn adc12sht1_4(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_4)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 5"]
    #[inline(always)]
    pub fn adc12sht1_5(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_5)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 6"]
    #[inline(always)]
    pub fn adc12sht1_6(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_6)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 7"]
    #[inline(always)]
    pub fn adc12sht1_7(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_7)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 8"]
    #[inline(always)]
    pub fn adc12sht1_8(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_8)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 9"]
    #[inline(always)]
    pub fn adc12sht1_9(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_9)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 10"]
    #[inline(always)]
    pub fn adc12sht1_10(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_10)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 11"]
    #[inline(always)]
    pub fn adc12sht1_11(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_11)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 12"]
    #[inline(always)]
    pub fn adc12sht1_12(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_12)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 13"]
    #[inline(always)]
    pub fn adc12sht1_13(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_13)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 14"]
    #[inline(always)]
    pub fn adc12sht1_14(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_14)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 15"]
    #[inline(always)]
    pub fn adc12sht1_15(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u16 & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADC12 Start Conversion"]
    #[inline(always)]
    pub fn adc12sc(&self) -> ADC12SC_R {
        ADC12SC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC12 Enable Conversion"]
    #[inline(always)]
    pub fn adc12enc(&self) -> ADC12ENC_R {
        ADC12ENC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC12 Timer Overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12tovie(&self) -> ADC12TOVIE_R {
        ADC12TOVIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC12 Overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12ovie(&self) -> ADC12OVIE_R {
        ADC12OVIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC12 On/enable"]
    #[inline(always)]
    pub fn adc12on(&self) -> ADC12ON_R {
        ADC12ON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC12 Reference on"]
    #[inline(always)]
    pub fn adc12refon(&self) -> ADC12REFON_R {
        ADC12REFON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC12 Ref 0:1.5V / 1:2.5V"]
    #[inline(always)]
    pub fn adc12ref2_5v(&self) -> ADC12REF2_5V_R {
        ADC12REF2_5V_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC12 Multiple SampleConversion"]
    #[inline(always)]
    pub fn adc12msc(&self) -> ADC12MSC_R {
        ADC12MSC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - ADC12 Sample Hold 0 Select Bit: 0"]
    #[inline(always)]
    pub fn adc12sht0(&self) -> ADC12SHT0_R {
        ADC12SHT0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - ADC12 Sample Hold 1 Select Bit: 0"]
    #[inline(always)]
    pub fn adc12sht1(&self) -> ADC12SHT1_R {
        ADC12SHT1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12 Start Conversion"]
    #[inline(always)]
    pub fn adc12sc(&mut self) -> ADC12SC_W {
        ADC12SC_W { w: self }
    }
    #[doc = "Bit 1 - ADC12 Enable Conversion"]
    #[inline(always)]
    pub fn adc12enc(&mut self) -> ADC12ENC_W {
        ADC12ENC_W { w: self }
    }
    #[doc = "Bit 2 - ADC12 Timer Overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12tovie(&mut self) -> ADC12TOVIE_W {
        ADC12TOVIE_W { w: self }
    }
    #[doc = "Bit 3 - ADC12 Overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12ovie(&mut self) -> ADC12OVIE_W {
        ADC12OVIE_W { w: self }
    }
    #[doc = "Bit 4 - ADC12 On/enable"]
    #[inline(always)]
    pub fn adc12on(&mut self) -> ADC12ON_W {
        ADC12ON_W { w: self }
    }
    #[doc = "Bit 5 - ADC12 Reference on"]
    #[inline(always)]
    pub fn adc12refon(&mut self) -> ADC12REFON_W {
        ADC12REFON_W { w: self }
    }
    #[doc = "Bit 6 - ADC12 Ref 0:1.5V / 1:2.5V"]
    #[inline(always)]
    pub fn adc12ref2_5v(&mut self) -> ADC12REF2_5V_W {
        ADC12REF2_5V_W { w: self }
    }
    #[doc = "Bit 7 - ADC12 Multiple SampleConversion"]
    #[inline(always)]
    pub fn adc12msc(&mut self) -> ADC12MSC_W {
        ADC12MSC_W { w: self }
    }
    #[doc = "Bits 8:11 - ADC12 Sample Hold 0 Select Bit: 0"]
    #[inline(always)]
    pub fn adc12sht0(&mut self) -> ADC12SHT0_W {
        ADC12SHT0_W { w: self }
    }
    #[doc = "Bits 12:15 - ADC12 Sample Hold 1 Select Bit: 0"]
    #[inline(always)]
    pub fn adc12sht1(&mut self) -> ADC12SHT1_W {
        ADC12SHT1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC12+ Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ctl0](index.html) module"]
pub struct ADC12CTL0_SPEC;
impl crate::RegisterSpec for ADC12CTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc12ctl0::R](R) reader structure"]
impl crate::Readable for ADC12CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc12ctl0::W](W) writer structure"]
impl crate::Writable for ADC12CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC12CTL0 to value 0"]
impl crate::Resettable for ADC12CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
