#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LPITIMERCTL {
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
pub struct EMAC_LPITIMERCTL_TWTR {
    bits: u16,
}
impl EMAC_LPITIMERCTL_TWTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_LPITIMERCTL_TWTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_LPITIMERCTL_TWTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(65535 << 0);
        self.w.bits |= ((value as u32) & 65535) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_LPITIMERCTL_LSTR {
    bits: u16,
}
impl EMAC_LPITIMERCTL_LSTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_LPITIMERCTL_LSTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_LPITIMERCTL_LSTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(1023 << 16);
        self.w.bits |= ((value as u32) & 1023) << 16;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - LPI TW Timer"]
    #[inline(always)]
    pub fn emac_lpitimerctl_twt(&self) -> EMAC_LPITIMERCTL_TWTR {
        let bits = ((self.bits >> 0) & 65535) as u16;
        EMAC_LPITIMERCTL_TWTR { bits }
    }
    #[doc = "Bits 16:25 - LPI LS Timer"]
    #[inline(always)]
    pub fn emac_lpitimerctl_lst(&self) -> EMAC_LPITIMERCTL_LSTR {
        let bits = ((self.bits >> 16) & 1023) as u16;
        EMAC_LPITIMERCTL_LSTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - LPI TW Timer"]
    #[inline(always)]
    pub fn emac_lpitimerctl_twt(&mut self) -> _EMAC_LPITIMERCTL_TWTW {
        _EMAC_LPITIMERCTL_TWTW { w: self }
    }
    #[doc = "Bits 16:25 - LPI LS Timer"]
    #[inline(always)]
    pub fn emac_lpitimerctl_lst(&mut self) -> _EMAC_LPITIMERCTL_LSTW {
        _EMAC_LPITIMERCTL_LSTW { w: self }
    }
}
