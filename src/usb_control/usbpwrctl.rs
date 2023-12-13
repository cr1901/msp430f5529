#[doc = "Register `USBPWRCTL` reader"]
pub type R = crate::R<USBPWRCTL_SPEC>;
#[doc = "Register `USBPWRCTL` writer"]
pub type W = crate::W<USBPWRCTL_SPEC>;
#[doc = "Field `VUOVLIFG` reader - USB - VUSB Overload Interrupt Flag"]
pub type VUOVLIFG_R = crate::BitReader;
#[doc = "Field `VUOVLIFG` writer - USB - VUSB Overload Interrupt Flag"]
pub type VUOVLIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBONIFG` reader - USB - VBUS \"Coming ON\" Interrupt Flag"]
pub type VBONIFG_R = crate::BitReader;
#[doc = "Field `VBONIFG` writer - USB - VBUS \"Coming ON\" Interrupt Flag"]
pub type VBONIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBOFFIFG` reader - USB - VBUS \"Going OFF\" Interrupt Flag"]
pub type VBOFFIFG_R = crate::BitReader;
#[doc = "Field `VBOFFIFG` writer - USB - VBUS \"Going OFF\" Interrupt Flag"]
pub type VBOFFIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBBGVBV` reader - USB - USB Bandgap and VBUS valid"]
pub type USBBGVBV_R = crate::BitReader;
#[doc = "Field `USBBGVBV` writer - USB - USB Bandgap and VBUS valid"]
pub type USBBGVBV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBDETEN` reader - USB - VBUS on/off events enable"]
pub type USBDETEN_R = crate::BitReader;
#[doc = "Field `USBDETEN` writer - USB - VBUS on/off events enable"]
pub type USBDETEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVLAOFF` reader - USB - LDO overload auto off enable"]
pub type OVLAOFF_R = crate::BitReader;
#[doc = "Field `OVLAOFF` writer - USB - LDO overload auto off enable"]
pub type OVLAOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLDOAON` reader - USB - Secondary LDO auto on enable"]
pub type SLDOAON_R = crate::BitReader;
#[doc = "Field `SLDOAON` writer - USB - Secondary LDO auto on enable"]
pub type SLDOAON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUOVLIE` reader - USB - Overload indication Interrupt Enable"]
pub type VUOVLIE_R = crate::BitReader;
#[doc = "Field `VUOVLIE` writer - USB - Overload indication Interrupt Enable"]
pub type VUOVLIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBONIE` reader - USB - VBUS \"Coming ON\" Interrupt Enable"]
pub type VBONIE_R = crate::BitReader;
#[doc = "Field `VBONIE` writer - USB - VBUS \"Coming ON\" Interrupt Enable"]
pub type VBONIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBOFFIE` reader - USB - VBUS \"Going OFF\" Interrupt Enable"]
pub type VBOFFIE_R = crate::BitReader;
#[doc = "Field `VBOFFIE` writer - USB - VBUS \"Going OFF\" Interrupt Enable"]
pub type VBOFFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUSBEN` reader - USB - LDO Enable (3.3V)"]
pub type VUSBEN_R = crate::BitReader;
#[doc = "Field `VUSBEN` writer - USB - LDO Enable (3.3V)"]
pub type VUSBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLDOEN` reader - USB - Secondary LDO Enable (1.8V)"]
pub type SLDOEN_R = crate::BitReader;
#[doc = "Field `SLDOEN` writer - USB - Secondary LDO Enable (1.8V)"]
pub type SLDOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB - VUSB Overload Interrupt Flag"]
    #[inline(always)]
    pub fn vuovlifg(&self) -> VUOVLIFG_R {
        VUOVLIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB - VBUS \"Coming ON\" Interrupt Flag"]
    #[inline(always)]
    pub fn vbonifg(&self) -> VBONIFG_R {
        VBONIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB - VBUS \"Going OFF\" Interrupt Flag"]
    #[inline(always)]
    pub fn vboffifg(&self) -> VBOFFIFG_R {
        VBOFFIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB - USB Bandgap and VBUS valid"]
    #[inline(always)]
    pub fn usbbgvbv(&self) -> USBBGVBV_R {
        USBBGVBV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB - VBUS on/off events enable"]
    #[inline(always)]
    pub fn usbdeten(&self) -> USBDETEN_R {
        USBDETEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB - LDO overload auto off enable"]
    #[inline(always)]
    pub fn ovlaoff(&self) -> OVLAOFF_R {
        OVLAOFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB - Secondary LDO auto on enable"]
    #[inline(always)]
    pub fn sldoaon(&self) -> SLDOAON_R {
        SLDOAON_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - USB - Overload indication Interrupt Enable"]
    #[inline(always)]
    pub fn vuovlie(&self) -> VUOVLIE_R {
        VUOVLIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USB - VBUS \"Coming ON\" Interrupt Enable"]
    #[inline(always)]
    pub fn vbonie(&self) -> VBONIE_R {
        VBONIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB - VBUS \"Going OFF\" Interrupt Enable"]
    #[inline(always)]
    pub fn vboffie(&self) -> VBOFFIE_R {
        VBOFFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB - LDO Enable (3.3V)"]
    #[inline(always)]
    pub fn vusben(&self) -> VUSBEN_R {
        VUSBEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB - Secondary LDO Enable (1.8V)"]
    #[inline(always)]
    pub fn sldoen(&self) -> SLDOEN_R {
        SLDOEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - VUSB Overload Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vuovlifg(&mut self) -> VUOVLIFG_W<USBPWRCTL_SPEC> {
        VUOVLIFG_W::new(self, 0)
    }
    #[doc = "Bit 1 - USB - VBUS \"Coming ON\" Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vbonifg(&mut self) -> VBONIFG_W<USBPWRCTL_SPEC> {
        VBONIFG_W::new(self, 1)
    }
    #[doc = "Bit 2 - USB - VBUS \"Going OFF\" Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vboffifg(&mut self) -> VBOFFIFG_W<USBPWRCTL_SPEC> {
        VBOFFIFG_W::new(self, 2)
    }
    #[doc = "Bit 3 - USB - USB Bandgap and VBUS valid"]
    #[inline(always)]
    #[must_use]
    pub fn usbbgvbv(&mut self) -> USBBGVBV_W<USBPWRCTL_SPEC> {
        USBBGVBV_W::new(self, 3)
    }
    #[doc = "Bit 4 - USB - VBUS on/off events enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbdeten(&mut self) -> USBDETEN_W<USBPWRCTL_SPEC> {
        USBDETEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - USB - LDO overload auto off enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovlaoff(&mut self) -> OVLAOFF_W<USBPWRCTL_SPEC> {
        OVLAOFF_W::new(self, 5)
    }
    #[doc = "Bit 6 - USB - Secondary LDO auto on enable"]
    #[inline(always)]
    #[must_use]
    pub fn sldoaon(&mut self) -> SLDOAON_W<USBPWRCTL_SPEC> {
        SLDOAON_W::new(self, 6)
    }
    #[doc = "Bit 8 - USB - Overload indication Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vuovlie(&mut self) -> VUOVLIE_W<USBPWRCTL_SPEC> {
        VUOVLIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - USB - VBUS \"Coming ON\" Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbonie(&mut self) -> VBONIE_W<USBPWRCTL_SPEC> {
        VBONIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - USB - VBUS \"Going OFF\" Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vboffie(&mut self) -> VBOFFIE_W<USBPWRCTL_SPEC> {
        VBOFFIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - USB - LDO Enable (3.3V)"]
    #[inline(always)]
    #[must_use]
    pub fn vusben(&mut self) -> VUSBEN_W<USBPWRCTL_SPEC> {
        VUSBEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - USB - Secondary LDO Enable (1.8V)"]
    #[inline(always)]
    #[must_use]
    pub fn sldoen(&mut self) -> SLDOEN_W<USBPWRCTL_SPEC> {
        SLDOEN_W::new(self, 12)
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
#[doc = "USB Power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbpwrctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbpwrctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBPWRCTL_SPEC;
impl crate::RegisterSpec for USBPWRCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usbpwrctl::R`](R) reader structure"]
impl crate::Readable for USBPWRCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbpwrctl::W`](W) writer structure"]
impl crate::Writable for USBPWRCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBPWRCTL to value 0"]
impl crate::Resettable for USBPWRCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
