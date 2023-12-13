#[doc = "Register `P8DIR` reader"]
pub type R = crate::R<P8DIR_SPEC>;
#[doc = "Register `P8DIR` writer"]
pub type W = crate::W<P8DIR_SPEC>;
#[doc = "Field `P8DIR0` reader - P8DIR0"]
pub type P8DIR0_R = crate::BitReader;
#[doc = "Field `P8DIR0` writer - P8DIR0"]
pub type P8DIR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8DIR1` reader - P8DIR1"]
pub type P8DIR1_R = crate::BitReader;
#[doc = "Field `P8DIR1` writer - P8DIR1"]
pub type P8DIR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8DIR2` reader - P8DIR2"]
pub type P8DIR2_R = crate::BitReader;
#[doc = "Field `P8DIR2` writer - P8DIR2"]
pub type P8DIR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8DIR3` reader - P8DIR3"]
pub type P8DIR3_R = crate::BitReader;
#[doc = "Field `P8DIR3` writer - P8DIR3"]
pub type P8DIR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8DIR4` reader - P8DIR4"]
pub type P8DIR4_R = crate::BitReader;
#[doc = "Field `P8DIR4` writer - P8DIR4"]
pub type P8DIR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8DIR5` reader - P8DIR5"]
pub type P8DIR5_R = crate::BitReader;
#[doc = "Field `P8DIR5` writer - P8DIR5"]
pub type P8DIR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8DIR6` reader - P8DIR6"]
pub type P8DIR6_R = crate::BitReader;
#[doc = "Field `P8DIR6` writer - P8DIR6"]
pub type P8DIR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8DIR7` reader - P8DIR7"]
pub type P8DIR7_R = crate::BitReader;
#[doc = "Field `P8DIR7` writer - P8DIR7"]
pub type P8DIR7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P8DIR0"]
    #[inline(always)]
    pub fn p8dir0(&self) -> P8DIR0_R {
        P8DIR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P8DIR1"]
    #[inline(always)]
    pub fn p8dir1(&self) -> P8DIR1_R {
        P8DIR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P8DIR2"]
    #[inline(always)]
    pub fn p8dir2(&self) -> P8DIR2_R {
        P8DIR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P8DIR3"]
    #[inline(always)]
    pub fn p8dir3(&self) -> P8DIR3_R {
        P8DIR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P8DIR4"]
    #[inline(always)]
    pub fn p8dir4(&self) -> P8DIR4_R {
        P8DIR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P8DIR5"]
    #[inline(always)]
    pub fn p8dir5(&self) -> P8DIR5_R {
        P8DIR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P8DIR6"]
    #[inline(always)]
    pub fn p8dir6(&self) -> P8DIR6_R {
        P8DIR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P8DIR7"]
    #[inline(always)]
    pub fn p8dir7(&self) -> P8DIR7_R {
        P8DIR7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P8DIR0"]
    #[inline(always)]
    #[must_use]
    pub fn p8dir0(&mut self) -> P8DIR0_W<P8DIR_SPEC> {
        P8DIR0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P8DIR1"]
    #[inline(always)]
    #[must_use]
    pub fn p8dir1(&mut self) -> P8DIR1_W<P8DIR_SPEC> {
        P8DIR1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P8DIR2"]
    #[inline(always)]
    #[must_use]
    pub fn p8dir2(&mut self) -> P8DIR2_W<P8DIR_SPEC> {
        P8DIR2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P8DIR3"]
    #[inline(always)]
    #[must_use]
    pub fn p8dir3(&mut self) -> P8DIR3_W<P8DIR_SPEC> {
        P8DIR3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P8DIR4"]
    #[inline(always)]
    #[must_use]
    pub fn p8dir4(&mut self) -> P8DIR4_W<P8DIR_SPEC> {
        P8DIR4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P8DIR5"]
    #[inline(always)]
    #[must_use]
    pub fn p8dir5(&mut self) -> P8DIR5_W<P8DIR_SPEC> {
        P8DIR5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P8DIR6"]
    #[inline(always)]
    #[must_use]
    pub fn p8dir6(&mut self) -> P8DIR6_W<P8DIR_SPEC> {
        P8DIR6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P8DIR7"]
    #[inline(always)]
    #[must_use]
    pub fn p8dir7(&mut self) -> P8DIR7_W<P8DIR_SPEC> {
        P8DIR7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 8 Direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p8dir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p8dir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P8DIR_SPEC;
impl crate::RegisterSpec for P8DIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p8dir::R`](R) reader structure"]
impl crate::Readable for P8DIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p8dir::W`](W) writer structure"]
impl crate::Writable for P8DIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P8DIR to value 0"]
impl crate::Resettable for P8DIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
