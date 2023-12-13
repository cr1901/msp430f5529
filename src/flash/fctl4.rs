#[doc = "Register `FCTL4` reader"]
pub type R = crate::R<FCTL4_SPEC>;
#[doc = "Register `FCTL4` writer"]
pub type W = crate::W<FCTL4_SPEC>;
#[doc = "Field `VPE` reader - Voltage Changed during Program Error Flag"]
pub type VPE_R = crate::BitReader;
#[doc = "Field `VPE` writer - Voltage Changed during Program Error Flag"]
pub type VPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MGR0` reader - Marginal read 0 mode."]
pub type MGR0_R = crate::BitReader;
#[doc = "Field `MGR0` writer - Marginal read 0 mode."]
pub type MGR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MGR1` reader - Marginal read 1 mode."]
pub type MGR1_R = crate::BitReader;
#[doc = "Field `MGR1` writer - Marginal read 1 mode."]
pub type MGR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKINFO` reader - Lock INFO Memory bit: read = 1 - Segment is locked (read only)"]
pub type LOCKINFO_R = crate::BitReader;
#[doc = "Field `LOCKINFO` writer - Lock INFO Memory bit: read = 1 - Segment is locked (read only)"]
pub type LOCKINFO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Voltage Changed during Program Error Flag"]
    #[inline(always)]
    pub fn vpe(&self) -> VPE_R {
        VPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Marginal read 0 mode."]
    #[inline(always)]
    pub fn mgr0(&self) -> MGR0_R {
        MGR0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Marginal read 1 mode."]
    #[inline(always)]
    pub fn mgr1(&self) -> MGR1_R {
        MGR1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Lock INFO Memory bit: read = 1 - Segment is locked (read only)"]
    #[inline(always)]
    pub fn lockinfo(&self) -> LOCKINFO_R {
        LOCKINFO_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Changed during Program Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vpe(&mut self) -> VPE_W<FCTL4_SPEC> {
        VPE_W::new(self, 0)
    }
    #[doc = "Bit 4 - Marginal read 0 mode."]
    #[inline(always)]
    #[must_use]
    pub fn mgr0(&mut self) -> MGR0_W<FCTL4_SPEC> {
        MGR0_W::new(self, 4)
    }
    #[doc = "Bit 5 - Marginal read 1 mode."]
    #[inline(always)]
    #[must_use]
    pub fn mgr1(&mut self) -> MGR1_W<FCTL4_SPEC> {
        MGR1_W::new(self, 5)
    }
    #[doc = "Bit 7 - Lock INFO Memory bit: read = 1 - Segment is locked (read only)"]
    #[inline(always)]
    #[must_use]
    pub fn lockinfo(&mut self) -> LOCKINFO_W<FCTL4_SPEC> {
        LOCKINFO_W::new(self, 7)
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
#[doc = "FLASH Control 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctl4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctl4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCTL4_SPEC;
impl crate::RegisterSpec for FCTL4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fctl4::R`](R) reader structure"]
impl crate::Readable for FCTL4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fctl4::W`](W) writer structure"]
impl crate::Writable for FCTL4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCTL4 to value 0"]
impl crate::Resettable for FCTL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
