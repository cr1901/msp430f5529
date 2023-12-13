#[doc = "Register `TB0CCTL2` reader"]
pub type R = crate::R<TB0CCTL2_SPEC>;
#[doc = "Register `TB0CCTL2` writer"]
pub type W = crate::W<TB0CCTL2_SPEC>;
#[doc = "Field `CCIFG` reader - Capture/compare interrupt flag"]
pub type CCIFG_R = crate::BitReader;
#[doc = "Field `CCIFG` writer - Capture/compare interrupt flag"]
pub type CCIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COV` reader - Capture/compare overflow flag"]
pub type COV_R = crate::BitReader;
#[doc = "Field `COV` writer - Capture/compare overflow flag"]
pub type COV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT` reader - PWM Output signal if output mode 0"]
pub type OUT_R = crate::BitReader;
#[doc = "Field `OUT` writer - PWM Output signal if output mode 0"]
pub type OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCI` reader - Capture input signal (read)"]
pub type CCI_R = crate::BitReader;
#[doc = "Field `CCI` writer - Capture input signal (read)"]
pub type CCI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCIE` reader - Capture/compare interrupt enable"]
pub type CCIE_R = crate::BitReader;
#[doc = "Field `CCIE` writer - Capture/compare interrupt enable"]
pub type CCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTMOD` reader - Output mode 2"]
pub type OUTMOD_R = crate::FieldReader<OUTMOD_A>;
#[doc = "Output mode 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUTMOD_A {
    #[doc = "0: PWM output mode: 0 - output only"]
    OUTMOD_0 = 0,
    #[doc = "1: PWM output mode: 1 - set"]
    OUTMOD_1 = 1,
    #[doc = "2: PWM output mode: 2 - PWM toggle/reset"]
    OUTMOD_2 = 2,
    #[doc = "3: PWM output mode: 3 - PWM set/reset"]
    OUTMOD_3 = 3,
    #[doc = "4: PWM output mode: 4 - toggle"]
    OUTMOD_4 = 4,
    #[doc = "5: PWM output mode: 5 - Reset"]
    OUTMOD_5 = 5,
    #[doc = "6: PWM output mode: 6 - PWM toggle/set"]
    OUTMOD_6 = 6,
    #[doc = "7: PWM output mode: 7 - PWM reset/set"]
    OUTMOD_7 = 7,
}
impl From<OUTMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTMOD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OUTMOD_A {
    type Ux = u8;
}
impl OUTMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OUTMOD_A {
        match self.bits {
            0 => OUTMOD_A::OUTMOD_0,
            1 => OUTMOD_A::OUTMOD_1,
            2 => OUTMOD_A::OUTMOD_2,
            3 => OUTMOD_A::OUTMOD_3,
            4 => OUTMOD_A::OUTMOD_4,
            5 => OUTMOD_A::OUTMOD_5,
            6 => OUTMOD_A::OUTMOD_6,
            7 => OUTMOD_A::OUTMOD_7,
            _ => unreachable!(),
        }
    }
    #[doc = "PWM output mode: 0 - output only"]
    #[inline(always)]
    pub fn is_outmod_0(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_0
    }
    #[doc = "PWM output mode: 1 - set"]
    #[inline(always)]
    pub fn is_outmod_1(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_1
    }
    #[doc = "PWM output mode: 2 - PWM toggle/reset"]
    #[inline(always)]
    pub fn is_outmod_2(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_2
    }
    #[doc = "PWM output mode: 3 - PWM set/reset"]
    #[inline(always)]
    pub fn is_outmod_3(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_3
    }
    #[doc = "PWM output mode: 4 - toggle"]
    #[inline(always)]
    pub fn is_outmod_4(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_4
    }
    #[doc = "PWM output mode: 5 - Reset"]
    #[inline(always)]
    pub fn is_outmod_5(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_5
    }
    #[doc = "PWM output mode: 6 - PWM toggle/set"]
    #[inline(always)]
    pub fn is_outmod_6(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_6
    }
    #[doc = "PWM output mode: 7 - PWM reset/set"]
    #[inline(always)]
    pub fn is_outmod_7(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_7
    }
}
#[doc = "Field `OUTMOD` writer - Output mode 2"]
pub type OUTMOD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, OUTMOD_A>;
impl<'a, REG> OUTMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PWM output mode: 0 - output only"]
    #[inline(always)]
    pub fn outmod_0(self) -> &'a mut crate::W<REG> {
        self.variant(OUTMOD_A::OUTMOD_0)
    }
    #[doc = "PWM output mode: 1 - set"]
    #[inline(always)]
    pub fn outmod_1(self) -> &'a mut crate::W<REG> {
        self.variant(OUTMOD_A::OUTMOD_1)
    }
    #[doc = "PWM output mode: 2 - PWM toggle/reset"]
    #[inline(always)]
    pub fn outmod_2(self) -> &'a mut crate::W<REG> {
        self.variant(OUTMOD_A::OUTMOD_2)
    }
    #[doc = "PWM output mode: 3 - PWM set/reset"]
    #[inline(always)]
    pub fn outmod_3(self) -> &'a mut crate::W<REG> {
        self.variant(OUTMOD_A::OUTMOD_3)
    }
    #[doc = "PWM output mode: 4 - toggle"]
    #[inline(always)]
    pub fn outmod_4(self) -> &'a mut crate::W<REG> {
        self.variant(OUTMOD_A::OUTMOD_4)
    }
    #[doc = "PWM output mode: 5 - Reset"]
    #[inline(always)]
    pub fn outmod_5(self) -> &'a mut crate::W<REG> {
        self.variant(OUTMOD_A::OUTMOD_5)
    }
    #[doc = "PWM output mode: 6 - PWM toggle/set"]
    #[inline(always)]
    pub fn outmod_6(self) -> &'a mut crate::W<REG> {
        self.variant(OUTMOD_A::OUTMOD_6)
    }
    #[doc = "PWM output mode: 7 - PWM reset/set"]
    #[inline(always)]
    pub fn outmod_7(self) -> &'a mut crate::W<REG> {
        self.variant(OUTMOD_A::OUTMOD_7)
    }
}
#[doc = "Field `CAP` reader - Capture mode: 1 /Compare mode : 0"]
pub type CAP_R = crate::BitReader;
#[doc = "Field `CAP` writer - Capture mode: 1 /Compare mode : 0"]
pub type CAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLLD` reader - Compare latch load source 1"]
pub type CLLD_R = crate::FieldReader<CLLD_A>;
#[doc = "Compare latch load source 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLLD_A {
    #[doc = "0: Compare latch load sourec : 0 - immediate"]
    CLLD_0 = 0,
    #[doc = "1: Compare latch load sourec : 1 - TBR counts to 0"]
    CLLD_1 = 1,
    #[doc = "2: Compare latch load sourec : 2 - up/down"]
    CLLD_2 = 2,
    #[doc = "3: Compare latch load sourec : 3 - TBR counts to TBCTL0"]
    CLLD_3 = 3,
}
impl From<CLLD_A> for u8 {
    #[inline(always)]
    fn from(variant: CLLD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLLD_A {
    type Ux = u8;
}
impl CLLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLLD_A {
        match self.bits {
            0 => CLLD_A::CLLD_0,
            1 => CLLD_A::CLLD_1,
            2 => CLLD_A::CLLD_2,
            3 => CLLD_A::CLLD_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Compare latch load sourec : 0 - immediate"]
    #[inline(always)]
    pub fn is_clld_0(&self) -> bool {
        *self == CLLD_A::CLLD_0
    }
    #[doc = "Compare latch load sourec : 1 - TBR counts to 0"]
    #[inline(always)]
    pub fn is_clld_1(&self) -> bool {
        *self == CLLD_A::CLLD_1
    }
    #[doc = "Compare latch load sourec : 2 - up/down"]
    #[inline(always)]
    pub fn is_clld_2(&self) -> bool {
        *self == CLLD_A::CLLD_2
    }
    #[doc = "Compare latch load sourec : 3 - TBR counts to TBCTL0"]
    #[inline(always)]
    pub fn is_clld_3(&self) -> bool {
        *self == CLLD_A::CLLD_3
    }
}
#[doc = "Field `CLLD` writer - Compare latch load source 1"]
pub type CLLD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CLLD_A>;
impl<'a, REG> CLLD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Compare latch load sourec : 0 - immediate"]
    #[inline(always)]
    pub fn clld_0(self) -> &'a mut crate::W<REG> {
        self.variant(CLLD_A::CLLD_0)
    }
    #[doc = "Compare latch load sourec : 1 - TBR counts to 0"]
    #[inline(always)]
    pub fn clld_1(self) -> &'a mut crate::W<REG> {
        self.variant(CLLD_A::CLLD_1)
    }
    #[doc = "Compare latch load sourec : 2 - up/down"]
    #[inline(always)]
    pub fn clld_2(self) -> &'a mut crate::W<REG> {
        self.variant(CLLD_A::CLLD_2)
    }
    #[doc = "Compare latch load sourec : 3 - TBR counts to TBCTL0"]
    #[inline(always)]
    pub fn clld_3(self) -> &'a mut crate::W<REG> {
        self.variant(CLLD_A::CLLD_3)
    }
}
#[doc = "Field `SCS` reader - Capture sychronize"]
pub type SCS_R = crate::BitReader;
#[doc = "Field `SCS` writer - Capture sychronize"]
pub type SCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCIS` reader - Capture input select 1"]
pub type CCIS_R = crate::FieldReader<CCIS_A>;
#[doc = "Capture input select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CCIS_A {
    #[doc = "0: Capture input select: 0 - CCIxA"]
    CCIS_0 = 0,
    #[doc = "1: Capture input select: 1 - CCIxB"]
    CCIS_1 = 1,
    #[doc = "2: Capture input select: 2 - GND"]
    CCIS_2 = 2,
    #[doc = "3: Capture input select: 3 - Vcc"]
    CCIS_3 = 3,
}
impl From<CCIS_A> for u8 {
    #[inline(always)]
    fn from(variant: CCIS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CCIS_A {
    type Ux = u8;
}
impl CCIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCIS_A {
        match self.bits {
            0 => CCIS_A::CCIS_0,
            1 => CCIS_A::CCIS_1,
            2 => CCIS_A::CCIS_2,
            3 => CCIS_A::CCIS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Capture input select: 0 - CCIxA"]
    #[inline(always)]
    pub fn is_ccis_0(&self) -> bool {
        *self == CCIS_A::CCIS_0
    }
    #[doc = "Capture input select: 1 - CCIxB"]
    #[inline(always)]
    pub fn is_ccis_1(&self) -> bool {
        *self == CCIS_A::CCIS_1
    }
    #[doc = "Capture input select: 2 - GND"]
    #[inline(always)]
    pub fn is_ccis_2(&self) -> bool {
        *self == CCIS_A::CCIS_2
    }
    #[doc = "Capture input select: 3 - Vcc"]
    #[inline(always)]
    pub fn is_ccis_3(&self) -> bool {
        *self == CCIS_A::CCIS_3
    }
}
#[doc = "Field `CCIS` writer - Capture input select 1"]
pub type CCIS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CCIS_A>;
impl<'a, REG> CCIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Capture input select: 0 - CCIxA"]
    #[inline(always)]
    pub fn ccis_0(self) -> &'a mut crate::W<REG> {
        self.variant(CCIS_A::CCIS_0)
    }
    #[doc = "Capture input select: 1 - CCIxB"]
    #[inline(always)]
    pub fn ccis_1(self) -> &'a mut crate::W<REG> {
        self.variant(CCIS_A::CCIS_1)
    }
    #[doc = "Capture input select: 2 - GND"]
    #[inline(always)]
    pub fn ccis_2(self) -> &'a mut crate::W<REG> {
        self.variant(CCIS_A::CCIS_2)
    }
    #[doc = "Capture input select: 3 - Vcc"]
    #[inline(always)]
    pub fn ccis_3(self) -> &'a mut crate::W<REG> {
        self.variant(CCIS_A::CCIS_3)
    }
}
#[doc = "Field `CM` reader - Capture mode 1"]
pub type CM_R = crate::FieldReader<CM_A>;
#[doc = "Capture mode 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CM_A {
    #[doc = "0: Capture mode: 0 - disabled"]
    CM_0 = 0,
    #[doc = "1: Capture mode: 1 - pos. edge"]
    CM_1 = 1,
    #[doc = "2: Capture mode: 1 - neg. edge"]
    CM_2 = 2,
    #[doc = "3: Capture mode: 1 - both edges"]
    CM_3 = 3,
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CM_A {
    type Ux = u8;
}
impl CM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CM_A {
        match self.bits {
            0 => CM_A::CM_0,
            1 => CM_A::CM_1,
            2 => CM_A::CM_2,
            3 => CM_A::CM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Capture mode: 0 - disabled"]
    #[inline(always)]
    pub fn is_cm_0(&self) -> bool {
        *self == CM_A::CM_0
    }
    #[doc = "Capture mode: 1 - pos. edge"]
    #[inline(always)]
    pub fn is_cm_1(&self) -> bool {
        *self == CM_A::CM_1
    }
    #[doc = "Capture mode: 1 - neg. edge"]
    #[inline(always)]
    pub fn is_cm_2(&self) -> bool {
        *self == CM_A::CM_2
    }
    #[doc = "Capture mode: 1 - both edges"]
    #[inline(always)]
    pub fn is_cm_3(&self) -> bool {
        *self == CM_A::CM_3
    }
}
#[doc = "Field `CM` writer - Capture mode 1"]
pub type CM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CM_A>;
impl<'a, REG> CM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Capture mode: 0 - disabled"]
    #[inline(always)]
    pub fn cm_0(self) -> &'a mut crate::W<REG> {
        self.variant(CM_A::CM_0)
    }
    #[doc = "Capture mode: 1 - pos. edge"]
    #[inline(always)]
    pub fn cm_1(self) -> &'a mut crate::W<REG> {
        self.variant(CM_A::CM_1)
    }
    #[doc = "Capture mode: 1 - neg. edge"]
    #[inline(always)]
    pub fn cm_2(self) -> &'a mut crate::W<REG> {
        self.variant(CM_A::CM_2)
    }
    #[doc = "Capture mode: 1 - both edges"]
    #[inline(always)]
    pub fn cm_3(self) -> &'a mut crate::W<REG> {
        self.variant(CM_A::CM_3)
    }
}
impl R {
    #[doc = "Bit 0 - Capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ccifg(&self) -> CCIFG_R {
        CCIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare overflow flag"]
    #[inline(always)]
    pub fn cov(&self) -> COV_R {
        COV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM Output signal if output mode 0"]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture input signal (read)"]
    #[inline(always)]
    pub fn cci(&self) -> CCI_R {
        CCI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ccie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Output mode 2"]
    #[inline(always)]
    pub fn outmod(&self) -> OUTMOD_R {
        OUTMOD_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Capture mode: 1 /Compare mode : 0"]
    #[inline(always)]
    pub fn cap(&self) -> CAP_R {
        CAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Compare latch load source 1"]
    #[inline(always)]
    pub fn clld(&self) -> CLLD_R {
        CLLD_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Capture sychronize"]
    #[inline(always)]
    pub fn scs(&self) -> SCS_R {
        SCS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Capture input select 1"]
    #[inline(always)]
    pub fn ccis(&self) -> CCIS_R {
        CCIS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Capture mode 1"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ccifg(&mut self) -> CCIFG_W<TB0CCTL2_SPEC> {
        CCIFG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/compare overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn cov(&mut self) -> COV_W<TB0CCTL2_SPEC> {
        COV_W::new(self, 1)
    }
    #[doc = "Bit 2 - PWM Output signal if output mode 0"]
    #[inline(always)]
    #[must_use]
    pub fn out(&mut self) -> OUT_W<TB0CCTL2_SPEC> {
        OUT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture input signal (read)"]
    #[inline(always)]
    #[must_use]
    pub fn cci(&mut self) -> CCI_W<TB0CCTL2_SPEC> {
        CCI_W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/compare interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccie(&mut self) -> CCIE_W<TB0CCTL2_SPEC> {
        CCIE_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - Output mode 2"]
    #[inline(always)]
    #[must_use]
    pub fn outmod(&mut self) -> OUTMOD_W<TB0CCTL2_SPEC> {
        OUTMOD_W::new(self, 5)
    }
    #[doc = "Bit 8 - Capture mode: 1 /Compare mode : 0"]
    #[inline(always)]
    #[must_use]
    pub fn cap(&mut self) -> CAP_W<TB0CCTL2_SPEC> {
        CAP_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - Compare latch load source 1"]
    #[inline(always)]
    #[must_use]
    pub fn clld(&mut self) -> CLLD_W<TB0CCTL2_SPEC> {
        CLLD_W::new(self, 9)
    }
    #[doc = "Bit 11 - Capture sychronize"]
    #[inline(always)]
    #[must_use]
    pub fn scs(&mut self) -> SCS_W<TB0CCTL2_SPEC> {
        SCS_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Capture input select 1"]
    #[inline(always)]
    #[must_use]
    pub fn ccis(&mut self) -> CCIS_W<TB0CCTL2_SPEC> {
        CCIS_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Capture mode 1"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<TB0CCTL2_SPEC> {
        CM_W::new(self, 14)
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
#[doc = "Timer0_B7 Capture/Compare Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tb0cctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tb0cctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TB0CCTL2_SPEC;
impl crate::RegisterSpec for TB0CCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tb0cctl2::R`](R) reader structure"]
impl crate::Readable for TB0CCTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tb0cctl2::W`](W) writer structure"]
impl crate::Writable for TB0CCTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TB0CCTL2 to value 0"]
impl crate::Resettable for TB0CCTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
