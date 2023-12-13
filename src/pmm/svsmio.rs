#[doc = "Register `SVSMIO` reader"]
pub type R = crate::R<SVSMIO_SPEC>;
#[doc = "Register `SVSMIO` writer"]
pub type W = crate::W<SVSMIO_SPEC>;
#[doc = "Field `SVMLOE` reader - SVM low side output enable"]
pub type SVMLOE_R = crate::BitReader;
#[doc = "Field `SVMLOE` writer - SVM low side output enable"]
pub type SVMLOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVMLVLROE` reader - SVM low side voltage level reached output enable"]
pub type SVMLVLROE_R = crate::BitReader;
#[doc = "Field `SVMLVLROE` writer - SVM low side voltage level reached output enable"]
pub type SVMLVLROE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVMOUTPOL` reader - SVMOUT pin polarity"]
pub type SVMOUTPOL_R = crate::BitReader;
#[doc = "Field `SVMOUTPOL` writer - SVMOUT pin polarity"]
pub type SVMOUTPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVMHOE` reader - SVM high side output enable"]
pub type SVMHOE_R = crate::BitReader;
#[doc = "Field `SVMHOE` writer - SVM high side output enable"]
pub type SVMHOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVMHVLROE` reader - SVM high side voltage level reached output enable"]
pub type SVMHVLROE_R = crate::BitReader;
#[doc = "Field `SVMHVLROE` writer - SVM high side voltage level reached output enable"]
pub type SVMHVLROE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - SVM low side output enable"]
    #[inline(always)]
    pub fn svmloe(&self) -> SVMLOE_R {
        SVMLOE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SVM low side voltage level reached output enable"]
    #[inline(always)]
    pub fn svmlvlroe(&self) -> SVMLVLROE_R {
        SVMLVLROE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SVMOUT pin polarity"]
    #[inline(always)]
    pub fn svmoutpol(&self) -> SVMOUTPOL_R {
        SVMOUTPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - SVM high side output enable"]
    #[inline(always)]
    pub fn svmhoe(&self) -> SVMHOE_R {
        SVMHOE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SVM high side voltage level reached output enable"]
    #[inline(always)]
    pub fn svmhvlroe(&self) -> SVMHVLROE_R {
        SVMHVLROE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - SVM low side output enable"]
    #[inline(always)]
    #[must_use]
    pub fn svmloe(&mut self) -> SVMLOE_W<SVSMIO_SPEC> {
        SVMLOE_W::new(self, 3)
    }
    #[doc = "Bit 4 - SVM low side voltage level reached output enable"]
    #[inline(always)]
    #[must_use]
    pub fn svmlvlroe(&mut self) -> SVMLVLROE_W<SVSMIO_SPEC> {
        SVMLVLROE_W::new(self, 4)
    }
    #[doc = "Bit 5 - SVMOUT pin polarity"]
    #[inline(always)]
    #[must_use]
    pub fn svmoutpol(&mut self) -> SVMOUTPOL_W<SVSMIO_SPEC> {
        SVMOUTPOL_W::new(self, 5)
    }
    #[doc = "Bit 11 - SVM high side output enable"]
    #[inline(always)]
    #[must_use]
    pub fn svmhoe(&mut self) -> SVMHOE_W<SVSMIO_SPEC> {
        SVMHOE_W::new(self, 11)
    }
    #[doc = "Bit 12 - SVM high side voltage level reached output enable"]
    #[inline(always)]
    #[must_use]
    pub fn svmhvlroe(&mut self) -> SVMHVLROE_W<SVSMIO_SPEC> {
        SVMHVLROE_W::new(self, 12)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SVSIN and SVSOUT control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`svsmio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`svsmio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SVSMIO_SPEC;
impl crate::RegisterSpec for SVSMIO_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`svsmio::R`](R) reader structure"]
impl crate::Readable for SVSMIO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`svsmio::W`](W) writer structure"]
impl crate::Writable for SVSMIO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SVSMIO to value 0"]
impl crate::Resettable for SVSMIO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
