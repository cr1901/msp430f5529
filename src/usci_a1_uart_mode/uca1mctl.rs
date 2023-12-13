#[doc = "Register `UCA1MCTL` reader"]
pub type R = crate::R<UCA1MCTL_SPEC>;
#[doc = "Register `UCA1MCTL` writer"]
pub type W = crate::W<UCA1MCTL_SPEC>;
#[doc = "Field `UCOS16` reader - USCI 16-times Oversampling enable"]
pub type UCOS16_R = crate::BitReader;
#[doc = "Field `UCOS16` writer - USCI 16-times Oversampling enable"]
pub type UCOS16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBRS` reader - USCI Second Stage Modulation Select 2"]
pub type UCBRS_R = crate::FieldReader<UCBRS_A>;
#[doc = "USCI Second Stage Modulation Select 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCBRS_A {
    #[doc = "0: USCI Second Stage Modulation: 0"]
    UCBRS_0 = 0,
    #[doc = "1: USCI Second Stage Modulation: 1"]
    UCBRS_1 = 1,
    #[doc = "2: USCI Second Stage Modulation: 2"]
    UCBRS_2 = 2,
    #[doc = "3: USCI Second Stage Modulation: 3"]
    UCBRS_3 = 3,
    #[doc = "4: USCI Second Stage Modulation: 4"]
    UCBRS_4 = 4,
    #[doc = "5: USCI Second Stage Modulation: 5"]
    UCBRS_5 = 5,
    #[doc = "6: USCI Second Stage Modulation: 6"]
    UCBRS_6 = 6,
    #[doc = "7: USCI Second Stage Modulation: 7"]
    UCBRS_7 = 7,
}
impl From<UCBRS_A> for u8 {
    #[inline(always)]
    fn from(variant: UCBRS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UCBRS_A {
    type Ux = u8;
}
impl UCBRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UCBRS_A {
        match self.bits {
            0 => UCBRS_A::UCBRS_0,
            1 => UCBRS_A::UCBRS_1,
            2 => UCBRS_A::UCBRS_2,
            3 => UCBRS_A::UCBRS_3,
            4 => UCBRS_A::UCBRS_4,
            5 => UCBRS_A::UCBRS_5,
            6 => UCBRS_A::UCBRS_6,
            7 => UCBRS_A::UCBRS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "USCI Second Stage Modulation: 0"]
    #[inline(always)]
    pub fn is_ucbrs_0(&self) -> bool {
        *self == UCBRS_A::UCBRS_0
    }
    #[doc = "USCI Second Stage Modulation: 1"]
    #[inline(always)]
    pub fn is_ucbrs_1(&self) -> bool {
        *self == UCBRS_A::UCBRS_1
    }
    #[doc = "USCI Second Stage Modulation: 2"]
    #[inline(always)]
    pub fn is_ucbrs_2(&self) -> bool {
        *self == UCBRS_A::UCBRS_2
    }
    #[doc = "USCI Second Stage Modulation: 3"]
    #[inline(always)]
    pub fn is_ucbrs_3(&self) -> bool {
        *self == UCBRS_A::UCBRS_3
    }
    #[doc = "USCI Second Stage Modulation: 4"]
    #[inline(always)]
    pub fn is_ucbrs_4(&self) -> bool {
        *self == UCBRS_A::UCBRS_4
    }
    #[doc = "USCI Second Stage Modulation: 5"]
    #[inline(always)]
    pub fn is_ucbrs_5(&self) -> bool {
        *self == UCBRS_A::UCBRS_5
    }
    #[doc = "USCI Second Stage Modulation: 6"]
    #[inline(always)]
    pub fn is_ucbrs_6(&self) -> bool {
        *self == UCBRS_A::UCBRS_6
    }
    #[doc = "USCI Second Stage Modulation: 7"]
    #[inline(always)]
    pub fn is_ucbrs_7(&self) -> bool {
        *self == UCBRS_A::UCBRS_7
    }
}
#[doc = "Field `UCBRS` writer - USCI Second Stage Modulation Select 2"]
pub type UCBRS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, UCBRS_A>;
impl<'a, REG> UCBRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USCI Second Stage Modulation: 0"]
    #[inline(always)]
    pub fn ucbrs_0(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRS_A::UCBRS_0)
    }
    #[doc = "USCI Second Stage Modulation: 1"]
    #[inline(always)]
    pub fn ucbrs_1(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRS_A::UCBRS_1)
    }
    #[doc = "USCI Second Stage Modulation: 2"]
    #[inline(always)]
    pub fn ucbrs_2(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRS_A::UCBRS_2)
    }
    #[doc = "USCI Second Stage Modulation: 3"]
    #[inline(always)]
    pub fn ucbrs_3(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRS_A::UCBRS_3)
    }
    #[doc = "USCI Second Stage Modulation: 4"]
    #[inline(always)]
    pub fn ucbrs_4(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRS_A::UCBRS_4)
    }
    #[doc = "USCI Second Stage Modulation: 5"]
    #[inline(always)]
    pub fn ucbrs_5(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRS_A::UCBRS_5)
    }
    #[doc = "USCI Second Stage Modulation: 6"]
    #[inline(always)]
    pub fn ucbrs_6(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRS_A::UCBRS_6)
    }
    #[doc = "USCI Second Stage Modulation: 7"]
    #[inline(always)]
    pub fn ucbrs_7(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRS_A::UCBRS_7)
    }
}
#[doc = "Field `UCBRF` reader - USCI First Stage Modulation Select 3"]
pub type UCBRF_R = crate::FieldReader<UCBRF_A>;
#[doc = "USCI First Stage Modulation Select 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCBRF_A {
    #[doc = "0: USCI First Stage Modulation: 0"]
    UCBRF_0 = 0,
    #[doc = "1: USCI First Stage Modulation: 1"]
    UCBRF_1 = 1,
    #[doc = "2: USCI First Stage Modulation: 2"]
    UCBRF_2 = 2,
    #[doc = "3: USCI First Stage Modulation: 3"]
    UCBRF_3 = 3,
    #[doc = "4: USCI First Stage Modulation: 4"]
    UCBRF_4 = 4,
    #[doc = "5: USCI First Stage Modulation: 5"]
    UCBRF_5 = 5,
    #[doc = "6: USCI First Stage Modulation: 6"]
    UCBRF_6 = 6,
    #[doc = "7: USCI First Stage Modulation: 7"]
    UCBRF_7 = 7,
    #[doc = "8: USCI First Stage Modulation: 8"]
    UCBRF_8 = 8,
    #[doc = "9: USCI First Stage Modulation: 9"]
    UCBRF_9 = 9,
    #[doc = "10: USCI First Stage Modulation: A"]
    UCBRF_10 = 10,
    #[doc = "11: USCI First Stage Modulation: B"]
    UCBRF_11 = 11,
    #[doc = "12: USCI First Stage Modulation: C"]
    UCBRF_12 = 12,
    #[doc = "13: USCI First Stage Modulation: D"]
    UCBRF_13 = 13,
    #[doc = "14: USCI First Stage Modulation: E"]
    UCBRF_14 = 14,
    #[doc = "15: USCI First Stage Modulation: F"]
    UCBRF_15 = 15,
}
impl From<UCBRF_A> for u8 {
    #[inline(always)]
    fn from(variant: UCBRF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UCBRF_A {
    type Ux = u8;
}
impl UCBRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UCBRF_A {
        match self.bits {
            0 => UCBRF_A::UCBRF_0,
            1 => UCBRF_A::UCBRF_1,
            2 => UCBRF_A::UCBRF_2,
            3 => UCBRF_A::UCBRF_3,
            4 => UCBRF_A::UCBRF_4,
            5 => UCBRF_A::UCBRF_5,
            6 => UCBRF_A::UCBRF_6,
            7 => UCBRF_A::UCBRF_7,
            8 => UCBRF_A::UCBRF_8,
            9 => UCBRF_A::UCBRF_9,
            10 => UCBRF_A::UCBRF_10,
            11 => UCBRF_A::UCBRF_11,
            12 => UCBRF_A::UCBRF_12,
            13 => UCBRF_A::UCBRF_13,
            14 => UCBRF_A::UCBRF_14,
            15 => UCBRF_A::UCBRF_15,
            _ => unreachable!(),
        }
    }
    #[doc = "USCI First Stage Modulation: 0"]
    #[inline(always)]
    pub fn is_ucbrf_0(&self) -> bool {
        *self == UCBRF_A::UCBRF_0
    }
    #[doc = "USCI First Stage Modulation: 1"]
    #[inline(always)]
    pub fn is_ucbrf_1(&self) -> bool {
        *self == UCBRF_A::UCBRF_1
    }
    #[doc = "USCI First Stage Modulation: 2"]
    #[inline(always)]
    pub fn is_ucbrf_2(&self) -> bool {
        *self == UCBRF_A::UCBRF_2
    }
    #[doc = "USCI First Stage Modulation: 3"]
    #[inline(always)]
    pub fn is_ucbrf_3(&self) -> bool {
        *self == UCBRF_A::UCBRF_3
    }
    #[doc = "USCI First Stage Modulation: 4"]
    #[inline(always)]
    pub fn is_ucbrf_4(&self) -> bool {
        *self == UCBRF_A::UCBRF_4
    }
    #[doc = "USCI First Stage Modulation: 5"]
    #[inline(always)]
    pub fn is_ucbrf_5(&self) -> bool {
        *self == UCBRF_A::UCBRF_5
    }
    #[doc = "USCI First Stage Modulation: 6"]
    #[inline(always)]
    pub fn is_ucbrf_6(&self) -> bool {
        *self == UCBRF_A::UCBRF_6
    }
    #[doc = "USCI First Stage Modulation: 7"]
    #[inline(always)]
    pub fn is_ucbrf_7(&self) -> bool {
        *self == UCBRF_A::UCBRF_7
    }
    #[doc = "USCI First Stage Modulation: 8"]
    #[inline(always)]
    pub fn is_ucbrf_8(&self) -> bool {
        *self == UCBRF_A::UCBRF_8
    }
    #[doc = "USCI First Stage Modulation: 9"]
    #[inline(always)]
    pub fn is_ucbrf_9(&self) -> bool {
        *self == UCBRF_A::UCBRF_9
    }
    #[doc = "USCI First Stage Modulation: A"]
    #[inline(always)]
    pub fn is_ucbrf_10(&self) -> bool {
        *self == UCBRF_A::UCBRF_10
    }
    #[doc = "USCI First Stage Modulation: B"]
    #[inline(always)]
    pub fn is_ucbrf_11(&self) -> bool {
        *self == UCBRF_A::UCBRF_11
    }
    #[doc = "USCI First Stage Modulation: C"]
    #[inline(always)]
    pub fn is_ucbrf_12(&self) -> bool {
        *self == UCBRF_A::UCBRF_12
    }
    #[doc = "USCI First Stage Modulation: D"]
    #[inline(always)]
    pub fn is_ucbrf_13(&self) -> bool {
        *self == UCBRF_A::UCBRF_13
    }
    #[doc = "USCI First Stage Modulation: E"]
    #[inline(always)]
    pub fn is_ucbrf_14(&self) -> bool {
        *self == UCBRF_A::UCBRF_14
    }
    #[doc = "USCI First Stage Modulation: F"]
    #[inline(always)]
    pub fn is_ucbrf_15(&self) -> bool {
        *self == UCBRF_A::UCBRF_15
    }
}
#[doc = "Field `UCBRF` writer - USCI First Stage Modulation Select 3"]
pub type UCBRF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, UCBRF_A>;
impl<'a, REG> UCBRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USCI First Stage Modulation: 0"]
    #[inline(always)]
    pub fn ucbrf_0(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRF_A::UCBRF_0)
    }
    #[doc = "USCI First Stage Modulation: 1"]
    #[inline(always)]
    pub fn ucbrf_1(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRF_A::UCBRF_1)
    }
    #[doc = "USCI First Stage Modulation: 2"]
    #[inline(always)]
    pub fn ucbrf_2(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRF_A::UCBRF_2)
    }
    #[doc = "USCI First Stage Modulation: 3"]
    #[inline(always)]
    pub fn ucbrf_3(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRF_A::UCBRF_3)
    }
    #[doc = "USCI First Stage Modulation: 4"]
    #[inline(always)]
    pub fn ucbrf_4(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRF_A::UCBRF_4)
    }
    #[doc = "USCI First Stage Modulation: 5"]
    #[inline(always)]
    pub fn ucbrf_5(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRF_A::UCBRF_5)
    }
    #[doc = "USCI First Stage Modulation: 6"]
    #[inline(always)]
    pub fn ucbrf_6(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRF_A::UCBRF_6)
    }
    #[doc = "USCI First Stage Modulation: 7"]
    #[inline(always)]
    pub fn ucbrf_7(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRF_A::UCBRF_7)
    }
    #[doc = "USCI First Stage Modulation: 8"]
    #[inline(always)]
    pub fn ucbrf_8(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRF_A::UCBRF_8)
    }
    #[doc = "USCI First Stage Modulation: 9"]
    #[inline(always)]
    pub fn ucbrf_9(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRF_A::UCBRF_9)
    }
    #[doc = "USCI First Stage Modulation: A"]
    #[inline(always)]
    pub fn ucbrf_10(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRF_A::UCBRF_10)
    }
    #[doc = "USCI First Stage Modulation: B"]
    #[inline(always)]
    pub fn ucbrf_11(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRF_A::UCBRF_11)
    }
    #[doc = "USCI First Stage Modulation: C"]
    #[inline(always)]
    pub fn ucbrf_12(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRF_A::UCBRF_12)
    }
    #[doc = "USCI First Stage Modulation: D"]
    #[inline(always)]
    pub fn ucbrf_13(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRF_A::UCBRF_13)
    }
    #[doc = "USCI First Stage Modulation: E"]
    #[inline(always)]
    pub fn ucbrf_14(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRF_A::UCBRF_14)
    }
    #[doc = "USCI First Stage Modulation: F"]
    #[inline(always)]
    pub fn ucbrf_15(self) -> &'a mut crate::W<REG> {
        self.variant(UCBRF_A::UCBRF_15)
    }
}
impl R {
    #[doc = "Bit 0 - USCI 16-times Oversampling enable"]
    #[inline(always)]
    pub fn ucos16(&self) -> UCOS16_R {
        UCOS16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - USCI Second Stage Modulation Select 2"]
    #[inline(always)]
    pub fn ucbrs(&self) -> UCBRS_R {
        UCBRS_R::new((self.bits >> 1) & 7)
    }
    #[doc = "Bits 4:7 - USCI First Stage Modulation Select 3"]
    #[inline(always)]
    pub fn ucbrf(&self) -> UCBRF_R {
        UCBRF_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bit 0 - USCI 16-times Oversampling enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucos16(&mut self) -> UCOS16_W<UCA1MCTL_SPEC> {
        UCOS16_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - USCI Second Stage Modulation Select 2"]
    #[inline(always)]
    #[must_use]
    pub fn ucbrs(&mut self) -> UCBRS_W<UCA1MCTL_SPEC> {
        UCBRS_W::new(self, 1)
    }
    #[doc = "Bits 4:7 - USCI First Stage Modulation Select 3"]
    #[inline(always)]
    #[must_use]
    pub fn ucbrf(&mut self) -> UCBRF_W<UCA1MCTL_SPEC> {
        UCBRF_W::new(self, 4)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USCI A1 Modulation Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1mctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1mctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA1MCTL_SPEC;
impl crate::RegisterSpec for UCA1MCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca1mctl::R`](R) reader structure"]
impl crate::Readable for UCA1MCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca1mctl::W`](W) writer structure"]
impl crate::Writable for UCA1MCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA1MCTL to value 0"]
impl crate::Resettable for UCA1MCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
