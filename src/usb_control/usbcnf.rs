#[doc = "Register `USBCNF` reader"]
pub type R = crate::R<USBCNF_SPEC>;
#[doc = "Register `USBCNF` writer"]
pub type W = crate::W<USBCNF_SPEC>;
#[doc = "Field `USB_EN` reader - USB - Module enable"]
pub type USB_EN_R = crate::BitReader;
#[doc = "Field `USB_EN` writer - USB - Module enable"]
pub type USB_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUR_EN` reader - USB - PUR pin enable"]
pub type PUR_EN_R = crate::BitReader;
#[doc = "Field `PUR_EN` writer - USB - PUR pin enable"]
pub type PUR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUR_IN` reader - USB - PUR pin input value"]
pub type PUR_IN_R = crate::BitReader;
#[doc = "Field `PUR_IN` writer - USB - PUR pin input value"]
pub type PUR_IN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLKRDY` reader - USB - Block ready signal for DMA"]
pub type BLKRDY_R = crate::BitReader;
#[doc = "Field `BLKRDY` writer - USB - Block ready signal for DMA"]
pub type BLKRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FNTEN` reader - USB - Frame Number receive Trigger enable for DMA"]
pub type FNTEN_R = crate::BitReader;
#[doc = "Field `FNTEN` writer - USB - Frame Number receive Trigger enable for DMA"]
pub type FNTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB - Module enable"]
    #[inline(always)]
    pub fn usb_en(&self) -> USB_EN_R {
        USB_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB - PUR pin enable"]
    #[inline(always)]
    pub fn pur_en(&self) -> PUR_EN_R {
        PUR_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB - PUR pin input value"]
    #[inline(always)]
    pub fn pur_in(&self) -> PUR_IN_R {
        PUR_IN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB - Block ready signal for DMA"]
    #[inline(always)]
    pub fn blkrdy(&self) -> BLKRDY_R {
        BLKRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB - Frame Number receive Trigger enable for DMA"]
    #[inline(always)]
    pub fn fnten(&self) -> FNTEN_R {
        FNTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - Module enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_en(&mut self) -> USB_EN_W<USBCNF_SPEC> {
        USB_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - USB - PUR pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn pur_en(&mut self) -> PUR_EN_W<USBCNF_SPEC> {
        PUR_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - USB - PUR pin input value"]
    #[inline(always)]
    #[must_use]
    pub fn pur_in(&mut self) -> PUR_IN_W<USBCNF_SPEC> {
        PUR_IN_W::new(self, 2)
    }
    #[doc = "Bit 3 - USB - Block ready signal for DMA"]
    #[inline(always)]
    #[must_use]
    pub fn blkrdy(&mut self) -> BLKRDY_W<USBCNF_SPEC> {
        BLKRDY_W::new(self, 3)
    }
    #[doc = "Bit 4 - USB - Frame Number receive Trigger enable for DMA"]
    #[inline(always)]
    #[must_use]
    pub fn fnten(&mut self) -> FNTEN_W<USBCNF_SPEC> {
        FNTEN_W::new(self, 4)
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
#[doc = "USB Module configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbcnf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbcnf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBCNF_SPEC;
impl crate::RegisterSpec for USBCNF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usbcnf::R`](R) reader structure"]
impl crate::Readable for USBCNF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbcnf::W`](W) writer structure"]
impl crate::Writable for USBCNF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBCNF to value 0"]
impl crate::Resettable for USBCNF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
