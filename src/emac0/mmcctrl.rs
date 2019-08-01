#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MMCCTRL {
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
pub struct EMAC_MMCCTRL_CNTRSTR {
    bits: bool,
}
impl EMAC_MMCCTRL_CNTRSTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_MMCCTRL_CNTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_MMCCTRL_CNTRSTW<'a> {
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
        self.w.bits &= !(1 << 0);
        self.w.bits |= ((value as u32) & 1) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_MMCCTRL_CNTSTPROR {
    bits: bool,
}
impl EMAC_MMCCTRL_CNTSTPROR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_MMCCTRL_CNTSTPROW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_MMCCTRL_CNTSTPROW<'a> {
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
        self.w.bits &= !(1 << 1);
        self.w.bits |= ((value as u32) & 1) << 1;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_MMCCTRL_RSTONRDR {
    bits: bool,
}
impl EMAC_MMCCTRL_RSTONRDR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_MMCCTRL_RSTONRDW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_MMCCTRL_RSTONRDW<'a> {
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
        self.w.bits &= !(1 << 2);
        self.w.bits |= ((value as u32) & 1) << 2;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_MMCCTRL_CNTFREEZR {
    bits: bool,
}
impl EMAC_MMCCTRL_CNTFREEZR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_MMCCTRL_CNTFREEZW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_MMCCTRL_CNTFREEZW<'a> {
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
        self.w.bits &= !(1 << 3);
        self.w.bits |= ((value as u32) & 1) << 3;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_MMCCTRL_CNTPRSTR {
    bits: bool,
}
impl EMAC_MMCCTRL_CNTPRSTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_MMCCTRL_CNTPRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_MMCCTRL_CNTPRSTW<'a> {
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
        self.w.bits &= !(1 << 4);
        self.w.bits |= ((value as u32) & 1) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_MMCCTRL_CNTPRSTLVLR {
    bits: bool,
}
impl EMAC_MMCCTRL_CNTPRSTLVLR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_MMCCTRL_CNTPRSTLVLW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_MMCCTRL_CNTPRSTLVLW<'a> {
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
        self.w.bits &= !(1 << 5);
        self.w.bits |= ((value as u32) & 1) << 5;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_MMCCTRL_UCDBCR {
    bits: bool,
}
impl EMAC_MMCCTRL_UCDBCR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_MMCCTRL_UCDBCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_MMCCTRL_UCDBCW<'a> {
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
        self.w.bits &= !(1 << 8);
        self.w.bits |= ((value as u32) & 1) << 8;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Counters Reset"]
    #[inline(always)]
    pub fn emac_mmcctrl_cntrst(&self) -> EMAC_MMCCTRL_CNTRSTR {
        let bits = ((self.bits >> 0) & 1) != 0;
        EMAC_MMCCTRL_CNTRSTR { bits }
    }
    #[doc = "Bit 1 - Counters Stop Rollover"]
    #[inline(always)]
    pub fn emac_mmcctrl_cntstpro(&self) -> EMAC_MMCCTRL_CNTSTPROR {
        let bits = ((self.bits >> 1) & 1) != 0;
        EMAC_MMCCTRL_CNTSTPROR { bits }
    }
    #[doc = "Bit 2 - Reset on Read"]
    #[inline(always)]
    pub fn emac_mmcctrl_rstonrd(&self) -> EMAC_MMCCTRL_RSTONRDR {
        let bits = ((self.bits >> 2) & 1) != 0;
        EMAC_MMCCTRL_RSTONRDR { bits }
    }
    #[doc = "Bit 3 - MMC Counter Freeze"]
    #[inline(always)]
    pub fn emac_mmcctrl_cntfreez(&self) -> EMAC_MMCCTRL_CNTFREEZR {
        let bits = ((self.bits >> 3) & 1) != 0;
        EMAC_MMCCTRL_CNTFREEZR { bits }
    }
    #[doc = "Bit 4 - Counters Preset"]
    #[inline(always)]
    pub fn emac_mmcctrl_cntprst(&self) -> EMAC_MMCCTRL_CNTPRSTR {
        let bits = ((self.bits >> 4) & 1) != 0;
        EMAC_MMCCTRL_CNTPRSTR { bits }
    }
    #[doc = "Bit 5 - Full/Half Preset Level Value"]
    #[inline(always)]
    pub fn emac_mmcctrl_cntprstlvl(&self) -> EMAC_MMCCTRL_CNTPRSTLVLR {
        let bits = ((self.bits >> 5) & 1) != 0;
        EMAC_MMCCTRL_CNTPRSTLVLR { bits }
    }
    #[doc = "Bit 8 - Update MMC Counters for Dropped Broadcast Frames"]
    #[inline(always)]
    pub fn emac_mmcctrl_ucdbc(&self) -> EMAC_MMCCTRL_UCDBCR {
        let bits = ((self.bits >> 8) & 1) != 0;
        EMAC_MMCCTRL_UCDBCR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Counters Reset"]
    #[inline(always)]
    pub fn emac_mmcctrl_cntrst(&mut self) -> _EMAC_MMCCTRL_CNTRSTW {
        _EMAC_MMCCTRL_CNTRSTW { w: self }
    }
    #[doc = "Bit 1 - Counters Stop Rollover"]
    #[inline(always)]
    pub fn emac_mmcctrl_cntstpro(&mut self) -> _EMAC_MMCCTRL_CNTSTPROW {
        _EMAC_MMCCTRL_CNTSTPROW { w: self }
    }
    #[doc = "Bit 2 - Reset on Read"]
    #[inline(always)]
    pub fn emac_mmcctrl_rstonrd(&mut self) -> _EMAC_MMCCTRL_RSTONRDW {
        _EMAC_MMCCTRL_RSTONRDW { w: self }
    }
    #[doc = "Bit 3 - MMC Counter Freeze"]
    #[inline(always)]
    pub fn emac_mmcctrl_cntfreez(&mut self) -> _EMAC_MMCCTRL_CNTFREEZW {
        _EMAC_MMCCTRL_CNTFREEZW { w: self }
    }
    #[doc = "Bit 4 - Counters Preset"]
    #[inline(always)]
    pub fn emac_mmcctrl_cntprst(&mut self) -> _EMAC_MMCCTRL_CNTPRSTW {
        _EMAC_MMCCTRL_CNTPRSTW { w: self }
    }
    #[doc = "Bit 5 - Full/Half Preset Level Value"]
    #[inline(always)]
    pub fn emac_mmcctrl_cntprstlvl(&mut self) -> _EMAC_MMCCTRL_CNTPRSTLVLW {
        _EMAC_MMCCTRL_CNTPRSTLVLW { w: self }
    }
    #[doc = "Bit 8 - Update MMC Counters for Dropped Broadcast Frames"]
    #[inline(always)]
    pub fn emac_mmcctrl_ucdbc(&mut self) -> _EMAC_MMCCTRL_UCDBCW {
        _EMAC_MMCCTRL_UCDBCW { w: self }
    }
}
