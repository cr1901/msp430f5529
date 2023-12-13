#[doc = "Register `USBPLLDIVB` reader"]
pub type R = crate::R<USBPLLDIVB_SPEC>;
#[doc = "Register `USBPLLDIVB` writer"]
pub type W = crate::W<USBPLLDIVB_SPEC>;
#[doc = "Field `UPMB0` reader - USB - PLL feedback divider buffer Bit 0"]
pub type UPMB0_R = crate::BitReader;
#[doc = "Field `UPMB0` writer - USB - PLL feedback divider buffer Bit 0"]
pub type UPMB0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPMB1` reader - USB - PLL feedback divider buffer Bit 1"]
pub type UPMB1_R = crate::BitReader;
#[doc = "Field `UPMB1` writer - USB - PLL feedback divider buffer Bit 1"]
pub type UPMB1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPMB2` reader - USB - PLL feedback divider buffer Bit 2"]
pub type UPMB2_R = crate::BitReader;
#[doc = "Field `UPMB2` writer - USB - PLL feedback divider buffer Bit 2"]
pub type UPMB2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPMB3` reader - USB - PLL feedback divider buffer Bit 3"]
pub type UPMB3_R = crate::BitReader;
#[doc = "Field `UPMB3` writer - USB - PLL feedback divider buffer Bit 3"]
pub type UPMB3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPMB4` reader - USB - PLL feedback divider buffer Bit 4"]
pub type UPMB4_R = crate::BitReader;
#[doc = "Field `UPMB4` writer - USB - PLL feedback divider buffer Bit 4"]
pub type UPMB4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPMB5` reader - USB - PLL feedback divider buffer Bit 5"]
pub type UPMB5_R = crate::BitReader;
#[doc = "Field `UPMB5` writer - USB - PLL feedback divider buffer Bit 5"]
pub type UPMB5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPQB0` reader - USB - PLL prescale divider buffer Bit 0"]
pub type UPQB0_R = crate::BitReader;
#[doc = "Field `UPQB0` writer - USB - PLL prescale divider buffer Bit 0"]
pub type UPQB0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPQB1` reader - USB - PLL prescale divider buffer Bit 1"]
pub type UPQB1_R = crate::BitReader;
#[doc = "Field `UPQB1` writer - USB - PLL prescale divider buffer Bit 1"]
pub type UPQB1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPQB2` reader - USB - PLL prescale divider buffer Bit 2"]
pub type UPQB2_R = crate::BitReader;
#[doc = "Field `UPQB2` writer - USB - PLL prescale divider buffer Bit 2"]
pub type UPQB2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB - PLL feedback divider buffer Bit 0"]
    #[inline(always)]
    pub fn upmb0(&self) -> UPMB0_R {
        UPMB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB - PLL feedback divider buffer Bit 1"]
    #[inline(always)]
    pub fn upmb1(&self) -> UPMB1_R {
        UPMB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB - PLL feedback divider buffer Bit 2"]
    #[inline(always)]
    pub fn upmb2(&self) -> UPMB2_R {
        UPMB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB - PLL feedback divider buffer Bit 3"]
    #[inline(always)]
    pub fn upmb3(&self) -> UPMB3_R {
        UPMB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB - PLL feedback divider buffer Bit 4"]
    #[inline(always)]
    pub fn upmb4(&self) -> UPMB4_R {
        UPMB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB - PLL feedback divider buffer Bit 5"]
    #[inline(always)]
    pub fn upmb5(&self) -> UPMB5_R {
        UPMB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - USB - PLL prescale divider buffer Bit 0"]
    #[inline(always)]
    pub fn upqb0(&self) -> UPQB0_R {
        UPQB0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USB - PLL prescale divider buffer Bit 1"]
    #[inline(always)]
    pub fn upqb1(&self) -> UPQB1_R {
        UPQB1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB - PLL prescale divider buffer Bit 2"]
    #[inline(always)]
    pub fn upqb2(&self) -> UPQB2_R {
        UPQB2_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - PLL feedback divider buffer Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn upmb0(&mut self) -> UPMB0_W<USBPLLDIVB_SPEC> {
        UPMB0_W::new(self, 0)
    }
    #[doc = "Bit 1 - USB - PLL feedback divider buffer Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn upmb1(&mut self) -> UPMB1_W<USBPLLDIVB_SPEC> {
        UPMB1_W::new(self, 1)
    }
    #[doc = "Bit 2 - USB - PLL feedback divider buffer Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn upmb2(&mut self) -> UPMB2_W<USBPLLDIVB_SPEC> {
        UPMB2_W::new(self, 2)
    }
    #[doc = "Bit 3 - USB - PLL feedback divider buffer Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn upmb3(&mut self) -> UPMB3_W<USBPLLDIVB_SPEC> {
        UPMB3_W::new(self, 3)
    }
    #[doc = "Bit 4 - USB - PLL feedback divider buffer Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn upmb4(&mut self) -> UPMB4_W<USBPLLDIVB_SPEC> {
        UPMB4_W::new(self, 4)
    }
    #[doc = "Bit 5 - USB - PLL feedback divider buffer Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn upmb5(&mut self) -> UPMB5_W<USBPLLDIVB_SPEC> {
        UPMB5_W::new(self, 5)
    }
    #[doc = "Bit 8 - USB - PLL prescale divider buffer Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn upqb0(&mut self) -> UPQB0_W<USBPLLDIVB_SPEC> {
        UPQB0_W::new(self, 8)
    }
    #[doc = "Bit 9 - USB - PLL prescale divider buffer Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn upqb1(&mut self) -> UPQB1_W<USBPLLDIVB_SPEC> {
        UPQB1_W::new(self, 9)
    }
    #[doc = "Bit 10 - USB - PLL prescale divider buffer Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn upqb2(&mut self) -> UPQB2_W<USBPLLDIVB_SPEC> {
        UPQB2_W::new(self, 10)
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
#[doc = "USB PLL Clock Divider Buffer control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbplldivb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbplldivb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBPLLDIVB_SPEC;
impl crate::RegisterSpec for USBPLLDIVB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usbplldivb::R`](R) reader structure"]
impl crate::Readable for USBPLLDIVB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbplldivb::W`](W) writer structure"]
impl crate::Writable for USBPLLDIVB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBPLLDIVB to value 0"]
impl crate::Resettable for USBPLLDIVB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
