#[doc = "Register `ADC12CTL2` reader"]
pub struct R(crate::R<ADC12CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC12CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC12CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC12CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC12CTL2` writer"]
pub struct W(crate::W<ADC12CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC12CTL2_SPEC>;
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
impl From<crate::W<ADC12CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC12CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC12REFBURST` reader - ADC12+ Reference Burst"]
pub struct ADC12REFBURST_R(crate::FieldReader<bool, bool>);
impl ADC12REFBURST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12REFBURST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12REFBURST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12REFBURST` writer - ADC12+ Reference Burst"]
pub struct ADC12REFBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12REFBURST_W<'a> {
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
#[doc = "Field `ADC12REFOUT` reader - ADC12+ Reference Out"]
pub struct ADC12REFOUT_R(crate::FieldReader<bool, bool>);
impl ADC12REFOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12REFOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12REFOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12REFOUT` writer - ADC12+ Reference Out"]
pub struct ADC12REFOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12REFOUT_W<'a> {
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
#[doc = "Field `ADC12SR` reader - ADC12+ Sampling Rate"]
pub struct ADC12SR_R(crate::FieldReader<bool, bool>);
impl ADC12SR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12SR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12SR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12SR` writer - ADC12+ Sampling Rate"]
pub struct ADC12SR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12SR_W<'a> {
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
#[doc = "Field `ADC12DF` reader - ADC12+ Data Format"]
pub struct ADC12DF_R(crate::FieldReader<bool, bool>);
impl ADC12DF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12DF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12DF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12DF` writer - ADC12+ Data Format"]
pub struct ADC12DF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12DF_W<'a> {
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
#[doc = "ADC12+ Resolution Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12RES_A {
    #[doc = "0: ADC12+ Resolution : 8 Bit"]
    ADC12RES_0 = 0,
    #[doc = "1: ADC12+ Resolution : 10 Bit"]
    ADC12RES_1 = 1,
    #[doc = "2: ADC12+ Resolution : 12 Bit"]
    ADC12RES_2 = 2,
    #[doc = "3: ADC12+ Resolution : reserved"]
    ADC12RES_3 = 3,
}
impl From<ADC12RES_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12RES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC12RES` reader - ADC12+ Resolution Bit: 0"]
pub struct ADC12RES_R(crate::FieldReader<u8, ADC12RES_A>);
impl ADC12RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC12RES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12RES_A {
        match self.bits {
            0 => ADC12RES_A::ADC12RES_0,
            1 => ADC12RES_A::ADC12RES_1,
            2 => ADC12RES_A::ADC12RES_2,
            3 => ADC12RES_A::ADC12RES_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12RES_0`"]
    #[inline(always)]
    pub fn is_adc12res_0(&self) -> bool {
        **self == ADC12RES_A::ADC12RES_0
    }
    #[doc = "Checks if the value of the field is `ADC12RES_1`"]
    #[inline(always)]
    pub fn is_adc12res_1(&self) -> bool {
        **self == ADC12RES_A::ADC12RES_1
    }
    #[doc = "Checks if the value of the field is `ADC12RES_2`"]
    #[inline(always)]
    pub fn is_adc12res_2(&self) -> bool {
        **self == ADC12RES_A::ADC12RES_2
    }
    #[doc = "Checks if the value of the field is `ADC12RES_3`"]
    #[inline(always)]
    pub fn is_adc12res_3(&self) -> bool {
        **self == ADC12RES_A::ADC12RES_3
    }
}
impl core::ops::Deref for ADC12RES_R {
    type Target = crate::FieldReader<u8, ADC12RES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12RES` writer - ADC12+ Resolution Bit: 0"]
pub struct ADC12RES_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12RES_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "ADC12+ Resolution : 8 Bit"]
    #[inline(always)]
    pub fn adc12res_0(self) -> &'a mut W {
        self.variant(ADC12RES_A::ADC12RES_0)
    }
    #[doc = "ADC12+ Resolution : 10 Bit"]
    #[inline(always)]
    pub fn adc12res_1(self) -> &'a mut W {
        self.variant(ADC12RES_A::ADC12RES_1)
    }
    #[doc = "ADC12+ Resolution : 12 Bit"]
    #[inline(always)]
    pub fn adc12res_2(self) -> &'a mut W {
        self.variant(ADC12RES_A::ADC12RES_2)
    }
    #[doc = "ADC12+ Resolution : reserved"]
    #[inline(always)]
    pub fn adc12res_3(self) -> &'a mut W {
        self.variant(ADC12RES_A::ADC12RES_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u16 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `ADC12TCOFF` reader - ADC12+ Temperature Sensor Off"]
pub struct ADC12TCOFF_R(crate::FieldReader<bool, bool>);
impl ADC12TCOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12TCOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12TCOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12TCOFF` writer - ADC12+ Temperature Sensor Off"]
pub struct ADC12TCOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12TCOFF_W<'a> {
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
#[doc = "Field `ADC12PDIV` reader - ADC12+ predivider 0:/1 1:/4"]
pub struct ADC12PDIV_R(crate::FieldReader<bool, bool>);
impl ADC12PDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12PDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12PDIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12PDIV` writer - ADC12+ predivider 0:/1 1:/4"]
pub struct ADC12PDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12PDIV_W<'a> {
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
impl R {
    #[doc = "Bit 0 - ADC12+ Reference Burst"]
    #[inline(always)]
    pub fn adc12refburst(&self) -> ADC12REFBURST_R {
        ADC12REFBURST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC12+ Reference Out"]
    #[inline(always)]
    pub fn adc12refout(&self) -> ADC12REFOUT_R {
        ADC12REFOUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC12+ Sampling Rate"]
    #[inline(always)]
    pub fn adc12sr(&self) -> ADC12SR_R {
        ADC12SR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC12+ Data Format"]
    #[inline(always)]
    pub fn adc12df(&self) -> ADC12DF_R {
        ADC12DF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - ADC12+ Resolution Bit: 0"]
    #[inline(always)]
    pub fn adc12res(&self) -> ADC12RES_R {
        ADC12RES_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - ADC12+ Temperature Sensor Off"]
    #[inline(always)]
    pub fn adc12tcoff(&self) -> ADC12TCOFF_R {
        ADC12TCOFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC12+ predivider 0:/1 1:/4"]
    #[inline(always)]
    pub fn adc12pdiv(&self) -> ADC12PDIV_R {
        ADC12PDIV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12+ Reference Burst"]
    #[inline(always)]
    pub fn adc12refburst(&mut self) -> ADC12REFBURST_W {
        ADC12REFBURST_W { w: self }
    }
    #[doc = "Bit 1 - ADC12+ Reference Out"]
    #[inline(always)]
    pub fn adc12refout(&mut self) -> ADC12REFOUT_W {
        ADC12REFOUT_W { w: self }
    }
    #[doc = "Bit 2 - ADC12+ Sampling Rate"]
    #[inline(always)]
    pub fn adc12sr(&mut self) -> ADC12SR_W {
        ADC12SR_W { w: self }
    }
    #[doc = "Bit 3 - ADC12+ Data Format"]
    #[inline(always)]
    pub fn adc12df(&mut self) -> ADC12DF_W {
        ADC12DF_W { w: self }
    }
    #[doc = "Bits 4:5 - ADC12+ Resolution Bit: 0"]
    #[inline(always)]
    pub fn adc12res(&mut self) -> ADC12RES_W {
        ADC12RES_W { w: self }
    }
    #[doc = "Bit 7 - ADC12+ Temperature Sensor Off"]
    #[inline(always)]
    pub fn adc12tcoff(&mut self) -> ADC12TCOFF_W {
        ADC12TCOFF_W { w: self }
    }
    #[doc = "Bit 8 - ADC12+ predivider 0:/1 1:/4"]
    #[inline(always)]
    pub fn adc12pdiv(&mut self) -> ADC12PDIV_W {
        ADC12PDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC12+ Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ctl2](index.html) module"]
pub struct ADC12CTL2_SPEC;
impl crate::RegisterSpec for ADC12CTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc12ctl2::R](R) reader structure"]
impl crate::Readable for ADC12CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc12ctl2::W](W) writer structure"]
impl crate::Writable for ADC12CTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC12CTL2 to value 0"]
impl crate::Resettable for ADC12CTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
