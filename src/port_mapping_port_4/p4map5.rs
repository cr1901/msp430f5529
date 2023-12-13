#[doc = "Register `P4MAP5` reader"]
pub type R = crate::R<P4MAP5_SPEC>;
#[doc = "Register `P4MAP5` writer"]
pub type W = crate::W<P4MAP5_SPEC>;
#[doc = "Field `PMAP0` reader - PMAP0"]
pub type PMAP0_R = crate::BitReader;
#[doc = "Field `PMAP0` writer - PMAP0"]
pub type PMAP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMAP1` reader - PMAP1"]
pub type PMAP1_R = crate::BitReader;
#[doc = "Field `PMAP1` writer - PMAP1"]
pub type PMAP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMAP2` reader - PMAP2"]
pub type PMAP2_R = crate::BitReader;
#[doc = "Field `PMAP2` writer - PMAP2"]
pub type PMAP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMAP3` reader - PMAP3"]
pub type PMAP3_R = crate::BitReader;
#[doc = "Field `PMAP3` writer - PMAP3"]
pub type PMAP3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMAP4` reader - PMAP4"]
pub type PMAP4_R = crate::BitReader;
#[doc = "Field `PMAP4` writer - PMAP4"]
pub type PMAP4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMAP5` reader - PMAP5"]
pub type PMAP5_R = crate::BitReader;
#[doc = "Field `PMAP5` writer - PMAP5"]
pub type PMAP5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMAP6` reader - PMAP6"]
pub type PMAP6_R = crate::BitReader;
#[doc = "Field `PMAP6` writer - PMAP6"]
pub type PMAP6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMAP7` reader - PMAP7"]
pub type PMAP7_R = crate::BitReader;
#[doc = "Field `PMAP7` writer - PMAP7"]
pub type PMAP7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PMAP0"]
    #[inline(always)]
    pub fn pmap0(&self) -> PMAP0_R {
        PMAP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PMAP1"]
    #[inline(always)]
    pub fn pmap1(&self) -> PMAP1_R {
        PMAP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PMAP2"]
    #[inline(always)]
    pub fn pmap2(&self) -> PMAP2_R {
        PMAP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PMAP3"]
    #[inline(always)]
    pub fn pmap3(&self) -> PMAP3_R {
        PMAP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMAP4"]
    #[inline(always)]
    pub fn pmap4(&self) -> PMAP4_R {
        PMAP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PMAP5"]
    #[inline(always)]
    pub fn pmap5(&self) -> PMAP5_R {
        PMAP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PMAP6"]
    #[inline(always)]
    pub fn pmap6(&self) -> PMAP6_R {
        PMAP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PMAP7"]
    #[inline(always)]
    pub fn pmap7(&self) -> PMAP7_R {
        PMAP7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PMAP0"]
    #[inline(always)]
    #[must_use]
    pub fn pmap0(&mut self) -> PMAP0_W<P4MAP5_SPEC> {
        PMAP0_W::new(self, 0)
    }
    #[doc = "Bit 1 - PMAP1"]
    #[inline(always)]
    #[must_use]
    pub fn pmap1(&mut self) -> PMAP1_W<P4MAP5_SPEC> {
        PMAP1_W::new(self, 1)
    }
    #[doc = "Bit 2 - PMAP2"]
    #[inline(always)]
    #[must_use]
    pub fn pmap2(&mut self) -> PMAP2_W<P4MAP5_SPEC> {
        PMAP2_W::new(self, 2)
    }
    #[doc = "Bit 3 - PMAP3"]
    #[inline(always)]
    #[must_use]
    pub fn pmap3(&mut self) -> PMAP3_W<P4MAP5_SPEC> {
        PMAP3_W::new(self, 3)
    }
    #[doc = "Bit 4 - PMAP4"]
    #[inline(always)]
    #[must_use]
    pub fn pmap4(&mut self) -> PMAP4_W<P4MAP5_SPEC> {
        PMAP4_W::new(self, 4)
    }
    #[doc = "Bit 5 - PMAP5"]
    #[inline(always)]
    #[must_use]
    pub fn pmap5(&mut self) -> PMAP5_W<P4MAP5_SPEC> {
        PMAP5_W::new(self, 5)
    }
    #[doc = "Bit 6 - PMAP6"]
    #[inline(always)]
    #[must_use]
    pub fn pmap6(&mut self) -> PMAP6_W<P4MAP5_SPEC> {
        PMAP6_W::new(self, 6)
    }
    #[doc = "Bit 7 - PMAP7"]
    #[inline(always)]
    #[must_use]
    pub fn pmap7(&mut self) -> PMAP7_W<P4MAP5_SPEC> {
        PMAP7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port P4.5 mapping register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p4map5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p4map5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4MAP5_SPEC;
impl crate::RegisterSpec for P4MAP5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4map5::R`](R) reader structure"]
impl crate::Readable for P4MAP5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p4map5::W`](W) writer structure"]
impl crate::Writable for P4MAP5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P4MAP5 to value 0"]
impl crate::Resettable for P4MAP5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
