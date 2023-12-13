#[doc = "Register `SFRRPCR` reader"]
pub type R = crate::R<SFRRPCR_SPEC>;
#[doc = "Register `SFRRPCR` writer"]
pub type W = crate::W<SFRRPCR_SPEC>;
#[doc = "Field `SYSNMI` reader - NMI select"]
pub type SYSNMI_R = crate::BitReader;
#[doc = "Field `SYSNMI` writer - NMI select"]
pub type SYSNMI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSNMIIES` reader - NMI edge select"]
pub type SYSNMIIES_R = crate::BitReader;
#[doc = "Field `SYSNMIIES` writer - NMI edge select"]
pub type SYSNMIIES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSRSTUP` reader - RESET Pin pull down/up select"]
pub type SYSRSTUP_R = crate::BitReader;
#[doc = "Field `SYSRSTUP` writer - RESET Pin pull down/up select"]
pub type SYSRSTUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSRSTRE` reader - RESET Pin Resistor enable"]
pub type SYSRSTRE_R = crate::BitReader;
#[doc = "Field `SYSRSTRE` writer - RESET Pin Resistor enable"]
pub type SYSRSTRE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NMI select"]
    #[inline(always)]
    pub fn sysnmi(&self) -> SYSNMI_R {
        SYSNMI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NMI edge select"]
    #[inline(always)]
    pub fn sysnmiies(&self) -> SYSNMIIES_R {
        SYSNMIIES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RESET Pin pull down/up select"]
    #[inline(always)]
    pub fn sysrstup(&self) -> SYSRSTUP_R {
        SYSRSTUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RESET Pin Resistor enable"]
    #[inline(always)]
    pub fn sysrstre(&self) -> SYSRSTRE_R {
        SYSRSTRE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NMI select"]
    #[inline(always)]
    #[must_use]
    pub fn sysnmi(&mut self) -> SYSNMI_W<SFRRPCR_SPEC> {
        SYSNMI_W::new(self, 0)
    }
    #[doc = "Bit 1 - NMI edge select"]
    #[inline(always)]
    #[must_use]
    pub fn sysnmiies(&mut self) -> SYSNMIIES_W<SFRRPCR_SPEC> {
        SYSNMIIES_W::new(self, 1)
    }
    #[doc = "Bit 2 - RESET Pin pull down/up select"]
    #[inline(always)]
    #[must_use]
    pub fn sysrstup(&mut self) -> SYSRSTUP_W<SFRRPCR_SPEC> {
        SYSRSTUP_W::new(self, 2)
    }
    #[doc = "Bit 3 - RESET Pin Resistor enable"]
    #[inline(always)]
    #[must_use]
    pub fn sysrstre(&mut self) -> SYSRSTRE_W<SFRRPCR_SPEC> {
        SYSRSTRE_W::new(self, 3)
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
#[doc = "RESET Pin Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfrrpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfrrpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SFRRPCR_SPEC;
impl crate::RegisterSpec for SFRRPCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sfrrpcr::R`](R) reader structure"]
impl crate::Readable for SFRRPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sfrrpcr::W`](W) writer structure"]
impl crate::Writable for SFRRPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFRRPCR to value 0"]
impl crate::Resettable for SFRRPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
