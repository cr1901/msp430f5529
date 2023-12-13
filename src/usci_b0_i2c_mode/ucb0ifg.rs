#[doc = "Register `UCB0IFG` reader"]
pub type R = crate::R<UCB0IFG_SPEC>;
#[doc = "Register `UCB0IFG` writer"]
pub type W = crate::W<UCB0IFG_SPEC>;
#[doc = "Field `UCRXIFG` reader - USCI Receive Interrupt Flag"]
pub type UCRXIFG_R = crate::BitReader;
#[doc = "Field `UCRXIFG` writer - USCI Receive Interrupt Flag"]
pub type UCRXIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXIFG` reader - USCI Transmit Interrupt Flag"]
pub type UCTXIFG_R = crate::BitReader;
#[doc = "Field `UCTXIFG` writer - USCI Transmit Interrupt Flag"]
pub type UCTXIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTTIFG` reader - START Condition interrupt Flag"]
pub type UCSTTIFG_R = crate::BitReader;
#[doc = "Field `UCSTTIFG` writer - START Condition interrupt Flag"]
pub type UCSTTIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTPIFG` reader - STOP Condition interrupt Flag"]
pub type UCSTPIFG_R = crate::BitReader;
#[doc = "Field `UCSTPIFG` writer - STOP Condition interrupt Flag"]
pub type UCSTPIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCALIFG` reader - Arbitration Lost interrupt Flag"]
pub type UCALIFG_R = crate::BitReader;
#[doc = "Field `UCALIFG` writer - Arbitration Lost interrupt Flag"]
pub type UCALIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCNACKIFG` reader - NAK Condition interrupt Flag"]
pub type UCNACKIFG_R = crate::BitReader;
#[doc = "Field `UCNACKIFG` writer - NAK Condition interrupt Flag"]
pub type UCNACKIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USCI Receive Interrupt Flag"]
    #[inline(always)]
    pub fn ucrxifg(&self) -> UCRXIFG_R {
        UCRXIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Flag"]
    #[inline(always)]
    pub fn uctxifg(&self) -> UCTXIFG_R {
        UCTXIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - START Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucsttifg(&self) -> UCSTTIFG_R {
        UCSTTIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STOP Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucstpifg(&self) -> UCSTPIFG_R {
        UCSTPIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Arbitration Lost interrupt Flag"]
    #[inline(always)]
    pub fn ucalifg(&self) -> UCALIFG_R {
        UCALIFG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NAK Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucnackifg(&self) -> UCNACKIFG_R {
        UCNACKIFG_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Receive Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucrxifg(&mut self) -> UCRXIFG_W<UCB0IFG_SPEC> {
        UCRXIFG_W::new(self, 0)
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn uctxifg(&mut self) -> UCTXIFG_W<UCB0IFG_SPEC> {
        UCTXIFG_W::new(self, 1)
    }
    #[doc = "Bit 2 - START Condition interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucsttifg(&mut self) -> UCSTTIFG_W<UCB0IFG_SPEC> {
        UCSTTIFG_W::new(self, 2)
    }
    #[doc = "Bit 3 - STOP Condition interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucstpifg(&mut self) -> UCSTPIFG_W<UCB0IFG_SPEC> {
        UCSTPIFG_W::new(self, 3)
    }
    #[doc = "Bit 4 - Arbitration Lost interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucalifg(&mut self) -> UCALIFG_W<UCB0IFG_SPEC> {
        UCALIFG_W::new(self, 4)
    }
    #[doc = "Bit 5 - NAK Condition interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucnackifg(&mut self) -> UCNACKIFG_W<UCB0IFG_SPEC> {
        UCNACKIFG_W::new(self, 5)
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
#[doc = "USCI B0 Interrupt Flags Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0ifg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0ifg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCB0IFG_SPEC;
impl crate::RegisterSpec for UCB0IFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb0ifg::R`](R) reader structure"]
impl crate::Readable for UCB0IFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucb0ifg::W`](W) writer structure"]
impl crate::Writable for UCB0IFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCB0IFG to value 0"]
impl crate::Resettable for UCB0IFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
