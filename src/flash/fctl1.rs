#[doc = "Register `FCTL1` reader"]
pub type R = crate::R<FCTL1_SPEC>;
#[doc = "Register `FCTL1` writer"]
pub type W = crate::W<FCTL1_SPEC>;
#[doc = "Field `ERASE` reader - Enable bit for Flash segment erase"]
pub type ERASE_R = crate::BitReader;
#[doc = "Field `ERASE` writer - Enable bit for Flash segment erase"]
pub type ERASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERAS` reader - Enable bit for Flash mass erase"]
pub type MERAS_R = crate::BitReader;
#[doc = "Field `MERAS` writer - Enable bit for Flash mass erase"]
pub type MERAS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRT` reader - Smart Write enable"]
pub type SWRT_R = crate::BitReader;
#[doc = "Field `SWRT` writer - Smart Write enable"]
pub type SWRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRT` reader - Enable bit for Flash write"]
pub type WRT_R = crate::BitReader;
#[doc = "Field `WRT` writer - Enable bit for Flash write"]
pub type WRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLKWRT` reader - Enable bit for Flash segment write"]
pub type BLKWRT_R = crate::BitReader;
#[doc = "Field `BLKWRT` writer - Enable bit for Flash segment write"]
pub type BLKWRT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Enable bit for Flash segment erase"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable bit for Flash mass erase"]
    #[inline(always)]
    pub fn meras(&self) -> MERAS_R {
        MERAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Smart Write enable"]
    #[inline(always)]
    pub fn swrt(&self) -> SWRT_R {
        SWRT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable bit for Flash write"]
    #[inline(always)]
    pub fn wrt(&self) -> WRT_R {
        WRT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable bit for Flash segment write"]
    #[inline(always)]
    pub fn blkwrt(&self) -> BLKWRT_R {
        BLKWRT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable bit for Flash segment erase"]
    #[inline(always)]
    #[must_use]
    pub fn erase(&mut self) -> ERASE_W<FCTL1_SPEC> {
        ERASE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable bit for Flash mass erase"]
    #[inline(always)]
    #[must_use]
    pub fn meras(&mut self) -> MERAS_W<FCTL1_SPEC> {
        MERAS_W::new(self, 2)
    }
    #[doc = "Bit 5 - Smart Write enable"]
    #[inline(always)]
    #[must_use]
    pub fn swrt(&mut self) -> SWRT_W<FCTL1_SPEC> {
        SWRT_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable bit for Flash write"]
    #[inline(always)]
    #[must_use]
    pub fn wrt(&mut self) -> WRT_W<FCTL1_SPEC> {
        WRT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable bit for Flash segment write"]
    #[inline(always)]
    #[must_use]
    pub fn blkwrt(&mut self) -> BLKWRT_W<FCTL1_SPEC> {
        BLKWRT_W::new(self, 7)
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
#[doc = "FLASH Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCTL1_SPEC;
impl crate::RegisterSpec for FCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fctl1::R`](R) reader structure"]
impl crate::Readable for FCTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fctl1::W`](W) writer structure"]
impl crate::Writable for FCTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCTL1 to value 0"]
impl crate::Resettable for FCTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
