#[doc = "Register `P1IES` reader"]
pub type R = crate::R<P1IES_SPEC>;
#[doc = "Register `P1IES` writer"]
pub type W = crate::W<P1IES_SPEC>;
#[doc = "Field `P1IES0` reader - P1IES0"]
pub type P1IES0_R = crate::BitReader;
#[doc = "Field `P1IES0` writer - P1IES0"]
pub type P1IES0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IES1` reader - P1IES1"]
pub type P1IES1_R = crate::BitReader;
#[doc = "Field `P1IES1` writer - P1IES1"]
pub type P1IES1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IES2` reader - P1IES2"]
pub type P1IES2_R = crate::BitReader;
#[doc = "Field `P1IES2` writer - P1IES2"]
pub type P1IES2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IES3` reader - P1IES3"]
pub type P1IES3_R = crate::BitReader;
#[doc = "Field `P1IES3` writer - P1IES3"]
pub type P1IES3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IES4` reader - P1IES4"]
pub type P1IES4_R = crate::BitReader;
#[doc = "Field `P1IES4` writer - P1IES4"]
pub type P1IES4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IES5` reader - P1IES5"]
pub type P1IES5_R = crate::BitReader;
#[doc = "Field `P1IES5` writer - P1IES5"]
pub type P1IES5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IES6` reader - P1IES6"]
pub type P1IES6_R = crate::BitReader;
#[doc = "Field `P1IES6` writer - P1IES6"]
pub type P1IES6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IES7` reader - P1IES7"]
pub type P1IES7_R = crate::BitReader;
#[doc = "Field `P1IES7` writer - P1IES7"]
pub type P1IES7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P1IES0"]
    #[inline(always)]
    pub fn p1ies0(&self) -> P1IES0_R {
        P1IES0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P1IES1"]
    #[inline(always)]
    pub fn p1ies1(&self) -> P1IES1_R {
        P1IES1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P1IES2"]
    #[inline(always)]
    pub fn p1ies2(&self) -> P1IES2_R {
        P1IES2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P1IES3"]
    #[inline(always)]
    pub fn p1ies3(&self) -> P1IES3_R {
        P1IES3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P1IES4"]
    #[inline(always)]
    pub fn p1ies4(&self) -> P1IES4_R {
        P1IES4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P1IES5"]
    #[inline(always)]
    pub fn p1ies5(&self) -> P1IES5_R {
        P1IES5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P1IES6"]
    #[inline(always)]
    pub fn p1ies6(&self) -> P1IES6_R {
        P1IES6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P1IES7"]
    #[inline(always)]
    pub fn p1ies7(&self) -> P1IES7_R {
        P1IES7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1IES0"]
    #[inline(always)]
    #[must_use]
    pub fn p1ies0(&mut self) -> P1IES0_W<P1IES_SPEC> {
        P1IES0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P1IES1"]
    #[inline(always)]
    #[must_use]
    pub fn p1ies1(&mut self) -> P1IES1_W<P1IES_SPEC> {
        P1IES1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P1IES2"]
    #[inline(always)]
    #[must_use]
    pub fn p1ies2(&mut self) -> P1IES2_W<P1IES_SPEC> {
        P1IES2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P1IES3"]
    #[inline(always)]
    #[must_use]
    pub fn p1ies3(&mut self) -> P1IES3_W<P1IES_SPEC> {
        P1IES3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P1IES4"]
    #[inline(always)]
    #[must_use]
    pub fn p1ies4(&mut self) -> P1IES4_W<P1IES_SPEC> {
        P1IES4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P1IES5"]
    #[inline(always)]
    #[must_use]
    pub fn p1ies5(&mut self) -> P1IES5_W<P1IES_SPEC> {
        P1IES5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P1IES6"]
    #[inline(always)]
    #[must_use]
    pub fn p1ies6(&mut self) -> P1IES6_W<P1IES_SPEC> {
        P1IES6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P1IES7"]
    #[inline(always)]
    #[must_use]
    pub fn p1ies7(&mut self) -> P1IES7_W<P1IES_SPEC> {
        P1IES7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 1 Interrupt Edge Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p1ies::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p1ies::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1IES_SPEC;
impl crate::RegisterSpec for P1IES_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1ies::R`](R) reader structure"]
impl crate::Readable for P1IES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p1ies::W`](W) writer structure"]
impl crate::Writable for P1IES_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P1IES to value 0"]
impl crate::Resettable for P1IES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
