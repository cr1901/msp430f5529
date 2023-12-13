#[doc = "Register `UCA0CTL0_SPI` reader"]
pub type R = crate::R<UCA0CTL0_SPI_SPEC>;
#[doc = "Register `UCA0CTL0_SPI` writer"]
pub type W = crate::W<UCA0CTL0_SPI_SPEC>;
#[doc = "Field `UCSYNC` reader - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
pub type UCSYNC_R = crate::BitReader;
#[doc = "Field `UCSYNC` writer - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
pub type UCSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCMODE` reader - Sync. Mode: USCI Mode 1"]
pub type UCMODE_R = crate::FieldReader<UCMODE_A>;
#[doc = "Sync. Mode: USCI Mode 1\n\nValue on reset: 0"]
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
#[doc = "Field `UCMODE` writer - Sync. Mode: USCI Mode 1"]
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
#[doc = "Field `UCMST` reader - Sync. Mode: Master Select"]
pub type UCMST_R = crate::BitReader;
#[doc = "Field `UCMST` writer - Sync. Mode: Master Select"]
pub type UCMST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UC7BIT` reader - Sync. Mode: Data Bits 0:8-bits / 1:7-bits"]
pub type UC7BIT_R = crate::BitReader;
#[doc = "Field `UC7BIT` writer - Sync. Mode: Data Bits 0:8-bits / 1:7-bits"]
pub type UC7BIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCMSB` reader - Sync. Mode: MSB first 0:LSB / 1:MSB"]
pub type UCMSB_R = crate::BitReader;
#[doc = "Field `UCMSB` writer - Sync. Mode: MSB first 0:LSB / 1:MSB"]
pub type UCMSB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCCKPL` reader - Sync. Mode: Clock Polarity"]
pub type UCCKPL_R = crate::BitReader;
#[doc = "Field `UCCKPL` writer - Sync. Mode: Clock Polarity"]
pub type UCCKPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCCKPH` reader - Sync. Mode: Clock Phase"]
pub type UCCKPH_R = crate::BitReader;
#[doc = "Field `UCCKPH` writer - Sync. Mode: Clock Phase"]
pub type UCCKPH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
    #[inline(always)]
    pub fn ucsync(&self) -> UCSYNC_R {
        UCSYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Sync. Mode: USCI Mode 1"]
    #[inline(always)]
    pub fn ucmode(&self) -> UCMODE_R {
        UCMODE_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - Sync. Mode: Master Select"]
    #[inline(always)]
    pub fn ucmst(&self) -> UCMST_R {
        UCMST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sync. Mode: Data Bits 0:8-bits / 1:7-bits"]
    #[inline(always)]
    pub fn uc7bit(&self) -> UC7BIT_R {
        UC7BIT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sync. Mode: MSB first 0:LSB / 1:MSB"]
    #[inline(always)]
    pub fn ucmsb(&self) -> UCMSB_R {
        UCMSB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sync. Mode: Clock Polarity"]
    #[inline(always)]
    pub fn ucckpl(&self) -> UCCKPL_R {
        UCCKPL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sync. Mode: Clock Phase"]
    #[inline(always)]
    pub fn ucckph(&self) -> UCCKPH_R {
        UCCKPH_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ucsync(&mut self) -> UCSYNC_W<UCA0CTL0_SPI_SPEC> {
        UCSYNC_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Sync. Mode: USCI Mode 1"]
    #[inline(always)]
    #[must_use]
    pub fn ucmode(&mut self) -> UCMODE_W<UCA0CTL0_SPI_SPEC> {
        UCMODE_W::new(self, 1)
    }
    #[doc = "Bit 3 - Sync. Mode: Master Select"]
    #[inline(always)]
    #[must_use]
    pub fn ucmst(&mut self) -> UCMST_W<UCA0CTL0_SPI_SPEC> {
        UCMST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Sync. Mode: Data Bits 0:8-bits / 1:7-bits"]
    #[inline(always)]
    #[must_use]
    pub fn uc7bit(&mut self) -> UC7BIT_W<UCA0CTL0_SPI_SPEC> {
        UC7BIT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Sync. Mode: MSB first 0:LSB / 1:MSB"]
    #[inline(always)]
    #[must_use]
    pub fn ucmsb(&mut self) -> UCMSB_W<UCA0CTL0_SPI_SPEC> {
        UCMSB_W::new(self, 5)
    }
    #[doc = "Bit 6 - Sync. Mode: Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ucckpl(&mut self) -> UCCKPL_W<UCA0CTL0_SPI_SPEC> {
        UCCKPL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Sync. Mode: Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn ucckph(&mut self) -> UCCKPH_W<UCA0CTL0_SPI_SPEC> {
        UCCKPH_W::new(self, 7)
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
#[doc = "USCI A0 Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0ctl0_spi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0ctl0_spi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA0CTL0_SPI_SPEC;
impl crate::RegisterSpec for UCA0CTL0_SPI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0ctl0_spi::R`](R) reader structure"]
impl crate::Readable for UCA0CTL0_SPI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca0ctl0_spi::W`](W) writer structure"]
impl crate::Writable for UCA0CTL0_SPI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA0CTL0_SPI to value 0"]
impl crate::Resettable for UCA0CTL0_SPI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
