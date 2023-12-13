#[doc = "Register `USBCTL` reader"]
pub type R = crate::R<USBCTL_SPEC>;
#[doc = "Register `USBCTL` writer"]
pub type W = crate::W<USBCTL_SPEC>;
#[doc = "Field `DIR` reader - USB - Data Response Bit"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `DIR` writer - USB - Data Response Bit"]
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRSTE` reader - USB - Function Reset Connection Enable"]
pub type FRSTE_R = crate::BitReader;
#[doc = "Field `FRSTE` writer - USB - Function Reset Connection Enable"]
pub type FRSTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWUP` reader - USB - Device Remote Wakeup Request"]
pub type RWUP_R = crate::BitReader;
#[doc = "Field `RWUP` writer - USB - Device Remote Wakeup Request"]
pub type RWUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN` reader - USB - Function Enable Bit"]
pub type FEN_R = crate::BitReader;
#[doc = "Field `FEN` writer - USB - Function Enable Bit"]
pub type FEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB - Data Response Bit"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - USB - Function Reset Connection Enable"]
    #[inline(always)]
    pub fn frste(&self) -> FRSTE_R {
        FRSTE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB - Device Remote Wakeup Request"]
    #[inline(always)]
    pub fn rwup(&self) -> RWUP_R {
        RWUP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB - Function Enable Bit"]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - Data Response Bit"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<USBCTL_SPEC> {
        DIR_W::new(self, 0)
    }
    #[doc = "Bit 4 - USB - Function Reset Connection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frste(&mut self) -> FRSTE_W<USBCTL_SPEC> {
        FRSTE_W::new(self, 4)
    }
    #[doc = "Bit 5 - USB - Device Remote Wakeup Request"]
    #[inline(always)]
    #[must_use]
    pub fn rwup(&mut self) -> RWUP_W<USBCTL_SPEC> {
        RWUP_W::new(self, 5)
    }
    #[doc = "Bit 6 - USB - Function Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn fen(&mut self) -> FEN_W<USBCTL_SPEC> {
        FEN_W::new(self, 6)
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
#[doc = "USB control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBCTL_SPEC;
impl crate::RegisterSpec for USBCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usbctl::R`](R) reader structure"]
impl crate::Readable for USBCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbctl::W`](W) writer structure"]
impl crate::Writable for USBCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBCTL to value 0"]
impl crate::Resettable for USBCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
