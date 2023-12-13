#[doc = "Register `USBIE` reader"]
pub type R = crate::R<USBIE_SPEC>;
#[doc = "Register `USBIE` writer"]
pub type W = crate::W<USBIE_SPEC>;
#[doc = "Field `STPOWIE` reader - USB - Setup Overwrite Interrupt Enable"]
pub type STPOWIE_R = crate::BitReader;
#[doc = "Field `STPOWIE` writer - USB - Setup Overwrite Interrupt Enable"]
pub type STPOWIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUPIE` reader - USB - Setup Interrupt Enable"]
pub type SETUPIE_R = crate::BitReader;
#[doc = "Field `SETUPIE` writer - USB - Setup Interrupt Enable"]
pub type SETUPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESRIE` reader - USB - Function Resume Request Interrupt Enable"]
pub type RESRIE_R = crate::BitReader;
#[doc = "Field `RESRIE` writer - USB - Function Resume Request Interrupt Enable"]
pub type RESRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSRIE` reader - USB - Function Suspend Request Interrupt Enable"]
pub type SUSRIE_R = crate::BitReader;
#[doc = "Field `SUSRIE` writer - USB - Function Suspend Request Interrupt Enable"]
pub type SUSRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTRIE` reader - USB - Function Reset Request Interrupt Enable"]
pub type RSTRIE_R = crate::BitReader;
#[doc = "Field `RSTRIE` writer - USB - Function Reset Request Interrupt Enable"]
pub type RSTRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB - Setup Overwrite Interrupt Enable"]
    #[inline(always)]
    pub fn stpowie(&self) -> STPOWIE_R {
        STPOWIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - USB - Setup Interrupt Enable"]
    #[inline(always)]
    pub fn setupie(&self) -> SETUPIE_R {
        SETUPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - USB - Function Resume Request Interrupt Enable"]
    #[inline(always)]
    pub fn resrie(&self) -> RESRIE_R {
        RESRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB - Function Suspend Request Interrupt Enable"]
    #[inline(always)]
    pub fn susrie(&self) -> SUSRIE_R {
        SUSRIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB - Function Reset Request Interrupt Enable"]
    #[inline(always)]
    pub fn rstrie(&self) -> RSTRIE_R {
        RSTRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - Setup Overwrite Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stpowie(&mut self) -> STPOWIE_W<USBIE_SPEC> {
        STPOWIE_W::new(self, 0)
    }
    #[doc = "Bit 2 - USB - Setup Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn setupie(&mut self) -> SETUPIE_W<USBIE_SPEC> {
        SETUPIE_W::new(self, 2)
    }
    #[doc = "Bit 5 - USB - Function Resume Request Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn resrie(&mut self) -> RESRIE_W<USBIE_SPEC> {
        RESRIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - USB - Function Suspend Request Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn susrie(&mut self) -> SUSRIE_W<USBIE_SPEC> {
        SUSRIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - USB - Function Reset Request Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstrie(&mut self) -> RSTRIE_W<USBIE_SPEC> {
        RSTRIE_W::new(self, 7)
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
#[doc = "USB interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBIE_SPEC;
impl crate::RegisterSpec for USBIE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usbie::R`](R) reader structure"]
impl crate::Readable for USBIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbie::W`](W) writer structure"]
impl crate::Writable for USBIE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBIE to value 0"]
impl crate::Resettable for USBIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
