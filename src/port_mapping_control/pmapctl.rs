#[doc = "Register `PMAPCTL` reader"]
pub type R = crate::R<PMAPCTL_SPEC>;
#[doc = "Register `PMAPCTL` writer"]
pub type W = crate::W<PMAPCTL_SPEC>;
#[doc = "Field `PMAPLOCKED` reader - Port Mapping Lock bit. Read only"]
pub type PMAPLOCKED_R = crate::BitReader;
#[doc = "Field `PMAPLOCKED` writer - Port Mapping Lock bit. Read only"]
pub type PMAPLOCKED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMAPRECFG` reader - Port Mapping re-configuration control bit"]
pub type PMAPRECFG_R = crate::BitReader;
#[doc = "Field `PMAPRECFG` writer - Port Mapping re-configuration control bit"]
pub type PMAPRECFG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port Mapping Lock bit. Read only"]
    #[inline(always)]
    pub fn pmaplocked(&self) -> PMAPLOCKED_R {
        PMAPLOCKED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Mapping re-configuration control bit"]
    #[inline(always)]
    pub fn pmaprecfg(&self) -> PMAPRECFG_R {
        PMAPRECFG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Mapping Lock bit. Read only"]
    #[inline(always)]
    #[must_use]
    pub fn pmaplocked(&mut self) -> PMAPLOCKED_W<PMAPCTL_SPEC> {
        PMAPLOCKED_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port Mapping re-configuration control bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmaprecfg(&mut self) -> PMAPRECFG_W<PMAPCTL_SPEC> {
        PMAPRECFG_W::new(self, 1)
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
#[doc = "Port Mapping control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmapctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmapctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMAPCTL_SPEC;
impl crate::RegisterSpec for PMAPCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pmapctl::R`](R) reader structure"]
impl crate::Readable for PMAPCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmapctl::W`](W) writer structure"]
impl crate::Writable for PMAPCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMAPCTL to value 0"]
impl crate::Resettable for PMAPCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
