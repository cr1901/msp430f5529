#[doc = "Register `ADC12IFG` reader"]
pub type R = crate::R<ADC12IFG_SPEC>;
#[doc = "Register `ADC12IFG` writer"]
pub type W = crate::W<ADC12IFG_SPEC>;
#[doc = "Field `ADC12IFG0` reader - ADC12 Memory 0 Interrupt Flag"]
pub type ADC12IFG0_R = crate::BitReader;
#[doc = "Field `ADC12IFG0` writer - ADC12 Memory 0 Interrupt Flag"]
pub type ADC12IFG0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IFG1` reader - ADC12 Memory 1 Interrupt Flag"]
pub type ADC12IFG1_R = crate::BitReader;
#[doc = "Field `ADC12IFG1` writer - ADC12 Memory 1 Interrupt Flag"]
pub type ADC12IFG1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IFG2` reader - ADC12 Memory 2 Interrupt Flag"]
pub type ADC12IFG2_R = crate::BitReader;
#[doc = "Field `ADC12IFG2` writer - ADC12 Memory 2 Interrupt Flag"]
pub type ADC12IFG2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IFG3` reader - ADC12 Memory 3 Interrupt Flag"]
pub type ADC12IFG3_R = crate::BitReader;
#[doc = "Field `ADC12IFG3` writer - ADC12 Memory 3 Interrupt Flag"]
pub type ADC12IFG3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IFG4` reader - ADC12 Memory 4 Interrupt Flag"]
pub type ADC12IFG4_R = crate::BitReader;
#[doc = "Field `ADC12IFG4` writer - ADC12 Memory 4 Interrupt Flag"]
pub type ADC12IFG4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IFG5` reader - ADC12 Memory 5 Interrupt Flag"]
pub type ADC12IFG5_R = crate::BitReader;
#[doc = "Field `ADC12IFG5` writer - ADC12 Memory 5 Interrupt Flag"]
pub type ADC12IFG5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IFG6` reader - ADC12 Memory 6 Interrupt Flag"]
pub type ADC12IFG6_R = crate::BitReader;
#[doc = "Field `ADC12IFG6` writer - ADC12 Memory 6 Interrupt Flag"]
pub type ADC12IFG6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IFG7` reader - ADC12 Memory 7 Interrupt Flag"]
pub type ADC12IFG7_R = crate::BitReader;
#[doc = "Field `ADC12IFG7` writer - ADC12 Memory 7 Interrupt Flag"]
pub type ADC12IFG7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IFG8` reader - ADC12 Memory 8 Interrupt Flag"]
pub type ADC12IFG8_R = crate::BitReader;
#[doc = "Field `ADC12IFG8` writer - ADC12 Memory 8 Interrupt Flag"]
pub type ADC12IFG8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IFG9` reader - ADC12 Memory 9 Interrupt Flag"]
pub type ADC12IFG9_R = crate::BitReader;
#[doc = "Field `ADC12IFG9` writer - ADC12 Memory 9 Interrupt Flag"]
pub type ADC12IFG9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IFG10` reader - ADC12 Memory 10 Interrupt Flag"]
pub type ADC12IFG10_R = crate::BitReader;
#[doc = "Field `ADC12IFG10` writer - ADC12 Memory 10 Interrupt Flag"]
pub type ADC12IFG10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IFG11` reader - ADC12 Memory 11 Interrupt Flag"]
pub type ADC12IFG11_R = crate::BitReader;
#[doc = "Field `ADC12IFG11` writer - ADC12 Memory 11 Interrupt Flag"]
pub type ADC12IFG11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IFG12` reader - ADC12 Memory 12 Interrupt Flag"]
pub type ADC12IFG12_R = crate::BitReader;
#[doc = "Field `ADC12IFG12` writer - ADC12 Memory 12 Interrupt Flag"]
pub type ADC12IFG12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IFG13` reader - ADC12 Memory 13 Interrupt Flag"]
pub type ADC12IFG13_R = crate::BitReader;
#[doc = "Field `ADC12IFG13` writer - ADC12 Memory 13 Interrupt Flag"]
pub type ADC12IFG13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IFG14` reader - ADC12 Memory 14 Interrupt Flag"]
pub type ADC12IFG14_R = crate::BitReader;
#[doc = "Field `ADC12IFG14` writer - ADC12 Memory 14 Interrupt Flag"]
pub type ADC12IFG14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12IFG15` reader - ADC12 Memory 15 Interrupt Flag"]
pub type ADC12IFG15_R = crate::BitReader;
#[doc = "Field `ADC12IFG15` writer - ADC12 Memory 15 Interrupt Flag"]
pub type ADC12IFG15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC12 Memory 0 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg0(&self) -> ADC12IFG0_R {
        ADC12IFG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC12 Memory 1 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg1(&self) -> ADC12IFG1_R {
        ADC12IFG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC12 Memory 2 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg2(&self) -> ADC12IFG2_R {
        ADC12IFG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC12 Memory 3 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg3(&self) -> ADC12IFG3_R {
        ADC12IFG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC12 Memory 4 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg4(&self) -> ADC12IFG4_R {
        ADC12IFG4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC12 Memory 5 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg5(&self) -> ADC12IFG5_R {
        ADC12IFG5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC12 Memory 6 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg6(&self) -> ADC12IFG6_R {
        ADC12IFG6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC12 Memory 7 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg7(&self) -> ADC12IFG7_R {
        ADC12IFG7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC12 Memory 8 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg8(&self) -> ADC12IFG8_R {
        ADC12IFG8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC12 Memory 9 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg9(&self) -> ADC12IFG9_R {
        ADC12IFG9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC12 Memory 10 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg10(&self) -> ADC12IFG10_R {
        ADC12IFG10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ADC12 Memory 11 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg11(&self) -> ADC12IFG11_R {
        ADC12IFG11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ADC12 Memory 12 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg12(&self) -> ADC12IFG12_R {
        ADC12IFG12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC12 Memory 13 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg13(&self) -> ADC12IFG13_R {
        ADC12IFG13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC12 Memory 14 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg14(&self) -> ADC12IFG14_R {
        ADC12IFG14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC12 Memory 15 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg15(&self) -> ADC12IFG15_R {
        ADC12IFG15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12 Memory 0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg0(&mut self) -> ADC12IFG0_W<ADC12IFG_SPEC> {
        ADC12IFG0_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC12 Memory 1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg1(&mut self) -> ADC12IFG1_W<ADC12IFG_SPEC> {
        ADC12IFG1_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC12 Memory 2 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg2(&mut self) -> ADC12IFG2_W<ADC12IFG_SPEC> {
        ADC12IFG2_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC12 Memory 3 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg3(&mut self) -> ADC12IFG3_W<ADC12IFG_SPEC> {
        ADC12IFG3_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC12 Memory 4 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg4(&mut self) -> ADC12IFG4_W<ADC12IFG_SPEC> {
        ADC12IFG4_W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC12 Memory 5 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg5(&mut self) -> ADC12IFG5_W<ADC12IFG_SPEC> {
        ADC12IFG5_W::new(self, 5)
    }
    #[doc = "Bit 6 - ADC12 Memory 6 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg6(&mut self) -> ADC12IFG6_W<ADC12IFG_SPEC> {
        ADC12IFG6_W::new(self, 6)
    }
    #[doc = "Bit 7 - ADC12 Memory 7 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg7(&mut self) -> ADC12IFG7_W<ADC12IFG_SPEC> {
        ADC12IFG7_W::new(self, 7)
    }
    #[doc = "Bit 8 - ADC12 Memory 8 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg8(&mut self) -> ADC12IFG8_W<ADC12IFG_SPEC> {
        ADC12IFG8_W::new(self, 8)
    }
    #[doc = "Bit 9 - ADC12 Memory 9 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg9(&mut self) -> ADC12IFG9_W<ADC12IFG_SPEC> {
        ADC12IFG9_W::new(self, 9)
    }
    #[doc = "Bit 10 - ADC12 Memory 10 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg10(&mut self) -> ADC12IFG10_W<ADC12IFG_SPEC> {
        ADC12IFG10_W::new(self, 10)
    }
    #[doc = "Bit 11 - ADC12 Memory 11 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg11(&mut self) -> ADC12IFG11_W<ADC12IFG_SPEC> {
        ADC12IFG11_W::new(self, 11)
    }
    #[doc = "Bit 12 - ADC12 Memory 12 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg12(&mut self) -> ADC12IFG12_W<ADC12IFG_SPEC> {
        ADC12IFG12_W::new(self, 12)
    }
    #[doc = "Bit 13 - ADC12 Memory 13 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg13(&mut self) -> ADC12IFG13_W<ADC12IFG_SPEC> {
        ADC12IFG13_W::new(self, 13)
    }
    #[doc = "Bit 14 - ADC12 Memory 14 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg14(&mut self) -> ADC12IFG14_W<ADC12IFG_SPEC> {
        ADC12IFG14_W::new(self, 14)
    }
    #[doc = "Bit 15 - ADC12 Memory 15 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg15(&mut self) -> ADC12IFG15_W<ADC12IFG_SPEC> {
        ADC12IFG15_W::new(self, 15)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC12+ Interrupt Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12ifg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12ifg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC12IFG_SPEC;
impl crate::RegisterSpec for ADC12IFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc12ifg::R`](R) reader structure"]
impl crate::Readable for ADC12IFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc12ifg::W`](W) writer structure"]
impl crate::Writable for ADC12IFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC12IFG to value 0"]
impl crate::Resettable for ADC12IFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
