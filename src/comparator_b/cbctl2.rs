#[doc = "Register `CBCTL2` reader"]
pub type R = crate::R<CBCTL2_SPEC>;
#[doc = "Register `CBCTL2` writer"]
pub type W = crate::W<CBCTL2_SPEC>;
#[doc = "Field `CBREF0` reader - Comp. B Reference 0 Resistor Select Bit : 0"]
pub type CBREF0_R = crate::FieldReader<CBREF0_A>;
#[doc = "Comp. B Reference 0 Resistor Select Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for CBREF0_A {
    type Ux = u8;
}
impl CBREF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CBREF0_A {
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
    #[doc = "Comp. B Int. Ref.0 Select 0 : 1/32"]
    #[inline(always)]
    pub fn is_cbref0_0(&self) -> bool {
        *self == CBREF0_A::CBREF0_0
    }
    #[doc = "Comp. B Int. Ref.0 Select 1 : 2/32"]
    #[inline(always)]
    pub fn is_cbref0_1(&self) -> bool {
        *self == CBREF0_A::CBREF0_1
    }
    #[doc = "Comp. B Int. Ref.0 Select 2 : 3/32"]
    #[inline(always)]
    pub fn is_cbref0_2(&self) -> bool {
        *self == CBREF0_A::CBREF0_2
    }
    #[doc = "Comp. B Int. Ref.0 Select 3 : 4/32"]
    #[inline(always)]
    pub fn is_cbref0_3(&self) -> bool {
        *self == CBREF0_A::CBREF0_3
    }
    #[doc = "Comp. B Int. Ref.0 Select 4 : 5/32"]
    #[inline(always)]
    pub fn is_cbref0_4(&self) -> bool {
        *self == CBREF0_A::CBREF0_4
    }
    #[doc = "Comp. B Int. Ref.0 Select 5 : 6/32"]
    #[inline(always)]
    pub fn is_cbref0_5(&self) -> bool {
        *self == CBREF0_A::CBREF0_5
    }
    #[doc = "Comp. B Int. Ref.0 Select 6 : 7/32"]
    #[inline(always)]
    pub fn is_cbref0_6(&self) -> bool {
        *self == CBREF0_A::CBREF0_6
    }
    #[doc = "Comp. B Int. Ref.0 Select 7 : 8/32"]
    #[inline(always)]
    pub fn is_cbref0_7(&self) -> bool {
        *self == CBREF0_A::CBREF0_7
    }
    #[doc = "Comp. B Int. Ref.0 Select 0 : 9/32"]
    #[inline(always)]
    pub fn is_cbref0_8(&self) -> bool {
        *self == CBREF0_A::CBREF0_8
    }
    #[doc = "Comp. B Int. Ref.0 Select 1 : 10/32"]
    #[inline(always)]
    pub fn is_cbref0_9(&self) -> bool {
        *self == CBREF0_A::CBREF0_9
    }
    #[doc = "Comp. B Int. Ref.0 Select 2 : 11/32"]
    #[inline(always)]
    pub fn is_cbref0_10(&self) -> bool {
        *self == CBREF0_A::CBREF0_10
    }
    #[doc = "Comp. B Int. Ref.0 Select 3 : 12/32"]
    #[inline(always)]
    pub fn is_cbref0_11(&self) -> bool {
        *self == CBREF0_A::CBREF0_11
    }
    #[doc = "Comp. B Int. Ref.0 Select 4 : 13/32"]
    #[inline(always)]
    pub fn is_cbref0_12(&self) -> bool {
        *self == CBREF0_A::CBREF0_12
    }
    #[doc = "Comp. B Int. Ref.0 Select 5 : 14/32"]
    #[inline(always)]
    pub fn is_cbref0_13(&self) -> bool {
        *self == CBREF0_A::CBREF0_13
    }
    #[doc = "Comp. B Int. Ref.0 Select 6 : 15/32"]
    #[inline(always)]
    pub fn is_cbref0_14(&self) -> bool {
        *self == CBREF0_A::CBREF0_14
    }
    #[doc = "Comp. B Int. Ref.0 Select 7 : 16/32"]
    #[inline(always)]
    pub fn is_cbref0_15(&self) -> bool {
        *self == CBREF0_A::CBREF0_15
    }
    #[doc = "Comp. B Int. Ref.0 Select 0 : 17/32"]
    #[inline(always)]
    pub fn is_cbref0_16(&self) -> bool {
        *self == CBREF0_A::CBREF0_16
    }
    #[doc = "Comp. B Int. Ref.0 Select 1 : 18/32"]
    #[inline(always)]
    pub fn is_cbref0_17(&self) -> bool {
        *self == CBREF0_A::CBREF0_17
    }
    #[doc = "Comp. B Int. Ref.0 Select 2 : 19/32"]
    #[inline(always)]
    pub fn is_cbref0_18(&self) -> bool {
        *self == CBREF0_A::CBREF0_18
    }
    #[doc = "Comp. B Int. Ref.0 Select 3 : 20/32"]
    #[inline(always)]
    pub fn is_cbref0_19(&self) -> bool {
        *self == CBREF0_A::CBREF0_19
    }
    #[doc = "Comp. B Int. Ref.0 Select 4 : 21/32"]
    #[inline(always)]
    pub fn is_cbref0_20(&self) -> bool {
        *self == CBREF0_A::CBREF0_20
    }
    #[doc = "Comp. B Int. Ref.0 Select 5 : 22/32"]
    #[inline(always)]
    pub fn is_cbref0_21(&self) -> bool {
        *self == CBREF0_A::CBREF0_21
    }
    #[doc = "Comp. B Int. Ref.0 Select 6 : 23/32"]
    #[inline(always)]
    pub fn is_cbref0_22(&self) -> bool {
        *self == CBREF0_A::CBREF0_22
    }
    #[doc = "Comp. B Int. Ref.0 Select 7 : 24/32"]
    #[inline(always)]
    pub fn is_cbref0_23(&self) -> bool {
        *self == CBREF0_A::CBREF0_23
    }
    #[doc = "Comp. B Int. Ref.0 Select 0 : 25/32"]
    #[inline(always)]
    pub fn is_cbref0_24(&self) -> bool {
        *self == CBREF0_A::CBREF0_24
    }
    #[doc = "Comp. B Int. Ref.0 Select 1 : 26/32"]
    #[inline(always)]
    pub fn is_cbref0_25(&self) -> bool {
        *self == CBREF0_A::CBREF0_25
    }
    #[doc = "Comp. B Int. Ref.0 Select 2 : 27/32"]
    #[inline(always)]
    pub fn is_cbref0_26(&self) -> bool {
        *self == CBREF0_A::CBREF0_26
    }
    #[doc = "Comp. B Int. Ref.0 Select 3 : 28/32"]
    #[inline(always)]
    pub fn is_cbref0_27(&self) -> bool {
        *self == CBREF0_A::CBREF0_27
    }
    #[doc = "Comp. B Int. Ref.0 Select 4 : 29/32"]
    #[inline(always)]
    pub fn is_cbref0_28(&self) -> bool {
        *self == CBREF0_A::CBREF0_28
    }
    #[doc = "Comp. B Int. Ref.0 Select 5 : 30/32"]
    #[inline(always)]
    pub fn is_cbref0_29(&self) -> bool {
        *self == CBREF0_A::CBREF0_29
    }
    #[doc = "Comp. B Int. Ref.0 Select 6 : 31/32"]
    #[inline(always)]
    pub fn is_cbref0_30(&self) -> bool {
        *self == CBREF0_A::CBREF0_30
    }
    #[doc = "Comp. B Int. Ref.0 Select 7 : 32/32"]
    #[inline(always)]
    pub fn is_cbref0_31(&self) -> bool {
        *self == CBREF0_A::CBREF0_31
    }
}
#[doc = "Field `CBREF0` writer - Comp. B Reference 0 Resistor Select Bit : 0"]
pub type CBREF0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5, CBREF0_A>;
impl<'a, REG> CBREF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Comp. B Int. Ref.0 Select 0 : 1/32"]
    #[inline(always)]
    pub fn cbref0_0(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_0)
    }
    #[doc = "Comp. B Int. Ref.0 Select 1 : 2/32"]
    #[inline(always)]
    pub fn cbref0_1(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_1)
    }
    #[doc = "Comp. B Int. Ref.0 Select 2 : 3/32"]
    #[inline(always)]
    pub fn cbref0_2(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_2)
    }
    #[doc = "Comp. B Int. Ref.0 Select 3 : 4/32"]
    #[inline(always)]
    pub fn cbref0_3(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_3)
    }
    #[doc = "Comp. B Int. Ref.0 Select 4 : 5/32"]
    #[inline(always)]
    pub fn cbref0_4(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_4)
    }
    #[doc = "Comp. B Int. Ref.0 Select 5 : 6/32"]
    #[inline(always)]
    pub fn cbref0_5(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_5)
    }
    #[doc = "Comp. B Int. Ref.0 Select 6 : 7/32"]
    #[inline(always)]
    pub fn cbref0_6(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_6)
    }
    #[doc = "Comp. B Int. Ref.0 Select 7 : 8/32"]
    #[inline(always)]
    pub fn cbref0_7(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_7)
    }
    #[doc = "Comp. B Int. Ref.0 Select 0 : 9/32"]
    #[inline(always)]
    pub fn cbref0_8(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_8)
    }
    #[doc = "Comp. B Int. Ref.0 Select 1 : 10/32"]
    #[inline(always)]
    pub fn cbref0_9(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_9)
    }
    #[doc = "Comp. B Int. Ref.0 Select 2 : 11/32"]
    #[inline(always)]
    pub fn cbref0_10(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_10)
    }
    #[doc = "Comp. B Int. Ref.0 Select 3 : 12/32"]
    #[inline(always)]
    pub fn cbref0_11(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_11)
    }
    #[doc = "Comp. B Int. Ref.0 Select 4 : 13/32"]
    #[inline(always)]
    pub fn cbref0_12(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_12)
    }
    #[doc = "Comp. B Int. Ref.0 Select 5 : 14/32"]
    #[inline(always)]
    pub fn cbref0_13(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_13)
    }
    #[doc = "Comp. B Int. Ref.0 Select 6 : 15/32"]
    #[inline(always)]
    pub fn cbref0_14(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_14)
    }
    #[doc = "Comp. B Int. Ref.0 Select 7 : 16/32"]
    #[inline(always)]
    pub fn cbref0_15(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_15)
    }
    #[doc = "Comp. B Int. Ref.0 Select 0 : 17/32"]
    #[inline(always)]
    pub fn cbref0_16(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_16)
    }
    #[doc = "Comp. B Int. Ref.0 Select 1 : 18/32"]
    #[inline(always)]
    pub fn cbref0_17(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_17)
    }
    #[doc = "Comp. B Int. Ref.0 Select 2 : 19/32"]
    #[inline(always)]
    pub fn cbref0_18(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_18)
    }
    #[doc = "Comp. B Int. Ref.0 Select 3 : 20/32"]
    #[inline(always)]
    pub fn cbref0_19(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_19)
    }
    #[doc = "Comp. B Int. Ref.0 Select 4 : 21/32"]
    #[inline(always)]
    pub fn cbref0_20(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_20)
    }
    #[doc = "Comp. B Int. Ref.0 Select 5 : 22/32"]
    #[inline(always)]
    pub fn cbref0_21(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_21)
    }
    #[doc = "Comp. B Int. Ref.0 Select 6 : 23/32"]
    #[inline(always)]
    pub fn cbref0_22(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_22)
    }
    #[doc = "Comp. B Int. Ref.0 Select 7 : 24/32"]
    #[inline(always)]
    pub fn cbref0_23(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_23)
    }
    #[doc = "Comp. B Int. Ref.0 Select 0 : 25/32"]
    #[inline(always)]
    pub fn cbref0_24(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_24)
    }
    #[doc = "Comp. B Int. Ref.0 Select 1 : 26/32"]
    #[inline(always)]
    pub fn cbref0_25(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_25)
    }
    #[doc = "Comp. B Int. Ref.0 Select 2 : 27/32"]
    #[inline(always)]
    pub fn cbref0_26(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_26)
    }
    #[doc = "Comp. B Int. Ref.0 Select 3 : 28/32"]
    #[inline(always)]
    pub fn cbref0_27(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_27)
    }
    #[doc = "Comp. B Int. Ref.0 Select 4 : 29/32"]
    #[inline(always)]
    pub fn cbref0_28(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_28)
    }
    #[doc = "Comp. B Int. Ref.0 Select 5 : 30/32"]
    #[inline(always)]
    pub fn cbref0_29(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_29)
    }
    #[doc = "Comp. B Int. Ref.0 Select 6 : 31/32"]
    #[inline(always)]
    pub fn cbref0_30(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_30)
    }
    #[doc = "Comp. B Int. Ref.0 Select 7 : 32/32"]
    #[inline(always)]
    pub fn cbref0_31(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF0_A::CBREF0_31)
    }
}
#[doc = "Field `CBRSEL` reader - Comp. B Reference select"]
pub type CBRSEL_R = crate::BitReader;
#[doc = "Field `CBRSEL` writer - Comp. B Reference select"]
pub type CBRSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBRS` reader - Comp. B Reference Source Bit : 0"]
pub type CBRS_R = crate::FieldReader<CBRS_A>;
#[doc = "Comp. B Reference Source Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for CBRS_A {
    type Ux = u8;
}
impl CBRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CBRS_A {
        match self.bits {
            0 => CBRS_A::CBRS_0,
            1 => CBRS_A::CBRS_1,
            2 => CBRS_A::CBRS_2,
            3 => CBRS_A::CBRS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Comp. B Reference Source 0 : Off"]
    #[inline(always)]
    pub fn is_cbrs_0(&self) -> bool {
        *self == CBRS_A::CBRS_0
    }
    #[doc = "Comp. B Reference Source 1 : Vcc"]
    #[inline(always)]
    pub fn is_cbrs_1(&self) -> bool {
        *self == CBRS_A::CBRS_1
    }
    #[doc = "Comp. B Reference Source 2 : Shared Ref."]
    #[inline(always)]
    pub fn is_cbrs_2(&self) -> bool {
        *self == CBRS_A::CBRS_2
    }
    #[doc = "Comp. B Reference Source 3 : Shared Ref. / Off"]
    #[inline(always)]
    pub fn is_cbrs_3(&self) -> bool {
        *self == CBRS_A::CBRS_3
    }
}
#[doc = "Field `CBRS` writer - Comp. B Reference Source Bit : 0"]
pub type CBRS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CBRS_A>;
impl<'a, REG> CBRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Comp. B Reference Source 0 : Off"]
    #[inline(always)]
    pub fn cbrs_0(self) -> &'a mut crate::W<REG> {
        self.variant(CBRS_A::CBRS_0)
    }
    #[doc = "Comp. B Reference Source 1 : Vcc"]
    #[inline(always)]
    pub fn cbrs_1(self) -> &'a mut crate::W<REG> {
        self.variant(CBRS_A::CBRS_1)
    }
    #[doc = "Comp. B Reference Source 2 : Shared Ref."]
    #[inline(always)]
    pub fn cbrs_2(self) -> &'a mut crate::W<REG> {
        self.variant(CBRS_A::CBRS_2)
    }
    #[doc = "Comp. B Reference Source 3 : Shared Ref. / Off"]
    #[inline(always)]
    pub fn cbrs_3(self) -> &'a mut crate::W<REG> {
        self.variant(CBRS_A::CBRS_3)
    }
}
#[doc = "Field `CBREF1` reader - Comp. B Reference 1 Resistor Select Bit : 0"]
pub type CBREF1_R = crate::FieldReader<CBREF1_A>;
#[doc = "Comp. B Reference 1 Resistor Select Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for CBREF1_A {
    type Ux = u8;
}
impl CBREF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CBREF1_A {
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
    #[doc = "Comp. B Int. Ref.1 Select 0 : 1/32"]
    #[inline(always)]
    pub fn is_cbref1_0(&self) -> bool {
        *self == CBREF1_A::CBREF1_0
    }
    #[doc = "Comp. B Int. Ref.1 Select 1 : 2/32"]
    #[inline(always)]
    pub fn is_cbref1_1(&self) -> bool {
        *self == CBREF1_A::CBREF1_1
    }
    #[doc = "Comp. B Int. Ref.1 Select 2 : 3/32"]
    #[inline(always)]
    pub fn is_cbref1_2(&self) -> bool {
        *self == CBREF1_A::CBREF1_2
    }
    #[doc = "Comp. B Int. Ref.1 Select 3 : 4/32"]
    #[inline(always)]
    pub fn is_cbref1_3(&self) -> bool {
        *self == CBREF1_A::CBREF1_3
    }
    #[doc = "Comp. B Int. Ref.1 Select 4 : 5/32"]
    #[inline(always)]
    pub fn is_cbref1_4(&self) -> bool {
        *self == CBREF1_A::CBREF1_4
    }
    #[doc = "Comp. B Int. Ref.1 Select 5 : 6/32"]
    #[inline(always)]
    pub fn is_cbref1_5(&self) -> bool {
        *self == CBREF1_A::CBREF1_5
    }
    #[doc = "Comp. B Int. Ref.1 Select 6 : 7/32"]
    #[inline(always)]
    pub fn is_cbref1_6(&self) -> bool {
        *self == CBREF1_A::CBREF1_6
    }
    #[doc = "Comp. B Int. Ref.1 Select 7 : 8/32"]
    #[inline(always)]
    pub fn is_cbref1_7(&self) -> bool {
        *self == CBREF1_A::CBREF1_7
    }
    #[doc = "Comp. B Int. Ref.1 Select 0 : 9/32"]
    #[inline(always)]
    pub fn is_cbref1_8(&self) -> bool {
        *self == CBREF1_A::CBREF1_8
    }
    #[doc = "Comp. B Int. Ref.1 Select 1 : 10/32"]
    #[inline(always)]
    pub fn is_cbref1_9(&self) -> bool {
        *self == CBREF1_A::CBREF1_9
    }
    #[doc = "Comp. B Int. Ref.1 Select 2 : 11/32"]
    #[inline(always)]
    pub fn is_cbref1_10(&self) -> bool {
        *self == CBREF1_A::CBREF1_10
    }
    #[doc = "Comp. B Int. Ref.1 Select 3 : 12/32"]
    #[inline(always)]
    pub fn is_cbref1_11(&self) -> bool {
        *self == CBREF1_A::CBREF1_11
    }
    #[doc = "Comp. B Int. Ref.1 Select 4 : 13/32"]
    #[inline(always)]
    pub fn is_cbref1_12(&self) -> bool {
        *self == CBREF1_A::CBREF1_12
    }
    #[doc = "Comp. B Int. Ref.1 Select 5 : 14/32"]
    #[inline(always)]
    pub fn is_cbref1_13(&self) -> bool {
        *self == CBREF1_A::CBREF1_13
    }
    #[doc = "Comp. B Int. Ref.1 Select 6 : 15/32"]
    #[inline(always)]
    pub fn is_cbref1_14(&self) -> bool {
        *self == CBREF1_A::CBREF1_14
    }
    #[doc = "Comp. B Int. Ref.1 Select 7 : 16/32"]
    #[inline(always)]
    pub fn is_cbref1_15(&self) -> bool {
        *self == CBREF1_A::CBREF1_15
    }
    #[doc = "Comp. B Int. Ref.1 Select 0 : 17/32"]
    #[inline(always)]
    pub fn is_cbref1_16(&self) -> bool {
        *self == CBREF1_A::CBREF1_16
    }
    #[doc = "Comp. B Int. Ref.1 Select 1 : 18/32"]
    #[inline(always)]
    pub fn is_cbref1_17(&self) -> bool {
        *self == CBREF1_A::CBREF1_17
    }
    #[doc = "Comp. B Int. Ref.1 Select 2 : 19/32"]
    #[inline(always)]
    pub fn is_cbref1_18(&self) -> bool {
        *self == CBREF1_A::CBREF1_18
    }
    #[doc = "Comp. B Int. Ref.1 Select 3 : 20/32"]
    #[inline(always)]
    pub fn is_cbref1_19(&self) -> bool {
        *self == CBREF1_A::CBREF1_19
    }
    #[doc = "Comp. B Int. Ref.1 Select 4 : 21/32"]
    #[inline(always)]
    pub fn is_cbref1_20(&self) -> bool {
        *self == CBREF1_A::CBREF1_20
    }
    #[doc = "Comp. B Int. Ref.1 Select 5 : 22/32"]
    #[inline(always)]
    pub fn is_cbref1_21(&self) -> bool {
        *self == CBREF1_A::CBREF1_21
    }
    #[doc = "Comp. B Int. Ref.1 Select 6 : 23/32"]
    #[inline(always)]
    pub fn is_cbref1_22(&self) -> bool {
        *self == CBREF1_A::CBREF1_22
    }
    #[doc = "Comp. B Int. Ref.1 Select 7 : 24/32"]
    #[inline(always)]
    pub fn is_cbref1_23(&self) -> bool {
        *self == CBREF1_A::CBREF1_23
    }
    #[doc = "Comp. B Int. Ref.1 Select 0 : 25/32"]
    #[inline(always)]
    pub fn is_cbref1_24(&self) -> bool {
        *self == CBREF1_A::CBREF1_24
    }
    #[doc = "Comp. B Int. Ref.1 Select 1 : 26/32"]
    #[inline(always)]
    pub fn is_cbref1_25(&self) -> bool {
        *self == CBREF1_A::CBREF1_25
    }
    #[doc = "Comp. B Int. Ref.1 Select 2 : 27/32"]
    #[inline(always)]
    pub fn is_cbref1_26(&self) -> bool {
        *self == CBREF1_A::CBREF1_26
    }
    #[doc = "Comp. B Int. Ref.1 Select 3 : 28/32"]
    #[inline(always)]
    pub fn is_cbref1_27(&self) -> bool {
        *self == CBREF1_A::CBREF1_27
    }
    #[doc = "Comp. B Int. Ref.1 Select 4 : 29/32"]
    #[inline(always)]
    pub fn is_cbref1_28(&self) -> bool {
        *self == CBREF1_A::CBREF1_28
    }
    #[doc = "Comp. B Int. Ref.1 Select 5 : 30/32"]
    #[inline(always)]
    pub fn is_cbref1_29(&self) -> bool {
        *self == CBREF1_A::CBREF1_29
    }
    #[doc = "Comp. B Int. Ref.1 Select 6 : 31/32"]
    #[inline(always)]
    pub fn is_cbref1_30(&self) -> bool {
        *self == CBREF1_A::CBREF1_30
    }
    #[doc = "Comp. B Int. Ref.1 Select 7 : 32/32"]
    #[inline(always)]
    pub fn is_cbref1_31(&self) -> bool {
        *self == CBREF1_A::CBREF1_31
    }
}
#[doc = "Field `CBREF1` writer - Comp. B Reference 1 Resistor Select Bit : 0"]
pub type CBREF1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5, CBREF1_A>;
impl<'a, REG> CBREF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Comp. B Int. Ref.1 Select 0 : 1/32"]
    #[inline(always)]
    pub fn cbref1_0(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_0)
    }
    #[doc = "Comp. B Int. Ref.1 Select 1 : 2/32"]
    #[inline(always)]
    pub fn cbref1_1(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_1)
    }
    #[doc = "Comp. B Int. Ref.1 Select 2 : 3/32"]
    #[inline(always)]
    pub fn cbref1_2(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_2)
    }
    #[doc = "Comp. B Int. Ref.1 Select 3 : 4/32"]
    #[inline(always)]
    pub fn cbref1_3(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_3)
    }
    #[doc = "Comp. B Int. Ref.1 Select 4 : 5/32"]
    #[inline(always)]
    pub fn cbref1_4(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_4)
    }
    #[doc = "Comp. B Int. Ref.1 Select 5 : 6/32"]
    #[inline(always)]
    pub fn cbref1_5(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_5)
    }
    #[doc = "Comp. B Int. Ref.1 Select 6 : 7/32"]
    #[inline(always)]
    pub fn cbref1_6(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_6)
    }
    #[doc = "Comp. B Int. Ref.1 Select 7 : 8/32"]
    #[inline(always)]
    pub fn cbref1_7(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_7)
    }
    #[doc = "Comp. B Int. Ref.1 Select 0 : 9/32"]
    #[inline(always)]
    pub fn cbref1_8(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_8)
    }
    #[doc = "Comp. B Int. Ref.1 Select 1 : 10/32"]
    #[inline(always)]
    pub fn cbref1_9(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_9)
    }
    #[doc = "Comp. B Int. Ref.1 Select 2 : 11/32"]
    #[inline(always)]
    pub fn cbref1_10(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_10)
    }
    #[doc = "Comp. B Int. Ref.1 Select 3 : 12/32"]
    #[inline(always)]
    pub fn cbref1_11(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_11)
    }
    #[doc = "Comp. B Int. Ref.1 Select 4 : 13/32"]
    #[inline(always)]
    pub fn cbref1_12(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_12)
    }
    #[doc = "Comp. B Int. Ref.1 Select 5 : 14/32"]
    #[inline(always)]
    pub fn cbref1_13(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_13)
    }
    #[doc = "Comp. B Int. Ref.1 Select 6 : 15/32"]
    #[inline(always)]
    pub fn cbref1_14(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_14)
    }
    #[doc = "Comp. B Int. Ref.1 Select 7 : 16/32"]
    #[inline(always)]
    pub fn cbref1_15(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_15)
    }
    #[doc = "Comp. B Int. Ref.1 Select 0 : 17/32"]
    #[inline(always)]
    pub fn cbref1_16(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_16)
    }
    #[doc = "Comp. B Int. Ref.1 Select 1 : 18/32"]
    #[inline(always)]
    pub fn cbref1_17(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_17)
    }
    #[doc = "Comp. B Int. Ref.1 Select 2 : 19/32"]
    #[inline(always)]
    pub fn cbref1_18(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_18)
    }
    #[doc = "Comp. B Int. Ref.1 Select 3 : 20/32"]
    #[inline(always)]
    pub fn cbref1_19(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_19)
    }
    #[doc = "Comp. B Int. Ref.1 Select 4 : 21/32"]
    #[inline(always)]
    pub fn cbref1_20(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_20)
    }
    #[doc = "Comp. B Int. Ref.1 Select 5 : 22/32"]
    #[inline(always)]
    pub fn cbref1_21(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_21)
    }
    #[doc = "Comp. B Int. Ref.1 Select 6 : 23/32"]
    #[inline(always)]
    pub fn cbref1_22(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_22)
    }
    #[doc = "Comp. B Int. Ref.1 Select 7 : 24/32"]
    #[inline(always)]
    pub fn cbref1_23(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_23)
    }
    #[doc = "Comp. B Int. Ref.1 Select 0 : 25/32"]
    #[inline(always)]
    pub fn cbref1_24(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_24)
    }
    #[doc = "Comp. B Int. Ref.1 Select 1 : 26/32"]
    #[inline(always)]
    pub fn cbref1_25(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_25)
    }
    #[doc = "Comp. B Int. Ref.1 Select 2 : 27/32"]
    #[inline(always)]
    pub fn cbref1_26(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_26)
    }
    #[doc = "Comp. B Int. Ref.1 Select 3 : 28/32"]
    #[inline(always)]
    pub fn cbref1_27(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_27)
    }
    #[doc = "Comp. B Int. Ref.1 Select 4 : 29/32"]
    #[inline(always)]
    pub fn cbref1_28(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_28)
    }
    #[doc = "Comp. B Int. Ref.1 Select 5 : 30/32"]
    #[inline(always)]
    pub fn cbref1_29(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_29)
    }
    #[doc = "Comp. B Int. Ref.1 Select 6 : 31/32"]
    #[inline(always)]
    pub fn cbref1_30(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_30)
    }
    #[doc = "Comp. B Int. Ref.1 Select 7 : 32/32"]
    #[inline(always)]
    pub fn cbref1_31(self) -> &'a mut crate::W<REG> {
        self.variant(CBREF1_A::CBREF1_31)
    }
}
#[doc = "Field `CBREFL` reader - Comp. B Reference voltage level Bit : 0"]
pub type CBREFL_R = crate::FieldReader<CBREFL_A>;
#[doc = "Comp. B Reference voltage level Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for CBREFL_A {
    type Ux = u8;
}
impl CBREFL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CBREFL_A {
        match self.bits {
            0 => CBREFL_A::CBREFL_0,
            1 => CBREFL_A::CBREFL_1,
            2 => CBREFL_A::CBREFL_2,
            3 => CBREFL_A::CBREFL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Comp. B Reference voltage level 0 : None"]
    #[inline(always)]
    pub fn is_cbrefl_0(&self) -> bool {
        *self == CBREFL_A::CBREFL_0
    }
    #[doc = "Comp. B Reference voltage level 1 : 1.5V"]
    #[inline(always)]
    pub fn is_cbrefl_1(&self) -> bool {
        *self == CBREFL_A::CBREFL_1
    }
    #[doc = "Comp. B Reference voltage level 2 : 2.0V"]
    #[inline(always)]
    pub fn is_cbrefl_2(&self) -> bool {
        *self == CBREFL_A::CBREFL_2
    }
    #[doc = "Comp. B Reference voltage level 3 : 2.5V"]
    #[inline(always)]
    pub fn is_cbrefl_3(&self) -> bool {
        *self == CBREFL_A::CBREFL_3
    }
}
#[doc = "Field `CBREFL` writer - Comp. B Reference voltage level Bit : 0"]
pub type CBREFL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CBREFL_A>;
impl<'a, REG> CBREFL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Comp. B Reference voltage level 0 : None"]
    #[inline(always)]
    pub fn cbrefl_0(self) -> &'a mut crate::W<REG> {
        self.variant(CBREFL_A::CBREFL_0)
    }
    #[doc = "Comp. B Reference voltage level 1 : 1.5V"]
    #[inline(always)]
    pub fn cbrefl_1(self) -> &'a mut crate::W<REG> {
        self.variant(CBREFL_A::CBREFL_1)
    }
    #[doc = "Comp. B Reference voltage level 2 : 2.0V"]
    #[inline(always)]
    pub fn cbrefl_2(self) -> &'a mut crate::W<REG> {
        self.variant(CBREFL_A::CBREFL_2)
    }
    #[doc = "Comp. B Reference voltage level 3 : 2.5V"]
    #[inline(always)]
    pub fn cbrefl_3(self) -> &'a mut crate::W<REG> {
        self.variant(CBREFL_A::CBREFL_3)
    }
}
#[doc = "Field `CBREFACC` reader - Comp. B Reference Accuracy"]
pub type CBREFACC_R = crate::BitReader;
#[doc = "Field `CBREFACC` writer - Comp. B Reference Accuracy"]
pub type CBREFACC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Comp. B Reference 0 Resistor Select Bit : 0"]
    #[inline(always)]
    pub fn cbref0(&self) -> CBREF0_R {
        CBREF0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Comp. B Reference select"]
    #[inline(always)]
    pub fn cbrsel(&self) -> CBRSEL_R {
        CBRSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Comp. B Reference Source Bit : 0"]
    #[inline(always)]
    pub fn cbrs(&self) -> CBRS_R {
        CBRS_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Comp. B Reference 1 Resistor Select Bit : 0"]
    #[inline(always)]
    pub fn cbref1(&self) -> CBREF1_R {
        CBREF1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - Comp. B Reference voltage level Bit : 0"]
    #[inline(always)]
    pub fn cbrefl(&self) -> CBREFL_R {
        CBREFL_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Comp. B Reference Accuracy"]
    #[inline(always)]
    pub fn cbrefacc(&self) -> CBREFACC_R {
        CBREFACC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Comp. B Reference 0 Resistor Select Bit : 0"]
    #[inline(always)]
    #[must_use]
    pub fn cbref0(&mut self) -> CBREF0_W<CBCTL2_SPEC> {
        CBREF0_W::new(self, 0)
    }
    #[doc = "Bit 5 - Comp. B Reference select"]
    #[inline(always)]
    #[must_use]
    pub fn cbrsel(&mut self) -> CBRSEL_W<CBCTL2_SPEC> {
        CBRSEL_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Comp. B Reference Source Bit : 0"]
    #[inline(always)]
    #[must_use]
    pub fn cbrs(&mut self) -> CBRS_W<CBCTL2_SPEC> {
        CBRS_W::new(self, 6)
    }
    #[doc = "Bits 8:12 - Comp. B Reference 1 Resistor Select Bit : 0"]
    #[inline(always)]
    #[must_use]
    pub fn cbref1(&mut self) -> CBREF1_W<CBCTL2_SPEC> {
        CBREF1_W::new(self, 8)
    }
    #[doc = "Bits 13:14 - Comp. B Reference voltage level Bit : 0"]
    #[inline(always)]
    #[must_use]
    pub fn cbrefl(&mut self) -> CBREFL_W<CBCTL2_SPEC> {
        CBREFL_W::new(self, 13)
    }
    #[doc = "Bit 15 - Comp. B Reference Accuracy"]
    #[inline(always)]
    #[must_use]
    pub fn cbrefacc(&mut self) -> CBREFACC_W<CBCTL2_SPEC> {
        CBREFACC_W::new(self, 15)
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
#[doc = "Comparator B Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cbctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cbctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CBCTL2_SPEC;
impl crate::RegisterSpec for CBCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cbctl2::R`](R) reader structure"]
impl crate::Readable for CBCTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cbctl2::W`](W) writer structure"]
impl crate::Writable for CBCTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CBCTL2 to value 0"]
impl crate::Resettable for CBCTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
