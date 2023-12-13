#[doc = "Register `UCA0IRRCTL` reader"]
pub type R = crate::R<UCA0IRRCTL_SPEC>;
#[doc = "Register `UCA0IRRCTL` writer"]
pub type W = crate::W<UCA0IRRCTL_SPEC>;
#[doc = "Field `UCIRRXFE` reader - IRDA Receive Filter enable"]
pub type UCIRRXFE_R = crate::BitReader;
#[doc = "Field `UCIRRXFE` writer - IRDA Receive Filter enable"]
pub type UCIRRXFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRRXPL` reader - IRDA Receive Input Polarity"]
pub type UCIRRXPL_R = crate::BitReader;
#[doc = "Field `UCIRRXPL` writer - IRDA Receive Input Polarity"]
pub type UCIRRXPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRRXFL0` reader - IRDA Receive Filter Length 0"]
pub type UCIRRXFL0_R = crate::BitReader;
#[doc = "Field `UCIRRXFL0` writer - IRDA Receive Filter Length 0"]
pub type UCIRRXFL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRRXFL1` reader - IRDA Receive Filter Length 1"]
pub type UCIRRXFL1_R = crate::BitReader;
#[doc = "Field `UCIRRXFL1` writer - IRDA Receive Filter Length 1"]
pub type UCIRRXFL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRRXFL2` reader - IRDA Receive Filter Length 2"]
pub type UCIRRXFL2_R = crate::BitReader;
#[doc = "Field `UCIRRXFL2` writer - IRDA Receive Filter Length 2"]
pub type UCIRRXFL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRRXFL3` reader - IRDA Receive Filter Length 3"]
pub type UCIRRXFL3_R = crate::BitReader;
#[doc = "Field `UCIRRXFL3` writer - IRDA Receive Filter Length 3"]
pub type UCIRRXFL3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRRXFL4` reader - IRDA Receive Filter Length 4"]
pub type UCIRRXFL4_R = crate::BitReader;
#[doc = "Field `UCIRRXFL4` writer - IRDA Receive Filter Length 4"]
pub type UCIRRXFL4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRRXFL5` reader - IRDA Receive Filter Length 5"]
pub type UCIRRXFL5_R = crate::BitReader;
#[doc = "Field `UCIRRXFL5` writer - IRDA Receive Filter Length 5"]
pub type UCIRRXFL5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IRDA Receive Filter enable"]
    #[inline(always)]
    pub fn ucirrxfe(&self) -> UCIRRXFE_R {
        UCIRRXFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRDA Receive Input Polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&self) -> UCIRRXPL_R {
        UCIRRXPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRDA Receive Filter Length 0"]
    #[inline(always)]
    pub fn ucirrxfl0(&self) -> UCIRRXFL0_R {
        UCIRRXFL0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IRDA Receive Filter Length 1"]
    #[inline(always)]
    pub fn ucirrxfl1(&self) -> UCIRRXFL1_R {
        UCIRRXFL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IRDA Receive Filter Length 2"]
    #[inline(always)]
    pub fn ucirrxfl2(&self) -> UCIRRXFL2_R {
        UCIRRXFL2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IRDA Receive Filter Length 3"]
    #[inline(always)]
    pub fn ucirrxfl3(&self) -> UCIRRXFL3_R {
        UCIRRXFL3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IRDA Receive Filter Length 4"]
    #[inline(always)]
    pub fn ucirrxfl4(&self) -> UCIRRXFL4_R {
        UCIRRXFL4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IRDA Receive Filter Length 5"]
    #[inline(always)]
    pub fn ucirrxfl5(&self) -> UCIRRXFL5_R {
        UCIRRXFL5_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRDA Receive Filter enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucirrxfe(&mut self) -> UCIRRXFE_W<UCA0IRRCTL_SPEC> {
        UCIRRXFE_W::new(self, 0)
    }
    #[doc = "Bit 1 - IRDA Receive Input Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ucirrxpl(&mut self) -> UCIRRXPL_W<UCA0IRRCTL_SPEC> {
        UCIRRXPL_W::new(self, 1)
    }
    #[doc = "Bit 2 - IRDA Receive Filter Length 0"]
    #[inline(always)]
    #[must_use]
    pub fn ucirrxfl0(&mut self) -> UCIRRXFL0_W<UCA0IRRCTL_SPEC> {
        UCIRRXFL0_W::new(self, 2)
    }
    #[doc = "Bit 3 - IRDA Receive Filter Length 1"]
    #[inline(always)]
    #[must_use]
    pub fn ucirrxfl1(&mut self) -> UCIRRXFL1_W<UCA0IRRCTL_SPEC> {
        UCIRRXFL1_W::new(self, 3)
    }
    #[doc = "Bit 4 - IRDA Receive Filter Length 2"]
    #[inline(always)]
    #[must_use]
    pub fn ucirrxfl2(&mut self) -> UCIRRXFL2_W<UCA0IRRCTL_SPEC> {
        UCIRRXFL2_W::new(self, 4)
    }
    #[doc = "Bit 5 - IRDA Receive Filter Length 3"]
    #[inline(always)]
    #[must_use]
    pub fn ucirrxfl3(&mut self) -> UCIRRXFL3_W<UCA0IRRCTL_SPEC> {
        UCIRRXFL3_W::new(self, 5)
    }
    #[doc = "Bit 6 - IRDA Receive Filter Length 4"]
    #[inline(always)]
    #[must_use]
    pub fn ucirrxfl4(&mut self) -> UCIRRXFL4_W<UCA0IRRCTL_SPEC> {
        UCIRRXFL4_W::new(self, 6)
    }
    #[doc = "Bit 7 - IRDA Receive Filter Length 5"]
    #[inline(always)]
    #[must_use]
    pub fn ucirrxfl5(&mut self) -> UCIRRXFL5_W<UCA0IRRCTL_SPEC> {
        UCIRRXFL5_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USCI A0 IrDA Receive Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0irrctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0irrctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA0IRRCTL_SPEC;
impl crate::RegisterSpec for UCA0IRRCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0irrctl::R`](R) reader structure"]
impl crate::Readable for UCA0IRRCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca0irrctl::W`](W) writer structure"]
impl crate::Writable for UCA0IRRCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA0IRRCTL to value 0"]
impl crate::Resettable for UCA0IRRCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
