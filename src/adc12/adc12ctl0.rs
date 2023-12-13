#[doc = "Register `ADC12CTL0` reader"]
pub type R = crate::R<ADC12CTL0_SPEC>;
#[doc = "Register `ADC12CTL0` writer"]
pub type W = crate::W<ADC12CTL0_SPEC>;
#[doc = "Field `ADC12SC` reader - ADC12 Start Conversion"]
pub type ADC12SC_R = crate::BitReader;
#[doc = "Field `ADC12SC` writer - ADC12 Start Conversion"]
pub type ADC12SC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12ENC` reader - ADC12 Enable Conversion"]
pub type ADC12ENC_R = crate::BitReader;
#[doc = "Field `ADC12ENC` writer - ADC12 Enable Conversion"]
pub type ADC12ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12TOVIE` reader - ADC12 Timer Overflow interrupt enable"]
pub type ADC12TOVIE_R = crate::BitReader;
#[doc = "Field `ADC12TOVIE` writer - ADC12 Timer Overflow interrupt enable"]
pub type ADC12TOVIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12OVIE` reader - ADC12 Overflow interrupt enable"]
pub type ADC12OVIE_R = crate::BitReader;
#[doc = "Field `ADC12OVIE` writer - ADC12 Overflow interrupt enable"]
pub type ADC12OVIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12ON` reader - ADC12 On/enable"]
pub type ADC12ON_R = crate::BitReader;
#[doc = "Field `ADC12ON` writer - ADC12 On/enable"]
pub type ADC12ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12REFON` reader - ADC12 Reference on"]
pub type ADC12REFON_R = crate::BitReader;
#[doc = "Field `ADC12REFON` writer - ADC12 Reference on"]
pub type ADC12REFON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12REF2_5V` reader - ADC12 Ref 0:1.5V / 1:2.5V"]
pub type ADC12REF2_5V_R = crate::BitReader;
#[doc = "Field `ADC12REF2_5V` writer - ADC12 Ref 0:1.5V / 1:2.5V"]
pub type ADC12REF2_5V_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12MSC` reader - ADC12 Multiple SampleConversion"]
pub type ADC12MSC_R = crate::BitReader;
#[doc = "Field `ADC12MSC` writer - ADC12 Multiple SampleConversion"]
pub type ADC12MSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12SHT0` reader - ADC12 Sample Hold 0 Select Bit: 0"]
pub type ADC12SHT0_R = crate::FieldReader<ADC12SHT0_A>;
#[doc = "ADC12 Sample Hold 0 Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for ADC12SHT0_A {
    type Ux = u8;
}
impl ADC12SHT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC12SHT0_A {
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
    #[doc = "ADC12 Sample Hold 0 Select Bit: 0"]
    #[inline(always)]
    pub fn is_adc12sht0_0(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_0
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 1"]
    #[inline(always)]
    pub fn is_adc12sht0_1(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_1
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 2"]
    #[inline(always)]
    pub fn is_adc12sht0_2(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_2
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 3"]
    #[inline(always)]
    pub fn is_adc12sht0_3(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_3
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 4"]
    #[inline(always)]
    pub fn is_adc12sht0_4(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_4
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 5"]
    #[inline(always)]
    pub fn is_adc12sht0_5(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_5
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 6"]
    #[inline(always)]
    pub fn is_adc12sht0_6(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_6
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 7"]
    #[inline(always)]
    pub fn is_adc12sht0_7(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_7
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 8"]
    #[inline(always)]
    pub fn is_adc12sht0_8(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_8
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 9"]
    #[inline(always)]
    pub fn is_adc12sht0_9(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_9
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 10"]
    #[inline(always)]
    pub fn is_adc12sht0_10(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_10
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 11"]
    #[inline(always)]
    pub fn is_adc12sht0_11(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_11
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 12"]
    #[inline(always)]
    pub fn is_adc12sht0_12(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_12
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 13"]
    #[inline(always)]
    pub fn is_adc12sht0_13(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_13
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 14"]
    #[inline(always)]
    pub fn is_adc12sht0_14(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_14
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 15"]
    #[inline(always)]
    pub fn is_adc12sht0_15(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_15
    }
}
#[doc = "Field `ADC12SHT0` writer - ADC12 Sample Hold 0 Select Bit: 0"]
pub type ADC12SHT0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, ADC12SHT0_A>;
impl<'a, REG> ADC12SHT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC12 Sample Hold 0 Select Bit: 0"]
    #[inline(always)]
    pub fn adc12sht0_0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT0_A::ADC12SHT0_0)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 1"]
    #[inline(always)]
    pub fn adc12sht0_1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT0_A::ADC12SHT0_1)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 2"]
    #[inline(always)]
    pub fn adc12sht0_2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT0_A::ADC12SHT0_2)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 3"]
    #[inline(always)]
    pub fn adc12sht0_3(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT0_A::ADC12SHT0_3)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 4"]
    #[inline(always)]
    pub fn adc12sht0_4(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT0_A::ADC12SHT0_4)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 5"]
    #[inline(always)]
    pub fn adc12sht0_5(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT0_A::ADC12SHT0_5)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 6"]
    #[inline(always)]
    pub fn adc12sht0_6(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT0_A::ADC12SHT0_6)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 7"]
    #[inline(always)]
    pub fn adc12sht0_7(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT0_A::ADC12SHT0_7)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 8"]
    #[inline(always)]
    pub fn adc12sht0_8(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT0_A::ADC12SHT0_8)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 9"]
    #[inline(always)]
    pub fn adc12sht0_9(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT0_A::ADC12SHT0_9)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 10"]
    #[inline(always)]
    pub fn adc12sht0_10(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT0_A::ADC12SHT0_10)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 11"]
    #[inline(always)]
    pub fn adc12sht0_11(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT0_A::ADC12SHT0_11)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 12"]
    #[inline(always)]
    pub fn adc12sht0_12(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT0_A::ADC12SHT0_12)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 13"]
    #[inline(always)]
    pub fn adc12sht0_13(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT0_A::ADC12SHT0_13)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 14"]
    #[inline(always)]
    pub fn adc12sht0_14(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT0_A::ADC12SHT0_14)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 15"]
    #[inline(always)]
    pub fn adc12sht0_15(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT0_A::ADC12SHT0_15)
    }
}
#[doc = "Field `ADC12SHT1` reader - ADC12 Sample Hold 1 Select Bit: 0"]
pub type ADC12SHT1_R = crate::FieldReader<ADC12SHT1_A>;
#[doc = "ADC12 Sample Hold 1 Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for ADC12SHT1_A {
    type Ux = u8;
}
impl ADC12SHT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC12SHT1_A {
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
    #[doc = "ADC12 Sample Hold 1 Select Bit: 0"]
    #[inline(always)]
    pub fn is_adc12sht1_0(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_0
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 1"]
    #[inline(always)]
    pub fn is_adc12sht1_1(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_1
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 2"]
    #[inline(always)]
    pub fn is_adc12sht1_2(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_2
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 3"]
    #[inline(always)]
    pub fn is_adc12sht1_3(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_3
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 4"]
    #[inline(always)]
    pub fn is_adc12sht1_4(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_4
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 5"]
    #[inline(always)]
    pub fn is_adc12sht1_5(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_5
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 6"]
    #[inline(always)]
    pub fn is_adc12sht1_6(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_6
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 7"]
    #[inline(always)]
    pub fn is_adc12sht1_7(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_7
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 8"]
    #[inline(always)]
    pub fn is_adc12sht1_8(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_8
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 9"]
    #[inline(always)]
    pub fn is_adc12sht1_9(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_9
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 10"]
    #[inline(always)]
    pub fn is_adc12sht1_10(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_10
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 11"]
    #[inline(always)]
    pub fn is_adc12sht1_11(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_11
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 12"]
    #[inline(always)]
    pub fn is_adc12sht1_12(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_12
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 13"]
    #[inline(always)]
    pub fn is_adc12sht1_13(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_13
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 14"]
    #[inline(always)]
    pub fn is_adc12sht1_14(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_14
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 15"]
    #[inline(always)]
    pub fn is_adc12sht1_15(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_15
    }
}
#[doc = "Field `ADC12SHT1` writer - ADC12 Sample Hold 1 Select Bit: 0"]
pub type ADC12SHT1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, ADC12SHT1_A>;
impl<'a, REG> ADC12SHT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC12 Sample Hold 1 Select Bit: 0"]
    #[inline(always)]
    pub fn adc12sht1_0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT1_A::ADC12SHT1_0)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 1"]
    #[inline(always)]
    pub fn adc12sht1_1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT1_A::ADC12SHT1_1)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 2"]
    #[inline(always)]
    pub fn adc12sht1_2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT1_A::ADC12SHT1_2)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 3"]
    #[inline(always)]
    pub fn adc12sht1_3(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT1_A::ADC12SHT1_3)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 4"]
    #[inline(always)]
    pub fn adc12sht1_4(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT1_A::ADC12SHT1_4)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 5"]
    #[inline(always)]
    pub fn adc12sht1_5(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT1_A::ADC12SHT1_5)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 6"]
    #[inline(always)]
    pub fn adc12sht1_6(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT1_A::ADC12SHT1_6)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 7"]
    #[inline(always)]
    pub fn adc12sht1_7(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT1_A::ADC12SHT1_7)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 8"]
    #[inline(always)]
    pub fn adc12sht1_8(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT1_A::ADC12SHT1_8)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 9"]
    #[inline(always)]
    pub fn adc12sht1_9(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT1_A::ADC12SHT1_9)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 10"]
    #[inline(always)]
    pub fn adc12sht1_10(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT1_A::ADC12SHT1_10)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 11"]
    #[inline(always)]
    pub fn adc12sht1_11(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT1_A::ADC12SHT1_11)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 12"]
    #[inline(always)]
    pub fn adc12sht1_12(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT1_A::ADC12SHT1_12)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 13"]
    #[inline(always)]
    pub fn adc12sht1_13(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT1_A::ADC12SHT1_13)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 14"]
    #[inline(always)]
    pub fn adc12sht1_14(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT1_A::ADC12SHT1_14)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 15"]
    #[inline(always)]
    pub fn adc12sht1_15(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SHT1_A::ADC12SHT1_15)
    }
}
impl R {
    #[doc = "Bit 0 - ADC12 Start Conversion"]
    #[inline(always)]
    pub fn adc12sc(&self) -> ADC12SC_R {
        ADC12SC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC12 Enable Conversion"]
    #[inline(always)]
    pub fn adc12enc(&self) -> ADC12ENC_R {
        ADC12ENC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC12 Timer Overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12tovie(&self) -> ADC12TOVIE_R {
        ADC12TOVIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC12 Overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12ovie(&self) -> ADC12OVIE_R {
        ADC12OVIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC12 On/enable"]
    #[inline(always)]
    pub fn adc12on(&self) -> ADC12ON_R {
        ADC12ON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC12 Reference on"]
    #[inline(always)]
    pub fn adc12refon(&self) -> ADC12REFON_R {
        ADC12REFON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC12 Ref 0:1.5V / 1:2.5V"]
    #[inline(always)]
    pub fn adc12ref2_5v(&self) -> ADC12REF2_5V_R {
        ADC12REF2_5V_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC12 Multiple SampleConversion"]
    #[inline(always)]
    pub fn adc12msc(&self) -> ADC12MSC_R {
        ADC12MSC_R::new(((self.bits >> 7) & 1) != 0)
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
    #[must_use]
    pub fn adc12sc(&mut self) -> ADC12SC_W<ADC12CTL0_SPEC> {
        ADC12SC_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC12 Enable Conversion"]
    #[inline(always)]
    #[must_use]
    pub fn adc12enc(&mut self) -> ADC12ENC_W<ADC12CTL0_SPEC> {
        ADC12ENC_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC12 Timer Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12tovie(&mut self) -> ADC12TOVIE_W<ADC12CTL0_SPEC> {
        ADC12TOVIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC12 Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ovie(&mut self) -> ADC12OVIE_W<ADC12CTL0_SPEC> {
        ADC12OVIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC12 On/enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12on(&mut self) -> ADC12ON_W<ADC12CTL0_SPEC> {
        ADC12ON_W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC12 Reference on"]
    #[inline(always)]
    #[must_use]
    pub fn adc12refon(&mut self) -> ADC12REFON_W<ADC12CTL0_SPEC> {
        ADC12REFON_W::new(self, 5)
    }
    #[doc = "Bit 6 - ADC12 Ref 0:1.5V / 1:2.5V"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ref2_5v(&mut self) -> ADC12REF2_5V_W<ADC12CTL0_SPEC> {
        ADC12REF2_5V_W::new(self, 6)
    }
    #[doc = "Bit 7 - ADC12 Multiple SampleConversion"]
    #[inline(always)]
    #[must_use]
    pub fn adc12msc(&mut self) -> ADC12MSC_W<ADC12CTL0_SPEC> {
        ADC12MSC_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - ADC12 Sample Hold 0 Select Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn adc12sht0(&mut self) -> ADC12SHT0_W<ADC12CTL0_SPEC> {
        ADC12SHT0_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - ADC12 Sample Hold 1 Select Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn adc12sht1(&mut self) -> ADC12SHT1_W<ADC12CTL0_SPEC> {
        ADC12SHT1_W::new(self, 12)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC12+ Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC12CTL0_SPEC;
impl crate::RegisterSpec for ADC12CTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc12ctl0::R`](R) reader structure"]
impl crate::Readable for ADC12CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc12ctl0::W`](W) writer structure"]
impl crate::Writable for ADC12CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC12CTL0 to value 0"]
impl crate::Resettable for ADC12CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
