#[doc = "Register `UCA1IE_SPI` reader"]
pub type R = crate::R<UCA1IE_SPI_SPEC>;
#[doc = "Register `UCA1IE_SPI` writer"]
pub type W = crate::W<UCA1IE_SPI_SPEC>;
#[doc = "Field `UCRXIE` reader - USCI Receive Interrupt Enable"]
pub type UCRXIE_R = crate::BitReader;
#[doc = "Field `UCRXIE` writer - USCI Receive Interrupt Enable"]
pub type UCRXIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXIE` reader - USCI Transmit Interrupt Enable"]
pub type UCTXIE_R = crate::BitReader;
#[doc = "Field `UCTXIE` writer - USCI Transmit Interrupt Enable"]
pub type UCTXIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTTIE` reader - START Condition interrupt enable"]
pub type UCSTTIE_R = crate::BitReader;
#[doc = "Field `UCSTTIE` writer - START Condition interrupt enable"]
pub type UCSTTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTPIE` reader - STOP Condition interrupt enable"]
pub type UCSTPIE_R = crate::BitReader;
#[doc = "Field `UCSTPIE` writer - STOP Condition interrupt enable"]
pub type UCSTPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCALIE` reader - Arbitration Lost interrupt enable"]
pub type UCALIE_R = crate::BitReader;
#[doc = "Field `UCALIE` writer - Arbitration Lost interrupt enable"]
pub type UCALIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCNACKIE` reader - NACK Condition interrupt enable"]
pub type UCNACKIE_R = crate::BitReader;
#[doc = "Field `UCNACKIE` writer - NACK Condition interrupt enable"]
pub type UCNACKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USCI Receive Interrupt Enable"]
    #[inline(always)]
    pub fn ucrxie(&self) -> UCRXIE_R {
        UCRXIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn uctxie(&self) -> UCTXIE_R {
        UCTXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - START Condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&self) -> UCSTTIE_R {
        UCSTTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STOP Condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&self) -> UCSTPIE_R {
        UCSTPIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Arbitration Lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&self) -> UCALIE_R {
        UCALIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NACK Condition interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&self) -> UCNACKIE_R {
        UCNACKIE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucrxie(&mut self) -> UCRXIE_W<UCA1IE_SPI_SPEC> {
        UCRXIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uctxie(&mut self) -> UCTXIE_W<UCA1IE_SPI_SPEC> {
        UCTXIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - START Condition interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucsttie(&mut self) -> UCSTTIE_W<UCA1IE_SPI_SPEC> {
        UCSTTIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - STOP Condition interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucstpie(&mut self) -> UCSTPIE_W<UCA1IE_SPI_SPEC> {
        UCSTPIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Arbitration Lost interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucalie(&mut self) -> UCALIE_W<UCA1IE_SPI_SPEC> {
        UCALIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - NACK Condition interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucnackie(&mut self) -> UCNACKIE_W<UCA1IE_SPI_SPEC> {
        UCNACKIE_W::new(self, 5)
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
#[doc = "USCI A1 Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1ie_spi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1ie_spi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA1IE_SPI_SPEC;
impl crate::RegisterSpec for UCA1IE_SPI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca1ie_spi::R`](R) reader structure"]
impl crate::Readable for UCA1IE_SPI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca1ie_spi::W`](W) writer structure"]
impl crate::Writable for UCA1IE_SPI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA1IE_SPI to value 0"]
impl crate::Resettable for UCA1IE_SPI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
