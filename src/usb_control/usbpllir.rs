#[doc = "Register `USBPLLIR` reader"]
pub type R = crate::R<USBPLLIR_SPEC>;
#[doc = "Register `USBPLLIR` writer"]
pub type W = crate::W<USBPLLIR_SPEC>;
#[doc = "Field `USBOOLIFG` reader - USB - PLL out of lock Interrupt Flag"]
pub type USBOOLIFG_R = crate::BitReader;
#[doc = "Field `USBOOLIFG` writer - USB - PLL out of lock Interrupt Flag"]
pub type USBOOLIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBLOSIFG` reader - USB - PLL loss of signal Interrupt Flag"]
pub type USBLOSIFG_R = crate::BitReader;
#[doc = "Field `USBLOSIFG` writer - USB - PLL loss of signal Interrupt Flag"]
pub type USBLOSIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBOORIFG` reader - USB - PLL out of range Interrupt Flag"]
pub type USBOORIFG_R = crate::BitReader;
#[doc = "Field `USBOORIFG` writer - USB - PLL out of range Interrupt Flag"]
pub type USBOORIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBOOLIE` reader - USB - PLL out of lock Interrupt enable"]
pub type USBOOLIE_R = crate::BitReader;
#[doc = "Field `USBOOLIE` writer - USB - PLL out of lock Interrupt enable"]
pub type USBOOLIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBLOSIE` reader - USB - PLL loss of signal Interrupt enable"]
pub type USBLOSIE_R = crate::BitReader;
#[doc = "Field `USBLOSIE` writer - USB - PLL loss of signal Interrupt enable"]
pub type USBLOSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBOORIE` reader - USB - PLL out of range Interrupt enable"]
pub type USBOORIE_R = crate::BitReader;
#[doc = "Field `USBOORIE` writer - USB - PLL out of range Interrupt enable"]
pub type USBOORIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB - PLL out of lock Interrupt Flag"]
    #[inline(always)]
    pub fn usboolifg(&self) -> USBOOLIFG_R {
        USBOOLIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB - PLL loss of signal Interrupt Flag"]
    #[inline(always)]
    pub fn usblosifg(&self) -> USBLOSIFG_R {
        USBLOSIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB - PLL out of range Interrupt Flag"]
    #[inline(always)]
    pub fn usboorifg(&self) -> USBOORIFG_R {
        USBOORIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - USB - PLL out of lock Interrupt enable"]
    #[inline(always)]
    pub fn usboolie(&self) -> USBOOLIE_R {
        USBOOLIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USB - PLL loss of signal Interrupt enable"]
    #[inline(always)]
    pub fn usblosie(&self) -> USBLOSIE_R {
        USBLOSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB - PLL out of range Interrupt enable"]
    #[inline(always)]
    pub fn usboorie(&self) -> USBOORIE_R {
        USBOORIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - PLL out of lock Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn usboolifg(&mut self) -> USBOOLIFG_W<USBPLLIR_SPEC> {
        USBOOLIFG_W::new(self, 0)
    }
    #[doc = "Bit 1 - USB - PLL loss of signal Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn usblosifg(&mut self) -> USBLOSIFG_W<USBPLLIR_SPEC> {
        USBLOSIFG_W::new(self, 1)
    }
    #[doc = "Bit 2 - USB - PLL out of range Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn usboorifg(&mut self) -> USBOORIFG_W<USBPLLIR_SPEC> {
        USBOORIFG_W::new(self, 2)
    }
    #[doc = "Bit 8 - USB - PLL out of lock Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn usboolie(&mut self) -> USBOOLIE_W<USBPLLIR_SPEC> {
        USBOOLIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - USB - PLL loss of signal Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn usblosie(&mut self) -> USBLOSIE_W<USBPLLIR_SPEC> {
        USBLOSIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - USB - PLL out of range Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn usboorie(&mut self) -> USBOORIE_W<USBPLLIR_SPEC> {
        USBOORIE_W::new(self, 10)
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
#[doc = "USB PLL Interrupt control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbpllir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbpllir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBPLLIR_SPEC;
impl crate::RegisterSpec for USBPLLIR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usbpllir::R`](R) reader structure"]
impl crate::Readable for USBPLLIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbpllir::W`](W) writer structure"]
impl crate::Writable for USBPLLIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBPLLIR to value 0"]
impl crate::Resettable for USBPLLIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
