#[doc = "Register `DMACTL4` reader"]
pub type R = crate::R<DMACTL4_SPEC>;
#[doc = "Register `DMACTL4` writer"]
pub type W = crate::W<DMACTL4_SPEC>;
#[doc = "Field `ENNMI` reader - Enable NMI interruption of DMA"]
pub type ENNMI_R = crate::BitReader;
#[doc = "Field `ENNMI` writer - Enable NMI interruption of DMA"]
pub type ENNMI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROUNDROBIN` reader - Round-Robin DMA channel priorities"]
pub type ROUNDROBIN_R = crate::BitReader;
#[doc = "Field `ROUNDROBIN` writer - Round-Robin DMA channel priorities"]
pub type ROUNDROBIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMARMWDIS` reader - Inhibited DMA transfers during read-modify-write CPU operations"]
pub type DMARMWDIS_R = crate::BitReader;
#[doc = "Field `DMARMWDIS` writer - Inhibited DMA transfers during read-modify-write CPU operations"]
pub type DMARMWDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable NMI interruption of DMA"]
    #[inline(always)]
    pub fn ennmi(&self) -> ENNMI_R {
        ENNMI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Round-Robin DMA channel priorities"]
    #[inline(always)]
    pub fn roundrobin(&self) -> ROUNDROBIN_R {
        ROUNDROBIN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Inhibited DMA transfers during read-modify-write CPU operations"]
    #[inline(always)]
    pub fn dmarmwdis(&self) -> DMARMWDIS_R {
        DMARMWDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable NMI interruption of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn ennmi(&mut self) -> ENNMI_W<DMACTL4_SPEC> {
        ENNMI_W::new(self, 0)
    }
    #[doc = "Bit 1 - Round-Robin DMA channel priorities"]
    #[inline(always)]
    #[must_use]
    pub fn roundrobin(&mut self) -> ROUNDROBIN_W<DMACTL4_SPEC> {
        ROUNDROBIN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Inhibited DMA transfers during read-modify-write CPU operations"]
    #[inline(always)]
    #[must_use]
    pub fn dmarmwdis(&mut self) -> DMARMWDIS_W<DMACTL4_SPEC> {
        DMARMWDIS_W::new(self, 2)
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
#[doc = "DMA Module Control 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactl4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactl4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACTL4_SPEC;
impl crate::RegisterSpec for DMACTL4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dmactl4::R`](R) reader structure"]
impl crate::Readable for DMACTL4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmactl4::W`](W) writer structure"]
impl crate::Writable for DMACTL4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACTL4 to value 0"]
impl crate::Resettable for DMACTL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
