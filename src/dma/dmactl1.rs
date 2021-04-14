#[doc = "Register `DMACTL1` reader"]
pub struct R(crate::R<DMACTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACTL1` writer"]
pub struct W(crate::W<DMACTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACTL1_SPEC>;
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
impl From<crate::W<DMACTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DMA channel 2 transfer select bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMA2TSEL_A {
    #[doc = "0: DMA channel 2 transfer select 0: DMA_REQ (sw)"]
    DMA2TSEL_0 = 0,
    #[doc = "1: DMA channel 2 transfer select 1: Timer0_A (TA0CCR0.IFG)"]
    DMA2TSEL_1 = 1,
    #[doc = "2: DMA channel 2 transfer select 2: Timer0_A (TA0CCR2.IFG)"]
    DMA2TSEL_2 = 2,
    #[doc = "3: DMA channel 2 transfer select 3: Timer1_A (TA1CCR0.IFG)"]
    DMA2TSEL_3 = 3,
    #[doc = "4: DMA channel 2 transfer select 4: Timer1_A (TA1CCR2.IFG)"]
    DMA2TSEL_4 = 4,
    #[doc = "5: DMA channel 2 transfer select 5: Timer2_A (TA2CCR0.IFG)"]
    DMA2TSEL_5 = 5,
    #[doc = "6: DMA channel 2 transfer select 6: Timer2_A (TA2CCR2.IFG)"]
    DMA2TSEL_6 = 6,
    #[doc = "7: DMA channel 2 transfer select 7: TimerB (TB0CCR0.IFG)"]
    DMA2TSEL_7 = 7,
    #[doc = "8: DMA channel 2 transfer select 8: TimerB (TB0CCR2.IFG)"]
    DMA2TSEL_8 = 8,
    #[doc = "9: DMA channel 2 transfer select 9: Reserved"]
    DMA2TSEL_9 = 9,
    #[doc = "10: DMA channel 2 transfer select 10: Reserved"]
    DMA2TSEL_10 = 10,
    #[doc = "11: DMA channel 2 transfer select 11: Reserved"]
    DMA2TSEL_11 = 11,
    #[doc = "12: DMA channel 2 transfer select 12: Reserved"]
    DMA2TSEL_12 = 12,
    #[doc = "13: DMA channel 2 transfer select 13: Reserved"]
    DMA2TSEL_13 = 13,
    #[doc = "14: DMA channel 2 transfer select 14: Reserved"]
    DMA2TSEL_14 = 14,
    #[doc = "15: DMA channel 2 transfer select 15: Reserved"]
    DMA2TSEL_15 = 15,
    #[doc = "16: DMA channel 2 transfer select 16: USCIA0 receive"]
    DMA2TSEL_16 = 16,
    #[doc = "17: DMA channel 2 transfer select 17: USCIA0 transmit"]
    DMA2TSEL_17 = 17,
    #[doc = "18: DMA channel 2 transfer select 18: USCIB0 receive"]
    DMA2TSEL_18 = 18,
    #[doc = "19: DMA channel 2 transfer select 19: USCIB0 transmit"]
    DMA2TSEL_19 = 19,
    #[doc = "20: DMA channel 2 transfer select 20: USCIA1 receive"]
    DMA2TSEL_20 = 20,
    #[doc = "21: DMA channel 2 transfer select 21: USCIA1 transmit"]
    DMA2TSEL_21 = 21,
    #[doc = "22: DMA channel 2 transfer select 22: USCIB1 receive"]
    DMA2TSEL_22 = 22,
    #[doc = "23: DMA channel 2 transfer select 23: USCIB1 transmit"]
    DMA2TSEL_23 = 23,
    #[doc = "24: DMA channel 2 transfer select 24: ADC12IFGx"]
    DMA2TSEL_24 = 24,
    #[doc = "25: DMA channel 2 transfer select 25: Reserved"]
    DMA2TSEL_25 = 25,
    #[doc = "26: DMA channel 2 transfer select 26: Reserved"]
    DMA2TSEL_26 = 26,
    #[doc = "27: DMA channel 2 transfer select 27: USB FNRXD"]
    DMA2TSEL_27 = 27,
    #[doc = "28: DMA channel 2 transfer select 28: USB ready"]
    DMA2TSEL_28 = 28,
    #[doc = "29: DMA channel 2 transfer select 29: Multiplier ready"]
    DMA2TSEL_29 = 29,
    #[doc = "30: DMA channel 2 transfer select 30: previous DMA channel DMA1IFG"]
    DMA2TSEL_30 = 30,
    #[doc = "31: DMA channel 2 transfer select 31: ext. Trigger (DMAE0)"]
    DMA2TSEL_31 = 31,
}
impl From<DMA2TSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA2TSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DMA2TSEL` reader - DMA channel 2 transfer select bit 0"]
pub struct DMA2TSEL_R(crate::FieldReader<u8, DMA2TSEL_A>);
impl DMA2TSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA2TSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA2TSEL_A {
        match self.bits {
            0 => DMA2TSEL_A::DMA2TSEL_0,
            1 => DMA2TSEL_A::DMA2TSEL_1,
            2 => DMA2TSEL_A::DMA2TSEL_2,
            3 => DMA2TSEL_A::DMA2TSEL_3,
            4 => DMA2TSEL_A::DMA2TSEL_4,
            5 => DMA2TSEL_A::DMA2TSEL_5,
            6 => DMA2TSEL_A::DMA2TSEL_6,
            7 => DMA2TSEL_A::DMA2TSEL_7,
            8 => DMA2TSEL_A::DMA2TSEL_8,
            9 => DMA2TSEL_A::DMA2TSEL_9,
            10 => DMA2TSEL_A::DMA2TSEL_10,
            11 => DMA2TSEL_A::DMA2TSEL_11,
            12 => DMA2TSEL_A::DMA2TSEL_12,
            13 => DMA2TSEL_A::DMA2TSEL_13,
            14 => DMA2TSEL_A::DMA2TSEL_14,
            15 => DMA2TSEL_A::DMA2TSEL_15,
            16 => DMA2TSEL_A::DMA2TSEL_16,
            17 => DMA2TSEL_A::DMA2TSEL_17,
            18 => DMA2TSEL_A::DMA2TSEL_18,
            19 => DMA2TSEL_A::DMA2TSEL_19,
            20 => DMA2TSEL_A::DMA2TSEL_20,
            21 => DMA2TSEL_A::DMA2TSEL_21,
            22 => DMA2TSEL_A::DMA2TSEL_22,
            23 => DMA2TSEL_A::DMA2TSEL_23,
            24 => DMA2TSEL_A::DMA2TSEL_24,
            25 => DMA2TSEL_A::DMA2TSEL_25,
            26 => DMA2TSEL_A::DMA2TSEL_26,
            27 => DMA2TSEL_A::DMA2TSEL_27,
            28 => DMA2TSEL_A::DMA2TSEL_28,
            29 => DMA2TSEL_A::DMA2TSEL_29,
            30 => DMA2TSEL_A::DMA2TSEL_30,
            31 => DMA2TSEL_A::DMA2TSEL_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_0`"]
    #[inline(always)]
    pub fn is_dma2tsel_0(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_0
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_1`"]
    #[inline(always)]
    pub fn is_dma2tsel_1(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_1
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_2`"]
    #[inline(always)]
    pub fn is_dma2tsel_2(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_2
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_3`"]
    #[inline(always)]
    pub fn is_dma2tsel_3(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_3
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_4`"]
    #[inline(always)]
    pub fn is_dma2tsel_4(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_4
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_5`"]
    #[inline(always)]
    pub fn is_dma2tsel_5(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_5
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_6`"]
    #[inline(always)]
    pub fn is_dma2tsel_6(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_6
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_7`"]
    #[inline(always)]
    pub fn is_dma2tsel_7(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_7
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_8`"]
    #[inline(always)]
    pub fn is_dma2tsel_8(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_8
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_9`"]
    #[inline(always)]
    pub fn is_dma2tsel_9(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_9
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_10`"]
    #[inline(always)]
    pub fn is_dma2tsel_10(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_10
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_11`"]
    #[inline(always)]
    pub fn is_dma2tsel_11(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_11
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_12`"]
    #[inline(always)]
    pub fn is_dma2tsel_12(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_12
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_13`"]
    #[inline(always)]
    pub fn is_dma2tsel_13(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_13
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_14`"]
    #[inline(always)]
    pub fn is_dma2tsel_14(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_14
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_15`"]
    #[inline(always)]
    pub fn is_dma2tsel_15(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_15
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_16`"]
    #[inline(always)]
    pub fn is_dma2tsel_16(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_16
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_17`"]
    #[inline(always)]
    pub fn is_dma2tsel_17(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_17
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_18`"]
    #[inline(always)]
    pub fn is_dma2tsel_18(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_18
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_19`"]
    #[inline(always)]
    pub fn is_dma2tsel_19(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_19
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_20`"]
    #[inline(always)]
    pub fn is_dma2tsel_20(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_20
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_21`"]
    #[inline(always)]
    pub fn is_dma2tsel_21(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_21
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_22`"]
    #[inline(always)]
    pub fn is_dma2tsel_22(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_22
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_23`"]
    #[inline(always)]
    pub fn is_dma2tsel_23(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_23
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_24`"]
    #[inline(always)]
    pub fn is_dma2tsel_24(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_24
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_25`"]
    #[inline(always)]
    pub fn is_dma2tsel_25(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_25
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_26`"]
    #[inline(always)]
    pub fn is_dma2tsel_26(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_26
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_27`"]
    #[inline(always)]
    pub fn is_dma2tsel_27(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_27
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_28`"]
    #[inline(always)]
    pub fn is_dma2tsel_28(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_28
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_29`"]
    #[inline(always)]
    pub fn is_dma2tsel_29(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_29
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_30`"]
    #[inline(always)]
    pub fn is_dma2tsel_30(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_30
    }
    #[doc = "Checks if the value of the field is `DMA2TSEL_31`"]
    #[inline(always)]
    pub fn is_dma2tsel_31(&self) -> bool {
        **self == DMA2TSEL_A::DMA2TSEL_31
    }
}
impl core::ops::Deref for DMA2TSEL_R {
    type Target = crate::FieldReader<u8, DMA2TSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2TSEL` writer - DMA channel 2 transfer select bit 0"]
pub struct DMA2TSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2TSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA2TSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA channel 2 transfer select 0: DMA_REQ (sw)"]
    #[inline(always)]
    pub fn dma2tsel_0(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_0)
    }
    #[doc = "DMA channel 2 transfer select 1: Timer0_A (TA0CCR0.IFG)"]
    #[inline(always)]
    pub fn dma2tsel_1(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_1)
    }
    #[doc = "DMA channel 2 transfer select 2: Timer0_A (TA0CCR2.IFG)"]
    #[inline(always)]
    pub fn dma2tsel_2(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_2)
    }
    #[doc = "DMA channel 2 transfer select 3: Timer1_A (TA1CCR0.IFG)"]
    #[inline(always)]
    pub fn dma2tsel_3(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_3)
    }
    #[doc = "DMA channel 2 transfer select 4: Timer1_A (TA1CCR2.IFG)"]
    #[inline(always)]
    pub fn dma2tsel_4(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_4)
    }
    #[doc = "DMA channel 2 transfer select 5: Timer2_A (TA2CCR0.IFG)"]
    #[inline(always)]
    pub fn dma2tsel_5(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_5)
    }
    #[doc = "DMA channel 2 transfer select 6: Timer2_A (TA2CCR2.IFG)"]
    #[inline(always)]
    pub fn dma2tsel_6(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_6)
    }
    #[doc = "DMA channel 2 transfer select 7: TimerB (TB0CCR0.IFG)"]
    #[inline(always)]
    pub fn dma2tsel_7(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_7)
    }
    #[doc = "DMA channel 2 transfer select 8: TimerB (TB0CCR2.IFG)"]
    #[inline(always)]
    pub fn dma2tsel_8(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_8)
    }
    #[doc = "DMA channel 2 transfer select 9: Reserved"]
    #[inline(always)]
    pub fn dma2tsel_9(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_9)
    }
    #[doc = "DMA channel 2 transfer select 10: Reserved"]
    #[inline(always)]
    pub fn dma2tsel_10(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_10)
    }
    #[doc = "DMA channel 2 transfer select 11: Reserved"]
    #[inline(always)]
    pub fn dma2tsel_11(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_11)
    }
    #[doc = "DMA channel 2 transfer select 12: Reserved"]
    #[inline(always)]
    pub fn dma2tsel_12(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_12)
    }
    #[doc = "DMA channel 2 transfer select 13: Reserved"]
    #[inline(always)]
    pub fn dma2tsel_13(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_13)
    }
    #[doc = "DMA channel 2 transfer select 14: Reserved"]
    #[inline(always)]
    pub fn dma2tsel_14(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_14)
    }
    #[doc = "DMA channel 2 transfer select 15: Reserved"]
    #[inline(always)]
    pub fn dma2tsel_15(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_15)
    }
    #[doc = "DMA channel 2 transfer select 16: USCIA0 receive"]
    #[inline(always)]
    pub fn dma2tsel_16(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_16)
    }
    #[doc = "DMA channel 2 transfer select 17: USCIA0 transmit"]
    #[inline(always)]
    pub fn dma2tsel_17(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_17)
    }
    #[doc = "DMA channel 2 transfer select 18: USCIB0 receive"]
    #[inline(always)]
    pub fn dma2tsel_18(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_18)
    }
    #[doc = "DMA channel 2 transfer select 19: USCIB0 transmit"]
    #[inline(always)]
    pub fn dma2tsel_19(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_19)
    }
    #[doc = "DMA channel 2 transfer select 20: USCIA1 receive"]
    #[inline(always)]
    pub fn dma2tsel_20(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_20)
    }
    #[doc = "DMA channel 2 transfer select 21: USCIA1 transmit"]
    #[inline(always)]
    pub fn dma2tsel_21(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_21)
    }
    #[doc = "DMA channel 2 transfer select 22: USCIB1 receive"]
    #[inline(always)]
    pub fn dma2tsel_22(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_22)
    }
    #[doc = "DMA channel 2 transfer select 23: USCIB1 transmit"]
    #[inline(always)]
    pub fn dma2tsel_23(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_23)
    }
    #[doc = "DMA channel 2 transfer select 24: ADC12IFGx"]
    #[inline(always)]
    pub fn dma2tsel_24(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_24)
    }
    #[doc = "DMA channel 2 transfer select 25: Reserved"]
    #[inline(always)]
    pub fn dma2tsel_25(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_25)
    }
    #[doc = "DMA channel 2 transfer select 26: Reserved"]
    #[inline(always)]
    pub fn dma2tsel_26(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_26)
    }
    #[doc = "DMA channel 2 transfer select 27: USB FNRXD"]
    #[inline(always)]
    pub fn dma2tsel_27(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_27)
    }
    #[doc = "DMA channel 2 transfer select 28: USB ready"]
    #[inline(always)]
    pub fn dma2tsel_28(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_28)
    }
    #[doc = "DMA channel 2 transfer select 29: Multiplier ready"]
    #[inline(always)]
    pub fn dma2tsel_29(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_29)
    }
    #[doc = "DMA channel 2 transfer select 30: previous DMA channel DMA1IFG"]
    #[inline(always)]
    pub fn dma2tsel_30(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_30)
    }
    #[doc = "DMA channel 2 transfer select 31: ext. Trigger (DMAE0)"]
    #[inline(always)]
    pub fn dma2tsel_31(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TSEL_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u16 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - DMA channel 2 transfer select bit 0"]
    #[inline(always)]
    pub fn dma2tsel(&self) -> DMA2TSEL_R {
        DMA2TSEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA channel 2 transfer select bit 0"]
    #[inline(always)]
    pub fn dma2tsel(&mut self) -> DMA2TSEL_W {
        DMA2TSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Module Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl1](index.html) module"]
pub struct DMACTL1_SPEC;
impl crate::RegisterSpec for DMACTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dmactl1::R](R) reader structure"]
impl crate::Readable for DMACTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmactl1::W](W) writer structure"]
impl crate::Writable for DMACTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACTL1 to value 0"]
impl crate::Resettable for DMACTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
