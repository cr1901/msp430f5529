#[doc = "Reader of register USBVECINT"]
pub type R = crate::R<u16, super::USBVECINT>;
#[doc = "Writer for register USBVECINT"]
pub type W = crate::W<u16, super::USBVECINT>;
#[doc = "Register USBVECINT `reset()`'s with value 0"]
impl crate::ResetValue for super::USBVECINT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
