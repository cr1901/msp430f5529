#[doc = "Register `UCA0ABCTL` reader"]
pub type R = crate::R<UCA0ABCTL_SPEC>;
#[doc = "Register `UCA0ABCTL` writer"]
pub type W = crate::W<UCA0ABCTL_SPEC>;
#[doc = "Field `UCABDEN` reader - Auto Baud Rate detect enable"]
pub type UCABDEN_R = crate::BitReader;
#[doc = "Field `UCABDEN` writer - Auto Baud Rate detect enable"]
pub type UCABDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBTOE` reader - Break Timeout error"]
pub type UCBTOE_R = crate::BitReader;
#[doc = "Field `UCBTOE` writer - Break Timeout error"]
pub type UCBTOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTOE` reader - Sync-Field Timeout error"]
pub type UCSTOE_R = crate::BitReader;
#[doc = "Field `UCSTOE` writer - Sync-Field Timeout error"]
pub type UCSTOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCDELIM` reader - Break Sync Delimiter 0"]
pub type UCDELIM_R = crate::FieldReader;
#[doc = "Field `UCDELIM` writer - Break Sync Delimiter 0"]
pub type UCDELIM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Auto Baud Rate detect enable"]
    #[inline(always)]
    pub fn ucabden(&self) -> UCABDEN_R {
        UCABDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Break Timeout error"]
    #[inline(always)]
    pub fn ucbtoe(&self) -> UCBTOE_R {
        UCBTOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sync-Field Timeout error"]
    #[inline(always)]
    pub fn ucstoe(&self) -> UCSTOE_R {
        UCSTOE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Break Sync Delimiter 0"]
    #[inline(always)]
    pub fn ucdelim(&self) -> UCDELIM_R {
        UCDELIM_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Auto Baud Rate detect enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucabden(&mut self) -> UCABDEN_W<UCA0ABCTL_SPEC> {
        UCABDEN_W::new(self, 0)
    }
    #[doc = "Bit 2 - Break Timeout error"]
    #[inline(always)]
    #[must_use]
    pub fn ucbtoe(&mut self) -> UCBTOE_W<UCA0ABCTL_SPEC> {
        UCBTOE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Sync-Field Timeout error"]
    #[inline(always)]
    #[must_use]
    pub fn ucstoe(&mut self) -> UCSTOE_W<UCA0ABCTL_SPEC> {
        UCSTOE_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Break Sync Delimiter 0"]
    #[inline(always)]
    #[must_use]
    pub fn ucdelim(&mut self) -> UCDELIM_W<UCA0ABCTL_SPEC> {
        UCDELIM_W::new(self, 4)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USCI A0 LIN Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0abctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0abctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA0ABCTL_SPEC;
impl crate::RegisterSpec for UCA0ABCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0abctl::R`](R) reader structure"]
impl crate::Readable for UCA0ABCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca0abctl::W`](W) writer structure"]
impl crate::Writable for UCA0ABCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA0ABCTL to value 0"]
impl crate::Resettable for UCA0ABCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
