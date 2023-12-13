#[doc = "Register `ADC12IE` reader"]
pub type R = crate::R<ADC12IE_SPEC>;
#[doc = "Register `ADC12IE` writer"]
pub type W = crate::W<ADC12IE_SPEC>;
#[doc = "Field `ADC12IE0` reader - ADC12 Memory 0 Interrupt Enable"]
pub type ADC12IE0_R = crate::BitReader;
#[doc = "Field `ADC12IE0` writer - ADC12 Memory 0 Interrupt Enable"]
pub type ADC12IE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IE1` reader - ADC12 Memory 1 Interrupt Enable"]
pub type ADC12IE1_R = crate::BitReader;
#[doc = "Field `ADC12IE1` writer - ADC12 Memory 1 Interrupt Enable"]
pub type ADC12IE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IE2` reader - ADC12 Memory 2 Interrupt Enable"]
pub type ADC12IE2_R = crate::BitReader;
#[doc = "Field `ADC12IE2` writer - ADC12 Memory 2 Interrupt Enable"]
pub type ADC12IE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IE3` reader - ADC12 Memory 3 Interrupt Enable"]
pub type ADC12IE3_R = crate::BitReader;
#[doc = "Field `ADC12IE3` writer - ADC12 Memory 3 Interrupt Enable"]
pub type ADC12IE3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IE4` reader - ADC12 Memory 4 Interrupt Enable"]
pub type ADC12IE4_R = crate::BitReader;
#[doc = "Field `ADC12IE4` writer - ADC12 Memory 4 Interrupt Enable"]
pub type ADC12IE4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IE5` reader - ADC12 Memory 5 Interrupt Enable"]
pub type ADC12IE5_R = crate::BitReader;
#[doc = "Field `ADC12IE5` writer - ADC12 Memory 5 Interrupt Enable"]
pub type ADC12IE5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IE6` reader - ADC12 Memory 6 Interrupt Enable"]
pub type ADC12IE6_R = crate::BitReader;
#[doc = "Field `ADC12IE6` writer - ADC12 Memory 6 Interrupt Enable"]
pub type ADC12IE6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IE7` reader - ADC12 Memory 7 Interrupt Enable"]
pub type ADC12IE7_R = crate::BitReader;
#[doc = "Field `ADC12IE7` writer - ADC12 Memory 7 Interrupt Enable"]
pub type ADC12IE7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IE8` reader - ADC12 Memory 8 Interrupt Enable"]
pub type ADC12IE8_R = crate::BitReader;
#[doc = "Field `ADC12IE8` writer - ADC12 Memory 8 Interrupt Enable"]
pub type ADC12IE8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IE9` reader - ADC12 Memory 9 Interrupt Enable"]
pub type ADC12IE9_R = crate::BitReader;
#[doc = "Field `ADC12IE9` writer - ADC12 Memory 9 Interrupt Enable"]
pub type ADC12IE9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IE10` reader - ADC12 Memory 10 Interrupt Enable"]
pub type ADC12IE10_R = crate::BitReader;
#[doc = "Field `ADC12IE10` writer - ADC12 Memory 10 Interrupt Enable"]
pub type ADC12IE10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IE11` reader - ADC12 Memory 11 Interrupt Enable"]
pub type ADC12IE11_R = crate::BitReader;
#[doc = "Field `ADC12IE11` writer - ADC12 Memory 11 Interrupt Enable"]
pub type ADC12IE11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IE12` reader - ADC12 Memory 12 Interrupt Enable"]
pub type ADC12IE12_R = crate::BitReader;
#[doc = "Field `ADC12IE12` writer - ADC12 Memory 12 Interrupt Enable"]
pub type ADC12IE12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IE13` reader - ADC12 Memory 13 Interrupt Enable"]
pub type ADC12IE13_R = crate::BitReader;
#[doc = "Field `ADC12IE13` writer - ADC12 Memory 13 Interrupt Enable"]
pub type ADC12IE13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IE14` reader - ADC12 Memory 14 Interrupt Enable"]
pub type ADC12IE14_R = crate::BitReader;
#[doc = "Field `ADC12IE14` writer - ADC12 Memory 14 Interrupt Enable"]
pub type ADC12IE14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IE15` reader - ADC12 Memory 15 Interrupt Enable"]
pub type ADC12IE15_R = crate::BitReader;
#[doc = "Field `ADC12IE15` writer - ADC12 Memory 15 Interrupt Enable"]
pub type ADC12IE15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC12 Memory 0 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie0(&self) -> ADC12IE0_R {
        ADC12IE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC12 Memory 1 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie1(&self) -> ADC12IE1_R {
        ADC12IE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC12 Memory 2 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie2(&self) -> ADC12IE2_R {
        ADC12IE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC12 Memory 3 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie3(&self) -> ADC12IE3_R {
        ADC12IE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC12 Memory 4 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie4(&self) -> ADC12IE4_R {
        ADC12IE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC12 Memory 5 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie5(&self) -> ADC12IE5_R {
        ADC12IE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC12 Memory 6 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie6(&self) -> ADC12IE6_R {
        ADC12IE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC12 Memory 7 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie7(&self) -> ADC12IE7_R {
        ADC12IE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC12 Memory 8 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie8(&self) -> ADC12IE8_R {
        ADC12IE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC12 Memory 9 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie9(&self) -> ADC12IE9_R {
        ADC12IE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC12 Memory 10 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie10(&self) -> ADC12IE10_R {
        ADC12IE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ADC12 Memory 11 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie11(&self) -> ADC12IE11_R {
        ADC12IE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ADC12 Memory 12 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie12(&self) -> ADC12IE12_R {
        ADC12IE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC12 Memory 13 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie13(&self) -> ADC12IE13_R {
        ADC12IE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC12 Memory 14 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie14(&self) -> ADC12IE14_R {
        ADC12IE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC12 Memory 15 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie15(&self) -> ADC12IE15_R {
        ADC12IE15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12 Memory 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie0(&mut self) -> ADC12IE0_W<ADC12IE_SPEC> {
        ADC12IE0_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC12 Memory 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie1(&mut self) -> ADC12IE1_W<ADC12IE_SPEC> {
        ADC12IE1_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC12 Memory 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie2(&mut self) -> ADC12IE2_W<ADC12IE_SPEC> {
        ADC12IE2_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC12 Memory 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie3(&mut self) -> ADC12IE3_W<ADC12IE_SPEC> {
        ADC12IE3_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC12 Memory 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie4(&mut self) -> ADC12IE4_W<ADC12IE_SPEC> {
        ADC12IE4_W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC12 Memory 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie5(&mut self) -> ADC12IE5_W<ADC12IE_SPEC> {
        ADC12IE5_W::new(self, 5)
    }
    #[doc = "Bit 6 - ADC12 Memory 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie6(&mut self) -> ADC12IE6_W<ADC12IE_SPEC> {
        ADC12IE6_W::new(self, 6)
    }
    #[doc = "Bit 7 - ADC12 Memory 7 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie7(&mut self) -> ADC12IE7_W<ADC12IE_SPEC> {
        ADC12IE7_W::new(self, 7)
    }
    #[doc = "Bit 8 - ADC12 Memory 8 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie8(&mut self) -> ADC12IE8_W<ADC12IE_SPEC> {
        ADC12IE8_W::new(self, 8)
    }
    #[doc = "Bit 9 - ADC12 Memory 9 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie9(&mut self) -> ADC12IE9_W<ADC12IE_SPEC> {
        ADC12IE9_W::new(self, 9)
    }
    #[doc = "Bit 10 - ADC12 Memory 10 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie10(&mut self) -> ADC12IE10_W<ADC12IE_SPEC> {
        ADC12IE10_W::new(self, 10)
    }
    #[doc = "Bit 11 - ADC12 Memory 11 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie11(&mut self) -> ADC12IE11_W<ADC12IE_SPEC> {
        ADC12IE11_W::new(self, 11)
    }
    #[doc = "Bit 12 - ADC12 Memory 12 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie12(&mut self) -> ADC12IE12_W<ADC12IE_SPEC> {
        ADC12IE12_W::new(self, 12)
    }
    #[doc = "Bit 13 - ADC12 Memory 13 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie13(&mut self) -> ADC12IE13_W<ADC12IE_SPEC> {
        ADC12IE13_W::new(self, 13)
    }
    #[doc = "Bit 14 - ADC12 Memory 14 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie14(&mut self) -> ADC12IE14_W<ADC12IE_SPEC> {
        ADC12IE14_W::new(self, 14)
    }
    #[doc = "Bit 15 - ADC12 Memory 15 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie15(&mut self) -> ADC12IE15_W<ADC12IE_SPEC> {
        ADC12IE15_W::new(self, 15)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC12+ Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12ie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12ie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC12IE_SPEC;
impl crate::RegisterSpec for ADC12IE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc12ie::R`](R) reader structure"]
impl crate::Readable for ADC12IE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc12ie::W`](W) writer structure"]
impl crate::Writable for ADC12IE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC12IE to value 0"]
impl crate::Resettable for ADC12IE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
