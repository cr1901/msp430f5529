#[doc = "Register `UCSCTL8` reader"]
pub type R = crate::R<UCSCTL8_SPEC>;
#[doc = "Register `UCSCTL8` writer"]
pub type W = crate::W<UCSCTL8_SPEC>;
#[doc = "Field `ACLKREQEN` reader - ACLK Clock Request Enable"]
pub type ACLKREQEN_R = crate::BitReader;
#[doc = "Field `ACLKREQEN` writer - ACLK Clock Request Enable"]
pub type ACLKREQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCLKREQEN` reader - MCLK Clock Request Enable"]
pub type MCLKREQEN_R = crate::BitReader;
#[doc = "Field `MCLKREQEN` writer - MCLK Clock Request Enable"]
pub type MCLKREQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMCLKREQEN` reader - SMCLK Clock Request Enable"]
pub type SMCLKREQEN_R = crate::BitReader;
#[doc = "Field `SMCLKREQEN` writer - SMCLK Clock Request Enable"]
pub type SMCLKREQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODOSCREQEN` reader - MODOSC Clock Request Enable"]
pub type MODOSCREQEN_R = crate::BitReader;
#[doc = "Field `MODOSCREQEN` writer - MODOSC Clock Request Enable"]
pub type MODOSCREQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ACLK Clock Request Enable"]
    #[inline(always)]
    pub fn aclkreqen(&self) -> ACLKREQEN_R {
        ACLKREQEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCLK Clock Request Enable"]
    #[inline(always)]
    pub fn mclkreqen(&self) -> MCLKREQEN_R {
        MCLKREQEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMCLK Clock Request Enable"]
    #[inline(always)]
    pub fn smclkreqen(&self) -> SMCLKREQEN_R {
        SMCLKREQEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MODOSC Clock Request Enable"]
    #[inline(always)]
    pub fn modoscreqen(&self) -> MODOSCREQEN_R {
        MODOSCREQEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACLK Clock Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aclkreqen(&mut self) -> ACLKREQEN_W<UCSCTL8_SPEC> {
        ACLKREQEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - MCLK Clock Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mclkreqen(&mut self) -> MCLKREQEN_W<UCSCTL8_SPEC> {
        MCLKREQEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - SMCLK Clock Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smclkreqen(&mut self) -> SMCLKREQEN_W<UCSCTL8_SPEC> {
        SMCLKREQEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - MODOSC Clock Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn modoscreqen(&mut self) -> MODOSCREQEN_W<UCSCTL8_SPEC> {
        MODOSCREQEN_W::new(self, 3)
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
#[doc = "UCS Control Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsctl8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsctl8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSCTL8_SPEC;
impl crate::RegisterSpec for UCSCTL8_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucsctl8::R`](R) reader structure"]
impl crate::Readable for UCSCTL8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsctl8::W`](W) writer structure"]
impl crate::Writable for UCSCTL8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSCTL8 to value 0"]
impl crate::Resettable for UCSCTL8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
