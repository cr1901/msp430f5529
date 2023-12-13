#[doc = "Register `CBINT` reader"]
pub type R = crate::R<CBINT_SPEC>;
#[doc = "Register `CBINT` writer"]
pub type W = crate::W<CBINT_SPEC>;
#[doc = "Field `CBIFG` reader - Comp. B Interrupt Flag"]
pub type CBIFG_R = crate::BitReader;
#[doc = "Field `CBIFG` writer - Comp. B Interrupt Flag"]
pub type CBIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBIIFG` reader - Comp. B Interrupt Flag Inverted Polarity"]
pub type CBIIFG_R = crate::BitReader;
#[doc = "Field `CBIIFG` writer - Comp. B Interrupt Flag Inverted Polarity"]
pub type CBIIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBIE` reader - Comp. B Interrupt Enable"]
pub type CBIE_R = crate::BitReader;
#[doc = "Field `CBIE` writer - Comp. B Interrupt Enable"]
pub type CBIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBIIE` reader - Comp. B Interrupt Enable Inverted Polarity"]
pub type CBIIE_R = crate::BitReader;
#[doc = "Field `CBIIE` writer - Comp. B Interrupt Enable Inverted Polarity"]
pub type CBIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comp. B Interrupt Flag"]
    #[inline(always)]
    pub fn cbifg(&self) -> CBIFG_R {
        CBIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comp. B Interrupt Flag Inverted Polarity"]
    #[inline(always)]
    pub fn cbiifg(&self) -> CBIIFG_R {
        CBIIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Comp. B Interrupt Enable"]
    #[inline(always)]
    pub fn cbie(&self) -> CBIE_R {
        CBIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Comp. B Interrupt Enable Inverted Polarity"]
    #[inline(always)]
    pub fn cbiie(&self) -> CBIIE_R {
        CBIIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. B Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cbifg(&mut self) -> CBIFG_W<CBINT_SPEC> {
        CBIFG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Comp. B Interrupt Flag Inverted Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cbiifg(&mut self) -> CBIIFG_W<CBINT_SPEC> {
        CBIIFG_W::new(self, 1)
    }
    #[doc = "Bit 8 - Comp. B Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cbie(&mut self) -> CBIE_W<CBINT_SPEC> {
        CBIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Comp. B Interrupt Enable Inverted Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cbiie(&mut self) -> CBIIE_W<CBINT_SPEC> {
        CBIIE_W::new(self, 9)
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
#[doc = "Comparator B Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cbint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cbint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CBINT_SPEC;
impl crate::RegisterSpec for CBINT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cbint::R`](R) reader structure"]
impl crate::Readable for CBINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cbint::W`](W) writer structure"]
impl crate::Writable for CBINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CBINT to value 0"]
impl crate::Resettable for CBINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
