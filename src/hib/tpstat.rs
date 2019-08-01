#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TPSTAT {
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
pub struct HIB_TPSTAT_XOSCFAILR {
    bits: bool,
}
impl HIB_TPSTAT_XOSCFAILR {
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
pub struct _HIB_TPSTAT_XOSCFAILW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPSTAT_XOSCFAILW<'a> {
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
pub struct HIB_TPSTAT_XOSCSTR {
    bits: bool,
}
impl HIB_TPSTAT_XOSCSTR {
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
pub struct _HIB_TPSTAT_XOSCSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPSTAT_XOSCSTW<'a> {
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
#[doc = "Possible values of the field `HIB_TPSTAT_STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIB_TPSTAT_STATER {
    #[doc = "Tamper disabled"]
    HIB_TPSTAT_STATE_DISABLED,
    #[doc = "Tamper configured"]
    HIB_TPSTAT_STATE_CONFIGED,
    #[doc = "Tamper pin event occurred"]
    HIB_TPSTAT_STATE_ERROR,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl HIB_TPSTAT_STATER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            HIB_TPSTAT_STATER::HIB_TPSTAT_STATE_DISABLED => 0,
            HIB_TPSTAT_STATER::HIB_TPSTAT_STATE_CONFIGED => 1,
            HIB_TPSTAT_STATER::HIB_TPSTAT_STATE_ERROR => 2,
            HIB_TPSTAT_STATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> HIB_TPSTAT_STATER {
        match value {
            0 => HIB_TPSTAT_STATER::HIB_TPSTAT_STATE_DISABLED,
            1 => HIB_TPSTAT_STATER::HIB_TPSTAT_STATE_CONFIGED,
            2 => HIB_TPSTAT_STATER::HIB_TPSTAT_STATE_ERROR,
            i => HIB_TPSTAT_STATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `HIB_TPSTAT_STATE_DISABLED`"]
    #[inline(always)]
    pub fn is_hib_tpstat_state_disabled(&self) -> bool {
        *self == HIB_TPSTAT_STATER::HIB_TPSTAT_STATE_DISABLED
    }
    #[doc = "Checks if the value of the field is `HIB_TPSTAT_STATE_CONFIGED`"]
    #[inline(always)]
    pub fn is_hib_tpstat_state_configed(&self) -> bool {
        *self == HIB_TPSTAT_STATER::HIB_TPSTAT_STATE_CONFIGED
    }
    #[doc = "Checks if the value of the field is `HIB_TPSTAT_STATE_ERROR`"]
    #[inline(always)]
    pub fn is_hib_tpstat_state_error(&self) -> bool {
        *self == HIB_TPSTAT_STATER::HIB_TPSTAT_STATE_ERROR
    }
}
#[doc = "Values that can be written to the field `HIB_TPSTAT_STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIB_TPSTAT_STATEW {
    #[doc = "Tamper disabled"]
    HIB_TPSTAT_STATE_DISABLED,
    #[doc = "Tamper configured"]
    HIB_TPSTAT_STATE_CONFIGED,
    #[doc = "Tamper pin event occurred"]
    HIB_TPSTAT_STATE_ERROR,
}
impl HIB_TPSTAT_STATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            HIB_TPSTAT_STATEW::HIB_TPSTAT_STATE_DISABLED => 0,
            HIB_TPSTAT_STATEW::HIB_TPSTAT_STATE_CONFIGED => 1,
            HIB_TPSTAT_STATEW::HIB_TPSTAT_STATE_ERROR => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _HIB_TPSTAT_STATEW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPSTAT_STATEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIB_TPSTAT_STATEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Tamper disabled"]
    #[inline(always)]
    pub fn hib_tpstat_state_disabled(self) -> &'a mut W {
        self.variant(HIB_TPSTAT_STATEW::HIB_TPSTAT_STATE_DISABLED)
    }
    #[doc = "Tamper configured"]
    #[inline(always)]
    pub fn hib_tpstat_state_configed(self) -> &'a mut W {
        self.variant(HIB_TPSTAT_STATEW::HIB_TPSTAT_STATE_CONFIGED)
    }
    #[doc = "Tamper pin event occurred"]
    #[inline(always)]
    pub fn hib_tpstat_state_error(self) -> &'a mut W {
        self.variant(HIB_TPSTAT_STATEW::HIB_TPSTAT_STATE_ERROR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 2);
        self.w.bits |= ((value as u32) & 3) << 2;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - External Oscillator Failure"]
    #[inline(always)]
    pub fn hib_tpstat_xoscfail(&self) -> HIB_TPSTAT_XOSCFAILR {
        let bits = ((self.bits >> 0) & 1) != 0;
        HIB_TPSTAT_XOSCFAILR { bits }
    }
    #[doc = "Bit 1 - External Oscillator Status"]
    #[inline(always)]
    pub fn hib_tpstat_xoscst(&self) -> HIB_TPSTAT_XOSCSTR {
        let bits = ((self.bits >> 1) & 1) != 0;
        HIB_TPSTAT_XOSCSTR { bits }
    }
    #[doc = "Bits 2:3 - Tamper Module Status"]
    #[inline(always)]
    pub fn hib_tpstat_state(&self) -> HIB_TPSTAT_STATER {
        HIB_TPSTAT_STATER::_from(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - External Oscillator Failure"]
    #[inline(always)]
    pub fn hib_tpstat_xoscfail(&mut self) -> _HIB_TPSTAT_XOSCFAILW {
        _HIB_TPSTAT_XOSCFAILW { w: self }
    }
    #[doc = "Bit 1 - External Oscillator Status"]
    #[inline(always)]
    pub fn hib_tpstat_xoscst(&mut self) -> _HIB_TPSTAT_XOSCSTW {
        _HIB_TPSTAT_XOSCSTW { w: self }
    }
    #[doc = "Bits 2:3 - Tamper Module Status"]
    #[inline(always)]
    pub fn hib_tpstat_state(&mut self) -> _HIB_TPSTAT_STATEW {
        _HIB_TPSTAT_STATEW { w: self }
    }
}
