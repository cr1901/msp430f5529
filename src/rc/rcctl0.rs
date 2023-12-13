#[doc = "Register `RCCTL0` reader"]
pub type R = crate::R<RCCTL0_SPEC>;
#[doc = "Register `RCCTL0` writer"]
pub type W = crate::W<RCCTL0_SPEC>;
#[doc = "Field `RCRS0OFF` reader - RAM Controller RAM Sector 0 Off"]
pub type RCRS0OFF_R = crate::BitReader;
#[doc = "Field `RCRS0OFF` writer - RAM Controller RAM Sector 0 Off"]
pub type RCRS0OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCRS1OFF` reader - RAM Controller RAM Sector 1 Off"]
pub type RCRS1OFF_R = crate::BitReader;
#[doc = "Field `RCRS1OFF` writer - RAM Controller RAM Sector 1 Off"]
pub type RCRS1OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCRS2OFF` reader - RAM Controller RAM Sector 2 Off"]
pub type RCRS2OFF_R = crate::BitReader;
#[doc = "Field `RCRS2OFF` writer - RAM Controller RAM Sector 2 Off"]
pub type RCRS2OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCRS3OFF` reader - RAM Controller RAM Sector 3 Off"]
pub type RCRS3OFF_R = crate::BitReader;
#[doc = "Field `RCRS3OFF` writer - RAM Controller RAM Sector 3 Off"]
pub type RCRS3OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCRS7OFF` reader - RAM Controller RAM Sector 7 (USB) Off"]
pub type RCRS7OFF_R = crate::BitReader;
#[doc = "Field `RCRS7OFF` writer - RAM Controller RAM Sector 7 (USB) Off"]
pub type RCRS7OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RAM Controller RAM Sector 0 Off"]
    #[inline(always)]
    pub fn rcrs0off(&self) -> RCRS0OFF_R {
        RCRS0OFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM Controller RAM Sector 1 Off"]
    #[inline(always)]
    pub fn rcrs1off(&self) -> RCRS1OFF_R {
        RCRS1OFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAM Controller RAM Sector 2 Off"]
    #[inline(always)]
    pub fn rcrs2off(&self) -> RCRS2OFF_R {
        RCRS2OFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RAM Controller RAM Sector 3 Off"]
    #[inline(always)]
    pub fn rcrs3off(&self) -> RCRS3OFF_R {
        RCRS3OFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - RAM Controller RAM Sector 7 (USB) Off"]
    #[inline(always)]
    pub fn rcrs7off(&self) -> RCRS7OFF_R {
        RCRS7OFF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM Controller RAM Sector 0 Off"]
    #[inline(always)]
    #[must_use]
    pub fn rcrs0off(&mut self) -> RCRS0OFF_W<RCCTL0_SPEC> {
        RCRS0OFF_W::new(self, 0)
    }
    #[doc = "Bit 1 - RAM Controller RAM Sector 1 Off"]
    #[inline(always)]
    #[must_use]
    pub fn rcrs1off(&mut self) -> RCRS1OFF_W<RCCTL0_SPEC> {
        RCRS1OFF_W::new(self, 1)
    }
    #[doc = "Bit 2 - RAM Controller RAM Sector 2 Off"]
    #[inline(always)]
    #[must_use]
    pub fn rcrs2off(&mut self) -> RCRS2OFF_W<RCCTL0_SPEC> {
        RCRS2OFF_W::new(self, 2)
    }
    #[doc = "Bit 3 - RAM Controller RAM Sector 3 Off"]
    #[inline(always)]
    #[must_use]
    pub fn rcrs3off(&mut self) -> RCRS3OFF_W<RCCTL0_SPEC> {
        RCRS3OFF_W::new(self, 3)
    }
    #[doc = "Bit 7 - RAM Controller RAM Sector 7 (USB) Off"]
    #[inline(always)]
    #[must_use]
    pub fn rcrs7off(&mut self) -> RCRS7OFF_W<RCCTL0_SPEC> {
        RCRS7OFF_W::new(self, 7)
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
#[doc = "Ram Controller Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCCTL0_SPEC;
impl crate::RegisterSpec for RCCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rcctl0::R`](R) reader structure"]
impl crate::Readable for RCCTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcctl0::W`](W) writer structure"]
impl crate::Writable for RCCTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCCTL0 to value 0"]
impl crate::Resettable for RCCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
