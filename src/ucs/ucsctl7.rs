#[doc = "Register `UCSCTL7` reader"]
pub type R = crate::R<UCSCTL7_SPEC>;
#[doc = "Register `UCSCTL7` writer"]
pub type W = crate::W<UCSCTL7_SPEC>;
#[doc = "Field `DCOFFG` reader - DCO Fault Flag"]
pub type DCOFFG_R = crate::BitReader;
#[doc = "Field `DCOFFG` writer - DCO Fault Flag"]
pub type DCOFFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XT1LFOFFG` reader - XT1 Low Frequency Oscillator Fault Flag"]
pub type XT1LFOFFG_R = crate::BitReader;
#[doc = "Field `XT1LFOFFG` writer - XT1 Low Frequency Oscillator Fault Flag"]
pub type XT1LFOFFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XT2OFFG` reader - High Frequency Oscillator 2 Fault Flag"]
pub type XT2OFFG_R = crate::BitReader;
#[doc = "Field `XT2OFFG` writer - High Frequency Oscillator 2 Fault Flag"]
pub type XT2OFFG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DCO Fault Flag"]
    #[inline(always)]
    pub fn dcoffg(&self) -> DCOFFG_R {
        DCOFFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XT1 Low Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn xt1lfoffg(&self) -> XT1LFOFFG_R {
        XT1LFOFFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - High Frequency Oscillator 2 Fault Flag"]
    #[inline(always)]
    pub fn xt2offg(&self) -> XT2OFFG_R {
        XT2OFFG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCO Fault Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcoffg(&mut self) -> DCOFFG_W<UCSCTL7_SPEC> {
        DCOFFG_W::new(self, 0)
    }
    #[doc = "Bit 1 - XT1 Low Frequency Oscillator Fault Flag"]
    #[inline(always)]
    #[must_use]
    pub fn xt1lfoffg(&mut self) -> XT1LFOFFG_W<UCSCTL7_SPEC> {
        XT1LFOFFG_W::new(self, 1)
    }
    #[doc = "Bit 3 - High Frequency Oscillator 2 Fault Flag"]
    #[inline(always)]
    #[must_use]
    pub fn xt2offg(&mut self) -> XT2OFFG_W<UCSCTL7_SPEC> {
        XT2OFFG_W::new(self, 3)
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
#[doc = "UCS Control Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsctl7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsctl7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSCTL7_SPEC;
impl crate::RegisterSpec for UCSCTL7_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucsctl7::R`](R) reader structure"]
impl crate::Readable for UCSCTL7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsctl7::W`](W) writer structure"]
impl crate::Writable for UCSCTL7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSCTL7 to value 0"]
impl crate::Resettable for UCSCTL7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
