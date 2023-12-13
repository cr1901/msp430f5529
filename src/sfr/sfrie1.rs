#[doc = "Register `SFRIE1` reader"]
pub type R = crate::R<SFRIE1_SPEC>;
#[doc = "Register `SFRIE1` writer"]
pub type W = crate::W<SFRIE1_SPEC>;
#[doc = "Field `WDTIE` reader - WDT Interrupt Enable"]
pub type WDTIE_R = crate::BitReader;
#[doc = "Field `WDTIE` writer - WDT Interrupt Enable"]
pub type WDTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFIE` reader - Osc Fault Enable"]
pub type OFIE_R = crate::BitReader;
#[doc = "Field `OFIE` writer - Osc Fault Enable"]
pub type OFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMAIE` reader - Vacant Memory Interrupt Enable"]
pub type VMAIE_R = crate::BitReader;
#[doc = "Field `VMAIE` writer - Vacant Memory Interrupt Enable"]
pub type VMAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMIIE` reader - NMI Interrupt Enable"]
pub type NMIIE_R = crate::BitReader;
#[doc = "Field `NMIIE` writer - NMI Interrupt Enable"]
pub type NMIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCVIE` reader - Flash Access Violation Interrupt Enable"]
pub type ACCVIE_R = crate::BitReader;
#[doc = "Field `ACCVIE` writer - Flash Access Violation Interrupt Enable"]
pub type ACCVIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JMBINIE` reader - JTAG Mail Box input Interrupt Enable"]
pub type JMBINIE_R = crate::BitReader;
#[doc = "Field `JMBINIE` writer - JTAG Mail Box input Interrupt Enable"]
pub type JMBINIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JMBOUTIE` reader - JTAG Mail Box output Interrupt Enable"]
pub type JMBOUTIE_R = crate::BitReader;
#[doc = "Field `JMBOUTIE` writer - JTAG Mail Box output Interrupt Enable"]
pub type JMBOUTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WDT Interrupt Enable"]
    #[inline(always)]
    pub fn wdtie(&self) -> WDTIE_R {
        WDTIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Osc Fault Enable"]
    #[inline(always)]
    pub fn ofie(&self) -> OFIE_R {
        OFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Vacant Memory Interrupt Enable"]
    #[inline(always)]
    pub fn vmaie(&self) -> VMAIE_R {
        VMAIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NMI Interrupt Enable"]
    #[inline(always)]
    pub fn nmiie(&self) -> NMIIE_R {
        NMIIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash Access Violation Interrupt Enable"]
    #[inline(always)]
    pub fn accvie(&self) -> ACCVIE_R {
        ACCVIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - JTAG Mail Box input Interrupt Enable"]
    #[inline(always)]
    pub fn jmbinie(&self) -> JMBINIE_R {
        JMBINIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - JTAG Mail Box output Interrupt Enable"]
    #[inline(always)]
    pub fn jmboutie(&self) -> JMBOUTIE_R {
        JMBOUTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdtie(&mut self) -> WDTIE_W<SFRIE1_SPEC> {
        WDTIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Osc Fault Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ofie(&mut self) -> OFIE_W<SFRIE1_SPEC> {
        OFIE_W::new(self, 1)
    }
    #[doc = "Bit 3 - Vacant Memory Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmaie(&mut self) -> VMAIE_W<SFRIE1_SPEC> {
        VMAIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - NMI Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nmiie(&mut self) -> NMIIE_W<SFRIE1_SPEC> {
        NMIIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Flash Access Violation Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn accvie(&mut self) -> ACCVIE_W<SFRIE1_SPEC> {
        ACCVIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - JTAG Mail Box input Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn jmbinie(&mut self) -> JMBINIE_W<SFRIE1_SPEC> {
        JMBINIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - JTAG Mail Box output Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn jmboutie(&mut self) -> JMBOUTIE_W<SFRIE1_SPEC> {
        JMBOUTIE_W::new(self, 7)
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
#[doc = "Interrupt Enable 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfrie1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfrie1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SFRIE1_SPEC;
impl crate::RegisterSpec for SFRIE1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sfrie1::R`](R) reader structure"]
impl crate::Readable for SFRIE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sfrie1::W`](W) writer structure"]
impl crate::Writable for SFRIE1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFRIE1 to value 0"]
impl crate::Resettable for SFRIE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
