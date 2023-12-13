#[doc = "Register `P1DIR` reader"]
pub type R = crate::R<P1DIR_SPEC>;
#[doc = "Register `P1DIR` writer"]
pub type W = crate::W<P1DIR_SPEC>;
#[doc = "Field `P1DIR0` reader - P1DIR0"]
pub type P1DIR0_R = crate::BitReader;
#[doc = "Field `P1DIR0` writer - P1DIR0"]
pub type P1DIR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1DIR1` reader - P1DIR1"]
pub type P1DIR1_R = crate::BitReader;
#[doc = "Field `P1DIR1` writer - P1DIR1"]
pub type P1DIR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1DIR2` reader - P1DIR2"]
pub type P1DIR2_R = crate::BitReader;
#[doc = "Field `P1DIR2` writer - P1DIR2"]
pub type P1DIR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1DIR3` reader - P1DIR3"]
pub type P1DIR3_R = crate::BitReader;
#[doc = "Field `P1DIR3` writer - P1DIR3"]
pub type P1DIR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1DIR4` reader - P1DIR4"]
pub type P1DIR4_R = crate::BitReader;
#[doc = "Field `P1DIR4` writer - P1DIR4"]
pub type P1DIR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1DIR5` reader - P1DIR5"]
pub type P1DIR5_R = crate::BitReader;
#[doc = "Field `P1DIR5` writer - P1DIR5"]
pub type P1DIR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1DIR6` reader - P1DIR6"]
pub type P1DIR6_R = crate::BitReader;
#[doc = "Field `P1DIR6` writer - P1DIR6"]
pub type P1DIR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1DIR7` reader - P1DIR7"]
pub type P1DIR7_R = crate::BitReader;
#[doc = "Field `P1DIR7` writer - P1DIR7"]
pub type P1DIR7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P1DIR0"]
    #[inline(always)]
    pub fn p1dir0(&self) -> P1DIR0_R {
        P1DIR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P1DIR1"]
    #[inline(always)]
    pub fn p1dir1(&self) -> P1DIR1_R {
        P1DIR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P1DIR2"]
    #[inline(always)]
    pub fn p1dir2(&self) -> P1DIR2_R {
        P1DIR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P1DIR3"]
    #[inline(always)]
    pub fn p1dir3(&self) -> P1DIR3_R {
        P1DIR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P1DIR4"]
    #[inline(always)]
    pub fn p1dir4(&self) -> P1DIR4_R {
        P1DIR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P1DIR5"]
    #[inline(always)]
    pub fn p1dir5(&self) -> P1DIR5_R {
        P1DIR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P1DIR6"]
    #[inline(always)]
    pub fn p1dir6(&self) -> P1DIR6_R {
        P1DIR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P1DIR7"]
    #[inline(always)]
    pub fn p1dir7(&self) -> P1DIR7_R {
        P1DIR7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1DIR0"]
    #[inline(always)]
    #[must_use]
    pub fn p1dir0(&mut self) -> P1DIR0_W<P1DIR_SPEC> {
        P1DIR0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P1DIR1"]
    #[inline(always)]
    #[must_use]
    pub fn p1dir1(&mut self) -> P1DIR1_W<P1DIR_SPEC> {
        P1DIR1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P1DIR2"]
    #[inline(always)]
    #[must_use]
    pub fn p1dir2(&mut self) -> P1DIR2_W<P1DIR_SPEC> {
        P1DIR2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P1DIR3"]
    #[inline(always)]
    #[must_use]
    pub fn p1dir3(&mut self) -> P1DIR3_W<P1DIR_SPEC> {
        P1DIR3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P1DIR4"]
    #[inline(always)]
    #[must_use]
    pub fn p1dir4(&mut self) -> P1DIR4_W<P1DIR_SPEC> {
        P1DIR4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P1DIR5"]
    #[inline(always)]
    #[must_use]
    pub fn p1dir5(&mut self) -> P1DIR5_W<P1DIR_SPEC> {
        P1DIR5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P1DIR6"]
    #[inline(always)]
    #[must_use]
    pub fn p1dir6(&mut self) -> P1DIR6_W<P1DIR_SPEC> {
        P1DIR6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P1DIR7"]
    #[inline(always)]
    #[must_use]
    pub fn p1dir7(&mut self) -> P1DIR7_W<P1DIR_SPEC> {
        P1DIR7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 1 Direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p1dir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p1dir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1DIR_SPEC;
impl crate::RegisterSpec for P1DIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1dir::R`](R) reader structure"]
impl crate::Readable for P1DIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p1dir::W`](W) writer structure"]
impl crate::Writable for P1DIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P1DIR to value 0"]
impl crate::Resettable for P1DIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
