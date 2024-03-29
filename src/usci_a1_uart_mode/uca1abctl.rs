#[doc = "Register `UCA1ABCTL` reader"]
pub type R = crate::R<UCA1ABCTL_SPEC>;
#[doc = "Register `UCA1ABCTL` writer"]
pub type W = crate::W<UCA1ABCTL_SPEC>;
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
#[doc = "Field `UCDELIM0` reader - Break Sync Delimiter 0"]
pub type UCDELIM0_R = crate::BitReader;
#[doc = "Field `UCDELIM0` writer - Break Sync Delimiter 0"]
pub type UCDELIM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCDELIM1` reader - Break Sync Delimiter 1"]
pub type UCDELIM1_R = crate::BitReader;
#[doc = "Field `UCDELIM1` writer - Break Sync Delimiter 1"]
pub type UCDELIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 4 - Break Sync Delimiter 0"]
    #[inline(always)]
    pub fn ucdelim0(&self) -> UCDELIM0_R {
        UCDELIM0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Break Sync Delimiter 1"]
    #[inline(always)]
    pub fn ucdelim1(&self) -> UCDELIM1_R {
        UCDELIM1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Auto Baud Rate detect enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucabden(&mut self) -> UCABDEN_W<UCA1ABCTL_SPEC> {
        UCABDEN_W::new(self, 0)
    }
    #[doc = "Bit 2 - Break Timeout error"]
    #[inline(always)]
    #[must_use]
    pub fn ucbtoe(&mut self) -> UCBTOE_W<UCA1ABCTL_SPEC> {
        UCBTOE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Sync-Field Timeout error"]
    #[inline(always)]
    #[must_use]
    pub fn ucstoe(&mut self) -> UCSTOE_W<UCA1ABCTL_SPEC> {
        UCSTOE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Break Sync Delimiter 0"]
    #[inline(always)]
    #[must_use]
    pub fn ucdelim0(&mut self) -> UCDELIM0_W<UCA1ABCTL_SPEC> {
        UCDELIM0_W::new(self, 4)
    }
    #[doc = "Bit 5 - Break Sync Delimiter 1"]
    #[inline(always)]
    #[must_use]
    pub fn ucdelim1(&mut self) -> UCDELIM1_W<UCA1ABCTL_SPEC> {
        UCDELIM1_W::new(self, 5)
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
#[doc = "USCI A1 LIN Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1abctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1abctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA1ABCTL_SPEC;
impl crate::RegisterSpec for UCA1ABCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca1abctl::R`](R) reader structure"]
impl crate::Readable for UCA1ABCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca1abctl::W`](W) writer structure"]
impl crate::Writable for UCA1ABCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA1ABCTL to value 0"]
impl crate::Resettable for UCA1ABCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
