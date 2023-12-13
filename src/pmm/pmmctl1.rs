#[doc = "Register `PMMCTL1` reader"]
pub type R = crate::R<PMMCTL1_SPEC>;
#[doc = "Register `PMMCTL1` writer"]
pub type W = crate::W<PMMCTL1_SPEC>;
#[doc = "Field `PMMREFMD` reader - PMM Reference Mode"]
pub type PMMREFMD_R = crate::BitReader;
#[doc = "Field `PMMREFMD` writer - PMM Reference Mode"]
pub type PMMREFMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMMCMD0` reader - PMM Voltage Regulator Current Mode Bit: 0"]
pub type PMMCMD0_R = crate::BitReader;
#[doc = "Field `PMMCMD0` writer - PMM Voltage Regulator Current Mode Bit: 0"]
pub type PMMCMD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMMCMD1` reader - PMM Voltage Regulator Current Mode Bit: 1"]
pub type PMMCMD1_R = crate::BitReader;
#[doc = "Field `PMMCMD1` writer - PMM Voltage Regulator Current Mode Bit: 1"]
pub type PMMCMD1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PMM Reference Mode"]
    #[inline(always)]
    pub fn pmmrefmd(&self) -> PMMREFMD_R {
        PMMREFMD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - PMM Voltage Regulator Current Mode Bit: 0"]
    #[inline(always)]
    pub fn pmmcmd0(&self) -> PMMCMD0_R {
        PMMCMD0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PMM Voltage Regulator Current Mode Bit: 1"]
    #[inline(always)]
    pub fn pmmcmd1(&self) -> PMMCMD1_R {
        PMMCMD1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PMM Reference Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pmmrefmd(&mut self) -> PMMREFMD_W<PMMCTL1_SPEC> {
        PMMREFMD_W::new(self, 0)
    }
    #[doc = "Bit 4 - PMM Voltage Regulator Current Mode Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn pmmcmd0(&mut self) -> PMMCMD0_W<PMMCTL1_SPEC> {
        PMMCMD0_W::new(self, 4)
    }
    #[doc = "Bit 5 - PMM Voltage Regulator Current Mode Bit: 1"]
    #[inline(always)]
    #[must_use]
    pub fn pmmcmd1(&mut self) -> PMMCMD1_W<PMMCTL1_SPEC> {
        PMMCMD1_W::new(self, 5)
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
#[doc = "PMM Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmmctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmmctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMMCTL1_SPEC;
impl crate::RegisterSpec for PMMCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pmmctl1::R`](R) reader structure"]
impl crate::Readable for PMMCTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmmctl1::W`](W) writer structure"]
impl crate::Writable for PMMCTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMMCTL1 to value 0"]
impl crate::Resettable for PMMCTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
