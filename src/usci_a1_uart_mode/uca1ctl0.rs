#[doc = "Register `UCA1CTL0` reader"]
pub type R = crate::R<UCA1CTL0_SPEC>;
#[doc = "Register `UCA1CTL0` writer"]
pub type W = crate::W<UCA1CTL0_SPEC>;
#[doc = "Field `UCSYNC` reader - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
pub type UCSYNC_R = crate::BitReader;
#[doc = "Field `UCSYNC` writer - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
pub type UCSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCMODE` reader - Async. Mode: USCI Mode 1"]
pub type UCMODE_R = crate::FieldReader<UCMODE_A>;
#[doc = "Async. Mode: USCI Mode 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCMODE_A {
    #[doc = "0: Sync. Mode: USCI Mode: 0"]
    UCMODE_0 = 0,
    #[doc = "1: Sync. Mode: USCI Mode: 1"]
    UCMODE_1 = 1,
    #[doc = "2: Sync. Mode: USCI Mode: 2"]
    UCMODE_2 = 2,
    #[doc = "3: Sync. Mode: USCI Mode: 3"]
    UCMODE_3 = 3,
}
impl From<UCMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UCMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UCMODE_A {
    type Ux = u8;
}
impl UCMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UCMODE_A {
        match self.bits {
            0 => UCMODE_A::UCMODE_0,
            1 => UCMODE_A::UCMODE_1,
            2 => UCMODE_A::UCMODE_2,
            3 => UCMODE_A::UCMODE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Sync. Mode: USCI Mode: 0"]
    #[inline(always)]
    pub fn is_ucmode_0(&self) -> bool {
        *self == UCMODE_A::UCMODE_0
    }
    #[doc = "Sync. Mode: USCI Mode: 1"]
    #[inline(always)]
    pub fn is_ucmode_1(&self) -> bool {
        *self == UCMODE_A::UCMODE_1
    }
    #[doc = "Sync. Mode: USCI Mode: 2"]
    #[inline(always)]
    pub fn is_ucmode_2(&self) -> bool {
        *self == UCMODE_A::UCMODE_2
    }
    #[doc = "Sync. Mode: USCI Mode: 3"]
    #[inline(always)]
    pub fn is_ucmode_3(&self) -> bool {
        *self == UCMODE_A::UCMODE_3
    }
}
#[doc = "Field `UCMODE` writer - Async. Mode: USCI Mode 1"]
pub type UCMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, UCMODE_A>;
impl<'a, REG> UCMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sync. Mode: USCI Mode: 0"]
    #[inline(always)]
    pub fn ucmode_0(self) -> &'a mut crate::W<REG> {
        self.variant(UCMODE_A::UCMODE_0)
    }
    #[doc = "Sync. Mode: USCI Mode: 1"]
    #[inline(always)]
    pub fn ucmode_1(self) -> &'a mut crate::W<REG> {
        self.variant(UCMODE_A::UCMODE_1)
    }
    #[doc = "Sync. Mode: USCI Mode: 2"]
    #[inline(always)]
    pub fn ucmode_2(self) -> &'a mut crate::W<REG> {
        self.variant(UCMODE_A::UCMODE_2)
    }
    #[doc = "Sync. Mode: USCI Mode: 3"]
    #[inline(always)]
    pub fn ucmode_3(self) -> &'a mut crate::W<REG> {
        self.variant(UCMODE_A::UCMODE_3)
    }
}
#[doc = "Field `UCSPB` reader - Async. Mode: Stop Bits 0:one / 1: two"]
pub type UCSPB_R = crate::BitReader;
#[doc = "Field `UCSPB` writer - Async. Mode: Stop Bits 0:one / 1: two"]
pub type UCSPB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UC7BIT` reader - Async. Mode: Data Bits 0:8-bits / 1:7-bits"]
pub type UC7BIT_R = crate::BitReader;
#[doc = "Field `UC7BIT` writer - Async. Mode: Data Bits 0:8-bits / 1:7-bits"]
pub type UC7BIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCMSB` reader - Async. Mode: MSB first 0:LSB / 1:MSB"]
pub type UCMSB_R = crate::BitReader;
#[doc = "Field `UCMSB` writer - Async. Mode: MSB first 0:LSB / 1:MSB"]
pub type UCMSB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPAR` reader - Async. Mode: Parity 0:odd / 1:even"]
pub type UCPAR_R = crate::BitReader;
#[doc = "Field `UCPAR` writer - Async. Mode: Parity 0:odd / 1:even"]
pub type UCPAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPEN` reader - Async. Mode: Parity enable"]
pub type UCPEN_R = crate::BitReader;
#[doc = "Field `UCPEN` writer - Async. Mode: Parity enable"]
pub type UCPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
    #[inline(always)]
    pub fn ucsync(&self) -> UCSYNC_R {
        UCSYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Async. Mode: USCI Mode 1"]
    #[inline(always)]
    pub fn ucmode(&self) -> UCMODE_R {
        UCMODE_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - Async. Mode: Stop Bits 0:one / 1: two"]
    #[inline(always)]
    pub fn ucspb(&self) -> UCSPB_R {
        UCSPB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Async. Mode: Data Bits 0:8-bits / 1:7-bits"]
    #[inline(always)]
    pub fn uc7bit(&self) -> UC7BIT_R {
        UC7BIT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Async. Mode: MSB first 0:LSB / 1:MSB"]
    #[inline(always)]
    pub fn ucmsb(&self) -> UCMSB_R {
        UCMSB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Async. Mode: Parity 0:odd / 1:even"]
    #[inline(always)]
    pub fn ucpar(&self) -> UCPAR_R {
        UCPAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Async. Mode: Parity enable"]
    #[inline(always)]
    pub fn ucpen(&self) -> UCPEN_R {
        UCPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ucsync(&mut self) -> UCSYNC_W<UCA1CTL0_SPEC> {
        UCSYNC_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Async. Mode: USCI Mode 1"]
    #[inline(always)]
    #[must_use]
    pub fn ucmode(&mut self) -> UCMODE_W<UCA1CTL0_SPEC> {
        UCMODE_W::new(self, 1)
    }
    #[doc = "Bit 3 - Async. Mode: Stop Bits 0:one / 1: two"]
    #[inline(always)]
    #[must_use]
    pub fn ucspb(&mut self) -> UCSPB_W<UCA1CTL0_SPEC> {
        UCSPB_W::new(self, 3)
    }
    #[doc = "Bit 4 - Async. Mode: Data Bits 0:8-bits / 1:7-bits"]
    #[inline(always)]
    #[must_use]
    pub fn uc7bit(&mut self) -> UC7BIT_W<UCA1CTL0_SPEC> {
        UC7BIT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Async. Mode: MSB first 0:LSB / 1:MSB"]
    #[inline(always)]
    #[must_use]
    pub fn ucmsb(&mut self) -> UCMSB_W<UCA1CTL0_SPEC> {
        UCMSB_W::new(self, 5)
    }
    #[doc = "Bit 6 - Async. Mode: Parity 0:odd / 1:even"]
    #[inline(always)]
    #[must_use]
    pub fn ucpar(&mut self) -> UCPAR_W<UCA1CTL0_SPEC> {
        UCPAR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Async. Mode: Parity enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucpen(&mut self) -> UCPEN_W<UCA1CTL0_SPEC> {
        UCPEN_W::new(self, 7)
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
#[doc = "USCI A1 Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA1CTL0_SPEC;
impl crate::RegisterSpec for UCA1CTL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca1ctl0::R`](R) reader structure"]
impl crate::Readable for UCA1CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca1ctl0::W`](W) writer structure"]
impl crate::Writable for UCA1CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA1CTL0 to value 0"]
impl crate::Resettable for UCA1CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
