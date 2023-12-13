#[doc = "Register `UCSCTL6` reader"]
pub type R = crate::R<UCSCTL6_SPEC>;
#[doc = "Register `UCSCTL6` writer"]
pub type W = crate::W<UCSCTL6_SPEC>;
#[doc = "Field `XT1OFF` reader - High Frequency Oscillator 1 (XT1) disable"]
pub type XT1OFF_R = crate::BitReader;
#[doc = "Field `XT1OFF` writer - High Frequency Oscillator 1 (XT1) disable"]
pub type XT1OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMCLKOFF` reader - SMCLK Off"]
pub type SMCLKOFF_R = crate::BitReader;
#[doc = "Field `SMCLKOFF` writer - SMCLK Off"]
pub type SMCLKOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XCAP` reader - XIN/XOUT Cap Bit: 0"]
pub type XCAP_R = crate::FieldReader<XCAP_A>;
#[doc = "XIN/XOUT Cap Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum XCAP_A {
    #[doc = "0: XIN/XOUT Cap 0"]
    XCAP_0 = 0,
    #[doc = "1: XIN/XOUT Cap 1"]
    XCAP_1 = 1,
    #[doc = "2: XIN/XOUT Cap 2"]
    XCAP_2 = 2,
    #[doc = "3: XIN/XOUT Cap 3"]
    XCAP_3 = 3,
}
impl From<XCAP_A> for u8 {
    #[inline(always)]
    fn from(variant: XCAP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for XCAP_A {
    type Ux = u8;
}
impl XCAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XCAP_A {
        match self.bits {
            0 => XCAP_A::XCAP_0,
            1 => XCAP_A::XCAP_1,
            2 => XCAP_A::XCAP_2,
            3 => XCAP_A::XCAP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "XIN/XOUT Cap 0"]
    #[inline(always)]
    pub fn is_xcap_0(&self) -> bool {
        *self == XCAP_A::XCAP_0
    }
    #[doc = "XIN/XOUT Cap 1"]
    #[inline(always)]
    pub fn is_xcap_1(&self) -> bool {
        *self == XCAP_A::XCAP_1
    }
    #[doc = "XIN/XOUT Cap 2"]
    #[inline(always)]
    pub fn is_xcap_2(&self) -> bool {
        *self == XCAP_A::XCAP_2
    }
    #[doc = "XIN/XOUT Cap 3"]
    #[inline(always)]
    pub fn is_xcap_3(&self) -> bool {
        *self == XCAP_A::XCAP_3
    }
}
#[doc = "Field `XCAP` writer - XIN/XOUT Cap Bit: 0"]
pub type XCAP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, XCAP_A>;
impl<'a, REG> XCAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "XIN/XOUT Cap 0"]
    #[inline(always)]
    pub fn xcap_0(self) -> &'a mut crate::W<REG> {
        self.variant(XCAP_A::XCAP_0)
    }
    #[doc = "XIN/XOUT Cap 1"]
    #[inline(always)]
    pub fn xcap_1(self) -> &'a mut crate::W<REG> {
        self.variant(XCAP_A::XCAP_1)
    }
    #[doc = "XIN/XOUT Cap 2"]
    #[inline(always)]
    pub fn xcap_2(self) -> &'a mut crate::W<REG> {
        self.variant(XCAP_A::XCAP_2)
    }
    #[doc = "XIN/XOUT Cap 3"]
    #[inline(always)]
    pub fn xcap_3(self) -> &'a mut crate::W<REG> {
        self.variant(XCAP_A::XCAP_3)
    }
}
#[doc = "Field `XT1BYPASS` reader - XT1 bypass mode : 0: internal 1:sourced from external pin"]
pub type XT1BYPASS_R = crate::BitReader;
#[doc = "Field `XT1BYPASS` writer - XT1 bypass mode : 0: internal 1:sourced from external pin"]
pub type XT1BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTS` reader - 1: Selects high-freq. oscillator"]
pub type XTS_R = crate::BitReader;
#[doc = "Field `XTS` writer - 1: Selects high-freq. oscillator"]
pub type XTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XT1DRIVE` reader - XT1 Drive Level mode Bit 0"]
pub type XT1DRIVE_R = crate::FieldReader<XT1DRIVE_A>;
#[doc = "XT1 Drive Level mode Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum XT1DRIVE_A {
    #[doc = "0: XT1 Drive Level mode: 0"]
    XT1DRIVE_0 = 0,
    #[doc = "1: XT1 Drive Level mode: 1"]
    XT1DRIVE_1 = 1,
    #[doc = "2: XT1 Drive Level mode: 2"]
    XT1DRIVE_2 = 2,
    #[doc = "3: XT1 Drive Level mode: 3"]
    XT1DRIVE_3 = 3,
}
impl From<XT1DRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: XT1DRIVE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for XT1DRIVE_A {
    type Ux = u8;
}
impl XT1DRIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XT1DRIVE_A {
        match self.bits {
            0 => XT1DRIVE_A::XT1DRIVE_0,
            1 => XT1DRIVE_A::XT1DRIVE_1,
            2 => XT1DRIVE_A::XT1DRIVE_2,
            3 => XT1DRIVE_A::XT1DRIVE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "XT1 Drive Level mode: 0"]
    #[inline(always)]
    pub fn is_xt1drive_0(&self) -> bool {
        *self == XT1DRIVE_A::XT1DRIVE_0
    }
    #[doc = "XT1 Drive Level mode: 1"]
    #[inline(always)]
    pub fn is_xt1drive_1(&self) -> bool {
        *self == XT1DRIVE_A::XT1DRIVE_1
    }
    #[doc = "XT1 Drive Level mode: 2"]
    #[inline(always)]
    pub fn is_xt1drive_2(&self) -> bool {
        *self == XT1DRIVE_A::XT1DRIVE_2
    }
    #[doc = "XT1 Drive Level mode: 3"]
    #[inline(always)]
    pub fn is_xt1drive_3(&self) -> bool {
        *self == XT1DRIVE_A::XT1DRIVE_3
    }
}
#[doc = "Field `XT1DRIVE` writer - XT1 Drive Level mode Bit 0"]
pub type XT1DRIVE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, XT1DRIVE_A>;
impl<'a, REG> XT1DRIVE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "XT1 Drive Level mode: 0"]
    #[inline(always)]
    pub fn xt1drive_0(self) -> &'a mut crate::W<REG> {
        self.variant(XT1DRIVE_A::XT1DRIVE_0)
    }
    #[doc = "XT1 Drive Level mode: 1"]
    #[inline(always)]
    pub fn xt1drive_1(self) -> &'a mut crate::W<REG> {
        self.variant(XT1DRIVE_A::XT1DRIVE_1)
    }
    #[doc = "XT1 Drive Level mode: 2"]
    #[inline(always)]
    pub fn xt1drive_2(self) -> &'a mut crate::W<REG> {
        self.variant(XT1DRIVE_A::XT1DRIVE_2)
    }
    #[doc = "XT1 Drive Level mode: 3"]
    #[inline(always)]
    pub fn xt1drive_3(self) -> &'a mut crate::W<REG> {
        self.variant(XT1DRIVE_A::XT1DRIVE_3)
    }
}
#[doc = "Field `XT2OFF` reader - High Frequency Oscillator 2 (XT2) disable"]
pub type XT2OFF_R = crate::BitReader;
#[doc = "Field `XT2OFF` writer - High Frequency Oscillator 2 (XT2) disable"]
pub type XT2OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XT2BYPASS` reader - XT2 bypass mode : 0: internal 1:sourced from external pin"]
pub type XT2BYPASS_R = crate::BitReader;
#[doc = "Field `XT2BYPASS` writer - XT2 bypass mode : 0: internal 1:sourced from external pin"]
pub type XT2BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XT2DRIVE` reader - XT2 Drive Level mode Bit 0"]
pub type XT2DRIVE_R = crate::FieldReader<XT2DRIVE_A>;
#[doc = "XT2 Drive Level mode Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum XT2DRIVE_A {
    #[doc = "0: XT2 Drive Level mode: 0"]
    XT2DRIVE_0 = 0,
    #[doc = "1: XT2 Drive Level mode: 1"]
    XT2DRIVE_1 = 1,
    #[doc = "2: XT2 Drive Level mode: 2"]
    XT2DRIVE_2 = 2,
    #[doc = "3: XT2 Drive Level mode: 3"]
    XT2DRIVE_3 = 3,
}
impl From<XT2DRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: XT2DRIVE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for XT2DRIVE_A {
    type Ux = u8;
}
impl XT2DRIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XT2DRIVE_A {
        match self.bits {
            0 => XT2DRIVE_A::XT2DRIVE_0,
            1 => XT2DRIVE_A::XT2DRIVE_1,
            2 => XT2DRIVE_A::XT2DRIVE_2,
            3 => XT2DRIVE_A::XT2DRIVE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "XT2 Drive Level mode: 0"]
    #[inline(always)]
    pub fn is_xt2drive_0(&self) -> bool {
        *self == XT2DRIVE_A::XT2DRIVE_0
    }
    #[doc = "XT2 Drive Level mode: 1"]
    #[inline(always)]
    pub fn is_xt2drive_1(&self) -> bool {
        *self == XT2DRIVE_A::XT2DRIVE_1
    }
    #[doc = "XT2 Drive Level mode: 2"]
    #[inline(always)]
    pub fn is_xt2drive_2(&self) -> bool {
        *self == XT2DRIVE_A::XT2DRIVE_2
    }
    #[doc = "XT2 Drive Level mode: 3"]
    #[inline(always)]
    pub fn is_xt2drive_3(&self) -> bool {
        *self == XT2DRIVE_A::XT2DRIVE_3
    }
}
#[doc = "Field `XT2DRIVE` writer - XT2 Drive Level mode Bit 0"]
pub type XT2DRIVE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, XT2DRIVE_A>;
impl<'a, REG> XT2DRIVE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "XT2 Drive Level mode: 0"]
    #[inline(always)]
    pub fn xt2drive_0(self) -> &'a mut crate::W<REG> {
        self.variant(XT2DRIVE_A::XT2DRIVE_0)
    }
    #[doc = "XT2 Drive Level mode: 1"]
    #[inline(always)]
    pub fn xt2drive_1(self) -> &'a mut crate::W<REG> {
        self.variant(XT2DRIVE_A::XT2DRIVE_1)
    }
    #[doc = "XT2 Drive Level mode: 2"]
    #[inline(always)]
    pub fn xt2drive_2(self) -> &'a mut crate::W<REG> {
        self.variant(XT2DRIVE_A::XT2DRIVE_2)
    }
    #[doc = "XT2 Drive Level mode: 3"]
    #[inline(always)]
    pub fn xt2drive_3(self) -> &'a mut crate::W<REG> {
        self.variant(XT2DRIVE_A::XT2DRIVE_3)
    }
}
impl R {
    #[doc = "Bit 0 - High Frequency Oscillator 1 (XT1) disable"]
    #[inline(always)]
    pub fn xt1off(&self) -> XT1OFF_R {
        XT1OFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMCLK Off"]
    #[inline(always)]
    pub fn smclkoff(&self) -> SMCLKOFF_R {
        SMCLKOFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - XIN/XOUT Cap Bit: 0"]
    #[inline(always)]
    pub fn xcap(&self) -> XCAP_R {
        XCAP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - XT1 bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    pub fn xt1bypass(&self) -> XT1BYPASS_R {
        XT1BYPASS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Selects high-freq. oscillator"]
    #[inline(always)]
    pub fn xts(&self) -> XTS_R {
        XTS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - XT1 Drive Level mode Bit 0"]
    #[inline(always)]
    pub fn xt1drive(&self) -> XT1DRIVE_R {
        XT1DRIVE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - High Frequency Oscillator 2 (XT2) disable"]
    #[inline(always)]
    pub fn xt2off(&self) -> XT2OFF_R {
        XT2OFF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - XT2 bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    pub fn xt2bypass(&self) -> XT2BYPASS_R {
        XT2BYPASS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - XT2 Drive Level mode Bit 0"]
    #[inline(always)]
    pub fn xt2drive(&self) -> XT2DRIVE_R {
        XT2DRIVE_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - High Frequency Oscillator 1 (XT1) disable"]
    #[inline(always)]
    #[must_use]
    pub fn xt1off(&mut self) -> XT1OFF_W<UCSCTL6_SPEC> {
        XT1OFF_W::new(self, 0)
    }
    #[doc = "Bit 1 - SMCLK Off"]
    #[inline(always)]
    #[must_use]
    pub fn smclkoff(&mut self) -> SMCLKOFF_W<UCSCTL6_SPEC> {
        SMCLKOFF_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - XIN/XOUT Cap Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn xcap(&mut self) -> XCAP_W<UCSCTL6_SPEC> {
        XCAP_W::new(self, 2)
    }
    #[doc = "Bit 4 - XT1 bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    #[must_use]
    pub fn xt1bypass(&mut self) -> XT1BYPASS_W<UCSCTL6_SPEC> {
        XT1BYPASS_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Selects high-freq. oscillator"]
    #[inline(always)]
    #[must_use]
    pub fn xts(&mut self) -> XTS_W<UCSCTL6_SPEC> {
        XTS_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - XT1 Drive Level mode Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn xt1drive(&mut self) -> XT1DRIVE_W<UCSCTL6_SPEC> {
        XT1DRIVE_W::new(self, 6)
    }
    #[doc = "Bit 8 - High Frequency Oscillator 2 (XT2) disable"]
    #[inline(always)]
    #[must_use]
    pub fn xt2off(&mut self) -> XT2OFF_W<UCSCTL6_SPEC> {
        XT2OFF_W::new(self, 8)
    }
    #[doc = "Bit 12 - XT2 bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    #[must_use]
    pub fn xt2bypass(&mut self) -> XT2BYPASS_W<UCSCTL6_SPEC> {
        XT2BYPASS_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - XT2 Drive Level mode Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn xt2drive(&mut self) -> XT2DRIVE_W<UCSCTL6_SPEC> {
        XT2DRIVE_W::new(self, 14)
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
#[doc = "UCS Control Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsctl6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsctl6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSCTL6_SPEC;
impl crate::RegisterSpec for UCSCTL6_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucsctl6::R`](R) reader structure"]
impl crate::Readable for UCSCTL6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsctl6::W`](W) writer structure"]
impl crate::Writable for UCSCTL6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSCTL6 to value 0"]
impl crate::Resettable for UCSCTL6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
