#[doc = "Register `UCB1STAT` reader"]
pub type R = crate::R<UCB1STAT_SPEC>;
#[doc = "Register `UCB1STAT` writer"]
pub type W = crate::W<UCB1STAT_SPEC>;
#[doc = "Field `UCBBUSY` reader - Bus Busy Flag"]
pub type UCBBUSY_R = crate::BitReader;
#[doc = "Field `UCBBUSY` writer - Bus Busy Flag"]
pub type UCBBUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCGC` reader - General Call address received Flag"]
pub type UCGC_R = crate::BitReader;
#[doc = "Field `UCGC` writer - General Call address received Flag"]
pub type UCGC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSCLLOW` reader - SCL low"]
pub type UCSCLLOW_R = crate::BitReader;
#[doc = "Field `UCSCLLOW` writer - SCL low"]
pub type UCSCLLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCLISTEN` reader - USCI Listen mode"]
pub type UCLISTEN_R = crate::BitReader;
#[doc = "Field `UCLISTEN` writer - USCI Listen mode"]
pub type UCLISTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Bus Busy Flag"]
    #[inline(always)]
    pub fn ucbbusy(&self) -> UCBBUSY_R {
        UCBBUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General Call address received Flag"]
    #[inline(always)]
    pub fn ucgc(&self) -> UCGC_R {
        UCGC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCL low"]
    #[inline(always)]
    pub fn ucscllow(&self) -> UCSCLLOW_R {
        UCSCLLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USCI Listen mode"]
    #[inline(always)]
    pub fn uclisten(&self) -> UCLISTEN_R {
        UCLISTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Bus Busy Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucbbusy(&mut self) -> UCBBUSY_W<UCB1STAT_SPEC> {
        UCBBUSY_W::new(self, 4)
    }
    #[doc = "Bit 5 - General Call address received Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucgc(&mut self) -> UCGC_W<UCB1STAT_SPEC> {
        UCGC_W::new(self, 5)
    }
    #[doc = "Bit 6 - SCL low"]
    #[inline(always)]
    #[must_use]
    pub fn ucscllow(&mut self) -> UCSCLLOW_W<UCB1STAT_SPEC> {
        UCSCLLOW_W::new(self, 6)
    }
    #[doc = "Bit 7 - USCI Listen mode"]
    #[inline(always)]
    #[must_use]
    pub fn uclisten(&mut self) -> UCLISTEN_W<UCB1STAT_SPEC> {
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
#[doc = "USCI B1 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCB1STAT_SPEC;
impl crate::RegisterSpec for UCB1STAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb1stat::R`](R) reader structure"]
impl crate::Readable for UCB1STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucb1stat::W`](W) writer structure"]
impl crate::Writable for UCB1STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCB1STAT to value 0"]
impl crate::Resettable for UCB1STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
