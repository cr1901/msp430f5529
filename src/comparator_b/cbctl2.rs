#[doc = "Register `CBCTL2` reader"]
pub struct R(crate::R<CBCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CBCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CBCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CBCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CBCTL2` writer"]
pub struct W(crate::W<CBCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CBCTL2_SPEC>;
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
impl From<crate::W<CBCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CBCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Comp. B Reference 0 Resistor Select Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CBREF0_A {
    #[doc = "0: Comp. B Int. Ref.0 Select 0 : 1/32"]
    CBREF0_0 = 0,
    #[doc = "1: Comp. B Int. Ref.0 Select 1 : 2/32"]
    CBREF0_1 = 1,
    #[doc = "2: Comp. B Int. Ref.0 Select 2 : 3/32"]
    CBREF0_2 = 2,
    #[doc = "3: Comp. B Int. Ref.0 Select 3 : 4/32"]
    CBREF0_3 = 3,
    #[doc = "4: Comp. B Int. Ref.0 Select 4 : 5/32"]
    CBREF0_4 = 4,
    #[doc = "5: Comp. B Int. Ref.0 Select 5 : 6/32"]
    CBREF0_5 = 5,
    #[doc = "6: Comp. B Int. Ref.0 Select 6 : 7/32"]
    CBREF0_6 = 6,
    #[doc = "7: Comp. B Int. Ref.0 Select 7 : 8/32"]
    CBREF0_7 = 7,
    #[doc = "8: Comp. B Int. Ref.0 Select 0 : 9/32"]
    CBREF0_8 = 8,
    #[doc = "9: Comp. B Int. Ref.0 Select 1 : 10/32"]
    CBREF0_9 = 9,
    #[doc = "10: Comp. B Int. Ref.0 Select 2 : 11/32"]
    CBREF0_10 = 10,
    #[doc = "11: Comp. B Int. Ref.0 Select 3 : 12/32"]
    CBREF0_11 = 11,
    #[doc = "12: Comp. B Int. Ref.0 Select 4 : 13/32"]
    CBREF0_12 = 12,
    #[doc = "13: Comp. B Int. Ref.0 Select 5 : 14/32"]
    CBREF0_13 = 13,
    #[doc = "14: Comp. B Int. Ref.0 Select 6 : 15/32"]
    CBREF0_14 = 14,
    #[doc = "15: Comp. B Int. Ref.0 Select 7 : 16/32"]
    CBREF0_15 = 15,
    #[doc = "16: Comp. B Int. Ref.0 Select 0 : 17/32"]
    CBREF0_16 = 16,
    #[doc = "17: Comp. B Int. Ref.0 Select 1 : 18/32"]
    CBREF0_17 = 17,
    #[doc = "18: Comp. B Int. Ref.0 Select 2 : 19/32"]
    CBREF0_18 = 18,
    #[doc = "19: Comp. B Int. Ref.0 Select 3 : 20/32"]
    CBREF0_19 = 19,
    #[doc = "20: Comp. B Int. Ref.0 Select 4 : 21/32"]
    CBREF0_20 = 20,
    #[doc = "21: Comp. B Int. Ref.0 Select 5 : 22/32"]
    CBREF0_21 = 21,
    #[doc = "22: Comp. B Int. Ref.0 Select 6 : 23/32"]
    CBREF0_22 = 22,
    #[doc = "23: Comp. B Int. Ref.0 Select 7 : 24/32"]
    CBREF0_23 = 23,
    #[doc = "24: Comp. B Int. Ref.0 Select 0 : 25/32"]
    CBREF0_24 = 24,
    #[doc = "25: Comp. B Int. Ref.0 Select 1 : 26/32"]
    CBREF0_25 = 25,
    #[doc = "26: Comp. B Int. Ref.0 Select 2 : 27/32"]
    CBREF0_26 = 26,
    #[doc = "27: Comp. B Int. Ref.0 Select 3 : 28/32"]
    CBREF0_27 = 27,
    #[doc = "28: Comp. B Int. Ref.0 Select 4 : 29/32"]
    CBREF0_28 = 28,
    #[doc = "29: Comp. B Int. Ref.0 Select 5 : 30/32"]
    CBREF0_29 = 29,
    #[doc = "30: Comp. B Int. Ref.0 Select 6 : 31/32"]
    CBREF0_30 = 30,
    #[doc = "31: Comp. B Int. Ref.0 Select 7 : 32/32"]
    CBREF0_31 = 31,
}
impl From<CBREF0_A> for u8 {
    #[inline(always)]
    fn from(variant: CBREF0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CBREF0` reader - Comp. B Reference 0 Resistor Select Bit : 0"]
pub struct CBREF0_R(crate::FieldReader<u8, CBREF0_A>);
impl CBREF0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CBREF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBREF0_A {
        match self.bits {
            0 => CBREF0_A::CBREF0_0,
            1 => CBREF0_A::CBREF0_1,
            2 => CBREF0_A::CBREF0_2,
            3 => CBREF0_A::CBREF0_3,
            4 => CBREF0_A::CBREF0_4,
            5 => CBREF0_A::CBREF0_5,
            6 => CBREF0_A::CBREF0_6,
            7 => CBREF0_A::CBREF0_7,
            8 => CBREF0_A::CBREF0_8,
            9 => CBREF0_A::CBREF0_9,
            10 => CBREF0_A::CBREF0_10,
            11 => CBREF0_A::CBREF0_11,
            12 => CBREF0_A::CBREF0_12,
            13 => CBREF0_A::CBREF0_13,
            14 => CBREF0_A::CBREF0_14,
            15 => CBREF0_A::CBREF0_15,
            16 => CBREF0_A::CBREF0_16,
            17 => CBREF0_A::CBREF0_17,
            18 => CBREF0_A::CBREF0_18,
            19 => CBREF0_A::CBREF0_19,
            20 => CBREF0_A::CBREF0_20,
            21 => CBREF0_A::CBREF0_21,
            22 => CBREF0_A::CBREF0_22,
            23 => CBREF0_A::CBREF0_23,
            24 => CBREF0_A::CBREF0_24,
            25 => CBREF0_A::CBREF0_25,
            26 => CBREF0_A::CBREF0_26,
            27 => CBREF0_A::CBREF0_27,
            28 => CBREF0_A::CBREF0_28,
            29 => CBREF0_A::CBREF0_29,
            30 => CBREF0_A::CBREF0_30,
            31 => CBREF0_A::CBREF0_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CBREF0_0`"]
    #[inline(always)]
    pub fn is_cbref0_0(&self) -> bool {
        **self == CBREF0_A::CBREF0_0
    }
    #[doc = "Checks if the value of the field is `CBREF0_1`"]
    #[inline(always)]
    pub fn is_cbref0_1(&self) -> bool {
        **self == CBREF0_A::CBREF0_1
    }
    #[doc = "Checks if the value of the field is `CBREF0_2`"]
    #[inline(always)]
    pub fn is_cbref0_2(&self) -> bool {
        **self == CBREF0_A::CBREF0_2
    }
    #[doc = "Checks if the value of the field is `CBREF0_3`"]
    #[inline(always)]
    pub fn is_cbref0_3(&self) -> bool {
        **self == CBREF0_A::CBREF0_3
    }
    #[doc = "Checks if the value of the field is `CBREF0_4`"]
    #[inline(always)]
    pub fn is_cbref0_4(&self) -> bool {
        **self == CBREF0_A::CBREF0_4
    }
    #[doc = "Checks if the value of the field is `CBREF0_5`"]
    #[inline(always)]
    pub fn is_cbref0_5(&self) -> bool {
        **self == CBREF0_A::CBREF0_5
    }
    #[doc = "Checks if the value of the field is `CBREF0_6`"]
    #[inline(always)]
    pub fn is_cbref0_6(&self) -> bool {
        **self == CBREF0_A::CBREF0_6
    }
    #[doc = "Checks if the value of the field is `CBREF0_7`"]
    #[inline(always)]
    pub fn is_cbref0_7(&self) -> bool {
        **self == CBREF0_A::CBREF0_7
    }
    #[doc = "Checks if the value of the field is `CBREF0_8`"]
    #[inline(always)]
    pub fn is_cbref0_8(&self) -> bool {
        **self == CBREF0_A::CBREF0_8
    }
    #[doc = "Checks if the value of the field is `CBREF0_9`"]
    #[inline(always)]
    pub fn is_cbref0_9(&self) -> bool {
        **self == CBREF0_A::CBREF0_9
    }
    #[doc = "Checks if the value of the field is `CBREF0_10`"]
    #[inline(always)]
    pub fn is_cbref0_10(&self) -> bool {
        **self == CBREF0_A::CBREF0_10
    }
    #[doc = "Checks if the value of the field is `CBREF0_11`"]
    #[inline(always)]
    pub fn is_cbref0_11(&self) -> bool {
        **self == CBREF0_A::CBREF0_11
    }
    #[doc = "Checks if the value of the field is `CBREF0_12`"]
    #[inline(always)]
    pub fn is_cbref0_12(&self) -> bool {
        **self == CBREF0_A::CBREF0_12
    }
    #[doc = "Checks if the value of the field is `CBREF0_13`"]
    #[inline(always)]
    pub fn is_cbref0_13(&self) -> bool {
        **self == CBREF0_A::CBREF0_13
    }
    #[doc = "Checks if the value of the field is `CBREF0_14`"]
    #[inline(always)]
    pub fn is_cbref0_14(&self) -> bool {
        **self == CBREF0_A::CBREF0_14
    }
    #[doc = "Checks if the value of the field is `CBREF0_15`"]
    #[inline(always)]
    pub fn is_cbref0_15(&self) -> bool {
        **self == CBREF0_A::CBREF0_15
    }
    #[doc = "Checks if the value of the field is `CBREF0_16`"]
    #[inline(always)]
    pub fn is_cbref0_16(&self) -> bool {
        **self == CBREF0_A::CBREF0_16
    }
    #[doc = "Checks if the value of the field is `CBREF0_17`"]
    #[inline(always)]
    pub fn is_cbref0_17(&self) -> bool {
        **self == CBREF0_A::CBREF0_17
    }
    #[doc = "Checks if the value of the field is `CBREF0_18`"]
    #[inline(always)]
    pub fn is_cbref0_18(&self) -> bool {
        **self == CBREF0_A::CBREF0_18
    }
    #[doc = "Checks if the value of the field is `CBREF0_19`"]
    #[inline(always)]
    pub fn is_cbref0_19(&self) -> bool {
        **self == CBREF0_A::CBREF0_19
    }
    #[doc = "Checks if the value of the field is `CBREF0_20`"]
    #[inline(always)]
    pub fn is_cbref0_20(&self) -> bool {
        **self == CBREF0_A::CBREF0_20
    }
    #[doc = "Checks if the value of the field is `CBREF0_21`"]
    #[inline(always)]
    pub fn is_cbref0_21(&self) -> bool {
        **self == CBREF0_A::CBREF0_21
    }
    #[doc = "Checks if the value of the field is `CBREF0_22`"]
    #[inline(always)]
    pub fn is_cbref0_22(&self) -> bool {
        **self == CBREF0_A::CBREF0_22
    }
    #[doc = "Checks if the value of the field is `CBREF0_23`"]
    #[inline(always)]
    pub fn is_cbref0_23(&self) -> bool {
        **self == CBREF0_A::CBREF0_23
    }
    #[doc = "Checks if the value of the field is `CBREF0_24`"]
    #[inline(always)]
    pub fn is_cbref0_24(&self) -> bool {
        **self == CBREF0_A::CBREF0_24
    }
    #[doc = "Checks if the value of the field is `CBREF0_25`"]
    #[inline(always)]
    pub fn is_cbref0_25(&self) -> bool {
        **self == CBREF0_A::CBREF0_25
    }
    #[doc = "Checks if the value of the field is `CBREF0_26`"]
    #[inline(always)]
    pub fn is_cbref0_26(&self) -> bool {
        **self == CBREF0_A::CBREF0_26
    }
    #[doc = "Checks if the value of the field is `CBREF0_27`"]
    #[inline(always)]
    pub fn is_cbref0_27(&self) -> bool {
        **self == CBREF0_A::CBREF0_27
    }
    #[doc = "Checks if the value of the field is `CBREF0_28`"]
    #[inline(always)]
    pub fn is_cbref0_28(&self) -> bool {
        **self == CBREF0_A::CBREF0_28
    }
    #[doc = "Checks if the value of the field is `CBREF0_29`"]
    #[inline(always)]
    pub fn is_cbref0_29(&self) -> bool {
        **self == CBREF0_A::CBREF0_29
    }
    #[doc = "Checks if the value of the field is `CBREF0_30`"]
    #[inline(always)]
    pub fn is_cbref0_30(&self) -> bool {
        **self == CBREF0_A::CBREF0_30
    }
    #[doc = "Checks if the value of the field is `CBREF0_31`"]
    #[inline(always)]
    pub fn is_cbref0_31(&self) -> bool {
        **self == CBREF0_A::CBREF0_31
    }
}
impl core::ops::Deref for CBREF0_R {
    type Target = crate::FieldReader<u8, CBREF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBREF0` writer - Comp. B Reference 0 Resistor Select Bit : 0"]
pub struct CBREF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CBREF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBREF0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Comp. B Int. Ref.0 Select 0 : 1/32"]
    #[inline(always)]
    pub fn cbref0_0(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_0)
    }
    #[doc = "Comp. B Int. Ref.0 Select 1 : 2/32"]
    #[inline(always)]
    pub fn cbref0_1(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_1)
    }
    #[doc = "Comp. B Int. Ref.0 Select 2 : 3/32"]
    #[inline(always)]
    pub fn cbref0_2(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_2)
    }
    #[doc = "Comp. B Int. Ref.0 Select 3 : 4/32"]
    #[inline(always)]
    pub fn cbref0_3(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_3)
    }
    #[doc = "Comp. B Int. Ref.0 Select 4 : 5/32"]
    #[inline(always)]
    pub fn cbref0_4(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_4)
    }
    #[doc = "Comp. B Int. Ref.0 Select 5 : 6/32"]
    #[inline(always)]
    pub fn cbref0_5(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_5)
    }
    #[doc = "Comp. B Int. Ref.0 Select 6 : 7/32"]
    #[inline(always)]
    pub fn cbref0_6(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_6)
    }
    #[doc = "Comp. B Int. Ref.0 Select 7 : 8/32"]
    #[inline(always)]
    pub fn cbref0_7(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_7)
    }
    #[doc = "Comp. B Int. Ref.0 Select 0 : 9/32"]
    #[inline(always)]
    pub fn cbref0_8(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_8)
    }
    #[doc = "Comp. B Int. Ref.0 Select 1 : 10/32"]
    #[inline(always)]
    pub fn cbref0_9(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_9)
    }
    #[doc = "Comp. B Int. Ref.0 Select 2 : 11/32"]
    #[inline(always)]
    pub fn cbref0_10(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_10)
    }
    #[doc = "Comp. B Int. Ref.0 Select 3 : 12/32"]
    #[inline(always)]
    pub fn cbref0_11(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_11)
    }
    #[doc = "Comp. B Int. Ref.0 Select 4 : 13/32"]
    #[inline(always)]
    pub fn cbref0_12(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_12)
    }
    #[doc = "Comp. B Int. Ref.0 Select 5 : 14/32"]
    #[inline(always)]
    pub fn cbref0_13(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_13)
    }
    #[doc = "Comp. B Int. Ref.0 Select 6 : 15/32"]
    #[inline(always)]
    pub fn cbref0_14(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_14)
    }
    #[doc = "Comp. B Int. Ref.0 Select 7 : 16/32"]
    #[inline(always)]
    pub fn cbref0_15(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_15)
    }
    #[doc = "Comp. B Int. Ref.0 Select 0 : 17/32"]
    #[inline(always)]
    pub fn cbref0_16(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_16)
    }
    #[doc = "Comp. B Int. Ref.0 Select 1 : 18/32"]
    #[inline(always)]
    pub fn cbref0_17(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_17)
    }
    #[doc = "Comp. B Int. Ref.0 Select 2 : 19/32"]
    #[inline(always)]
    pub fn cbref0_18(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_18)
    }
    #[doc = "Comp. B Int. Ref.0 Select 3 : 20/32"]
    #[inline(always)]
    pub fn cbref0_19(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_19)
    }
    #[doc = "Comp. B Int. Ref.0 Select 4 : 21/32"]
    #[inline(always)]
    pub fn cbref0_20(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_20)
    }
    #[doc = "Comp. B Int. Ref.0 Select 5 : 22/32"]
    #[inline(always)]
    pub fn cbref0_21(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_21)
    }
    #[doc = "Comp. B Int. Ref.0 Select 6 : 23/32"]
    #[inline(always)]
    pub fn cbref0_22(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_22)
    }
    #[doc = "Comp. B Int. Ref.0 Select 7 : 24/32"]
    #[inline(always)]
    pub fn cbref0_23(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_23)
    }
    #[doc = "Comp. B Int. Ref.0 Select 0 : 25/32"]
    #[inline(always)]
    pub fn cbref0_24(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_24)
    }
    #[doc = "Comp. B Int. Ref.0 Select 1 : 26/32"]
    #[inline(always)]
    pub fn cbref0_25(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_25)
    }
    #[doc = "Comp. B Int. Ref.0 Select 2 : 27/32"]
    #[inline(always)]
    pub fn cbref0_26(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_26)
    }
    #[doc = "Comp. B Int. Ref.0 Select 3 : 28/32"]
    #[inline(always)]
    pub fn cbref0_27(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_27)
    }
    #[doc = "Comp. B Int. Ref.0 Select 4 : 29/32"]
    #[inline(always)]
    pub fn cbref0_28(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_28)
    }
    #[doc = "Comp. B Int. Ref.0 Select 5 : 30/32"]
    #[inline(always)]
    pub fn cbref0_29(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_29)
    }
    #[doc = "Comp. B Int. Ref.0 Select 6 : 31/32"]
    #[inline(always)]
    pub fn cbref0_30(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_30)
    }
    #[doc = "Comp. B Int. Ref.0 Select 7 : 32/32"]
    #[inline(always)]
    pub fn cbref0_31(self) -> &'a mut W {
        self.variant(CBREF0_A::CBREF0_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u16 & 0x1f);
        self.w
    }
}
#[doc = "Field `CBRSEL` reader - Comp. B Reference select"]
pub struct CBRSEL_R(crate::FieldReader<bool, bool>);
impl CBRSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBRSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBRSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBRSEL` writer - Comp. B Reference select"]
pub struct CBRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CBRSEL_W<'a> {
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
#[doc = "Comp. B Reference Source Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CBRS_A {
    #[doc = "0: Comp. B Reference Source 0 : Off"]
    CBRS_0 = 0,
    #[doc = "1: Comp. B Reference Source 1 : Vcc"]
    CBRS_1 = 1,
    #[doc = "2: Comp. B Reference Source 2 : Shared Ref."]
    CBRS_2 = 2,
    #[doc = "3: Comp. B Reference Source 3 : Shared Ref. / Off"]
    CBRS_3 = 3,
}
impl From<CBRS_A> for u8 {
    #[inline(always)]
    fn from(variant: CBRS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CBRS` reader - Comp. B Reference Source Bit : 0"]
pub struct CBRS_R(crate::FieldReader<u8, CBRS_A>);
impl CBRS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CBRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBRS_A {
        match self.bits {
            0 => CBRS_A::CBRS_0,
            1 => CBRS_A::CBRS_1,
            2 => CBRS_A::CBRS_2,
            3 => CBRS_A::CBRS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CBRS_0`"]
    #[inline(always)]
    pub fn is_cbrs_0(&self) -> bool {
        **self == CBRS_A::CBRS_0
    }
    #[doc = "Checks if the value of the field is `CBRS_1`"]
    #[inline(always)]
    pub fn is_cbrs_1(&self) -> bool {
        **self == CBRS_A::CBRS_1
    }
    #[doc = "Checks if the value of the field is `CBRS_2`"]
    #[inline(always)]
    pub fn is_cbrs_2(&self) -> bool {
        **self == CBRS_A::CBRS_2
    }
    #[doc = "Checks if the value of the field is `CBRS_3`"]
    #[inline(always)]
    pub fn is_cbrs_3(&self) -> bool {
        **self == CBRS_A::CBRS_3
    }
}
impl core::ops::Deref for CBRS_R {
    type Target = crate::FieldReader<u8, CBRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBRS` writer - Comp. B Reference Source Bit : 0"]
pub struct CBRS_W<'a> {
    w: &'a mut W,
}
impl<'a> CBRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBRS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Comp. B Reference Source 0 : Off"]
    #[inline(always)]
    pub fn cbrs_0(self) -> &'a mut W {
        self.variant(CBRS_A::CBRS_0)
    }
    #[doc = "Comp. B Reference Source 1 : Vcc"]
    #[inline(always)]
    pub fn cbrs_1(self) -> &'a mut W {
        self.variant(CBRS_A::CBRS_1)
    }
    #[doc = "Comp. B Reference Source 2 : Shared Ref."]
    #[inline(always)]
    pub fn cbrs_2(self) -> &'a mut W {
        self.variant(CBRS_A::CBRS_2)
    }
    #[doc = "Comp. B Reference Source 3 : Shared Ref. / Off"]
    #[inline(always)]
    pub fn cbrs_3(self) -> &'a mut W {
        self.variant(CBRS_A::CBRS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u16 & 0x03) << 6);
        self.w
    }
}
#[doc = "Comp. B Reference 1 Resistor Select Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CBREF1_A {
    #[doc = "0: Comp. B Int. Ref.1 Select 0 : 1/32"]
    CBREF1_0 = 0,
    #[doc = "1: Comp. B Int. Ref.1 Select 1 : 2/32"]
    CBREF1_1 = 1,
    #[doc = "2: Comp. B Int. Ref.1 Select 2 : 3/32"]
    CBREF1_2 = 2,
    #[doc = "3: Comp. B Int. Ref.1 Select 3 : 4/32"]
    CBREF1_3 = 3,
    #[doc = "4: Comp. B Int. Ref.1 Select 4 : 5/32"]
    CBREF1_4 = 4,
    #[doc = "5: Comp. B Int. Ref.1 Select 5 : 6/32"]
    CBREF1_5 = 5,
    #[doc = "6: Comp. B Int. Ref.1 Select 6 : 7/32"]
    CBREF1_6 = 6,
    #[doc = "7: Comp. B Int. Ref.1 Select 7 : 8/32"]
    CBREF1_7 = 7,
    #[doc = "8: Comp. B Int. Ref.1 Select 0 : 9/32"]
    CBREF1_8 = 8,
    #[doc = "9: Comp. B Int. Ref.1 Select 1 : 10/32"]
    CBREF1_9 = 9,
    #[doc = "10: Comp. B Int. Ref.1 Select 2 : 11/32"]
    CBREF1_10 = 10,
    #[doc = "11: Comp. B Int. Ref.1 Select 3 : 12/32"]
    CBREF1_11 = 11,
    #[doc = "12: Comp. B Int. Ref.1 Select 4 : 13/32"]
    CBREF1_12 = 12,
    #[doc = "13: Comp. B Int. Ref.1 Select 5 : 14/32"]
    CBREF1_13 = 13,
    #[doc = "14: Comp. B Int. Ref.1 Select 6 : 15/32"]
    CBREF1_14 = 14,
    #[doc = "15: Comp. B Int. Ref.1 Select 7 : 16/32"]
    CBREF1_15 = 15,
    #[doc = "16: Comp. B Int. Ref.1 Select 0 : 17/32"]
    CBREF1_16 = 16,
    #[doc = "17: Comp. B Int. Ref.1 Select 1 : 18/32"]
    CBREF1_17 = 17,
    #[doc = "18: Comp. B Int. Ref.1 Select 2 : 19/32"]
    CBREF1_18 = 18,
    #[doc = "19: Comp. B Int. Ref.1 Select 3 : 20/32"]
    CBREF1_19 = 19,
    #[doc = "20: Comp. B Int. Ref.1 Select 4 : 21/32"]
    CBREF1_20 = 20,
    #[doc = "21: Comp. B Int. Ref.1 Select 5 : 22/32"]
    CBREF1_21 = 21,
    #[doc = "22: Comp. B Int. Ref.1 Select 6 : 23/32"]
    CBREF1_22 = 22,
    #[doc = "23: Comp. B Int. Ref.1 Select 7 : 24/32"]
    CBREF1_23 = 23,
    #[doc = "24: Comp. B Int. Ref.1 Select 0 : 25/32"]
    CBREF1_24 = 24,
    #[doc = "25: Comp. B Int. Ref.1 Select 1 : 26/32"]
    CBREF1_25 = 25,
    #[doc = "26: Comp. B Int. Ref.1 Select 2 : 27/32"]
    CBREF1_26 = 26,
    #[doc = "27: Comp. B Int. Ref.1 Select 3 : 28/32"]
    CBREF1_27 = 27,
    #[doc = "28: Comp. B Int. Ref.1 Select 4 : 29/32"]
    CBREF1_28 = 28,
    #[doc = "29: Comp. B Int. Ref.1 Select 5 : 30/32"]
    CBREF1_29 = 29,
    #[doc = "30: Comp. B Int. Ref.1 Select 6 : 31/32"]
    CBREF1_30 = 30,
    #[doc = "31: Comp. B Int. Ref.1 Select 7 : 32/32"]
    CBREF1_31 = 31,
}
impl From<CBREF1_A> for u8 {
    #[inline(always)]
    fn from(variant: CBREF1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CBREF1` reader - Comp. B Reference 1 Resistor Select Bit : 0"]
pub struct CBREF1_R(crate::FieldReader<u8, CBREF1_A>);
impl CBREF1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CBREF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBREF1_A {
        match self.bits {
            0 => CBREF1_A::CBREF1_0,
            1 => CBREF1_A::CBREF1_1,
            2 => CBREF1_A::CBREF1_2,
            3 => CBREF1_A::CBREF1_3,
            4 => CBREF1_A::CBREF1_4,
            5 => CBREF1_A::CBREF1_5,
            6 => CBREF1_A::CBREF1_6,
            7 => CBREF1_A::CBREF1_7,
            8 => CBREF1_A::CBREF1_8,
            9 => CBREF1_A::CBREF1_9,
            10 => CBREF1_A::CBREF1_10,
            11 => CBREF1_A::CBREF1_11,
            12 => CBREF1_A::CBREF1_12,
            13 => CBREF1_A::CBREF1_13,
            14 => CBREF1_A::CBREF1_14,
            15 => CBREF1_A::CBREF1_15,
            16 => CBREF1_A::CBREF1_16,
            17 => CBREF1_A::CBREF1_17,
            18 => CBREF1_A::CBREF1_18,
            19 => CBREF1_A::CBREF1_19,
            20 => CBREF1_A::CBREF1_20,
            21 => CBREF1_A::CBREF1_21,
            22 => CBREF1_A::CBREF1_22,
            23 => CBREF1_A::CBREF1_23,
            24 => CBREF1_A::CBREF1_24,
            25 => CBREF1_A::CBREF1_25,
            26 => CBREF1_A::CBREF1_26,
            27 => CBREF1_A::CBREF1_27,
            28 => CBREF1_A::CBREF1_28,
            29 => CBREF1_A::CBREF1_29,
            30 => CBREF1_A::CBREF1_30,
            31 => CBREF1_A::CBREF1_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CBREF1_0`"]
    #[inline(always)]
    pub fn is_cbref1_0(&self) -> bool {
        **self == CBREF1_A::CBREF1_0
    }
    #[doc = "Checks if the value of the field is `CBREF1_1`"]
    #[inline(always)]
    pub fn is_cbref1_1(&self) -> bool {
        **self == CBREF1_A::CBREF1_1
    }
    #[doc = "Checks if the value of the field is `CBREF1_2`"]
    #[inline(always)]
    pub fn is_cbref1_2(&self) -> bool {
        **self == CBREF1_A::CBREF1_2
    }
    #[doc = "Checks if the value of the field is `CBREF1_3`"]
    #[inline(always)]
    pub fn is_cbref1_3(&self) -> bool {
        **self == CBREF1_A::CBREF1_3
    }
    #[doc = "Checks if the value of the field is `CBREF1_4`"]
    #[inline(always)]
    pub fn is_cbref1_4(&self) -> bool {
        **self == CBREF1_A::CBREF1_4
    }
    #[doc = "Checks if the value of the field is `CBREF1_5`"]
    #[inline(always)]
    pub fn is_cbref1_5(&self) -> bool {
        **self == CBREF1_A::CBREF1_5
    }
    #[doc = "Checks if the value of the field is `CBREF1_6`"]
    #[inline(always)]
    pub fn is_cbref1_6(&self) -> bool {
        **self == CBREF1_A::CBREF1_6
    }
    #[doc = "Checks if the value of the field is `CBREF1_7`"]
    #[inline(always)]
    pub fn is_cbref1_7(&self) -> bool {
        **self == CBREF1_A::CBREF1_7
    }
    #[doc = "Checks if the value of the field is `CBREF1_8`"]
    #[inline(always)]
    pub fn is_cbref1_8(&self) -> bool {
        **self == CBREF1_A::CBREF1_8
    }
    #[doc = "Checks if the value of the field is `CBREF1_9`"]
    #[inline(always)]
    pub fn is_cbref1_9(&self) -> bool {
        **self == CBREF1_A::CBREF1_9
    }
    #[doc = "Checks if the value of the field is `CBREF1_10`"]
    #[inline(always)]
    pub fn is_cbref1_10(&self) -> bool {
        **self == CBREF1_A::CBREF1_10
    }
    #[doc = "Checks if the value of the field is `CBREF1_11`"]
    #[inline(always)]
    pub fn is_cbref1_11(&self) -> bool {
        **self == CBREF1_A::CBREF1_11
    }
    #[doc = "Checks if the value of the field is `CBREF1_12`"]
    #[inline(always)]
    pub fn is_cbref1_12(&self) -> bool {
        **self == CBREF1_A::CBREF1_12
    }
    #[doc = "Checks if the value of the field is `CBREF1_13`"]
    #[inline(always)]
    pub fn is_cbref1_13(&self) -> bool {
        **self == CBREF1_A::CBREF1_13
    }
    #[doc = "Checks if the value of the field is `CBREF1_14`"]
    #[inline(always)]
    pub fn is_cbref1_14(&self) -> bool {
        **self == CBREF1_A::CBREF1_14
    }
    #[doc = "Checks if the value of the field is `CBREF1_15`"]
    #[inline(always)]
    pub fn is_cbref1_15(&self) -> bool {
        **self == CBREF1_A::CBREF1_15
    }
    #[doc = "Checks if the value of the field is `CBREF1_16`"]
    #[inline(always)]
    pub fn is_cbref1_16(&self) -> bool {
        **self == CBREF1_A::CBREF1_16
    }
    #[doc = "Checks if the value of the field is `CBREF1_17`"]
    #[inline(always)]
    pub fn is_cbref1_17(&self) -> bool {
        **self == CBREF1_A::CBREF1_17
    }
    #[doc = "Checks if the value of the field is `CBREF1_18`"]
    #[inline(always)]
    pub fn is_cbref1_18(&self) -> bool {
        **self == CBREF1_A::CBREF1_18
    }
    #[doc = "Checks if the value of the field is `CBREF1_19`"]
    #[inline(always)]
    pub fn is_cbref1_19(&self) -> bool {
        **self == CBREF1_A::CBREF1_19
    }
    #[doc = "Checks if the value of the field is `CBREF1_20`"]
    #[inline(always)]
    pub fn is_cbref1_20(&self) -> bool {
        **self == CBREF1_A::CBREF1_20
    }
    #[doc = "Checks if the value of the field is `CBREF1_21`"]
    #[inline(always)]
    pub fn is_cbref1_21(&self) -> bool {
        **self == CBREF1_A::CBREF1_21
    }
    #[doc = "Checks if the value of the field is `CBREF1_22`"]
    #[inline(always)]
    pub fn is_cbref1_22(&self) -> bool {
        **self == CBREF1_A::CBREF1_22
    }
    #[doc = "Checks if the value of the field is `CBREF1_23`"]
    #[inline(always)]
    pub fn is_cbref1_23(&self) -> bool {
        **self == CBREF1_A::CBREF1_23
    }
    #[doc = "Checks if the value of the field is `CBREF1_24`"]
    #[inline(always)]
    pub fn is_cbref1_24(&self) -> bool {
        **self == CBREF1_A::CBREF1_24
    }
    #[doc = "Checks if the value of the field is `CBREF1_25`"]
    #[inline(always)]
    pub fn is_cbref1_25(&self) -> bool {
        **self == CBREF1_A::CBREF1_25
    }
    #[doc = "Checks if the value of the field is `CBREF1_26`"]
    #[inline(always)]
    pub fn is_cbref1_26(&self) -> bool {
        **self == CBREF1_A::CBREF1_26
    }
    #[doc = "Checks if the value of the field is `CBREF1_27`"]
    #[inline(always)]
    pub fn is_cbref1_27(&self) -> bool {
        **self == CBREF1_A::CBREF1_27
    }
    #[doc = "Checks if the value of the field is `CBREF1_28`"]
    #[inline(always)]
    pub fn is_cbref1_28(&self) -> bool {
        **self == CBREF1_A::CBREF1_28
    }
    #[doc = "Checks if the value of the field is `CBREF1_29`"]
    #[inline(always)]
    pub fn is_cbref1_29(&self) -> bool {
        **self == CBREF1_A::CBREF1_29
    }
    #[doc = "Checks if the value of the field is `CBREF1_30`"]
    #[inline(always)]
    pub fn is_cbref1_30(&self) -> bool {
        **self == CBREF1_A::CBREF1_30
    }
    #[doc = "Checks if the value of the field is `CBREF1_31`"]
    #[inline(always)]
    pub fn is_cbref1_31(&self) -> bool {
        **self == CBREF1_A::CBREF1_31
    }
}
impl core::ops::Deref for CBREF1_R {
    type Target = crate::FieldReader<u8, CBREF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBREF1` writer - Comp. B Reference 1 Resistor Select Bit : 0"]
pub struct CBREF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CBREF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBREF1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Comp. B Int. Ref.1 Select 0 : 1/32"]
    #[inline(always)]
    pub fn cbref1_0(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_0)
    }
    #[doc = "Comp. B Int. Ref.1 Select 1 : 2/32"]
    #[inline(always)]
    pub fn cbref1_1(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_1)
    }
    #[doc = "Comp. B Int. Ref.1 Select 2 : 3/32"]
    #[inline(always)]
    pub fn cbref1_2(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_2)
    }
    #[doc = "Comp. B Int. Ref.1 Select 3 : 4/32"]
    #[inline(always)]
    pub fn cbref1_3(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_3)
    }
    #[doc = "Comp. B Int. Ref.1 Select 4 : 5/32"]
    #[inline(always)]
    pub fn cbref1_4(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_4)
    }
    #[doc = "Comp. B Int. Ref.1 Select 5 : 6/32"]
    #[inline(always)]
    pub fn cbref1_5(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_5)
    }
    #[doc = "Comp. B Int. Ref.1 Select 6 : 7/32"]
    #[inline(always)]
    pub fn cbref1_6(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_6)
    }
    #[doc = "Comp. B Int. Ref.1 Select 7 : 8/32"]
    #[inline(always)]
    pub fn cbref1_7(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_7)
    }
    #[doc = "Comp. B Int. Ref.1 Select 0 : 9/32"]
    #[inline(always)]
    pub fn cbref1_8(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_8)
    }
    #[doc = "Comp. B Int. Ref.1 Select 1 : 10/32"]
    #[inline(always)]
    pub fn cbref1_9(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_9)
    }
    #[doc = "Comp. B Int. Ref.1 Select 2 : 11/32"]
    #[inline(always)]
    pub fn cbref1_10(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_10)
    }
    #[doc = "Comp. B Int. Ref.1 Select 3 : 12/32"]
    #[inline(always)]
    pub fn cbref1_11(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_11)
    }
    #[doc = "Comp. B Int. Ref.1 Select 4 : 13/32"]
    #[inline(always)]
    pub fn cbref1_12(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_12)
    }
    #[doc = "Comp. B Int. Ref.1 Select 5 : 14/32"]
    #[inline(always)]
    pub fn cbref1_13(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_13)
    }
    #[doc = "Comp. B Int. Ref.1 Select 6 : 15/32"]
    #[inline(always)]
    pub fn cbref1_14(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_14)
    }
    #[doc = "Comp. B Int. Ref.1 Select 7 : 16/32"]
    #[inline(always)]
    pub fn cbref1_15(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_15)
    }
    #[doc = "Comp. B Int. Ref.1 Select 0 : 17/32"]
    #[inline(always)]
    pub fn cbref1_16(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_16)
    }
    #[doc = "Comp. B Int. Ref.1 Select 1 : 18/32"]
    #[inline(always)]
    pub fn cbref1_17(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_17)
    }
    #[doc = "Comp. B Int. Ref.1 Select 2 : 19/32"]
    #[inline(always)]
    pub fn cbref1_18(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_18)
    }
    #[doc = "Comp. B Int. Ref.1 Select 3 : 20/32"]
    #[inline(always)]
    pub fn cbref1_19(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_19)
    }
    #[doc = "Comp. B Int. Ref.1 Select 4 : 21/32"]
    #[inline(always)]
    pub fn cbref1_20(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_20)
    }
    #[doc = "Comp. B Int. Ref.1 Select 5 : 22/32"]
    #[inline(always)]
    pub fn cbref1_21(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_21)
    }
    #[doc = "Comp. B Int. Ref.1 Select 6 : 23/32"]
    #[inline(always)]
    pub fn cbref1_22(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_22)
    }
    #[doc = "Comp. B Int. Ref.1 Select 7 : 24/32"]
    #[inline(always)]
    pub fn cbref1_23(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_23)
    }
    #[doc = "Comp. B Int. Ref.1 Select 0 : 25/32"]
    #[inline(always)]
    pub fn cbref1_24(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_24)
    }
    #[doc = "Comp. B Int. Ref.1 Select 1 : 26/32"]
    #[inline(always)]
    pub fn cbref1_25(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_25)
    }
    #[doc = "Comp. B Int. Ref.1 Select 2 : 27/32"]
    #[inline(always)]
    pub fn cbref1_26(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_26)
    }
    #[doc = "Comp. B Int. Ref.1 Select 3 : 28/32"]
    #[inline(always)]
    pub fn cbref1_27(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_27)
    }
    #[doc = "Comp. B Int. Ref.1 Select 4 : 29/32"]
    #[inline(always)]
    pub fn cbref1_28(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_28)
    }
    #[doc = "Comp. B Int. Ref.1 Select 5 : 30/32"]
    #[inline(always)]
    pub fn cbref1_29(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_29)
    }
    #[doc = "Comp. B Int. Ref.1 Select 6 : 31/32"]
    #[inline(always)]
    pub fn cbref1_30(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_30)
    }
    #[doc = "Comp. B Int. Ref.1 Select 7 : 32/32"]
    #[inline(always)]
    pub fn cbref1_31(self) -> &'a mut W {
        self.variant(CBREF1_A::CBREF1_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u16 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Comp. B Reference voltage level Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CBREFL_A {
    #[doc = "0: Comp. B Reference voltage level 0 : None"]
    CBREFL_0 = 0,
    #[doc = "1: Comp. B Reference voltage level 1 : 1.5V"]
    CBREFL_1 = 1,
    #[doc = "2: Comp. B Reference voltage level 2 : 2.0V"]
    CBREFL_2 = 2,
    #[doc = "3: Comp. B Reference voltage level 3 : 2.5V"]
    CBREFL_3 = 3,
}
impl From<CBREFL_A> for u8 {
    #[inline(always)]
    fn from(variant: CBREFL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CBREFL` reader - Comp. B Reference voltage level Bit : 0"]
pub struct CBREFL_R(crate::FieldReader<u8, CBREFL_A>);
impl CBREFL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CBREFL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBREFL_A {
        match self.bits {
            0 => CBREFL_A::CBREFL_0,
            1 => CBREFL_A::CBREFL_1,
            2 => CBREFL_A::CBREFL_2,
            3 => CBREFL_A::CBREFL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CBREFL_0`"]
    #[inline(always)]
    pub fn is_cbrefl_0(&self) -> bool {
        **self == CBREFL_A::CBREFL_0
    }
    #[doc = "Checks if the value of the field is `CBREFL_1`"]
    #[inline(always)]
    pub fn is_cbrefl_1(&self) -> bool {
        **self == CBREFL_A::CBREFL_1
    }
    #[doc = "Checks if the value of the field is `CBREFL_2`"]
    #[inline(always)]
    pub fn is_cbrefl_2(&self) -> bool {
        **self == CBREFL_A::CBREFL_2
    }
    #[doc = "Checks if the value of the field is `CBREFL_3`"]
    #[inline(always)]
    pub fn is_cbrefl_3(&self) -> bool {
        **self == CBREFL_A::CBREFL_3
    }
}
impl core::ops::Deref for CBREFL_R {
    type Target = crate::FieldReader<u8, CBREFL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBREFL` writer - Comp. B Reference voltage level Bit : 0"]
pub struct CBREFL_W<'a> {
    w: &'a mut W,
}
impl<'a> CBREFL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBREFL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Comp. B Reference voltage level 0 : None"]
    #[inline(always)]
    pub fn cbrefl_0(self) -> &'a mut W {
        self.variant(CBREFL_A::CBREFL_0)
    }
    #[doc = "Comp. B Reference voltage level 1 : 1.5V"]
    #[inline(always)]
    pub fn cbrefl_1(self) -> &'a mut W {
        self.variant(CBREFL_A::CBREFL_1)
    }
    #[doc = "Comp. B Reference voltage level 2 : 2.0V"]
    #[inline(always)]
    pub fn cbrefl_2(self) -> &'a mut W {
        self.variant(CBREFL_A::CBREFL_2)
    }
    #[doc = "Comp. B Reference voltage level 3 : 2.5V"]
    #[inline(always)]
    pub fn cbrefl_3(self) -> &'a mut W {
        self.variant(CBREFL_A::CBREFL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u16 & 0x03) << 13);
        self.w
    }
}
#[doc = "Field `CBREFACC` reader - Comp. B Reference Accuracy"]
pub struct CBREFACC_R(crate::FieldReader<bool, bool>);
impl CBREFACC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBREFACC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBREFACC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBREFACC` writer - Comp. B Reference Accuracy"]
pub struct CBREFACC_W<'a> {
    w: &'a mut W,
}
impl<'a> CBREFACC_W<'a> {
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
    #[doc = "Bits 0:4 - Comp. B Reference 0 Resistor Select Bit : 0"]
    #[inline(always)]
    pub fn cbref0(&self) -> CBREF0_R {
        CBREF0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Comp. B Reference select"]
    #[inline(always)]
    pub fn cbrsel(&self) -> CBRSEL_R {
        CBRSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Comp. B Reference Source Bit : 0"]
    #[inline(always)]
    pub fn cbrs(&self) -> CBRS_R {
        CBRS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:12 - Comp. B Reference 1 Resistor Select Bit : 0"]
    #[inline(always)]
    pub fn cbref1(&self) -> CBREF1_R {
        CBREF1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - Comp. B Reference voltage level Bit : 0"]
    #[inline(always)]
    pub fn cbrefl(&self) -> CBREFL_R {
        CBREFL_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Comp. B Reference Accuracy"]
    #[inline(always)]
    pub fn cbrefacc(&self) -> CBREFACC_R {
        CBREFACC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Comp. B Reference 0 Resistor Select Bit : 0"]
    #[inline(always)]
    pub fn cbref0(&mut self) -> CBREF0_W {
        CBREF0_W { w: self }
    }
    #[doc = "Bit 5 - Comp. B Reference select"]
    #[inline(always)]
    pub fn cbrsel(&mut self) -> CBRSEL_W {
        CBRSEL_W { w: self }
    }
    #[doc = "Bits 6:7 - Comp. B Reference Source Bit : 0"]
    #[inline(always)]
    pub fn cbrs(&mut self) -> CBRS_W {
        CBRS_W { w: self }
    }
    #[doc = "Bits 8:12 - Comp. B Reference 1 Resistor Select Bit : 0"]
    #[inline(always)]
    pub fn cbref1(&mut self) -> CBREF1_W {
        CBREF1_W { w: self }
    }
    #[doc = "Bits 13:14 - Comp. B Reference voltage level Bit : 0"]
    #[inline(always)]
    pub fn cbrefl(&mut self) -> CBREFL_W {
        CBREFL_W { w: self }
    }
    #[doc = "Bit 15 - Comp. B Reference Accuracy"]
    #[inline(always)]
    pub fn cbrefacc(&mut self) -> CBREFACC_W {
        CBREFACC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator B Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbctl2](index.html) module"]
pub struct CBCTL2_SPEC;
impl crate::RegisterSpec for CBCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cbctl2::R](R) reader structure"]
impl crate::Readable for CBCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cbctl2::W](W) writer structure"]
impl crate::Writable for CBCTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CBCTL2 to value 0"]
impl crate::Resettable for CBCTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
