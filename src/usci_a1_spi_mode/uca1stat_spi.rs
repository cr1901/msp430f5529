#[doc = "Register `UCA1STAT_SPI` reader"]
pub type R = crate::R<UCA1STAT_SPI_SPEC>;
#[doc = "Register `UCA1STAT_SPI` writer"]
pub type W = crate::W<UCA1STAT_SPI_SPEC>;
#[doc = "Field `UCBUSY` reader - USCI Busy Flag"]
pub type UCBUSY_R = crate::BitReader;
#[doc = "Field `UCBUSY` writer - USCI Busy Flag"]
pub type UCBUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCOE` reader - USCI Overrun Error Flag"]
pub type UCOE_R = crate::BitReader;
#[doc = "Field `UCOE` writer - USCI Overrun Error Flag"]
pub type UCOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCFE` reader - USCI Frame Error Flag"]
pub type UCFE_R = crate::BitReader;
#[doc = "Field `UCFE` writer - USCI Frame Error Flag"]
pub type UCFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCLISTEN` reader - USCI Listen mode"]
pub type UCLISTEN_R = crate::BitReader;
#[doc = "Field `UCLISTEN` writer - USCI Listen mode"]
pub type UCLISTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USCI Busy Flag"]
    #[inline(always)]
    pub fn ucbusy(&self) -> UCBUSY_R {
        UCBUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - USCI Overrun Error Flag"]
    #[inline(always)]
    pub fn ucoe(&self) -> UCOE_R {
        UCOE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USCI Frame Error Flag"]
    #[inline(always)]
    pub fn ucfe(&self) -> UCFE_R {
        UCFE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USCI Listen mode"]
    #[inline(always)]
    pub fn uclisten(&self) -> UCLISTEN_R {
        UCLISTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Busy Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucbusy(&mut self) -> UCBUSY_W<UCA1STAT_SPI_SPEC> {
        UCBUSY_W::new(self, 0)
    }
    #[doc = "Bit 5 - USCI Overrun Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucoe(&mut self) -> UCOE_W<UCA1STAT_SPI_SPEC> {
        UCOE_W::new(self, 5)
    }
    #[doc = "Bit 6 - USCI Frame Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucfe(&mut self) -> UCFE_W<UCA1STAT_SPI_SPEC> {
        UCFE_W::new(self, 6)
    }
    #[doc = "Bit 7 - USCI Listen mode"]
    #[inline(always)]
    #[must_use]
    pub fn uclisten(&mut self) -> UCLISTEN_W<UCA1STAT_SPI_SPEC> {
        UCLISTEN_W::new(self, 7)
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
#[doc = "USCI A1 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1stat_spi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1stat_spi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA1STAT_SPI_SPEC;
impl crate::RegisterSpec for UCA1STAT_SPI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca1stat_spi::R`](R) reader structure"]
impl crate::Readable for UCA1STAT_SPI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca1stat_spi::W`](W) writer structure"]
impl crate::Writable for UCA1STAT_SPI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA1STAT_SPI to value 0"]
impl crate::Resettable for UCA1STAT_SPI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
