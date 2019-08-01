#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RXDLADDR {
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
pub struct EMAC_RXDLADDR_STRXLISTR {
    bits: u32,
}
impl EMAC_RXDLADDR_STRXLISTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_RXDLADDR_STRXLISTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_RXDLADDR_STRXLISTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits &= !(1073741823 << 2);
        self.w.bits |= ((value as u32) & 1073741823) << 2;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 2:31 - Start of Receive List"]
    #[inline(always)]
    pub fn emac_rxdladdr_strxlist(&self) -> EMAC_RXDLADDR_STRXLISTR {
        let bits = ((self.bits >> 2) & 1073741823) as u32;
        EMAC_RXDLADDR_STRXLISTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 2:31 - Start of Receive List"]
    #[inline(always)]
    pub fn emac_rxdladdr_strxlist(&mut self) -> _EMAC_RXDLADDR_STRXLISTW {
        _EMAC_RXDLADDR_STRXLISTW { w: self }
    }
}
