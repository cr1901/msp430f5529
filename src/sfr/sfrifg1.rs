#[doc = "Register `SFRIFG1` reader"]
pub type R = crate::R<SFRIFG1_SPEC>;
#[doc = "Register `SFRIFG1` writer"]
pub type W = crate::W<SFRIFG1_SPEC>;
#[doc = "Field `WDTIFG` reader - WDT Interrupt Flag"]
pub type WDTIFG_R = crate::BitReader;
#[doc = "Field `WDTIFG` writer - WDT Interrupt Flag"]
pub type WDTIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFIFG` reader - Osc Fault Flag"]
pub type OFIFG_R = crate::BitReader;
#[doc = "Field `OFIFG` writer - Osc Fault Flag"]
pub type OFIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMAIFG` reader - Vacant Memory Interrupt Flag"]
pub type VMAIFG_R = crate::BitReader;
#[doc = "Field `VMAIFG` writer - Vacant Memory Interrupt Flag"]
pub type VMAIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMIIFG` reader - NMI Interrupt Flag"]
pub type NMIIFG_R = crate::BitReader;
#[doc = "Field `NMIIFG` writer - NMI Interrupt Flag"]
pub type NMIIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JMBINIFG` reader - JTAG Mail Box input Interrupt Flag"]
pub type JMBINIFG_R = crate::BitReader;
#[doc = "Field `JMBINIFG` writer - JTAG Mail Box input Interrupt Flag"]
pub type JMBINIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JMBOUTIFG` reader - JTAG Mail Box output Interrupt Flag"]
pub type JMBOUTIFG_R = crate::BitReader;
#[doc = "Field `JMBOUTIFG` writer - JTAG Mail Box output Interrupt Flag"]
pub type JMBOUTIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WDT Interrupt Flag"]
    #[inline(always)]
    pub fn wdtifg(&self) -> WDTIFG_R {
        WDTIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Osc Fault Flag"]
    #[inline(always)]
    pub fn ofifg(&self) -> OFIFG_R {
        OFIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Vacant Memory Interrupt Flag"]
    #[inline(always)]
    pub fn vmaifg(&self) -> VMAIFG_R {
        VMAIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NMI Interrupt Flag"]
    #[inline(always)]
    pub fn nmiifg(&self) -> NMIIFG_R {
        NMIIFG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - JTAG Mail Box input Interrupt Flag"]
    #[inline(always)]
    pub fn jmbinifg(&self) -> JMBINIFG_R {
        JMBINIFG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - JTAG Mail Box output Interrupt Flag"]
    #[inline(always)]
    pub fn jmboutifg(&self) -> JMBOUTIFG_R {
        JMBOUTIFG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wdtifg(&mut self) -> WDTIFG_W<SFRIFG1_SPEC> {
        WDTIFG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Osc Fault Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ofifg(&mut self) -> OFIFG_W<SFRIFG1_SPEC> {
        OFIFG_W::new(self, 1)
    }
    #[doc = "Bit 3 - Vacant Memory Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmaifg(&mut self) -> VMAIFG_W<SFRIFG1_SPEC> {
        VMAIFG_W::new(self, 3)
    }
    #[doc = "Bit 4 - NMI Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn nmiifg(&mut self) -> NMIIFG_W<SFRIFG1_SPEC> {
        NMIIFG_W::new(self, 4)
    }
    #[doc = "Bit 6 - JTAG Mail Box input Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn jmbinifg(&mut self) -> JMBINIFG_W<SFRIFG1_SPEC> {
        JMBINIFG_W::new(self, 6)
    }
    #[doc = "Bit 7 - JTAG Mail Box output Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn jmboutifg(&mut self) -> JMBOUTIFG_W<SFRIFG1_SPEC> {
        JMBOUTIFG_W::new(self, 7)
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
#[doc = "Interrupt Flag 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfrifg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfrifg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SFRIFG1_SPEC;
impl crate::RegisterSpec for SFRIFG1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sfrifg1::R`](R) reader structure"]
impl crate::Readable for SFRIFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sfrifg1::W`](W) writer structure"]
impl crate::Writable for SFRIFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFRIFG1 to value 0"]
impl crate::Resettable for SFRIFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
