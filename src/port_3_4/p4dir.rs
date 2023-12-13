#[doc = "Register `P4DIR` reader"]
pub type R = crate::R<P4DIR_SPEC>;
#[doc = "Register `P4DIR` writer"]
pub type W = crate::W<P4DIR_SPEC>;
#[doc = "Field `P4DIR0` reader - P4DIR0"]
pub type P4DIR0_R = crate::BitReader;
#[doc = "Field `P4DIR0` writer - P4DIR0"]
pub type P4DIR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4DIR1` reader - P4DIR1"]
pub type P4DIR1_R = crate::BitReader;
#[doc = "Field `P4DIR1` writer - P4DIR1"]
pub type P4DIR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4DIR2` reader - P4DIR2"]
pub type P4DIR2_R = crate::BitReader;
#[doc = "Field `P4DIR2` writer - P4DIR2"]
pub type P4DIR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4DIR3` reader - P4DIR3"]
pub type P4DIR3_R = crate::BitReader;
#[doc = "Field `P4DIR3` writer - P4DIR3"]
pub type P4DIR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4DIR4` reader - P4DIR4"]
pub type P4DIR4_R = crate::BitReader;
#[doc = "Field `P4DIR4` writer - P4DIR4"]
pub type P4DIR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4DIR5` reader - P4DIR5"]
pub type P4DIR5_R = crate::BitReader;
#[doc = "Field `P4DIR5` writer - P4DIR5"]
pub type P4DIR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4DIR6` reader - P4DIR6"]
pub type P4DIR6_R = crate::BitReader;
#[doc = "Field `P4DIR6` writer - P4DIR6"]
pub type P4DIR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4DIR7` reader - P4DIR7"]
pub type P4DIR7_R = crate::BitReader;
#[doc = "Field `P4DIR7` writer - P4DIR7"]
pub type P4DIR7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P4DIR0"]
    #[inline(always)]
    pub fn p4dir0(&self) -> P4DIR0_R {
        P4DIR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P4DIR1"]
    #[inline(always)]
    pub fn p4dir1(&self) -> P4DIR1_R {
        P4DIR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P4DIR2"]
    #[inline(always)]
    pub fn p4dir2(&self) -> P4DIR2_R {
        P4DIR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P4DIR3"]
    #[inline(always)]
    pub fn p4dir3(&self) -> P4DIR3_R {
        P4DIR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P4DIR4"]
    #[inline(always)]
    pub fn p4dir4(&self) -> P4DIR4_R {
        P4DIR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P4DIR5"]
    #[inline(always)]
    pub fn p4dir5(&self) -> P4DIR5_R {
        P4DIR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P4DIR6"]
    #[inline(always)]
    pub fn p4dir6(&self) -> P4DIR6_R {
        P4DIR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P4DIR7"]
    #[inline(always)]
    pub fn p4dir7(&self) -> P4DIR7_R {
        P4DIR7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4DIR0"]
    #[inline(always)]
    #[must_use]
    pub fn p4dir0(&mut self) -> P4DIR0_W<P4DIR_SPEC> {
        P4DIR0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P4DIR1"]
    #[inline(always)]
    #[must_use]
    pub fn p4dir1(&mut self) -> P4DIR1_W<P4DIR_SPEC> {
        P4DIR1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P4DIR2"]
    #[inline(always)]
    #[must_use]
    pub fn p4dir2(&mut self) -> P4DIR2_W<P4DIR_SPEC> {
        P4DIR2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P4DIR3"]
    #[inline(always)]
    #[must_use]
    pub fn p4dir3(&mut self) -> P4DIR3_W<P4DIR_SPEC> {
        P4DIR3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P4DIR4"]
    #[inline(always)]
    #[must_use]
    pub fn p4dir4(&mut self) -> P4DIR4_W<P4DIR_SPEC> {
        P4DIR4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P4DIR5"]
    #[inline(always)]
    #[must_use]
    pub fn p4dir5(&mut self) -> P4DIR5_W<P4DIR_SPEC> {
        P4DIR5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P4DIR6"]
    #[inline(always)]
    #[must_use]
    pub fn p4dir6(&mut self) -> P4DIR6_W<P4DIR_SPEC> {
        P4DIR6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P4DIR7"]
    #[inline(always)]
    #[must_use]
    pub fn p4dir7(&mut self) -> P4DIR7_W<P4DIR_SPEC> {
        P4DIR7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 4 Direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p4dir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p4dir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4DIR_SPEC;
impl crate::RegisterSpec for P4DIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4dir::R`](R) reader structure"]
impl crate::Readable for P4DIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p4dir::W`](W) writer structure"]
impl crate::Writable for P4DIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P4DIR to value 0"]
impl crate::Resettable for P4DIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
