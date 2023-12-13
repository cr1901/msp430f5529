#[doc = "Register `PJDIR` reader"]
pub type R = crate::R<PJDIR_SPEC>;
#[doc = "Register `PJDIR` writer"]
pub type W = crate::W<PJDIR_SPEC>;
#[doc = "Field `PJDIR0` reader - PJDIR0"]
pub type PJDIR0_R = crate::BitReader;
#[doc = "Field `PJDIR0` writer - PJDIR0"]
pub type PJDIR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJDIR1` reader - PJDIR1"]
pub type PJDIR1_R = crate::BitReader;
#[doc = "Field `PJDIR1` writer - PJDIR1"]
pub type PJDIR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJDIR2` reader - PJDIR2"]
pub type PJDIR2_R = crate::BitReader;
#[doc = "Field `PJDIR2` writer - PJDIR2"]
pub type PJDIR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJDIR3` reader - PJDIR3"]
pub type PJDIR3_R = crate::BitReader;
#[doc = "Field `PJDIR3` writer - PJDIR3"]
pub type PJDIR3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PJDIR0"]
    #[inline(always)]
    pub fn pjdir0(&self) -> PJDIR0_R {
        PJDIR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PJDIR1"]
    #[inline(always)]
    pub fn pjdir1(&self) -> PJDIR1_R {
        PJDIR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PJDIR2"]
    #[inline(always)]
    pub fn pjdir2(&self) -> PJDIR2_R {
        PJDIR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PJDIR3"]
    #[inline(always)]
    pub fn pjdir3(&self) -> PJDIR3_R {
        PJDIR3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJDIR0"]
    #[inline(always)]
    #[must_use]
    pub fn pjdir0(&mut self) -> PJDIR0_W<PJDIR_SPEC> {
        PJDIR0_W::new(self, 0)
    }
    #[doc = "Bit 1 - PJDIR1"]
    #[inline(always)]
    #[must_use]
    pub fn pjdir1(&mut self) -> PJDIR1_W<PJDIR_SPEC> {
        PJDIR1_W::new(self, 1)
    }
    #[doc = "Bit 2 - PJDIR2"]
    #[inline(always)]
    #[must_use]
    pub fn pjdir2(&mut self) -> PJDIR2_W<PJDIR_SPEC> {
        PJDIR2_W::new(self, 2)
    }
    #[doc = "Bit 3 - PJDIR3"]
    #[inline(always)]
    #[must_use]
    pub fn pjdir3(&mut self) -> PJDIR3_W<PJDIR_SPEC> {
        PJDIR3_W::new(self, 3)
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
#[doc = "Port J Direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pjdir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pjdir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PJDIR_SPEC;
impl crate::RegisterSpec for PJDIR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pjdir::R`](R) reader structure"]
impl crate::Readable for PJDIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pjdir::W`](W) writer structure"]
impl crate::Writable for PJDIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PJDIR to value 0"]
impl crate::Resettable for PJDIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
