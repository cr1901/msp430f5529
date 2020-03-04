#[doc = "Reader of register SYSJMBC"]
pub type R = crate::R<u16, super::SYSJMBC>;
#[doc = "Writer for register SYSJMBC"]
pub type W = crate::W<u16, super::SYSJMBC>;
#[doc = "Register SYSJMBC `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSJMBC {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `JMBIN0FG`"]
pub type JMBIN0FG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JMBIN0FG`"]
pub struct JMBIN0FG_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBIN0FG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `JMBIN1FG`"]
pub type JMBIN1FG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JMBIN1FG`"]
pub struct JMBIN1FG_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBIN1FG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `JMBOUT0FG`"]
pub type JMBOUT0FG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JMBOUT0FG`"]
pub struct JMBOUT0FG_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBOUT0FG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `JMBOUT1FG`"]
pub type JMBOUT1FG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JMBOUT1FG`"]
pub struct JMBOUT1FG_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBOUT1FG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `JMBMODE`"]
pub type JMBMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JMBMODE`"]
pub struct JMBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBMODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `JMBCLR0OFF`"]
pub type JMBCLR0OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JMBCLR0OFF`"]
pub struct JMBCLR0OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBCLR0OFF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `JMBCLR1OFF`"]
pub type JMBCLR1OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JMBCLR1OFF`"]
pub struct JMBCLR1OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBCLR1OFF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SYS - Incoming JTAG Mailbox 0 Flag"]
    #[inline(always)]
    pub fn jmbin0fg(&self) -> JMBIN0FG_R {
        JMBIN0FG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SYS - Incoming JTAG Mailbox 1 Flag"]
    #[inline(always)]
    pub fn jmbin1fg(&self) -> JMBIN1FG_R {
        JMBIN1FG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SYS - Outgoing JTAG Mailbox 0 Flag"]
    #[inline(always)]
    pub fn jmbout0fg(&self) -> JMBOUT0FG_R {
        JMBOUT0FG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SYS - Outgoing JTAG Mailbox 1 Flag"]
    #[inline(always)]
    pub fn jmbout1fg(&self) -> JMBOUT1FG_R {
        JMBOUT1FG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SYS - JMB 16/32 Bit Mode"]
    #[inline(always)]
    pub fn jmbmode(&self) -> JMBMODE_R {
        JMBMODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SYS - Incoming JTAG Mailbox 0 Flag auto-clear disalbe"]
    #[inline(always)]
    pub fn jmbclr0off(&self) -> JMBCLR0OFF_R {
        JMBCLR0OFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SYS - Incoming JTAG Mailbox 1 Flag auto-clear disalbe"]
    #[inline(always)]
    pub fn jmbclr1off(&self) -> JMBCLR1OFF_R {
        JMBCLR1OFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYS - Incoming JTAG Mailbox 0 Flag"]
    #[inline(always)]
    pub fn jmbin0fg(&mut self) -> JMBIN0FG_W {
        JMBIN0FG_W { w: self }
    }
    #[doc = "Bit 1 - SYS - Incoming JTAG Mailbox 1 Flag"]
    #[inline(always)]
    pub fn jmbin1fg(&mut self) -> JMBIN1FG_W {
        JMBIN1FG_W { w: self }
    }
    #[doc = "Bit 2 - SYS - Outgoing JTAG Mailbox 0 Flag"]
    #[inline(always)]
    pub fn jmbout0fg(&mut self) -> JMBOUT0FG_W {
        JMBOUT0FG_W { w: self }
    }
    #[doc = "Bit 3 - SYS - Outgoing JTAG Mailbox 1 Flag"]
    #[inline(always)]
    pub fn jmbout1fg(&mut self) -> JMBOUT1FG_W {
        JMBOUT1FG_W { w: self }
    }
    #[doc = "Bit 4 - SYS - JMB 16/32 Bit Mode"]
    #[inline(always)]
    pub fn jmbmode(&mut self) -> JMBMODE_W {
        JMBMODE_W { w: self }
    }
    #[doc = "Bit 6 - SYS - Incoming JTAG Mailbox 0 Flag auto-clear disalbe"]
    #[inline(always)]
    pub fn jmbclr0off(&mut self) -> JMBCLR0OFF_W {
        JMBCLR0OFF_W { w: self }
    }
    #[doc = "Bit 7 - SYS - Incoming JTAG Mailbox 1 Flag auto-clear disalbe"]
    #[inline(always)]
    pub fn jmbclr1off(&mut self) -> JMBCLR1OFF_W {
        JMBCLR1OFF_W { w: self }
    }
}
