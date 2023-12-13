#[doc = "Register `P2IE` reader"]
pub type R = crate::R<P2IE_SPEC>;
#[doc = "Register `P2IE` writer"]
pub type W = crate::W<P2IE_SPEC>;
#[doc = "Field `P2IE0` reader - P2IE0"]
pub type P2IE0_R = crate::BitReader;
#[doc = "Field `P2IE0` writer - P2IE0"]
pub type P2IE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IE1` reader - P2IE1"]
pub type P2IE1_R = crate::BitReader;
#[doc = "Field `P2IE1` writer - P2IE1"]
pub type P2IE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IE2` reader - P2IE2"]
pub type P2IE2_R = crate::BitReader;
#[doc = "Field `P2IE2` writer - P2IE2"]
pub type P2IE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IE3` reader - P2IE3"]
pub type P2IE3_R = crate::BitReader;
#[doc = "Field `P2IE3` writer - P2IE3"]
pub type P2IE3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IE4` reader - P2IE4"]
pub type P2IE4_R = crate::BitReader;
#[doc = "Field `P2IE4` writer - P2IE4"]
pub type P2IE4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IE5` reader - P2IE5"]
pub type P2IE5_R = crate::BitReader;
#[doc = "Field `P2IE5` writer - P2IE5"]
pub type P2IE5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IE6` reader - P2IE6"]
pub type P2IE6_R = crate::BitReader;
#[doc = "Field `P2IE6` writer - P2IE6"]
pub type P2IE6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IE7` reader - P2IE7"]
pub type P2IE7_R = crate::BitReader;
#[doc = "Field `P2IE7` writer - P2IE7"]
pub type P2IE7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P2IE0"]
    #[inline(always)]
    pub fn p2ie0(&self) -> P2IE0_R {
        P2IE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P2IE1"]
    #[inline(always)]
    pub fn p2ie1(&self) -> P2IE1_R {
        P2IE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2IE2"]
    #[inline(always)]
    pub fn p2ie2(&self) -> P2IE2_R {
        P2IE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P2IE3"]
    #[inline(always)]
    pub fn p2ie3(&self) -> P2IE3_R {
        P2IE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P2IE4"]
    #[inline(always)]
    pub fn p2ie4(&self) -> P2IE4_R {
        P2IE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P2IE5"]
    #[inline(always)]
    pub fn p2ie5(&self) -> P2IE5_R {
        P2IE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P2IE6"]
    #[inline(always)]
    pub fn p2ie6(&self) -> P2IE6_R {
        P2IE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P2IE7"]
    #[inline(always)]
    pub fn p2ie7(&self) -> P2IE7_R {
        P2IE7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2IE0"]
    #[inline(always)]
    #[must_use]
    pub fn p2ie0(&mut self) -> P2IE0_W<P2IE_SPEC> {
        P2IE0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P2IE1"]
    #[inline(always)]
    #[must_use]
    pub fn p2ie1(&mut self) -> P2IE1_W<P2IE_SPEC> {
        P2IE1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P2IE2"]
    #[inline(always)]
    #[must_use]
    pub fn p2ie2(&mut self) -> P2IE2_W<P2IE_SPEC> {
        P2IE2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P2IE3"]
    #[inline(always)]
    #[must_use]
    pub fn p2ie3(&mut self) -> P2IE3_W<P2IE_SPEC> {
        P2IE3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P2IE4"]
    #[inline(always)]
    #[must_use]
    pub fn p2ie4(&mut self) -> P2IE4_W<P2IE_SPEC> {
        P2IE4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P2IE5"]
    #[inline(always)]
    #[must_use]
    pub fn p2ie5(&mut self) -> P2IE5_W<P2IE_SPEC> {
        P2IE5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P2IE6"]
    #[inline(always)]
    #[must_use]
    pub fn p2ie6(&mut self) -> P2IE6_W<P2IE_SPEC> {
        P2IE6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P2IE7"]
    #[inline(always)]
    #[must_use]
    pub fn p2ie7(&mut self) -> P2IE7_W<P2IE_SPEC> {
        P2IE7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 2 Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p2ie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p2ie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2IE_SPEC;
impl crate::RegisterSpec for P2IE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2ie::R`](R) reader structure"]
impl crate::Readable for P2IE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p2ie::W`](W) writer structure"]
impl crate::Writable for P2IE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P2IE to value 0"]
impl crate::Resettable for P2IE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
