#[doc = "Register `USBOEPCNF_5` reader"]
pub type R = crate::R<USBOEPCNF_5_SPEC>;
#[doc = "Register `USBOEPCNF_5` writer"]
pub type W = crate::W<USBOEPCNF_5_SPEC>;
#[doc = "Field `USBIIE` reader - USB - Transaction Interrupt indication enable"]
pub type USBIIE_R = crate::BitReader;
#[doc = "Field `USBIIE` writer - USB - Transaction Interrupt indication enable"]
pub type USBIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - USB - Stall Condition"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - USB - Stall Condition"]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUF` reader - USB - Double Buffer Enable"]
pub type DBUF_R = crate::BitReader;
#[doc = "Field `DBUF` writer - USB - Double Buffer Enable"]
pub type DBUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOGGLE` reader - USB - Toggle Bit"]
pub type TOGGLE_R = crate::BitReader;
#[doc = "Field `TOGGLE` writer - USB - Toggle Bit"]
pub type TOGGLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UBME` reader - USB - UBM In-Endpoint Enable"]
pub type UBME_R = crate::BitReader;
#[doc = "Field `UBME` writer - USB - UBM In-Endpoint Enable"]
pub type UBME_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - USB - Transaction Interrupt indication enable"]
    #[inline(always)]
    pub fn usbiie(&self) -> USBIIE_R {
        USBIIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB - Stall Condition"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB - Double Buffer Enable"]
    #[inline(always)]
    pub fn dbuf(&self) -> DBUF_R {
        DBUF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB - Toggle Bit"]
    #[inline(always)]
    pub fn toggle(&self) -> TOGGLE_R {
        TOGGLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - USB - UBM In-Endpoint Enable"]
    #[inline(always)]
    pub fn ubme(&self) -> UBME_R {
        UBME_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - USB - Transaction Interrupt indication enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbiie(&mut self) -> USBIIE_W<USBOEPCNF_5_SPEC> {
        USBIIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - USB - Stall Condition"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<USBOEPCNF_5_SPEC> {
        STALL_W::new(self, 3)
    }
    #[doc = "Bit 4 - USB - Double Buffer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbuf(&mut self) -> DBUF_W<USBOEPCNF_5_SPEC> {
        DBUF_W::new(self, 4)
    }
    #[doc = "Bit 5 - USB - Toggle Bit"]
    #[inline(always)]
    #[must_use]
    pub fn toggle(&mut self) -> TOGGLE_W<USBOEPCNF_5_SPEC> {
        TOGGLE_W::new(self, 5)
    }
    #[doc = "Bit 7 - USB - UBM In-Endpoint Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ubme(&mut self) -> UBME_W<USBOEPCNF_5_SPEC> {
        UBME_W::new(self, 7)
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
#[doc = "Output Endpoint_5: Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepcnf_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepcnf_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBOEPCNF_5_SPEC;
impl crate::RegisterSpec for USBOEPCNF_5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usboepcnf_5::R`](R) reader structure"]
impl crate::Readable for USBOEPCNF_5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usboepcnf_5::W`](W) writer structure"]
impl crate::Writable for USBOEPCNF_5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBOEPCNF_5 to value 0"]
impl crate::Resettable for USBOEPCNF_5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
