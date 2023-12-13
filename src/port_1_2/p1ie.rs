#[doc = "Register `P1IE` reader"]
pub type R = crate::R<P1IE_SPEC>;
#[doc = "Register `P1IE` writer"]
pub type W = crate::W<P1IE_SPEC>;
#[doc = "Field `P1IE0` reader - P1IE0"]
pub type P1IE0_R = crate::BitReader;
#[doc = "Field `P1IE0` writer - P1IE0"]
pub type P1IE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IE1` reader - P1IE1"]
pub type P1IE1_R = crate::BitReader;
#[doc = "Field `P1IE1` writer - P1IE1"]
pub type P1IE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IE2` reader - P1IE2"]
pub type P1IE2_R = crate::BitReader;
#[doc = "Field `P1IE2` writer - P1IE2"]
pub type P1IE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IE3` reader - P1IE3"]
pub type P1IE3_R = crate::BitReader;
#[doc = "Field `P1IE3` writer - P1IE3"]
pub type P1IE3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IE4` reader - P1IE4"]
pub type P1IE4_R = crate::BitReader;
#[doc = "Field `P1IE4` writer - P1IE4"]
pub type P1IE4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IE5` reader - P1IE5"]
pub type P1IE5_R = crate::BitReader;
#[doc = "Field `P1IE5` writer - P1IE5"]
pub type P1IE5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IE6` reader - P1IE6"]
pub type P1IE6_R = crate::BitReader;
#[doc = "Field `P1IE6` writer - P1IE6"]
pub type P1IE6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IE7` reader - P1IE7"]
pub type P1IE7_R = crate::BitReader;
#[doc = "Field `P1IE7` writer - P1IE7"]
pub type P1IE7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P1IE0"]
    #[inline(always)]
    pub fn p1ie0(&self) -> P1IE0_R {
        P1IE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P1IE1"]
    #[inline(always)]
    pub fn p1ie1(&self) -> P1IE1_R {
        P1IE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P1IE2"]
    #[inline(always)]
    pub fn p1ie2(&self) -> P1IE2_R {
        P1IE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P1IE3"]
    #[inline(always)]
    pub fn p1ie3(&self) -> P1IE3_R {
        P1IE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P1IE4"]
    #[inline(always)]
    pub fn p1ie4(&self) -> P1IE4_R {
        P1IE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P1IE5"]
    #[inline(always)]
    pub fn p1ie5(&self) -> P1IE5_R {
        P1IE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P1IE6"]
    #[inline(always)]
    pub fn p1ie6(&self) -> P1IE6_R {
        P1IE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P1IE7"]
    #[inline(always)]
    pub fn p1ie7(&self) -> P1IE7_R {
        P1IE7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1IE0"]
    #[inline(always)]
    #[must_use]
    pub fn p1ie0(&mut self) -> P1IE0_W<P1IE_SPEC> {
        P1IE0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P1IE1"]
    #[inline(always)]
    #[must_use]
    pub fn p1ie1(&mut self) -> P1IE1_W<P1IE_SPEC> {
        P1IE1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P1IE2"]
    #[inline(always)]
    #[must_use]
    pub fn p1ie2(&mut self) -> P1IE2_W<P1IE_SPEC> {
        P1IE2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P1IE3"]
    #[inline(always)]
    #[must_use]
    pub fn p1ie3(&mut self) -> P1IE3_W<P1IE_SPEC> {
        P1IE3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P1IE4"]
    #[inline(always)]
    #[must_use]
    pub fn p1ie4(&mut self) -> P1IE4_W<P1IE_SPEC> {
        P1IE4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P1IE5"]
    #[inline(always)]
    #[must_use]
    pub fn p1ie5(&mut self) -> P1IE5_W<P1IE_SPEC> {
        P1IE5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P1IE6"]
    #[inline(always)]
    #[must_use]
    pub fn p1ie6(&mut self) -> P1IE6_W<P1IE_SPEC> {
        P1IE6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P1IE7"]
    #[inline(always)]
    #[must_use]
    pub fn p1ie7(&mut self) -> P1IE7_W<P1IE_SPEC> {
        P1IE7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 1 Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p1ie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p1ie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1IE_SPEC;
impl crate::RegisterSpec for P1IE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1ie::R`](R) reader structure"]
impl crate::Readable for P1IE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p1ie::W`](W) writer structure"]
impl crate::Writable for P1IE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P1IE to value 0"]
impl crate::Resettable for P1IE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
