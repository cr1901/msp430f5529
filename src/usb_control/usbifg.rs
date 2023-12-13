#[doc = "Register `USBIFG` reader"]
pub type R = crate::R<USBIFG_SPEC>;
#[doc = "Register `USBIFG` writer"]
pub type W = crate::W<USBIFG_SPEC>;
#[doc = "Field `STPOWIFG` reader - USB - Setup Overwrite Interrupt Flag"]
pub type STPOWIFG_R = crate::BitReader;
#[doc = "Field `STPOWIFG` writer - USB - Setup Overwrite Interrupt Flag"]
pub type STPOWIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUPIFG` reader - USB - Setup Interrupt Flag"]
pub type SETUPIFG_R = crate::BitReader;
#[doc = "Field `SETUPIFG` writer - USB - Setup Interrupt Flag"]
pub type SETUPIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESRIFG` reader - USB - Function Resume Request Interrupt Flag"]
pub type RESRIFG_R = crate::BitReader;
#[doc = "Field `RESRIFG` writer - USB - Function Resume Request Interrupt Flag"]
pub type RESRIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSRIFG` reader - USB - Function Suspend Request Interrupt Flag"]
pub type SUSRIFG_R = crate::BitReader;
#[doc = "Field `SUSRIFG` writer - USB - Function Suspend Request Interrupt Flag"]
pub type SUSRIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTRIFG` reader - USB - Function Reset Request Interrupt Flag"]
pub type RSTRIFG_R = crate::BitReader;
#[doc = "Field `RSTRIFG` writer - USB - Function Reset Request Interrupt Flag"]
pub type RSTRIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB - Setup Overwrite Interrupt Flag"]
    #[inline(always)]
    pub fn stpowifg(&self) -> STPOWIFG_R {
        STPOWIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - USB - Setup Interrupt Flag"]
    #[inline(always)]
    pub fn setupifg(&self) -> SETUPIFG_R {
        SETUPIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - USB - Function Resume Request Interrupt Flag"]
    #[inline(always)]
    pub fn resrifg(&self) -> RESRIFG_R {
        RESRIFG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB - Function Suspend Request Interrupt Flag"]
    #[inline(always)]
    pub fn susrifg(&self) -> SUSRIFG_R {
        SUSRIFG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB - Function Reset Request Interrupt Flag"]
    #[inline(always)]
    pub fn rstrifg(&self) -> RSTRIFG_R {
        RSTRIFG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - Setup Overwrite Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn stpowifg(&mut self) -> STPOWIFG_W<USBIFG_SPEC> {
        STPOWIFG_W::new(self, 0)
    }
    #[doc = "Bit 2 - USB - Setup Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn setupifg(&mut self) -> SETUPIFG_W<USBIFG_SPEC> {
        SETUPIFG_W::new(self, 2)
    }
    #[doc = "Bit 5 - USB - Function Resume Request Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn resrifg(&mut self) -> RESRIFG_W<USBIFG_SPEC> {
        RESRIFG_W::new(self, 5)
    }
    #[doc = "Bit 6 - USB - Function Suspend Request Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn susrifg(&mut self) -> SUSRIFG_W<USBIFG_SPEC> {
        SUSRIFG_W::new(self, 6)
    }
    #[doc = "Bit 7 - USB - Function Reset Request Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rstrifg(&mut self) -> RSTRIFG_W<USBIFG_SPEC> {
        RSTRIFG_W::new(self, 7)
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
#[doc = "USB interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbifg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbifg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBIFG_SPEC;
impl crate::RegisterSpec for USBIFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usbifg::R`](R) reader structure"]
impl crate::Readable for USBIFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbifg::W`](W) writer structure"]
impl crate::Writable for USBIFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBIFG to value 0"]
impl crate::Resettable for USBIFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
