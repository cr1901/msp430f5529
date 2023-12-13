#[doc = "Register `FCTL3` reader"]
pub type R = crate::R<FCTL3_SPEC>;
#[doc = "Register `FCTL3` writer"]
pub type W = crate::W<FCTL3_SPEC>;
#[doc = "Field `BUSY` reader - Flash busy: 1"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `BUSY` writer - Flash busy: 1"]
pub type BUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYV` reader - Flash Key violation flag"]
pub type KEYV_R = crate::BitReader;
#[doc = "Field `KEYV` writer - Flash Key violation flag"]
pub type KEYV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCVIFG` reader - Flash Access violation flag"]
pub type ACCVIFG_R = crate::BitReader;
#[doc = "Field `ACCVIFG` writer - Flash Access violation flag"]
pub type ACCVIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT` reader - Wait flag for segment write"]
pub type WAIT_R = crate::BitReader;
#[doc = "Field `WAIT` writer - Wait flag for segment write"]
pub type WAIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - Lock bit: 1 - Flash is locked (read only)"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - Lock bit: 1 - Flash is locked (read only)"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMEX` reader - Flash Emergency Exit"]
pub type EMEX_R = crate::BitReader;
#[doc = "Field `EMEX` writer - Flash Emergency Exit"]
pub type EMEX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKA` reader - Segment A Lock bit: read = 1 - Segment is locked (read only)"]
pub type LOCKA_R = crate::BitReader;
#[doc = "Field `LOCKA` writer - Segment A Lock bit: read = 1 - Segment is locked (read only)"]
pub type LOCKA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flash busy: 1"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Key violation flag"]
    #[inline(always)]
    pub fn keyv(&self) -> KEYV_R {
        KEYV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flash Access violation flag"]
    #[inline(always)]
    pub fn accvifg(&self) -> ACCVIFG_R {
        ACCVIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wait flag for segment write"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lock bit: 1 - Flash is locked (read only)"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash Emergency Exit"]
    #[inline(always)]
    pub fn emex(&self) -> EMEX_R {
        EMEX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Segment A Lock bit: read = 1 - Segment is locked (read only)"]
    #[inline(always)]
    pub fn locka(&self) -> LOCKA_R {
        LOCKA_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash busy: 1"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<FCTL3_SPEC> {
        BUSY_W::new(self, 0)
    }
    #[doc = "Bit 1 - Flash Key violation flag"]
    #[inline(always)]
    #[must_use]
    pub fn keyv(&mut self) -> KEYV_W<FCTL3_SPEC> {
        KEYV_W::new(self, 1)
    }
    #[doc = "Bit 2 - Flash Access violation flag"]
    #[inline(always)]
    #[must_use]
    pub fn accvifg(&mut self) -> ACCVIFG_W<FCTL3_SPEC> {
        ACCVIFG_W::new(self, 2)
    }
    #[doc = "Bit 3 - Wait flag for segment write"]
    #[inline(always)]
    #[must_use]
    pub fn wait(&mut self) -> WAIT_W<FCTL3_SPEC> {
        WAIT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Lock bit: 1 - Flash is locked (read only)"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<FCTL3_SPEC> {
        LOCK_W::new(self, 4)
    }
    #[doc = "Bit 5 - Flash Emergency Exit"]
    #[inline(always)]
    #[must_use]
    pub fn emex(&mut self) -> EMEX_W<FCTL3_SPEC> {
        EMEX_W::new(self, 5)
    }
    #[doc = "Bit 6 - Segment A Lock bit: read = 1 - Segment is locked (read only)"]
    #[inline(always)]
    #[must_use]
    pub fn locka(&mut self) -> LOCKA_W<FCTL3_SPEC> {
        LOCKA_W::new(self, 6)
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
#[doc = "FLASH Control 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCTL3_SPEC;
impl crate::RegisterSpec for FCTL3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fctl3::R`](R) reader structure"]
impl crate::Readable for FCTL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fctl3::W`](W) writer structure"]
impl crate::Writable for FCTL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCTL3 to value 0"]
impl crate::Resettable for FCTL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
