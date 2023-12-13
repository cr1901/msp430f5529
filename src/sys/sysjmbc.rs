#[doc = "Register `SYSJMBC` reader"]
pub type R = crate::R<SYSJMBC_SPEC>;
#[doc = "Register `SYSJMBC` writer"]
pub type W = crate::W<SYSJMBC_SPEC>;
#[doc = "Field `JMBIN0FG` reader - SYS - Incoming JTAG Mailbox 0 Flag"]
pub type JMBIN0FG_R = crate::BitReader;
#[doc = "Field `JMBIN0FG` writer - SYS - Incoming JTAG Mailbox 0 Flag"]
pub type JMBIN0FG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JMBIN1FG` reader - SYS - Incoming JTAG Mailbox 1 Flag"]
pub type JMBIN1FG_R = crate::BitReader;
#[doc = "Field `JMBIN1FG` writer - SYS - Incoming JTAG Mailbox 1 Flag"]
pub type JMBIN1FG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JMBOUT0FG` reader - SYS - Outgoing JTAG Mailbox 0 Flag"]
pub type JMBOUT0FG_R = crate::BitReader;
#[doc = "Field `JMBOUT0FG` writer - SYS - Outgoing JTAG Mailbox 0 Flag"]
pub type JMBOUT0FG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JMBOUT1FG` reader - SYS - Outgoing JTAG Mailbox 1 Flag"]
pub type JMBOUT1FG_R = crate::BitReader;
#[doc = "Field `JMBOUT1FG` writer - SYS - Outgoing JTAG Mailbox 1 Flag"]
pub type JMBOUT1FG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JMBMODE` reader - SYS - JMB 16/32 Bit Mode"]
pub type JMBMODE_R = crate::BitReader;
#[doc = "Field `JMBMODE` writer - SYS - JMB 16/32 Bit Mode"]
pub type JMBMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JMBCLR0OFF` reader - SYS - Incoming JTAG Mailbox 0 Flag auto-clear disalbe"]
pub type JMBCLR0OFF_R = crate::BitReader;
#[doc = "Field `JMBCLR0OFF` writer - SYS - Incoming JTAG Mailbox 0 Flag auto-clear disalbe"]
pub type JMBCLR0OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JMBCLR1OFF` reader - SYS - Incoming JTAG Mailbox 1 Flag auto-clear disalbe"]
pub type JMBCLR1OFF_R = crate::BitReader;
#[doc = "Field `JMBCLR1OFF` writer - SYS - Incoming JTAG Mailbox 1 Flag auto-clear disalbe"]
pub type JMBCLR1OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYS - Incoming JTAG Mailbox 0 Flag"]
    #[inline(always)]
    pub fn jmbin0fg(&self) -> JMBIN0FG_R {
        JMBIN0FG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYS - Incoming JTAG Mailbox 1 Flag"]
    #[inline(always)]
    pub fn jmbin1fg(&self) -> JMBIN1FG_R {
        JMBIN1FG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SYS - Outgoing JTAG Mailbox 0 Flag"]
    #[inline(always)]
    pub fn jmbout0fg(&self) -> JMBOUT0FG_R {
        JMBOUT0FG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SYS - Outgoing JTAG Mailbox 1 Flag"]
    #[inline(always)]
    pub fn jmbout1fg(&self) -> JMBOUT1FG_R {
        JMBOUT1FG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SYS - JMB 16/32 Bit Mode"]
    #[inline(always)]
    pub fn jmbmode(&self) -> JMBMODE_R {
        JMBMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SYS - Incoming JTAG Mailbox 0 Flag auto-clear disalbe"]
    #[inline(always)]
    pub fn jmbclr0off(&self) -> JMBCLR0OFF_R {
        JMBCLR0OFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SYS - Incoming JTAG Mailbox 1 Flag auto-clear disalbe"]
    #[inline(always)]
    pub fn jmbclr1off(&self) -> JMBCLR1OFF_R {
        JMBCLR1OFF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYS - Incoming JTAG Mailbox 0 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn jmbin0fg(&mut self) -> JMBIN0FG_W<SYSJMBC_SPEC> {
        JMBIN0FG_W::new(self, 0)
    }
    #[doc = "Bit 1 - SYS - Incoming JTAG Mailbox 1 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn jmbin1fg(&mut self) -> JMBIN1FG_W<SYSJMBC_SPEC> {
        JMBIN1FG_W::new(self, 1)
    }
    #[doc = "Bit 2 - SYS - Outgoing JTAG Mailbox 0 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn jmbout0fg(&mut self) -> JMBOUT0FG_W<SYSJMBC_SPEC> {
        JMBOUT0FG_W::new(self, 2)
    }
    #[doc = "Bit 3 - SYS - Outgoing JTAG Mailbox 1 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn jmbout1fg(&mut self) -> JMBOUT1FG_W<SYSJMBC_SPEC> {
        JMBOUT1FG_W::new(self, 3)
    }
    #[doc = "Bit 4 - SYS - JMB 16/32 Bit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn jmbmode(&mut self) -> JMBMODE_W<SYSJMBC_SPEC> {
        JMBMODE_W::new(self, 4)
    }
    #[doc = "Bit 6 - SYS - Incoming JTAG Mailbox 0 Flag auto-clear disalbe"]
    #[inline(always)]
    #[must_use]
    pub fn jmbclr0off(&mut self) -> JMBCLR0OFF_W<SYSJMBC_SPEC> {
        JMBCLR0OFF_W::new(self, 6)
    }
    #[doc = "Bit 7 - SYS - Incoming JTAG Mailbox 1 Flag auto-clear disalbe"]
    #[inline(always)]
    #[must_use]
    pub fn jmbclr1off(&mut self) -> JMBCLR1OFF_W<SYSJMBC_SPEC> {
        JMBCLR1OFF_W::new(self, 7)
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
#[doc = "JTAG mailbox control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysjmbc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysjmbc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSJMBC_SPEC;
impl crate::RegisterSpec for SYSJMBC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysjmbc::R`](R) reader structure"]
impl crate::Readable for SYSJMBC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sysjmbc::W`](W) writer structure"]
impl crate::Writable for SYSJMBC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSJMBC to value 0"]
impl crate::Resettable for SYSJMBC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
