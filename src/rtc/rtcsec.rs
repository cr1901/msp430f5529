#[doc = "Register `RTCSEC` reader"]
pub type R = crate::R<RTCSEC_SPEC>;
#[doc = "Register `RTCSEC` writer"]
pub type W = crate::W<RTCSEC_SPEC>;
#[doc = "Field `SECONDS0` reader - Real Time Clock Seconds Bit: 0"]
pub type SECONDS0_R = crate::BitReader;
#[doc = "Field `SECONDS0` writer - Real Time Clock Seconds Bit: 0"]
pub type SECONDS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECONDS1` reader - Real Time Clock Seconds Bit: 1"]
pub type SECONDS1_R = crate::BitReader;
#[doc = "Field `SECONDS1` writer - Real Time Clock Seconds Bit: 1"]
pub type SECONDS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECONDS2` reader - Real Time Clock Seconds Bit: 2"]
pub type SECONDS2_R = crate::BitReader;
#[doc = "Field `SECONDS2` writer - Real Time Clock Seconds Bit: 2"]
pub type SECONDS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECONDS3` reader - Real Time Clock Seconds Bit: 3"]
pub type SECONDS3_R = crate::BitReader;
#[doc = "Field `SECONDS3` writer - Real Time Clock Seconds Bit: 3"]
pub type SECONDS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECONDS4` reader - Real Time Clock Seconds Bit: 4"]
pub type SECONDS4_R = crate::BitReader;
#[doc = "Field `SECONDS4` writer - Real Time Clock Seconds Bit: 4"]
pub type SECONDS4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECONDS5` reader - Real Time Clock Seconds Bit: 5"]
pub type SECONDS5_R = crate::BitReader;
#[doc = "Field `SECONDS5` writer - Real Time Clock Seconds Bit: 5"]
pub type SECONDS5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECONDS6` reader - Real Time Clock Seconds Bit: 6"]
pub type SECONDS6_R = crate::BitReader;
#[doc = "Field `SECONDS6` writer - Real Time Clock Seconds Bit: 6"]
pub type SECONDS6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Real Time Clock Seconds Bit: 0"]
    #[inline(always)]
    pub fn seconds0(&self) -> SECONDS0_R {
        SECONDS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock Seconds Bit: 1"]
    #[inline(always)]
    pub fn seconds1(&self) -> SECONDS1_R {
        SECONDS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock Seconds Bit: 2"]
    #[inline(always)]
    pub fn seconds2(&self) -> SECONDS2_R {
        SECONDS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Seconds Bit: 3"]
    #[inline(always)]
    pub fn seconds3(&self) -> SECONDS3_R {
        SECONDS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock Seconds Bit: 4"]
    #[inline(always)]
    pub fn seconds4(&self) -> SECONDS4_R {
        SECONDS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock Seconds Bit: 5"]
    #[inline(always)]
    pub fn seconds5(&self) -> SECONDS5_R {
        SECONDS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock Seconds Bit: 6"]
    #[inline(always)]
    pub fn seconds6(&self) -> SECONDS6_R {
        SECONDS6_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Seconds Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn seconds0(&mut self) -> SECONDS0_W<RTCSEC_SPEC> {
        SECONDS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Real Time Clock Seconds Bit: 1"]
    #[inline(always)]
    #[must_use]
    pub fn seconds1(&mut self) -> SECONDS1_W<RTCSEC_SPEC> {
        SECONDS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Real Time Clock Seconds Bit: 2"]
    #[inline(always)]
    #[must_use]
    pub fn seconds2(&mut self) -> SECONDS2_W<RTCSEC_SPEC> {
        SECONDS2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Real Time Clock Seconds Bit: 3"]
    #[inline(always)]
    #[must_use]
    pub fn seconds3(&mut self) -> SECONDS3_W<RTCSEC_SPEC> {
        SECONDS3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Real Time Clock Seconds Bit: 4"]
    #[inline(always)]
    #[must_use]
    pub fn seconds4(&mut self) -> SECONDS4_W<RTCSEC_SPEC> {
        SECONDS4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Real Time Clock Seconds Bit: 5"]
    #[inline(always)]
    #[must_use]
    pub fn seconds5(&mut self) -> SECONDS5_W<RTCSEC_SPEC> {
        SECONDS5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Real Time Clock Seconds Bit: 6"]
    #[inline(always)]
    #[must_use]
    pub fn seconds6(&mut self) -> SECONDS6_W<RTCSEC_SPEC> {
        SECONDS6_W::new(self, 6)
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
#[doc = "Real Time Clock Seconds\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCSEC_SPEC;
impl crate::RegisterSpec for RTCSEC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rtcsec::R`](R) reader structure"]
impl crate::Readable for RTCSEC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtcsec::W`](W) writer structure"]
impl crate::Writable for RTCSEC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCSEC to value 0"]
impl crate::Resettable for RTCSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
