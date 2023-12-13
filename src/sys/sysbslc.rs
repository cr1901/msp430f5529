#[doc = "Register `SYSBSLC` reader"]
pub type R = crate::R<SYSBSLC_SPEC>;
#[doc = "Register `SYSBSLC` writer"]
pub type W = crate::W<SYSBSLC_SPEC>;
#[doc = "Field `SYSBSLSIZE0` reader - SYS - BSL Protection Size 0"]
pub type SYSBSLSIZE0_R = crate::BitReader;
#[doc = "Field `SYSBSLSIZE0` writer - SYS - BSL Protection Size 0"]
pub type SYSBSLSIZE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSBSLSIZE1` reader - SYS - BSL Protection Size 1"]
pub type SYSBSLSIZE1_R = crate::BitReader;
#[doc = "Field `SYSBSLSIZE1` writer - SYS - BSL Protection Size 1"]
pub type SYSBSLSIZE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSBSLR` reader - SYS - RAM assigned to BSL"]
pub type SYSBSLR_R = crate::BitReader;
#[doc = "Field `SYSBSLR` writer - SYS - RAM assigned to BSL"]
pub type SYSBSLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSBSLOFF` reader - SYS - BSL Memory disabled"]
pub type SYSBSLOFF_R = crate::BitReader;
#[doc = "Field `SYSBSLOFF` writer - SYS - BSL Memory disabled"]
pub type SYSBSLOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSBSLPE` reader - SYS - BSL Memory protection enabled"]
pub type SYSBSLPE_R = crate::BitReader;
#[doc = "Field `SYSBSLPE` writer - SYS - BSL Memory protection enabled"]
pub type SYSBSLPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYS - BSL Protection Size 0"]
    #[inline(always)]
    pub fn sysbslsize0(&self) -> SYSBSLSIZE0_R {
        SYSBSLSIZE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYS - BSL Protection Size 1"]
    #[inline(always)]
    pub fn sysbslsize1(&self) -> SYSBSLSIZE1_R {
        SYSBSLSIZE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SYS - RAM assigned to BSL"]
    #[inline(always)]
    pub fn sysbslr(&self) -> SYSBSLR_R {
        SYSBSLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 14 - SYS - BSL Memory disabled"]
    #[inline(always)]
    pub fn sysbsloff(&self) -> SYSBSLOFF_R {
        SYSBSLOFF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SYS - BSL Memory protection enabled"]
    #[inline(always)]
    pub fn sysbslpe(&self) -> SYSBSLPE_R {
        SYSBSLPE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYS - BSL Protection Size 0"]
    #[inline(always)]
    #[must_use]
    pub fn sysbslsize0(&mut self) -> SYSBSLSIZE0_W<SYSBSLC_SPEC> {
        SYSBSLSIZE0_W::new(self, 0)
    }
    #[doc = "Bit 1 - SYS - BSL Protection Size 1"]
    #[inline(always)]
    #[must_use]
    pub fn sysbslsize1(&mut self) -> SYSBSLSIZE1_W<SYSBSLC_SPEC> {
        SYSBSLSIZE1_W::new(self, 1)
    }
    #[doc = "Bit 2 - SYS - RAM assigned to BSL"]
    #[inline(always)]
    #[must_use]
    pub fn sysbslr(&mut self) -> SYSBSLR_W<SYSBSLC_SPEC> {
        SYSBSLR_W::new(self, 2)
    }
    #[doc = "Bit 14 - SYS - BSL Memory disabled"]
    #[inline(always)]
    #[must_use]
    pub fn sysbsloff(&mut self) -> SYSBSLOFF_W<SYSBSLC_SPEC> {
        SYSBSLOFF_W::new(self, 14)
    }
    #[doc = "Bit 15 - SYS - BSL Memory protection enabled"]
    #[inline(always)]
    #[must_use]
    pub fn sysbslpe(&mut self) -> SYSBSLPE_W<SYSBSLC_SPEC> {
        SYSBSLPE_W::new(self, 15)
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
#[doc = "Boot strap configuration area\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysbslc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysbslc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSBSLC_SPEC;
impl crate::RegisterSpec for SYSBSLC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysbslc::R`](R) reader structure"]
impl crate::Readable for SYSBSLC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sysbslc::W`](W) writer structure"]
impl crate::Writable for SYSBSLC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSBSLC to value 0"]
impl crate::Resettable for SYSBSLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
