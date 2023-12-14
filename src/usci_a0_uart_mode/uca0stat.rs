#[doc = "Register `UCA0STAT` reader"]
pub type R = crate::R<UCA0STAT_SPEC>;
#[doc = "Register `UCA0STAT` writer"]
pub type W = crate::W<UCA0STAT_SPEC>;
#[doc = "Field `UCBUSY` reader - USCI Busy Flag"]
pub type UCBUSY_R = crate::BitReader;
#[doc = "Field `UCBUSY` writer - USCI Busy Flag"]
pub type UCBUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDR` reader - USCI Address received Flag"]
pub type UCADDR_R = crate::BitReader;
#[doc = "Field `UCADDR` writer - USCI Address received Flag"]
pub type UCADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIDLE` reader - Idle line detected"]
pub type UCIDLE_R = crate::BitReader;
#[doc = "Field `UCIDLE` writer - Idle line detected"]
pub type UCIDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCRXERR` reader - USCI RX Error Flag"]
pub type UCRXERR_R = crate::BitReader;
#[doc = "Field `UCRXERR` writer - USCI RX Error Flag"]
pub type UCRXERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBRK` reader - USCI Break received"]
pub type UCBRK_R = crate::BitReader;
#[doc = "Field `UCBRK` writer - USCI Break received"]
pub type UCBRK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPE` reader - USCI Parity Error Flag"]
pub type UCPE_R = crate::BitReader;
#[doc = "Field `UCPE` writer - USCI Parity Error Flag"]
pub type UCPE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 1 - USCI Address received Flag"]
    #[inline(always)]
    pub fn ucaddr(&self) -> UCADDR_R {
        UCADDR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 1 - Idle line detected"]
    #[inline(always)]
    pub fn ucidle(&self) -> UCIDLE_R {
        UCIDLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USCI RX Error Flag"]
    #[inline(always)]
    pub fn ucrxerr(&self) -> UCRXERR_R {
        UCRXERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USCI Break received"]
    #[inline(always)]
    pub fn ucbrk(&self) -> UCBRK_R {
        UCBRK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USCI Parity Error Flag"]
    #[inline(always)]
    pub fn ucpe(&self) -> UCPE_R {
        UCPE_R::new(((self.bits >> 4) & 1) != 0)
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
    pub fn ucbusy(&mut self) -> UCBUSY_W<UCA0STAT_SPEC> {
        UCBUSY_W::new(self, 0)
    }
    #[doc = "Bit 1 - USCI Address received Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucaddr(&mut self) -> UCADDR_W<UCA0STAT_SPEC> {
        UCADDR_W::new(self, 1)
    }
    #[doc = "Bit 1 - Idle line detected"]
    #[inline(always)]
    #[must_use]
    pub fn ucidle(&mut self) -> UCIDLE_W<UCA0STAT_SPEC> {
        UCIDLE_W::new(self, 1)
    }
    #[doc = "Bit 2 - USCI RX Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucrxerr(&mut self) -> UCRXERR_W<UCA0STAT_SPEC> {
        UCRXERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - USCI Break received"]
    #[inline(always)]
    #[must_use]
    pub fn ucbrk(&mut self) -> UCBRK_W<UCA0STAT_SPEC> {
        UCBRK_W::new(self, 3)
    }
    #[doc = "Bit 4 - USCI Parity Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucpe(&mut self) -> UCPE_W<UCA0STAT_SPEC> {
        UCPE_W::new(self, 4)
    }
    #[doc = "Bit 5 - USCI Overrun Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucoe(&mut self) -> UCOE_W<UCA0STAT_SPEC> {
        UCOE_W::new(self, 5)
    }
    #[doc = "Bit 6 - USCI Frame Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucfe(&mut self) -> UCFE_W<UCA0STAT_SPEC> {
        UCFE_W::new(self, 6)
    }
    #[doc = "Bit 7 - USCI Listen mode"]
    #[inline(always)]
    #[must_use]
    pub fn uclisten(&mut self) -> UCLISTEN_W<UCA0STAT_SPEC> {
        UCLISTEN_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USCI A0 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA0STAT_SPEC;
impl crate::RegisterSpec for UCA0STAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0stat::R`](R) reader structure"]
impl crate::Readable for UCA0STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca0stat::W`](W) writer structure"]
impl crate::Writable for UCA0STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA0STAT to value 0"]
impl crate::Resettable for UCA0STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
