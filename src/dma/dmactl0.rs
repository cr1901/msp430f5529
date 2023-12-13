#[doc = "Register `DMACTL0` reader"]
pub type R = crate::R<DMACTL0_SPEC>;
#[doc = "Register `DMACTL0` writer"]
pub type W = crate::W<DMACTL0_SPEC>;
#[doc = "Field `DMA0TSEL` reader - DMA channel 0 transfer select bit 0"]
pub type DMA0TSEL_R = crate::FieldReader<DMA0TSEL_A>;
#[doc = "DMA channel 0 transfer select bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA0TSEL_A {
    #[doc = "0: DMA channel 0 transfer select 0: DMA_REQ (sw)"]
    DMA0TSEL_0 = 0,
    #[doc = "1: DMA channel 0 transfer select 1: Timer0_A (TA0CCR0.IFG)"]
    DMA0TSEL_1 = 1,
    #[doc = "2: DMA channel 0 transfer select 2: Timer0_A (TA0CCR2.IFG)"]
    DMA0TSEL_2 = 2,
    #[doc = "3: DMA channel 0 transfer select 3: Timer1_A (TA1CCR0.IFG)"]
    DMA0TSEL_3 = 3,
    #[doc = "4: DMA channel 0 transfer select 4: Timer1_A (TA1CCR2.IFG)"]
    DMA0TSEL_4 = 4,
    #[doc = "5: DMA channel 0 transfer select 5: Timer2_A (TA2CCR0.IFG)"]
    DMA0TSEL_5 = 5,
    #[doc = "6: DMA channel 0 transfer select 6: Timer2_A (TA2CCR2.IFG)"]
    DMA0TSEL_6 = 6,
    #[doc = "7: DMA channel 0 transfer select 7: TimerB (TB0CCR0.IFG)"]
    DMA0TSEL_7 = 7,
    #[doc = "8: DMA channel 0 transfer select 8: TimerB (TB0CCR2.IFG)"]
    DMA0TSEL_8 = 8,
    #[doc = "9: DMA channel 0 transfer select 9: Reserved"]
    DMA0TSEL_9 = 9,
    #[doc = "10: DMA channel 0 transfer select 10: Reserved"]
    DMA0TSEL_10 = 10,
    #[doc = "11: DMA channel 0 transfer select 11: Reserved"]
    DMA0TSEL_11 = 11,
    #[doc = "12: DMA channel 0 transfer select 12: Reserved"]
    DMA0TSEL_12 = 12,
    #[doc = "13: DMA channel 0 transfer select 13: Reserved"]
    DMA0TSEL_13 = 13,
    #[doc = "14: DMA channel 0 transfer select 14: Reserved"]
    DMA0TSEL_14 = 14,
    #[doc = "15: DMA channel 0 transfer select 15: Reserved"]
    DMA0TSEL_15 = 15,
    #[doc = "16: DMA channel 0 transfer select 16: USCIA0 receive"]
    DMA0TSEL_16 = 16,
    #[doc = "17: DMA channel 0 transfer select 17: USCIA0 transmit"]
    DMA0TSEL_17 = 17,
    #[doc = "18: DMA channel 0 transfer select 18: USCIB0 receive"]
    DMA0TSEL_18 = 18,
    #[doc = "19: DMA channel 0 transfer select 19: USCIB0 transmit"]
    DMA0TSEL_19 = 19,
    #[doc = "20: DMA channel 0 transfer select 20: USCIA1 receive"]
    DMA0TSEL_20 = 20,
    #[doc = "21: DMA channel 0 transfer select 21: USCIA1 transmit"]
    DMA0TSEL_21 = 21,
    #[doc = "22: DMA channel 0 transfer select 22: USCIB1 receive"]
    DMA0TSEL_22 = 22,
    #[doc = "23: DMA channel 0 transfer select 23: USCIB1 transmit"]
    DMA0TSEL_23 = 23,
    #[doc = "24: DMA channel 0 transfer select 24: ADC12IFGx"]
    DMA0TSEL_24 = 24,
    #[doc = "25: DMA channel 0 transfer select 25: Reserved"]
    DMA0TSEL_25 = 25,
    #[doc = "26: DMA channel 0 transfer select 26: Reserved"]
    DMA0TSEL_26 = 26,
    #[doc = "27: DMA channel 0 transfer select 27: USB FNRXD"]
    DMA0TSEL_27 = 27,
    #[doc = "28: DMA channel 0 transfer select 28: USB ready"]
    DMA0TSEL_28 = 28,
    #[doc = "29: DMA channel 0 transfer select 29: Multiplier ready"]
    DMA0TSEL_29 = 29,
    #[doc = "30: DMA channel 0 transfer select 30: previous DMA channel DMA2IFG"]
    DMA0TSEL_30 = 30,
    #[doc = "31: DMA channel 0 transfer select 31: ext. Trigger (DMAE0)"]
    DMA0TSEL_31 = 31,
}
impl From<DMA0TSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA0TSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMA0TSEL_A {
    type Ux = u8;
}
impl DMA0TSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA0TSEL_A {
        match self.bits {
            0 => DMA0TSEL_A::DMA0TSEL_0,
            1 => DMA0TSEL_A::DMA0TSEL_1,
            2 => DMA0TSEL_A::DMA0TSEL_2,
            3 => DMA0TSEL_A::DMA0TSEL_3,
            4 => DMA0TSEL_A::DMA0TSEL_4,
            5 => DMA0TSEL_A::DMA0TSEL_5,
            6 => DMA0TSEL_A::DMA0TSEL_6,
            7 => DMA0TSEL_A::DMA0TSEL_7,
            8 => DMA0TSEL_A::DMA0TSEL_8,
            9 => DMA0TSEL_A::DMA0TSEL_9,
            10 => DMA0TSEL_A::DMA0TSEL_10,
            11 => DMA0TSEL_A::DMA0TSEL_11,
            12 => DMA0TSEL_A::DMA0TSEL_12,
            13 => DMA0TSEL_A::DMA0TSEL_13,
            14 => DMA0TSEL_A::DMA0TSEL_14,
            15 => DMA0TSEL_A::DMA0TSEL_15,
            16 => DMA0TSEL_A::DMA0TSEL_16,
            17 => DMA0TSEL_A::DMA0TSEL_17,
            18 => DMA0TSEL_A::DMA0TSEL_18,
            19 => DMA0TSEL_A::DMA0TSEL_19,
            20 => DMA0TSEL_A::DMA0TSEL_20,
            21 => DMA0TSEL_A::DMA0TSEL_21,
            22 => DMA0TSEL_A::DMA0TSEL_22,
            23 => DMA0TSEL_A::DMA0TSEL_23,
            24 => DMA0TSEL_A::DMA0TSEL_24,
            25 => DMA0TSEL_A::DMA0TSEL_25,
            26 => DMA0TSEL_A::DMA0TSEL_26,
            27 => DMA0TSEL_A::DMA0TSEL_27,
            28 => DMA0TSEL_A::DMA0TSEL_28,
            29 => DMA0TSEL_A::DMA0TSEL_29,
            30 => DMA0TSEL_A::DMA0TSEL_30,
            31 => DMA0TSEL_A::DMA0TSEL_31,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA channel 0 transfer select 0: DMA_REQ (sw)"]
    #[inline(always)]
    pub fn is_dma0tsel_0(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_0
    }
    #[doc = "DMA channel 0 transfer select 1: Timer0_A (TA0CCR0.IFG)"]
    #[inline(always)]
    pub fn is_dma0tsel_1(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_1
    }
    #[doc = "DMA channel 0 transfer select 2: Timer0_A (TA0CCR2.IFG)"]
    #[inline(always)]
    pub fn is_dma0tsel_2(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_2
    }
    #[doc = "DMA channel 0 transfer select 3: Timer1_A (TA1CCR0.IFG)"]
    #[inline(always)]
    pub fn is_dma0tsel_3(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_3
    }
    #[doc = "DMA channel 0 transfer select 4: Timer1_A (TA1CCR2.IFG)"]
    #[inline(always)]
    pub fn is_dma0tsel_4(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_4
    }
    #[doc = "DMA channel 0 transfer select 5: Timer2_A (TA2CCR0.IFG)"]
    #[inline(always)]
    pub fn is_dma0tsel_5(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_5
    }
    #[doc = "DMA channel 0 transfer select 6: Timer2_A (TA2CCR2.IFG)"]
    #[inline(always)]
    pub fn is_dma0tsel_6(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_6
    }
    #[doc = "DMA channel 0 transfer select 7: TimerB (TB0CCR0.IFG)"]
    #[inline(always)]
    pub fn is_dma0tsel_7(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_7
    }
    #[doc = "DMA channel 0 transfer select 8: TimerB (TB0CCR2.IFG)"]
    #[inline(always)]
    pub fn is_dma0tsel_8(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_8
    }
    #[doc = "DMA channel 0 transfer select 9: Reserved"]
    #[inline(always)]
    pub fn is_dma0tsel_9(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_9
    }
    #[doc = "DMA channel 0 transfer select 10: Reserved"]
    #[inline(always)]
    pub fn is_dma0tsel_10(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_10
    }
    #[doc = "DMA channel 0 transfer select 11: Reserved"]
    #[inline(always)]
    pub fn is_dma0tsel_11(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_11
    }
    #[doc = "DMA channel 0 transfer select 12: Reserved"]
    #[inline(always)]
    pub fn is_dma0tsel_12(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_12
    }
    #[doc = "DMA channel 0 transfer select 13: Reserved"]
    #[inline(always)]
    pub fn is_dma0tsel_13(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_13
    }
    #[doc = "DMA channel 0 transfer select 14: Reserved"]
    #[inline(always)]
    pub fn is_dma0tsel_14(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_14
    }
    #[doc = "DMA channel 0 transfer select 15: Reserved"]
    #[inline(always)]
    pub fn is_dma0tsel_15(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_15
    }
    #[doc = "DMA channel 0 transfer select 16: USCIA0 receive"]
    #[inline(always)]
    pub fn is_dma0tsel_16(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_16
    }
    #[doc = "DMA channel 0 transfer select 17: USCIA0 transmit"]
    #[inline(always)]
    pub fn is_dma0tsel_17(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_17
    }
    #[doc = "DMA channel 0 transfer select 18: USCIB0 receive"]
    #[inline(always)]
    pub fn is_dma0tsel_18(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_18
    }
    #[doc = "DMA channel 0 transfer select 19: USCIB0 transmit"]
    #[inline(always)]
    pub fn is_dma0tsel_19(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_19
    }
    #[doc = "DMA channel 0 transfer select 20: USCIA1 receive"]
    #[inline(always)]
    pub fn is_dma0tsel_20(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_20
    }
    #[doc = "DMA channel 0 transfer select 21: USCIA1 transmit"]
    #[inline(always)]
    pub fn is_dma0tsel_21(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_21
    }
    #[doc = "DMA channel 0 transfer select 22: USCIB1 receive"]
    #[inline(always)]
    pub fn is_dma0tsel_22(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_22
    }
    #[doc = "DMA channel 0 transfer select 23: USCIB1 transmit"]
    #[inline(always)]
    pub fn is_dma0tsel_23(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_23
    }
    #[doc = "DMA channel 0 transfer select 24: ADC12IFGx"]
    #[inline(always)]
    pub fn is_dma0tsel_24(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_24
    }
    #[doc = "DMA channel 0 transfer select 25: Reserved"]
    #[inline(always)]
    pub fn is_dma0tsel_25(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_25
    }
    #[doc = "DMA channel 0 transfer select 26: Reserved"]
    #[inline(always)]
    pub fn is_dma0tsel_26(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_26
    }
    #[doc = "DMA channel 0 transfer select 27: USB FNRXD"]
    #[inline(always)]
    pub fn is_dma0tsel_27(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_27
    }
    #[doc = "DMA channel 0 transfer select 28: USB ready"]
    #[inline(always)]
    pub fn is_dma0tsel_28(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_28
    }
    #[doc = "DMA channel 0 transfer select 29: Multiplier ready"]
    #[inline(always)]
    pub fn is_dma0tsel_29(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_29
    }
    #[doc = "DMA channel 0 transfer select 30: previous DMA channel DMA2IFG"]
    #[inline(always)]
    pub fn is_dma0tsel_30(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_30
    }
    #[doc = "DMA channel 0 transfer select 31: ext. Trigger (DMAE0)"]
    #[inline(always)]
    pub fn is_dma0tsel_31(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_31
    }
}
#[doc = "Field `DMA0TSEL` writer - DMA channel 0 transfer select bit 0"]
pub type DMA0TSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5, DMA0TSEL_A>;
impl<'a, REG> DMA0TSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA channel 0 transfer select 0: DMA_REQ (sw)"]
    #[inline(always)]
    pub fn dma0tsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_0)
    }
    #[doc = "DMA channel 0 transfer select 1: Timer0_A (TA0CCR0.IFG)"]
    #[inline(always)]
    pub fn dma0tsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_1)
    }
    #[doc = "DMA channel 0 transfer select 2: Timer0_A (TA0CCR2.IFG)"]
    #[inline(always)]
    pub fn dma0tsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_2)
    }
    #[doc = "DMA channel 0 transfer select 3: Timer1_A (TA1CCR0.IFG)"]
    #[inline(always)]
    pub fn dma0tsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_3)
    }
    #[doc = "DMA channel 0 transfer select 4: Timer1_A (TA1CCR2.IFG)"]
    #[inline(always)]
    pub fn dma0tsel_4(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_4)
    }
    #[doc = "DMA channel 0 transfer select 5: Timer2_A (TA2CCR0.IFG)"]
    #[inline(always)]
    pub fn dma0tsel_5(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_5)
    }
    #[doc = "DMA channel 0 transfer select 6: Timer2_A (TA2CCR2.IFG)"]
    #[inline(always)]
    pub fn dma0tsel_6(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_6)
    }
    #[doc = "DMA channel 0 transfer select 7: TimerB (TB0CCR0.IFG)"]
    #[inline(always)]
    pub fn dma0tsel_7(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_7)
    }
    #[doc = "DMA channel 0 transfer select 8: TimerB (TB0CCR2.IFG)"]
    #[inline(always)]
    pub fn dma0tsel_8(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_8)
    }
    #[doc = "DMA channel 0 transfer select 9: Reserved"]
    #[inline(always)]
    pub fn dma0tsel_9(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_9)
    }
    #[doc = "DMA channel 0 transfer select 10: Reserved"]
    #[inline(always)]
    pub fn dma0tsel_10(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_10)
    }
    #[doc = "DMA channel 0 transfer select 11: Reserved"]
    #[inline(always)]
    pub fn dma0tsel_11(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_11)
    }
    #[doc = "DMA channel 0 transfer select 12: Reserved"]
    #[inline(always)]
    pub fn dma0tsel_12(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_12)
    }
    #[doc = "DMA channel 0 transfer select 13: Reserved"]
    #[inline(always)]
    pub fn dma0tsel_13(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_13)
    }
    #[doc = "DMA channel 0 transfer select 14: Reserved"]
    #[inline(always)]
    pub fn dma0tsel_14(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_14)
    }
    #[doc = "DMA channel 0 transfer select 15: Reserved"]
    #[inline(always)]
    pub fn dma0tsel_15(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_15)
    }
    #[doc = "DMA channel 0 transfer select 16: USCIA0 receive"]
    #[inline(always)]
    pub fn dma0tsel_16(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_16)
    }
    #[doc = "DMA channel 0 transfer select 17: USCIA0 transmit"]
    #[inline(always)]
    pub fn dma0tsel_17(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_17)
    }
    #[doc = "DMA channel 0 transfer select 18: USCIB0 receive"]
    #[inline(always)]
    pub fn dma0tsel_18(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_18)
    }
    #[doc = "DMA channel 0 transfer select 19: USCIB0 transmit"]
    #[inline(always)]
    pub fn dma0tsel_19(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_19)
    }
    #[doc = "DMA channel 0 transfer select 20: USCIA1 receive"]
    #[inline(always)]
    pub fn dma0tsel_20(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_20)
    }
    #[doc = "DMA channel 0 transfer select 21: USCIA1 transmit"]
    #[inline(always)]
    pub fn dma0tsel_21(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_21)
    }
    #[doc = "DMA channel 0 transfer select 22: USCIB1 receive"]
    #[inline(always)]
    pub fn dma0tsel_22(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_22)
    }
    #[doc = "DMA channel 0 transfer select 23: USCIB1 transmit"]
    #[inline(always)]
    pub fn dma0tsel_23(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_23)
    }
    #[doc = "DMA channel 0 transfer select 24: ADC12IFGx"]
    #[inline(always)]
    pub fn dma0tsel_24(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_24)
    }
    #[doc = "DMA channel 0 transfer select 25: Reserved"]
    #[inline(always)]
    pub fn dma0tsel_25(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_25)
    }
    #[doc = "DMA channel 0 transfer select 26: Reserved"]
    #[inline(always)]
    pub fn dma0tsel_26(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_26)
    }
    #[doc = "DMA channel 0 transfer select 27: USB FNRXD"]
    #[inline(always)]
    pub fn dma0tsel_27(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_27)
    }
    #[doc = "DMA channel 0 transfer select 28: USB ready"]
    #[inline(always)]
    pub fn dma0tsel_28(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_28)
    }
    #[doc = "DMA channel 0 transfer select 29: Multiplier ready"]
    #[inline(always)]
    pub fn dma0tsel_29(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_29)
    }
    #[doc = "DMA channel 0 transfer select 30: previous DMA channel DMA2IFG"]
    #[inline(always)]
    pub fn dma0tsel_30(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_30)
    }
    #[doc = "DMA channel 0 transfer select 31: ext. Trigger (DMAE0)"]
    #[inline(always)]
    pub fn dma0tsel_31(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0TSEL_A::DMA0TSEL_31)
    }
}
#[doc = "Field `DMA1TSEL` reader - DMA channel 1 transfer select bit 0"]
pub type DMA1TSEL_R = crate::FieldReader<DMA1TSEL_A>;
#[doc = "DMA channel 1 transfer select bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA1TSEL_A {
    #[doc = "0: DMA channel 1 transfer select 0: DMA_REQ (sw)"]
    DMA1TSEL_0 = 0,
    #[doc = "1: DMA channel 1 transfer select 1: Timer0_A (TA0CCR0.IFG)"]
    DMA1TSEL_1 = 1,
    #[doc = "2: DMA channel 1 transfer select 2: Timer0_A (TA0CCR2.IFG)"]
    DMA1TSEL_2 = 2,
    #[doc = "3: DMA channel 1 transfer select 3: Timer1_A (TA1CCR0.IFG)"]
    DMA1TSEL_3 = 3,
    #[doc = "4: DMA channel 1 transfer select 4: Timer1_A (TA1CCR2.IFG)"]
    DMA1TSEL_4 = 4,
    #[doc = "5: DMA channel 1 transfer select 5: Timer2_A (TA2CCR0.IFG)"]
    DMA1TSEL_5 = 5,
    #[doc = "6: DMA channel 1 transfer select 6: Timer2_A (TA2CCR2.IFG)"]
    DMA1TSEL_6 = 6,
    #[doc = "7: DMA channel 1 transfer select 7: TimerB (TB0CCR0.IFG)"]
    DMA1TSEL_7 = 7,
    #[doc = "8: DMA channel 1 transfer select 8: TimerB (TB0CCR2.IFG)"]
    DMA1TSEL_8 = 8,
    #[doc = "9: DMA channel 1 transfer select 9: Reserved"]
    DMA1TSEL_9 = 9,
    #[doc = "10: DMA channel 1 transfer select 10: Reserved"]
    DMA1TSEL_10 = 10,
    #[doc = "11: DMA channel 1 transfer select 11: Reserved"]
    DMA1TSEL_11 = 11,
    #[doc = "12: DMA channel 1 transfer select 12: Reserved"]
    DMA1TSEL_12 = 12,
    #[doc = "13: DMA channel 1 transfer select 13: Reserved"]
    DMA1TSEL_13 = 13,
    #[doc = "14: DMA channel 1 transfer select 14: Reserved"]
    DMA1TSEL_14 = 14,
    #[doc = "15: DMA channel 1 transfer select 15: Reserved"]
    DMA1TSEL_15 = 15,
    #[doc = "16: DMA channel 1 transfer select 16: USCIA0 receive"]
    DMA1TSEL_16 = 16,
    #[doc = "17: DMA channel 1 transfer select 17: USCIA0 transmit"]
    DMA1TSEL_17 = 17,
    #[doc = "18: DMA channel 1 transfer select 18: USCIB0 receive"]
    DMA1TSEL_18 = 18,
    #[doc = "19: DMA channel 1 transfer select 19: USCIB0 transmit"]
    DMA1TSEL_19 = 19,
    #[doc = "20: DMA channel 1 transfer select 20: USCIA1 receive"]
    DMA1TSEL_20 = 20,
    #[doc = "21: DMA channel 1 transfer select 21: USCIA1 transmit"]
    DMA1TSEL_21 = 21,
    #[doc = "22: DMA channel 1 transfer select 22: USCIB1 receive"]
    DMA1TSEL_22 = 22,
    #[doc = "23: DMA channel 1 transfer select 23: USCIB1 transmit"]
    DMA1TSEL_23 = 23,
    #[doc = "24: DMA channel 1 transfer select 24: ADC12IFGx"]
    DMA1TSEL_24 = 24,
    #[doc = "25: DMA channel 1 transfer select 25: Reserved"]
    DMA1TSEL_25 = 25,
    #[doc = "26: DMA channel 1 transfer select 26: Reserved"]
    DMA1TSEL_26 = 26,
    #[doc = "27: DMA channel 1 transfer select 27: USB FNRXD"]
    DMA1TSEL_27 = 27,
    #[doc = "28: DMA channel 1 transfer select 28: USB ready"]
    DMA1TSEL_28 = 28,
    #[doc = "29: DMA channel 1 transfer select 29: Multiplier ready"]
    DMA1TSEL_29 = 29,
    #[doc = "30: DMA channel 1 transfer select 30: previous DMA channel DMA0IFG"]
    DMA1TSEL_30 = 30,
    #[doc = "31: DMA channel 1 transfer select 31: ext. Trigger (DMAE0)"]
    DMA1TSEL_31 = 31,
}
impl From<DMA1TSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA1TSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMA1TSEL_A {
    type Ux = u8;
}
impl DMA1TSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA1TSEL_A {
        match self.bits {
            0 => DMA1TSEL_A::DMA1TSEL_0,
            1 => DMA1TSEL_A::DMA1TSEL_1,
            2 => DMA1TSEL_A::DMA1TSEL_2,
            3 => DMA1TSEL_A::DMA1TSEL_3,
            4 => DMA1TSEL_A::DMA1TSEL_4,
            5 => DMA1TSEL_A::DMA1TSEL_5,
            6 => DMA1TSEL_A::DMA1TSEL_6,
            7 => DMA1TSEL_A::DMA1TSEL_7,
            8 => DMA1TSEL_A::DMA1TSEL_8,
            9 => DMA1TSEL_A::DMA1TSEL_9,
            10 => DMA1TSEL_A::DMA1TSEL_10,
            11 => DMA1TSEL_A::DMA1TSEL_11,
            12 => DMA1TSEL_A::DMA1TSEL_12,
            13 => DMA1TSEL_A::DMA1TSEL_13,
            14 => DMA1TSEL_A::DMA1TSEL_14,
            15 => DMA1TSEL_A::DMA1TSEL_15,
            16 => DMA1TSEL_A::DMA1TSEL_16,
            17 => DMA1TSEL_A::DMA1TSEL_17,
            18 => DMA1TSEL_A::DMA1TSEL_18,
            19 => DMA1TSEL_A::DMA1TSEL_19,
            20 => DMA1TSEL_A::DMA1TSEL_20,
            21 => DMA1TSEL_A::DMA1TSEL_21,
            22 => DMA1TSEL_A::DMA1TSEL_22,
            23 => DMA1TSEL_A::DMA1TSEL_23,
            24 => DMA1TSEL_A::DMA1TSEL_24,
            25 => DMA1TSEL_A::DMA1TSEL_25,
            26 => DMA1TSEL_A::DMA1TSEL_26,
            27 => DMA1TSEL_A::DMA1TSEL_27,
            28 => DMA1TSEL_A::DMA1TSEL_28,
            29 => DMA1TSEL_A::DMA1TSEL_29,
            30 => DMA1TSEL_A::DMA1TSEL_30,
            31 => DMA1TSEL_A::DMA1TSEL_31,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA channel 1 transfer select 0: DMA_REQ (sw)"]
    #[inline(always)]
    pub fn is_dma1tsel_0(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_0
    }
    #[doc = "DMA channel 1 transfer select 1: Timer0_A (TA0CCR0.IFG)"]
    #[inline(always)]
    pub fn is_dma1tsel_1(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_1
    }
    #[doc = "DMA channel 1 transfer select 2: Timer0_A (TA0CCR2.IFG)"]
    #[inline(always)]
    pub fn is_dma1tsel_2(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_2
    }
    #[doc = "DMA channel 1 transfer select 3: Timer1_A (TA1CCR0.IFG)"]
    #[inline(always)]
    pub fn is_dma1tsel_3(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_3
    }
    #[doc = "DMA channel 1 transfer select 4: Timer1_A (TA1CCR2.IFG)"]
    #[inline(always)]
    pub fn is_dma1tsel_4(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_4
    }
    #[doc = "DMA channel 1 transfer select 5: Timer2_A (TA2CCR0.IFG)"]
    #[inline(always)]
    pub fn is_dma1tsel_5(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_5
    }
    #[doc = "DMA channel 1 transfer select 6: Timer2_A (TA2CCR2.IFG)"]
    #[inline(always)]
    pub fn is_dma1tsel_6(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_6
    }
    #[doc = "DMA channel 1 transfer select 7: TimerB (TB0CCR0.IFG)"]
    #[inline(always)]
    pub fn is_dma1tsel_7(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_7
    }
    #[doc = "DMA channel 1 transfer select 8: TimerB (TB0CCR2.IFG)"]
    #[inline(always)]
    pub fn is_dma1tsel_8(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_8
    }
    #[doc = "DMA channel 1 transfer select 9: Reserved"]
    #[inline(always)]
    pub fn is_dma1tsel_9(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_9
    }
    #[doc = "DMA channel 1 transfer select 10: Reserved"]
    #[inline(always)]
    pub fn is_dma1tsel_10(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_10
    }
    #[doc = "DMA channel 1 transfer select 11: Reserved"]
    #[inline(always)]
    pub fn is_dma1tsel_11(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_11
    }
    #[doc = "DMA channel 1 transfer select 12: Reserved"]
    #[inline(always)]
    pub fn is_dma1tsel_12(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_12
    }
    #[doc = "DMA channel 1 transfer select 13: Reserved"]
    #[inline(always)]
    pub fn is_dma1tsel_13(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_13
    }
    #[doc = "DMA channel 1 transfer select 14: Reserved"]
    #[inline(always)]
    pub fn is_dma1tsel_14(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_14
    }
    #[doc = "DMA channel 1 transfer select 15: Reserved"]
    #[inline(always)]
    pub fn is_dma1tsel_15(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_15
    }
    #[doc = "DMA channel 1 transfer select 16: USCIA0 receive"]
    #[inline(always)]
    pub fn is_dma1tsel_16(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_16
    }
    #[doc = "DMA channel 1 transfer select 17: USCIA0 transmit"]
    #[inline(always)]
    pub fn is_dma1tsel_17(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_17
    }
    #[doc = "DMA channel 1 transfer select 18: USCIB0 receive"]
    #[inline(always)]
    pub fn is_dma1tsel_18(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_18
    }
    #[doc = "DMA channel 1 transfer select 19: USCIB0 transmit"]
    #[inline(always)]
    pub fn is_dma1tsel_19(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_19
    }
    #[doc = "DMA channel 1 transfer select 20: USCIA1 receive"]
    #[inline(always)]
    pub fn is_dma1tsel_20(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_20
    }
    #[doc = "DMA channel 1 transfer select 21: USCIA1 transmit"]
    #[inline(always)]
    pub fn is_dma1tsel_21(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_21
    }
    #[doc = "DMA channel 1 transfer select 22: USCIB1 receive"]
    #[inline(always)]
    pub fn is_dma1tsel_22(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_22
    }
    #[doc = "DMA channel 1 transfer select 23: USCIB1 transmit"]
    #[inline(always)]
    pub fn is_dma1tsel_23(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_23
    }
    #[doc = "DMA channel 1 transfer select 24: ADC12IFGx"]
    #[inline(always)]
    pub fn is_dma1tsel_24(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_24
    }
    #[doc = "DMA channel 1 transfer select 25: Reserved"]
    #[inline(always)]
    pub fn is_dma1tsel_25(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_25
    }
    #[doc = "DMA channel 1 transfer select 26: Reserved"]
    #[inline(always)]
    pub fn is_dma1tsel_26(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_26
    }
    #[doc = "DMA channel 1 transfer select 27: USB FNRXD"]
    #[inline(always)]
    pub fn is_dma1tsel_27(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_27
    }
    #[doc = "DMA channel 1 transfer select 28: USB ready"]
    #[inline(always)]
    pub fn is_dma1tsel_28(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_28
    }
    #[doc = "DMA channel 1 transfer select 29: Multiplier ready"]
    #[inline(always)]
    pub fn is_dma1tsel_29(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_29
    }
    #[doc = "DMA channel 1 transfer select 30: previous DMA channel DMA0IFG"]
    #[inline(always)]
    pub fn is_dma1tsel_30(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_30
    }
    #[doc = "DMA channel 1 transfer select 31: ext. Trigger (DMAE0)"]
    #[inline(always)]
    pub fn is_dma1tsel_31(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_31
    }
}
#[doc = "Field `DMA1TSEL` writer - DMA channel 1 transfer select bit 0"]
pub type DMA1TSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5, DMA1TSEL_A>;
impl<'a, REG> DMA1TSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA channel 1 transfer select 0: DMA_REQ (sw)"]
    #[inline(always)]
    pub fn dma1tsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_0)
    }
    #[doc = "DMA channel 1 transfer select 1: Timer0_A (TA0CCR0.IFG)"]
    #[inline(always)]
    pub fn dma1tsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_1)
    }
    #[doc = "DMA channel 1 transfer select 2: Timer0_A (TA0CCR2.IFG)"]
    #[inline(always)]
    pub fn dma1tsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_2)
    }
    #[doc = "DMA channel 1 transfer select 3: Timer1_A (TA1CCR0.IFG)"]
    #[inline(always)]
    pub fn dma1tsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_3)
    }
    #[doc = "DMA channel 1 transfer select 4: Timer1_A (TA1CCR2.IFG)"]
    #[inline(always)]
    pub fn dma1tsel_4(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_4)
    }
    #[doc = "DMA channel 1 transfer select 5: Timer2_A (TA2CCR0.IFG)"]
    #[inline(always)]
    pub fn dma1tsel_5(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_5)
    }
    #[doc = "DMA channel 1 transfer select 6: Timer2_A (TA2CCR2.IFG)"]
    #[inline(always)]
    pub fn dma1tsel_6(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_6)
    }
    #[doc = "DMA channel 1 transfer select 7: TimerB (TB0CCR0.IFG)"]
    #[inline(always)]
    pub fn dma1tsel_7(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_7)
    }
    #[doc = "DMA channel 1 transfer select 8: TimerB (TB0CCR2.IFG)"]
    #[inline(always)]
    pub fn dma1tsel_8(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_8)
    }
    #[doc = "DMA channel 1 transfer select 9: Reserved"]
    #[inline(always)]
    pub fn dma1tsel_9(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_9)
    }
    #[doc = "DMA channel 1 transfer select 10: Reserved"]
    #[inline(always)]
    pub fn dma1tsel_10(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_10)
    }
    #[doc = "DMA channel 1 transfer select 11: Reserved"]
    #[inline(always)]
    pub fn dma1tsel_11(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_11)
    }
    #[doc = "DMA channel 1 transfer select 12: Reserved"]
    #[inline(always)]
    pub fn dma1tsel_12(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_12)
    }
    #[doc = "DMA channel 1 transfer select 13: Reserved"]
    #[inline(always)]
    pub fn dma1tsel_13(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_13)
    }
    #[doc = "DMA channel 1 transfer select 14: Reserved"]
    #[inline(always)]
    pub fn dma1tsel_14(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_14)
    }
    #[doc = "DMA channel 1 transfer select 15: Reserved"]
    #[inline(always)]
    pub fn dma1tsel_15(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_15)
    }
    #[doc = "DMA channel 1 transfer select 16: USCIA0 receive"]
    #[inline(always)]
    pub fn dma1tsel_16(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_16)
    }
    #[doc = "DMA channel 1 transfer select 17: USCIA0 transmit"]
    #[inline(always)]
    pub fn dma1tsel_17(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_17)
    }
    #[doc = "DMA channel 1 transfer select 18: USCIB0 receive"]
    #[inline(always)]
    pub fn dma1tsel_18(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_18)
    }
    #[doc = "DMA channel 1 transfer select 19: USCIB0 transmit"]
    #[inline(always)]
    pub fn dma1tsel_19(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_19)
    }
    #[doc = "DMA channel 1 transfer select 20: USCIA1 receive"]
    #[inline(always)]
    pub fn dma1tsel_20(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_20)
    }
    #[doc = "DMA channel 1 transfer select 21: USCIA1 transmit"]
    #[inline(always)]
    pub fn dma1tsel_21(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_21)
    }
    #[doc = "DMA channel 1 transfer select 22: USCIB1 receive"]
    #[inline(always)]
    pub fn dma1tsel_22(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_22)
    }
    #[doc = "DMA channel 1 transfer select 23: USCIB1 transmit"]
    #[inline(always)]
    pub fn dma1tsel_23(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_23)
    }
    #[doc = "DMA channel 1 transfer select 24: ADC12IFGx"]
    #[inline(always)]
    pub fn dma1tsel_24(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_24)
    }
    #[doc = "DMA channel 1 transfer select 25: Reserved"]
    #[inline(always)]
    pub fn dma1tsel_25(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_25)
    }
    #[doc = "DMA channel 1 transfer select 26: Reserved"]
    #[inline(always)]
    pub fn dma1tsel_26(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_26)
    }
    #[doc = "DMA channel 1 transfer select 27: USB FNRXD"]
    #[inline(always)]
    pub fn dma1tsel_27(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_27)
    }
    #[doc = "DMA channel 1 transfer select 28: USB ready"]
    #[inline(always)]
    pub fn dma1tsel_28(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_28)
    }
    #[doc = "DMA channel 1 transfer select 29: Multiplier ready"]
    #[inline(always)]
    pub fn dma1tsel_29(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_29)
    }
    #[doc = "DMA channel 1 transfer select 30: previous DMA channel DMA0IFG"]
    #[inline(always)]
    pub fn dma1tsel_30(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_30)
    }
    #[doc = "DMA channel 1 transfer select 31: ext. Trigger (DMAE0)"]
    #[inline(always)]
    pub fn dma1tsel_31(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1TSEL_A::DMA1TSEL_31)
    }
}
impl R {
    #[doc = "Bits 0:4 - DMA channel 0 transfer select bit 0"]
    #[inline(always)]
    pub fn dma0tsel(&self) -> DMA0TSEL_R {
        DMA0TSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA channel 1 transfer select bit 0"]
    #[inline(always)]
    pub fn dma1tsel(&self) -> DMA1TSEL_R {
        DMA1TSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA channel 0 transfer select bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dma0tsel(&mut self) -> DMA0TSEL_W<DMACTL0_SPEC> {
        DMA0TSEL_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - DMA channel 1 transfer select bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dma1tsel(&mut self) -> DMA1TSEL_W<DMACTL0_SPEC> {
        DMA1TSEL_W::new(self, 8)
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
#[doc = "DMA Module Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACTL0_SPEC;
impl crate::RegisterSpec for DMACTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dmactl0::R`](R) reader structure"]
impl crate::Readable for DMACTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmactl0::W`](W) writer structure"]
impl crate::Writable for DMACTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACTL0 to value 0"]
impl crate::Resettable for DMACTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
