#[doc = "Reader of register ADC12MCTL6"]
pub type R = crate::R<u8, super::ADC12MCTL6>;
#[doc = "Writer for register ADC12MCTL6"]
pub type W = crate::W<u8, super::ADC12MCTL6>;
#[doc = "Register ADC12MCTL6 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12MCTL6 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `ADC12INCH`"]
pub type ADC12INCH_R = crate::R<u8, ADC12INCH_A>;
impl ADC12INCH_R {
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
        *self == ADC12INCH_A::ADC12INCH_0
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_1`"]
    #[inline(always)]
    pub fn is_adc12inch_1(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_1
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_2`"]
    #[inline(always)]
    pub fn is_adc12inch_2(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_2
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_3`"]
    #[inline(always)]
    pub fn is_adc12inch_3(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_3
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_4`"]
    #[inline(always)]
    pub fn is_adc12inch_4(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_4
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_5`"]
    #[inline(always)]
    pub fn is_adc12inch_5(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_5
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_6`"]
    #[inline(always)]
    pub fn is_adc12inch_6(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_6
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_7`"]
    #[inline(always)]
    pub fn is_adc12inch_7(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_7
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_8`"]
    #[inline(always)]
    pub fn is_adc12inch_8(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_8
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_9`"]
    #[inline(always)]
    pub fn is_adc12inch_9(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_9
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_10`"]
    #[inline(always)]
    pub fn is_adc12inch_10(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_10
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_11`"]
    #[inline(always)]
    pub fn is_adc12inch_11(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_11
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_12`"]
    #[inline(always)]
    pub fn is_adc12inch_12(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_12
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_13`"]
    #[inline(always)]
    pub fn is_adc12inch_13(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_13
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_14`"]
    #[inline(always)]
    pub fn is_adc12inch_14(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_14
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_15`"]
    #[inline(always)]
    pub fn is_adc12inch_15(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_15
    }
}
#[doc = "Write proxy for field `ADC12INCH`"]
pub struct ADC12INCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12INCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12INCH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
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
#[doc = "Reader of field `ADC12SREF`"]
pub type ADC12SREF_R = crate::R<u8, ADC12SREF_A>;
impl ADC12SREF_R {
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
        *self == ADC12SREF_A::ADC12SREF_0
    }
    #[doc = "Checks if the value of the field is `ADC12SREF_1`"]
    #[inline(always)]
    pub fn is_adc12sref_1(&self) -> bool {
        *self == ADC12SREF_A::ADC12SREF_1
    }
    #[doc = "Checks if the value of the field is `ADC12SREF_2`"]
    #[inline(always)]
    pub fn is_adc12sref_2(&self) -> bool {
        *self == ADC12SREF_A::ADC12SREF_2
    }
    #[doc = "Checks if the value of the field is `ADC12SREF_3`"]
    #[inline(always)]
    pub fn is_adc12sref_3(&self) -> bool {
        *self == ADC12SREF_A::ADC12SREF_3
    }
    #[doc = "Checks if the value of the field is `ADC12SREF_4`"]
    #[inline(always)]
    pub fn is_adc12sref_4(&self) -> bool {
        *self == ADC12SREF_A::ADC12SREF_4
    }
    #[doc = "Checks if the value of the field is `ADC12SREF_5`"]
    #[inline(always)]
    pub fn is_adc12sref_5(&self) -> bool {
        *self == ADC12SREF_A::ADC12SREF_5
    }
    #[doc = "Checks if the value of the field is `ADC12SREF_6`"]
    #[inline(always)]
    pub fn is_adc12sref_6(&self) -> bool {
        *self == ADC12SREF_A::ADC12SREF_6
    }
    #[doc = "Checks if the value of the field is `ADC12SREF_7`"]
    #[inline(always)]
    pub fn is_adc12sref_7(&self) -> bool {
        *self == ADC12SREF_A::ADC12SREF_7
    }
}
#[doc = "Write proxy for field `ADC12SREF`"]
pub struct ADC12SREF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12SREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12SREF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u8) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `ADC12EOS`"]
pub type ADC12EOS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12EOS`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
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
}
