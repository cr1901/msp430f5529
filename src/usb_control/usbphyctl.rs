#[doc = "Register `USBPHYCTL` reader"]
pub type R = crate::R<USBPHYCTL_SPEC>;
#[doc = "Register `USBPHYCTL` writer"]
pub type W = crate::W<USBPHYCTL_SPEC>;
#[doc = "Field `PUOUT0` reader - USB - USB Port Output Signal Bit 0"]
pub type PUOUT0_R = crate::BitReader;
#[doc = "Field `PUOUT0` writer - USB - USB Port Output Signal Bit 0"]
pub type PUOUT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUOUT1` reader - USB - USB Port Output Signal Bit 1"]
pub type PUOUT1_R = crate::BitReader;
#[doc = "Field `PUOUT1` writer - USB - USB Port Output Signal Bit 1"]
pub type PUOUT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUIN0` reader - USB - PU0/DP Input Data"]
pub type PUIN0_R = crate::BitReader;
#[doc = "Field `PUIN0` writer - USB - PU0/DP Input Data"]
pub type PUIN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUIN1` reader - USB - PU1/DM Input Data"]
pub type PUIN1_R = crate::BitReader;
#[doc = "Field `PUIN1` writer - USB - PU1/DM Input Data"]
pub type PUIN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUOPE` reader - USB - USB Port Output Enable"]
pub type PUOPE_R = crate::BitReader;
#[doc = "Field `PUOPE` writer - USB - USB Port Output Enable"]
pub type PUOPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUSEL` reader - USB - USB Port Function Select"]
pub type PUSEL_R = crate::BitReader;
#[doc = "Field `PUSEL` writer - USB - USB Port Function Select"]
pub type PUSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUIPE` reader - USB - PHY Single Ended Input enable"]
pub type PUIPE_R = crate::BitReader;
#[doc = "Field `PUIPE` writer - USB - PHY Single Ended Input enable"]
pub type PUIPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB - USB Port Output Signal Bit 0"]
    #[inline(always)]
    pub fn puout0(&self) -> PUOUT0_R {
        PUOUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB - USB Port Output Signal Bit 1"]
    #[inline(always)]
    pub fn puout1(&self) -> PUOUT1_R {
        PUOUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB - PU0/DP Input Data"]
    #[inline(always)]
    pub fn puin0(&self) -> PUIN0_R {
        PUIN0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB - PU1/DM Input Data"]
    #[inline(always)]
    pub fn puin1(&self) -> PUIN1_R {
        PUIN1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - USB - USB Port Output Enable"]
    #[inline(always)]
    pub fn puope(&self) -> PUOPE_R {
        PUOPE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - USB - USB Port Function Select"]
    #[inline(always)]
    pub fn pusel(&self) -> PUSEL_R {
        PUSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - USB - PHY Single Ended Input enable"]
    #[inline(always)]
    pub fn puipe(&self) -> PUIPE_R {
        PUIPE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - USB Port Output Signal Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn puout0(&mut self) -> PUOUT0_W<USBPHYCTL_SPEC> {
        PUOUT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - USB - USB Port Output Signal Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn puout1(&mut self) -> PUOUT1_W<USBPHYCTL_SPEC> {
        PUOUT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - USB - PU0/DP Input Data"]
    #[inline(always)]
    #[must_use]
    pub fn puin0(&mut self) -> PUIN0_W<USBPHYCTL_SPEC> {
        PUIN0_W::new(self, 2)
    }
    #[doc = "Bit 3 - USB - PU1/DM Input Data"]
    #[inline(always)]
    #[must_use]
    pub fn puin1(&mut self) -> PUIN1_W<USBPHYCTL_SPEC> {
        PUIN1_W::new(self, 3)
    }
    #[doc = "Bit 5 - USB - USB Port Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn puope(&mut self) -> PUOPE_W<USBPHYCTL_SPEC> {
        PUOPE_W::new(self, 5)
    }
    #[doc = "Bit 7 - USB - USB Port Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn pusel(&mut self) -> PUSEL_W<USBPHYCTL_SPEC> {
        PUSEL_W::new(self, 7)
    }
    #[doc = "Bit 8 - USB - PHY Single Ended Input enable"]
    #[inline(always)]
    #[must_use]
    pub fn puipe(&mut self) -> PUIPE_W<USBPHYCTL_SPEC> {
        PUIPE_W::new(self, 8)
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
#[doc = "USB PHY control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphyctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphyctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBPHYCTL_SPEC;
impl crate::RegisterSpec for USBPHYCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usbphyctl::R`](R) reader structure"]
impl crate::Readable for USBPHYCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbphyctl::W`](W) writer structure"]
impl crate::Writable for USBPHYCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBPHYCTL to value 0"]
impl crate::Resettable for USBPHYCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
