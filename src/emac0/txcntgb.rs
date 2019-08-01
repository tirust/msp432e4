#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TXCNTGB {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_TXCNTGB_TXFRMGBR {
    bits: u32,
}
impl EMAC_TXCNTGB_TXFRMGBR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_TXCNTGB_TXFRMGBW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_TXCNTGB_TXFRMGBW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits &= !(4294967295 << 0);
        self.w.bits |= ((value as u32) & 4294967295) << 0;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - This field indicates the number of good and bad frames transmitted, exclusive of retried frames"]
    #[inline(always)]
    pub fn emac_txcntgb_txfrmgb(&self) -> EMAC_TXCNTGB_TXFRMGBR {
        let bits = ((self.bits >> 0) & 4294967295) as u32;
        EMAC_TXCNTGB_TXFRMGBR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:31 - This field indicates the number of good and bad frames transmitted, exclusive of retried frames"]
    #[inline(always)]
    pub fn emac_txcntgb_txfrmgb(&mut self) -> _EMAC_TXCNTGB_TXFRMGBW {
        _EMAC_TXCNTGB_TXFRMGBW { w: self }
    }
}
