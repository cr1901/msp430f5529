#[doc = "Register `UCA0IRTCTL` reader"]
pub type R = crate::R<UCA0IRTCTL_SPEC>;
#[doc = "Register `UCA0IRTCTL` writer"]
pub type W = crate::W<UCA0IRTCTL_SPEC>;
#[doc = "Field `UCIREN` reader - IRDA Encoder/Decoder enable"]
pub type UCIREN_R = crate::BitReader;
#[doc = "Field `UCIREN` writer - IRDA Encoder/Decoder enable"]
pub type UCIREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRTXCLK` reader - IRDA Transmit Pulse Clock Select"]
pub type UCIRTXCLK_R = crate::BitReader;
#[doc = "Field `UCIRTXCLK` writer - IRDA Transmit Pulse Clock Select"]
pub type UCIRTXCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRTXPL` reader - IRDA Transmit Pulse Length 0"]
pub type UCIRTXPL_R = crate::FieldReader;
#[doc = "Field `UCIRTXPL` writer - IRDA Transmit Pulse Length 0"]
pub type UCIRTXPL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - IRDA Encoder/Decoder enable"]
    #[inline(always)]
    pub fn uciren(&self) -> UCIREN_R {
        UCIREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRDA Transmit Pulse Clock Select"]
    #[inline(always)]
    pub fn ucirtxclk(&self) -> UCIRTXCLK_R {
        UCIRTXCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - IRDA Transmit Pulse Length 0"]
    #[inline(always)]
    pub fn ucirtxpl(&self) -> UCIRTXPL_R {
        UCIRTXPL_R::new((self.bits >> 2) & 0x3f)
    }
}
impl W {
    #[doc = "Bit 0 - IRDA Encoder/Decoder enable"]
    #[inline(always)]
    #[must_use]
    pub fn uciren(&mut self) -> UCIREN_W<UCA0IRTCTL_SPEC> {
        UCIREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - IRDA Transmit Pulse Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn ucirtxclk(&mut self) -> UCIRTXCLK_W<UCA0IRTCTL_SPEC> {
        UCIRTXCLK_W::new(self, 1)
    }
    #[doc = "Bits 2:7 - IRDA Transmit Pulse Length 0"]
    #[inline(always)]
    #[must_use]
    pub fn ucirtxpl(&mut self) -> UCIRTXPL_W<UCA0IRTCTL_SPEC> {
        UCIRTXPL_W::new(self, 2)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USCI A0 IrDA Transmit Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0irtctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0irtctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA0IRTCTL_SPEC;
impl crate::RegisterSpec for UCA0IRTCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0irtctl::R`](R) reader structure"]
impl crate::Readable for UCA0IRTCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca0irtctl::W`](W) writer structure"]
impl crate::Writable for UCA0IRTCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA0IRTCTL to value 0"]
impl crate::Resettable for UCA0IRTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
