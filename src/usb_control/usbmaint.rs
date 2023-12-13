#[doc = "Register `USBMAINT` reader"]
pub type R = crate::R<USBMAINT_SPEC>;
#[doc = "Register `USBMAINT` writer"]
pub type W = crate::W<USBMAINT_SPEC>;
#[doc = "Field `UTIFG` reader - USB - Timer Interrupt Flag"]
pub type UTIFG_R = crate::BitReader;
#[doc = "Field `UTIFG` writer - USB - Timer Interrupt Flag"]
pub type UTIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTIE` reader - USB - Timer Interrupt Enable"]
pub type UTIE_R = crate::BitReader;
#[doc = "Field `UTIE` writer - USB - Timer Interrupt Enable"]
pub type UTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSGEN` reader - USB - Time Stamp Generator Enable"]
pub type TSGEN_R = crate::BitReader;
#[doc = "Field `TSGEN` writer - USB - Time Stamp Generator Enable"]
pub type TSGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSESEL` reader - USB - Time Stamp Event Select Bit 0"]
pub type TSESEL_R = crate::FieldReader<TSESEL_A>;
#[doc = "USB - Time Stamp Event Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSESEL_A {
    #[doc = "0: USB - Time Stamp Event Select: 0"]
    TSESEL_0 = 0,
    #[doc = "1: USB - Time Stamp Event Select: 1"]
    TSESEL_1 = 1,
    #[doc = "2: USB - Time Stamp Event Select: 2"]
    TSESEL_2 = 2,
    #[doc = "3: USB - Time Stamp Event Select: 3"]
    TSESEL_3 = 3,
}
impl From<TSESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TSESEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSESEL_A {
    type Ux = u8;
}
impl TSESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSESEL_A {
        match self.bits {
            0 => TSESEL_A::TSESEL_0,
            1 => TSESEL_A::TSESEL_1,
            2 => TSESEL_A::TSESEL_2,
            3 => TSESEL_A::TSESEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "USB - Time Stamp Event Select: 0"]
    #[inline(always)]
    pub fn is_tsesel_0(&self) -> bool {
        *self == TSESEL_A::TSESEL_0
    }
    #[doc = "USB - Time Stamp Event Select: 1"]
    #[inline(always)]
    pub fn is_tsesel_1(&self) -> bool {
        *self == TSESEL_A::TSESEL_1
    }
    #[doc = "USB - Time Stamp Event Select: 2"]
    #[inline(always)]
    pub fn is_tsesel_2(&self) -> bool {
        *self == TSESEL_A::TSESEL_2
    }
    #[doc = "USB - Time Stamp Event Select: 3"]
    #[inline(always)]
    pub fn is_tsesel_3(&self) -> bool {
        *self == TSESEL_A::TSESEL_3
    }
}
#[doc = "Field `TSESEL` writer - USB - Time Stamp Event Select Bit 0"]
pub type TSESEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TSESEL_A>;
impl<'a, REG> TSESEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USB - Time Stamp Event Select: 0"]
    #[inline(always)]
    pub fn tsesel_0(self) -> &'a mut crate::W<REG> {
        self.variant(TSESEL_A::TSESEL_0)
    }
    #[doc = "USB - Time Stamp Event Select: 1"]
    #[inline(always)]
    pub fn tsesel_1(self) -> &'a mut crate::W<REG> {
        self.variant(TSESEL_A::TSESEL_1)
    }
    #[doc = "USB - Time Stamp Event Select: 2"]
    #[inline(always)]
    pub fn tsesel_2(self) -> &'a mut crate::W<REG> {
        self.variant(TSESEL_A::TSESEL_2)
    }
    #[doc = "USB - Time Stamp Event Select: 3"]
    #[inline(always)]
    pub fn tsesel_3(self) -> &'a mut crate::W<REG> {
        self.variant(TSESEL_A::TSESEL_3)
    }
}
#[doc = "Field `TSE3` reader - USB - Time Stamp Event #3 Bit"]
pub type TSE3_R = crate::BitReader;
#[doc = "Field `TSE3` writer - USB - Time Stamp Event #3 Bit"]
pub type TSE3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTSEL` reader - USB - Timer Select Bit 0"]
pub type UTSEL_R = crate::FieldReader<UTSEL_A>;
#[doc = "USB - Timer Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UTSEL_A {
    #[doc = "0: USB - Timer Select: 0"]
    UTSEL_0 = 0,
    #[doc = "1: USB - Timer Select: 1"]
    UTSEL_1 = 1,
    #[doc = "2: USB - Timer Select: 2"]
    UTSEL_2 = 2,
    #[doc = "3: USB - Timer Select: 3"]
    UTSEL_3 = 3,
    #[doc = "4: USB - Timer Select: 4"]
    UTSEL_4 = 4,
    #[doc = "5: USB - Timer Select: 5"]
    UTSEL_5 = 5,
    #[doc = "6: USB - Timer Select: 6"]
    UTSEL_6 = 6,
    #[doc = "7: USB - Timer Select: 7"]
    UTSEL_7 = 7,
}
impl From<UTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UTSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UTSEL_A {
    type Ux = u8;
}
impl UTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UTSEL_A {
        match self.bits {
            0 => UTSEL_A::UTSEL_0,
            1 => UTSEL_A::UTSEL_1,
            2 => UTSEL_A::UTSEL_2,
            3 => UTSEL_A::UTSEL_3,
            4 => UTSEL_A::UTSEL_4,
            5 => UTSEL_A::UTSEL_5,
            6 => UTSEL_A::UTSEL_6,
            7 => UTSEL_A::UTSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "USB - Timer Select: 0"]
    #[inline(always)]
    pub fn is_utsel_0(&self) -> bool {
        *self == UTSEL_A::UTSEL_0
    }
    #[doc = "USB - Timer Select: 1"]
    #[inline(always)]
    pub fn is_utsel_1(&self) -> bool {
        *self == UTSEL_A::UTSEL_1
    }
    #[doc = "USB - Timer Select: 2"]
    #[inline(always)]
    pub fn is_utsel_2(&self) -> bool {
        *self == UTSEL_A::UTSEL_2
    }
    #[doc = "USB - Timer Select: 3"]
    #[inline(always)]
    pub fn is_utsel_3(&self) -> bool {
        *self == UTSEL_A::UTSEL_3
    }
    #[doc = "USB - Timer Select: 4"]
    #[inline(always)]
    pub fn is_utsel_4(&self) -> bool {
        *self == UTSEL_A::UTSEL_4
    }
    #[doc = "USB - Timer Select: 5"]
    #[inline(always)]
    pub fn is_utsel_5(&self) -> bool {
        *self == UTSEL_A::UTSEL_5
    }
    #[doc = "USB - Timer Select: 6"]
    #[inline(always)]
    pub fn is_utsel_6(&self) -> bool {
        *self == UTSEL_A::UTSEL_6
    }
    #[doc = "USB - Timer Select: 7"]
    #[inline(always)]
    pub fn is_utsel_7(&self) -> bool {
        *self == UTSEL_A::UTSEL_7
    }
}
#[doc = "Field `UTSEL` writer - USB - Timer Select Bit 0"]
pub type UTSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, UTSEL_A>;
impl<'a, REG> UTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USB - Timer Select: 0"]
    #[inline(always)]
    pub fn utsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(UTSEL_A::UTSEL_0)
    }
    #[doc = "USB - Timer Select: 1"]
    #[inline(always)]
    pub fn utsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(UTSEL_A::UTSEL_1)
    }
    #[doc = "USB - Timer Select: 2"]
    #[inline(always)]
    pub fn utsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(UTSEL_A::UTSEL_2)
    }
    #[doc = "USB - Timer Select: 3"]
    #[inline(always)]
    pub fn utsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(UTSEL_A::UTSEL_3)
    }
    #[doc = "USB - Timer Select: 4"]
    #[inline(always)]
    pub fn utsel_4(self) -> &'a mut crate::W<REG> {
        self.variant(UTSEL_A::UTSEL_4)
    }
    #[doc = "USB - Timer Select: 5"]
    #[inline(always)]
    pub fn utsel_5(self) -> &'a mut crate::W<REG> {
        self.variant(UTSEL_A::UTSEL_5)
    }
    #[doc = "USB - Timer Select: 6"]
    #[inline(always)]
    pub fn utsel_6(self) -> &'a mut crate::W<REG> {
        self.variant(UTSEL_A::UTSEL_6)
    }
    #[doc = "USB - Timer Select: 7"]
    #[inline(always)]
    pub fn utsel_7(self) -> &'a mut crate::W<REG> {
        self.variant(UTSEL_A::UTSEL_7)
    }
}
impl R {
    #[doc = "Bit 0 - USB - Timer Interrupt Flag"]
    #[inline(always)]
    pub fn utifg(&self) -> UTIFG_R {
        UTIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn utie(&self) -> UTIE_R {
        UTIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - USB - Time Stamp Generator Enable"]
    #[inline(always)]
    pub fn tsgen(&self) -> TSGEN_R {
        TSGEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - USB - Time Stamp Event Select Bit 0"]
    #[inline(always)]
    pub fn tsesel(&self) -> TSESEL_R {
        TSESEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - USB - Time Stamp Event #3 Bit"]
    #[inline(always)]
    pub fn tse3(&self) -> TSE3_R {
        TSE3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 13:15 - USB - Timer Select Bit 0"]
    #[inline(always)]
    pub fn utsel(&self) -> UTSEL_R {
        UTSEL_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USB - Timer Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn utifg(&mut self) -> UTIFG_W<USBMAINT_SPEC> {
        UTIFG_W::new(self, 0)
    }
    #[doc = "Bit 1 - USB - Timer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn utie(&mut self) -> UTIE_W<USBMAINT_SPEC> {
        UTIE_W::new(self, 1)
    }
    #[doc = "Bit 8 - USB - Time Stamp Generator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsgen(&mut self) -> TSGEN_W<USBMAINT_SPEC> {
        TSGEN_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - USB - Time Stamp Event Select Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tsesel(&mut self) -> TSESEL_W<USBMAINT_SPEC> {
        TSESEL_W::new(self, 9)
    }
    #[doc = "Bit 11 - USB - Time Stamp Event #3 Bit"]
    #[inline(always)]
    #[must_use]
    pub fn tse3(&mut self) -> TSE3_W<USBMAINT_SPEC> {
        TSE3_W::new(self, 11)
    }
    #[doc = "Bits 13:15 - USB - Timer Select Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn utsel(&mut self) -> UTSEL_W<USBMAINT_SPEC> {
        UTSEL_W::new(self, 13)
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
#[doc = "USB maintenance register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbmaint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbmaint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBMAINT_SPEC;
impl crate::RegisterSpec for USBMAINT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usbmaint::R`](R) reader structure"]
impl crate::Readable for USBMAINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbmaint::W`](W) writer structure"]
impl crate::Writable for USBMAINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBMAINT to value 0"]
impl crate::Resettable for USBMAINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
