#[doc = "Register `ADC12MCTL15` reader"]
pub struct R(crate::R<ADC12MCTL15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC12MCTL15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC12MCTL15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC12MCTL15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC12MCTL15` writer"]
pub struct W(crate::W<ADC12MCTL15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC12MCTL15_SPEC>;
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
impl From<crate::W<ADC12MCTL15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC12MCTL15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADC12 Input Channel Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12INCH_A {
    #[doc = "0: ADC12 Input Channel 0"]
    ADC12INCH_0 = 0,
    #[doc = "1: ADC12 Input Channel 1"]
    ADC12INCH_1 = 1,
    #[doc = "2: ADC12 Input Channel 2"]
    ADC12INCH_2 = 2,
    #[doc = "3: ADC12 Input Channel 3"]
    ADC12INCH_3 = 3,
    #[doc = "4: ADC12 Input Channel 4"]
    ADC12INCH_4 = 4,
    #[doc = "5: ADC12 Input Channel 5"]
    ADC12INCH_5 = 5,
    #[doc = "6: ADC12 Input Channel 6"]
    ADC12INCH_6 = 6,
    #[doc = "7: ADC12 Input Channel 7"]
    ADC12INCH_7 = 7,
    #[doc = "8: ADC12 Input Channel 8"]
    ADC12INCH_8 = 8,
    #[doc = "9: ADC12 Input Channel 9"]
    ADC12INCH_9 = 9,
    #[doc = "10: ADC12 Input Channel 10"]
    ADC12INCH_10 = 10,
    #[doc = "11: ADC12 Input Channel 11"]
    ADC12INCH_11 = 11,
    #[doc = "12: ADC12 Input Channel 12"]
    ADC12INCH_12 = 12,
    #[doc = "13: ADC12 Input Channel 13"]
    ADC12INCH_13 = 13,
    #[doc = "14: ADC12 Input Channel 14"]
    ADC12INCH_14 = 14,
    #[doc = "15: ADC12 Input Channel 15"]
    ADC12INCH_15 = 15,
}
impl From<ADC12INCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12INCH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC12INCH` reader - ADC12 Input Channel Select Bit 0"]
pub struct ADC12INCH_R(crate::FieldReader<u8, ADC12INCH_A>);
impl ADC12INCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC12INCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12INCH_A {
        match self.bits {
            0 => ADC12INCH_A::ADC12INCH_0,
            1 => ADC12INCH_A::ADC12INCH_1,
            2 => ADC12INCH_A::ADC12INCH_2,
            3 => ADC12INCH_A::ADC12INCH_3,
            4 => ADC12INCH_A::ADC12INCH_4,
            5 => ADC12INCH_A::ADC12INCH_5,
            6 => ADC12INCH_A::ADC12INCH_6,
            7 => ADC12INCH_A::ADC12INCH_7,
            8 => ADC12INCH_A::ADC12INCH_8,
            9 => ADC12INCH_A::ADC12INCH_9,
            10 => ADC12INCH_A::ADC12INCH_10,
            11 => ADC12INCH_A::ADC12INCH_11,
            12 => ADC12INCH_A::ADC12INCH_12,
            13 => ADC12INCH_A::ADC12INCH_13,
            14 => ADC12INCH_A::ADC12INCH_14,
            15 => ADC12INCH_A::ADC12INCH_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_0`"]
    #[inline(always)]
    pub fn is_adc12inch_0(&self) -> bool {
        **self == ADC12INCH_A::ADC12INCH_0
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_1`"]
    #[inline(always)]
    pub fn is_adc12inch_1(&self) -> bool {
        **self == ADC12INCH_A::ADC12INCH_1
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_2`"]
    #[inline(always)]
    pub fn is_adc12inch_2(&self) -> bool {
        **self == ADC12INCH_A::ADC12INCH_2
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_3`"]
    #[inline(always)]
    pub fn is_adc12inch_3(&self) -> bool {
        **self == ADC12INCH_A::ADC12INCH_3
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_4`"]
    #[inline(always)]
    pub fn is_adc12inch_4(&self) -> bool {
        **self == ADC12INCH_A::ADC12INCH_4
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_5`"]
    #[inline(always)]
    pub fn is_adc12inch_5(&self) -> bool {
        **self == ADC12INCH_A::ADC12INCH_5
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_6`"]
    #[inline(always)]
    pub fn is_adc12inch_6(&self) -> bool {
        **self == ADC12INCH_A::ADC12INCH_6
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_7`"]
    #[inline(always)]
    pub fn is_adc12inch_7(&self) -> bool {
        **self == ADC12INCH_A::ADC12INCH_7
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_8`"]
    #[inline(always)]
    pub fn is_adc12inch_8(&self) -> bool {
        **self == ADC12INCH_A::ADC12INCH_8
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_9`"]
    #[inline(always)]
    pub fn is_adc12inch_9(&self) -> bool {
        **self == ADC12INCH_A::ADC12INCH_9
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_10`"]
    #[inline(always)]
    pub fn is_adc12inch_10(&self) -> bool {
        **self == ADC12INCH_A::ADC12INCH_10
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_11`"]
    #[inline(always)]
    pub fn is_adc12inch_11(&self) -> bool {
        **self == ADC12INCH_A::ADC12INCH_11
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_12`"]
    #[inline(always)]
    pub fn is_adc12inch_12(&self) -> bool {
        **self == ADC12INCH_A::ADC12INCH_12
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_13`"]
    #[inline(always)]
    pub fn is_adc12inch_13(&self) -> bool {
        **self == ADC12INCH_A::ADC12INCH_13
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_14`"]
    #[inline(always)]
    pub fn is_adc12inch_14(&self) -> bool {
        **self == ADC12INCH_A::ADC12INCH_14
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_15`"]
    #[inline(always)]
    pub fn is_adc12inch_15(&self) -> bool {
        **self == ADC12INCH_A::ADC12INCH_15
    }
}
impl core::ops::Deref for ADC12INCH_R {
    type Target = crate::FieldReader<u8, ADC12INCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12INCH` writer - ADC12 Input Channel Select Bit 0"]
pub struct ADC12INCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12INCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12INCH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "ADC12 Input Channel 0"]
    #[inline(always)]
    pub fn adc12inch_0(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_0)
    }
    #[doc = "ADC12 Input Channel 1"]
    #[inline(always)]
    pub fn adc12inch_1(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_1)
    }
    #[doc = "ADC12 Input Channel 2"]
    #[inline(always)]
    pub fn adc12inch_2(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_2)
    }
    #[doc = "ADC12 Input Channel 3"]
    #[inline(always)]
    pub fn adc12inch_3(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_3)
    }
    #[doc = "ADC12 Input Channel 4"]
    #[inline(always)]
    pub fn adc12inch_4(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_4)
    }
    #[doc = "ADC12 Input Channel 5"]
    #[inline(always)]
    pub fn adc12inch_5(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_5)
    }
    #[doc = "ADC12 Input Channel 6"]
    #[inline(always)]
    pub fn adc12inch_6(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_6)
    }
    #[doc = "ADC12 Input Channel 7"]
    #[inline(always)]
    pub fn adc12inch_7(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_7)
    }
    #[doc = "ADC12 Input Channel 8"]
    #[inline(always)]
    pub fn adc12inch_8(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_8)
    }
    #[doc = "ADC12 Input Channel 9"]
    #[inline(always)]
    pub fn adc12inch_9(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_9)
    }
    #[doc = "ADC12 Input Channel 10"]
    #[inline(always)]
    pub fn adc12inch_10(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_10)
    }
    #[doc = "ADC12 Input Channel 11"]
    #[inline(always)]
    pub fn adc12inch_11(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_11)
    }
    #[doc = "ADC12 Input Channel 12"]
    #[inline(always)]
    pub fn adc12inch_12(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_12)
    }
    #[doc = "ADC12 Input Channel 13"]
    #[inline(always)]
    pub fn adc12inch_13(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_13)
    }
    #[doc = "ADC12 Input Channel 14"]
    #[inline(always)]
    pub fn adc12inch_14(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_14)
    }
    #[doc = "ADC12 Input Channel 15"]
    #[inline(always)]
    pub fn adc12inch_15(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
#[doc = "ADC12 Select Reference Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12SREF_A {
    #[doc = "0: ADC12 Select Reference 0"]
    ADC12SREF_0 = 0,
    #[doc = "1: ADC12 Select Reference 1"]
    ADC12SREF_1 = 1,
    #[doc = "2: ADC12 Select Reference 2"]
    ADC12SREF_2 = 2,
    #[doc = "3: ADC12 Select Reference 3"]
    ADC12SREF_3 = 3,
    #[doc = "4: ADC12 Select Reference 4"]
    ADC12SREF_4 = 4,
    #[doc = "5: ADC12 Select Reference 5"]
    ADC12SREF_5 = 5,
    #[doc = "6: ADC12 Select Reference 6"]
    ADC12SREF_6 = 6,
    #[doc = "7: ADC12 Select Reference 7"]
    ADC12SREF_7 = 7,
}
impl From<ADC12SREF_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12SREF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC12SREF` reader - ADC12 Select Reference Bit 0"]
pub struct ADC12SREF_R(crate::FieldReader<u8, ADC12SREF_A>);
impl ADC12SREF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC12SREF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12SREF_A {
        match self.bits {
            0 => ADC12SREF_A::ADC12SREF_0,
            1 => ADC12SREF_A::ADC12SREF_1,
            2 => ADC12SREF_A::ADC12SREF_2,
            3 => ADC12SREF_A::ADC12SREF_3,
            4 => ADC12SREF_A::ADC12SREF_4,
            5 => ADC12SREF_A::ADC12SREF_5,
            6 => ADC12SREF_A::ADC12SREF_6,
            7 => ADC12SREF_A::ADC12SREF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12SREF_0`"]
    #[inline(always)]
    pub fn is_adc12sref_0(&self) -> bool {
        **self == ADC12SREF_A::ADC12SREF_0
    }
    #[doc = "Checks if the value of the field is `ADC12SREF_1`"]
    #[inline(always)]
    pub fn is_adc12sref_1(&self) -> bool {
        **self == ADC12SREF_A::ADC12SREF_1
    }
    #[doc = "Checks if the value of the field is `ADC12SREF_2`"]
    #[inline(always)]
    pub fn is_adc12sref_2(&self) -> bool {
        **self == ADC12SREF_A::ADC12SREF_2
    }
    #[doc = "Checks if the value of the field is `ADC12SREF_3`"]
    #[inline(always)]
    pub fn is_adc12sref_3(&self) -> bool {
        **self == ADC12SREF_A::ADC12SREF_3
    }
    #[doc = "Checks if the value of the field is `ADC12SREF_4`"]
    #[inline(always)]
    pub fn is_adc12sref_4(&self) -> bool {
        **self == ADC12SREF_A::ADC12SREF_4
    }
    #[doc = "Checks if the value of the field is `ADC12SREF_5`"]
    #[inline(always)]
    pub fn is_adc12sref_5(&self) -> bool {
        **self == ADC12SREF_A::ADC12SREF_5
    }
    #[doc = "Checks if the value of the field is `ADC12SREF_6`"]
    #[inline(always)]
    pub fn is_adc12sref_6(&self) -> bool {
        **self == ADC12SREF_A::ADC12SREF_6
    }
    #[doc = "Checks if the value of the field is `ADC12SREF_7`"]
    #[inline(always)]
    pub fn is_adc12sref_7(&self) -> bool {
        **self == ADC12SREF_A::ADC12SREF_7
    }
}
impl core::ops::Deref for ADC12SREF_R {
    type Target = crate::FieldReader<u8, ADC12SREF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12SREF` writer - ADC12 Select Reference Bit 0"]
pub struct ADC12SREF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12SREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12SREF_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "ADC12 Select Reference 0"]
    #[inline(always)]
    pub fn adc12sref_0(self) -> &'a mut W {
        self.variant(ADC12SREF_A::ADC12SREF_0)
    }
    #[doc = "ADC12 Select Reference 1"]
    #[inline(always)]
    pub fn adc12sref_1(self) -> &'a mut W {
        self.variant(ADC12SREF_A::ADC12SREF_1)
    }
    #[doc = "ADC12 Select Reference 2"]
    #[inline(always)]
    pub fn adc12sref_2(self) -> &'a mut W {
        self.variant(ADC12SREF_A::ADC12SREF_2)
    }
    #[doc = "ADC12 Select Reference 3"]
    #[inline(always)]
    pub fn adc12sref_3(self) -> &'a mut W {
        self.variant(ADC12SREF_A::ADC12SREF_3)
    }
    #[doc = "ADC12 Select Reference 4"]
    #[inline(always)]
    pub fn adc12sref_4(self) -> &'a mut W {
        self.variant(ADC12SREF_A::ADC12SREF_4)
    }
    #[doc = "ADC12 Select Reference 5"]
    #[inline(always)]
    pub fn adc12sref_5(self) -> &'a mut W {
        self.variant(ADC12SREF_A::ADC12SREF_5)
    }
    #[doc = "ADC12 Select Reference 6"]
    #[inline(always)]
    pub fn adc12sref_6(self) -> &'a mut W {
        self.variant(ADC12SREF_A::ADC12SREF_6)
    }
    #[doc = "ADC12 Select Reference 7"]
    #[inline(always)]
    pub fn adc12sref_7(self) -> &'a mut W {
        self.variant(ADC12SREF_A::ADC12SREF_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u8 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `ADC12EOS` reader - ADC12 End of Sequence"]
pub struct ADC12EOS_R(crate::FieldReader<bool, bool>);
impl ADC12EOS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC12EOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12EOS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12EOS` writer - ADC12 End of Sequence"]
pub struct ADC12EOS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12EOS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC12 Input Channel Select Bit 0"]
    #[inline(always)]
    pub fn adc12inch(&self) -> ADC12INCH_R {
        ADC12INCH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - ADC12 Select Reference Bit 0"]
    #[inline(always)]
    pub fn adc12sref(&self) -> ADC12SREF_R {
        ADC12SREF_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - ADC12 End of Sequence"]
    #[inline(always)]
    pub fn adc12eos(&self) -> ADC12EOS_R {
        ADC12EOS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC12 Input Channel Select Bit 0"]
    #[inline(always)]
    pub fn adc12inch(&mut self) -> ADC12INCH_W {
        ADC12INCH_W { w: self }
    }
    #[doc = "Bits 4:6 - ADC12 Select Reference Bit 0"]
    #[inline(always)]
    pub fn adc12sref(&mut self) -> ADC12SREF_W {
        ADC12SREF_W { w: self }
    }
    #[doc = "Bit 7 - ADC12 End of Sequence"]
    #[inline(always)]
    pub fn adc12eos(&mut self) -> ADC12EOS_W {
        ADC12EOS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC12 Memory Control 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl15](index.html) module"]
pub struct ADC12MCTL15_SPEC;
impl crate::RegisterSpec for ADC12MCTL15_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adc12mctl15::R](R) reader structure"]
impl crate::Readable for ADC12MCTL15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc12mctl15::W](W) writer structure"]
impl crate::Writable for ADC12MCTL15_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC12MCTL15 to value 0"]
impl crate::Resettable for ADC12MCTL15_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
