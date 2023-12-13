#[doc = "Register `SYSCTL` reader"]
pub type R = crate::R<SYSCTL_SPEC>;
#[doc = "Register `SYSCTL` writer"]
pub type W = crate::W<SYSCTL_SPEC>;
#[doc = "Field `SYSRIVECT` reader - SYS - RAM based interrupt vectors"]
pub type SYSRIVECT_R = crate::BitReader;
#[doc = "Field `SYSRIVECT` writer - SYS - RAM based interrupt vectors"]
pub type SYSRIVECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSPMMPE` reader - SYS - PMM access protect"]
pub type SYSPMMPE_R = crate::BitReader;
#[doc = "Field `SYSPMMPE` writer - SYS - PMM access protect"]
pub type SYSPMMPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSBSLIND` reader - SYS - TCK/RST indication detected"]
pub type SYSBSLIND_R = crate::BitReader;
#[doc = "Field `SYSBSLIND` writer - SYS - TCK/RST indication detected"]
pub type SYSBSLIND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSJTAGPIN` reader - SYS - Dedicated JTAG pins enabled"]
pub type SYSJTAGPIN_R = crate::BitReader;
#[doc = "Field `SYSJTAGPIN` writer - SYS - Dedicated JTAG pins enabled"]
pub type SYSJTAGPIN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYS - RAM based interrupt vectors"]
    #[inline(always)]
    pub fn sysrivect(&self) -> SYSRIVECT_R {
        SYSRIVECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SYS - PMM access protect"]
    #[inline(always)]
    pub fn syspmmpe(&self) -> SYSPMMPE_R {
        SYSPMMPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - SYS - TCK/RST indication detected"]
    #[inline(always)]
    pub fn sysbslind(&self) -> SYSBSLIND_R {
        SYSBSLIND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SYS - Dedicated JTAG pins enabled"]
    #[inline(always)]
    pub fn sysjtagpin(&self) -> SYSJTAGPIN_R {
        SYSJTAGPIN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYS - RAM based interrupt vectors"]
    #[inline(always)]
    #[must_use]
    pub fn sysrivect(&mut self) -> SYSRIVECT_W<SYSCTL_SPEC> {
        SYSRIVECT_W::new(self, 0)
    }
    #[doc = "Bit 2 - SYS - PMM access protect"]
    #[inline(always)]
    #[must_use]
    pub fn syspmmpe(&mut self) -> SYSPMMPE_W<SYSCTL_SPEC> {
        SYSPMMPE_W::new(self, 2)
    }
    #[doc = "Bit 4 - SYS - TCK/RST indication detected"]
    #[inline(always)]
    #[must_use]
    pub fn sysbslind(&mut self) -> SYSBSLIND_W<SYSCTL_SPEC> {
        SYSBSLIND_W::new(self, 4)
    }
    #[doc = "Bit 5 - SYS - Dedicated JTAG pins enabled"]
    #[inline(always)]
    #[must_use]
    pub fn sysjtagpin(&mut self) -> SYSJTAGPIN_W<SYSCTL_SPEC> {
        SYSJTAGPIN_W::new(self, 5)
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
#[doc = "System control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCTL_SPEC;
impl crate::RegisterSpec for SYSCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysctl::R`](R) reader structure"]
impl crate::Readable for SYSCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sysctl::W`](W) writer structure"]
impl crate::Writable for SYSCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCTL to value 0"]
impl crate::Resettable for SYSCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
