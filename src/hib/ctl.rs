#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTL {
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
pub struct HIB_CTL_RTCENR {
    bits: bool,
}
impl HIB_CTL_RTCENR {
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
pub struct _HIB_CTL_RTCENW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CTL_RTCENW<'a> {
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
pub struct HIB_CTL_HIBREQR {
    bits: bool,
}
impl HIB_CTL_HIBREQR {
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
pub struct _HIB_CTL_HIBREQW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CTL_HIBREQW<'a> {
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
pub struct HIB_CTL_RTCWENR {
    bits: bool,
}
impl HIB_CTL_RTCWENR {
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
pub struct _HIB_CTL_RTCWENW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CTL_RTCWENW<'a> {
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
pub struct HIB_CTL_PINWENR {
    bits: bool,
}
impl HIB_CTL_PINWENR {
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
pub struct _HIB_CTL_PINWENW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CTL_PINWENW<'a> {
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
pub struct HIB_CTL_CLK32ENR {
    bits: bool,
}
impl HIB_CTL_CLK32ENR {
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
pub struct _HIB_CTL_CLK32ENW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CTL_CLK32ENW<'a> {
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
        self.w.bits &= !(1 << 6);
        self.w.bits |= ((value as u32) & 1) << 6;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_CTL_VABORTR {
    bits: bool,
}
impl HIB_CTL_VABORTR {
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
pub struct _HIB_CTL_VABORTW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CTL_VABORTW<'a> {
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
        self.w.bits &= !(1 << 7);
        self.w.bits |= ((value as u32) & 1) << 7;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_CTL_VDD3ONR {
    bits: bool,
}
impl HIB_CTL_VDD3ONR {
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
pub struct _HIB_CTL_VDD3ONW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CTL_VDD3ONW<'a> {
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
#[doc = r"Value of the field"]
pub struct HIB_CTL_BATWKENR {
    bits: bool,
}
impl HIB_CTL_BATWKENR {
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
pub struct _HIB_CTL_BATWKENW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CTL_BATWKENW<'a> {
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
        self.w.bits &= !(1 << 9);
        self.w.bits |= ((value as u32) & 1) << 9;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_CTL_BATCHKR {
    bits: bool,
}
impl HIB_CTL_BATCHKR {
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
pub struct _HIB_CTL_BATCHKW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CTL_BATCHKW<'a> {
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
        self.w.bits &= !(1 << 10);
        self.w.bits |= ((value as u32) & 1) << 10;
        self.w
    }
}
#[doc = "Possible values of the field `HIB_CTL_VBATSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIB_CTL_VBATSELR {
    #[doc = "1.9 Volts"]
    HIB_CTL_VBATSEL_1_9V,
    #[doc = "2.1 Volts (default)"]
    HIB_CTL_VBATSEL_2_1V,
    #[doc = "2.3 Volts"]
    HIB_CTL_VBATSEL_2_3V,
    #[doc = "2.5 Volts"]
    HIB_CTL_VBATSEL_2_5V,
}
impl HIB_CTL_VBATSELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            HIB_CTL_VBATSELR::HIB_CTL_VBATSEL_1_9V => 0,
            HIB_CTL_VBATSELR::HIB_CTL_VBATSEL_2_1V => 1,
            HIB_CTL_VBATSELR::HIB_CTL_VBATSEL_2_3V => 2,
            HIB_CTL_VBATSELR::HIB_CTL_VBATSEL_2_5V => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> HIB_CTL_VBATSELR {
        match value {
            0 => HIB_CTL_VBATSELR::HIB_CTL_VBATSEL_1_9V,
            1 => HIB_CTL_VBATSELR::HIB_CTL_VBATSEL_2_1V,
            2 => HIB_CTL_VBATSELR::HIB_CTL_VBATSEL_2_3V,
            3 => HIB_CTL_VBATSELR::HIB_CTL_VBATSEL_2_5V,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HIB_CTL_VBATSEL_1_9V`"]
    #[inline(always)]
    pub fn is_hib_ctl_vbatsel_1_9v(&self) -> bool {
        *self == HIB_CTL_VBATSELR::HIB_CTL_VBATSEL_1_9V
    }
    #[doc = "Checks if the value of the field is `HIB_CTL_VBATSEL_2_1V`"]
    #[inline(always)]
    pub fn is_hib_ctl_vbatsel_2_1v(&self) -> bool {
        *self == HIB_CTL_VBATSELR::HIB_CTL_VBATSEL_2_1V
    }
    #[doc = "Checks if the value of the field is `HIB_CTL_VBATSEL_2_3V`"]
    #[inline(always)]
    pub fn is_hib_ctl_vbatsel_2_3v(&self) -> bool {
        *self == HIB_CTL_VBATSELR::HIB_CTL_VBATSEL_2_3V
    }
    #[doc = "Checks if the value of the field is `HIB_CTL_VBATSEL_2_5V`"]
    #[inline(always)]
    pub fn is_hib_ctl_vbatsel_2_5v(&self) -> bool {
        *self == HIB_CTL_VBATSELR::HIB_CTL_VBATSEL_2_5V
    }
}
#[doc = "Values that can be written to the field `HIB_CTL_VBATSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIB_CTL_VBATSELW {
    #[doc = "1.9 Volts"]
    HIB_CTL_VBATSEL_1_9V,
    #[doc = "2.1 Volts (default)"]
    HIB_CTL_VBATSEL_2_1V,
    #[doc = "2.3 Volts"]
    HIB_CTL_VBATSEL_2_3V,
    #[doc = "2.5 Volts"]
    HIB_CTL_VBATSEL_2_5V,
}
impl HIB_CTL_VBATSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            HIB_CTL_VBATSELW::HIB_CTL_VBATSEL_1_9V => 0,
            HIB_CTL_VBATSELW::HIB_CTL_VBATSEL_2_1V => 1,
            HIB_CTL_VBATSELW::HIB_CTL_VBATSEL_2_3V => 2,
            HIB_CTL_VBATSELW::HIB_CTL_VBATSEL_2_5V => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _HIB_CTL_VBATSELW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CTL_VBATSELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIB_CTL_VBATSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1.9 Volts"]
    #[inline(always)]
    pub fn hib_ctl_vbatsel_1_9v(self) -> &'a mut W {
        self.variant(HIB_CTL_VBATSELW::HIB_CTL_VBATSEL_1_9V)
    }
    #[doc = "2.1 Volts (default)"]
    #[inline(always)]
    pub fn hib_ctl_vbatsel_2_1v(self) -> &'a mut W {
        self.variant(HIB_CTL_VBATSELW::HIB_CTL_VBATSEL_2_1V)
    }
    #[doc = "2.3 Volts"]
    #[inline(always)]
    pub fn hib_ctl_vbatsel_2_3v(self) -> &'a mut W {
        self.variant(HIB_CTL_VBATSELW::HIB_CTL_VBATSEL_2_3V)
    }
    #[doc = "2.5 Volts"]
    #[inline(always)]
    pub fn hib_ctl_vbatsel_2_5v(self) -> &'a mut W {
        self.variant(HIB_CTL_VBATSELW::HIB_CTL_VBATSEL_2_5V)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 13);
        self.w.bits |= ((value as u32) & 3) << 13;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_CTL_OSCBYPR {
    bits: bool,
}
impl HIB_CTL_OSCBYPR {
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
pub struct _HIB_CTL_OSCBYPW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CTL_OSCBYPW<'a> {
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
        self.w.bits &= !(1 << 16);
        self.w.bits |= ((value as u32) & 1) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_CTL_OSCDRVR {
    bits: bool,
}
impl HIB_CTL_OSCDRVR {
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
pub struct _HIB_CTL_OSCDRVW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CTL_OSCDRVW<'a> {
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
        self.w.bits &= !(1 << 17);
        self.w.bits |= ((value as u32) & 1) << 17;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_CTL_OSCSELR {
    bits: bool,
}
impl HIB_CTL_OSCSELR {
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
pub struct _HIB_CTL_OSCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CTL_OSCSELW<'a> {
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
        self.w.bits &= !(1 << 19);
        self.w.bits |= ((value as u32) & 1) << 19;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_CTL_RETCLRR {
    bits: bool,
}
impl HIB_CTL_RETCLRR {
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
pub struct _HIB_CTL_RETCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CTL_RETCLRW<'a> {
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
        self.w.bits &= !(1 << 30);
        self.w.bits |= ((value as u32) & 1) << 30;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_CTL_WRCR {
    bits: bool,
}
impl HIB_CTL_WRCR {
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
pub struct _HIB_CTL_WRCW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CTL_WRCW<'a> {
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
        self.w.bits &= !(1 << 31);
        self.w.bits |= ((value as u32) & 1) << 31;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - RTC Timer Enable"]
    #[inline(always)]
    pub fn hib_ctl_rtcen(&self) -> HIB_CTL_RTCENR {
        let bits = ((self.bits >> 0) & 1) != 0;
        HIB_CTL_RTCENR { bits }
    }
    #[doc = "Bit 1 - Hibernation Request"]
    #[inline(always)]
    pub fn hib_ctl_hibreq(&self) -> HIB_CTL_HIBREQR {
        let bits = ((self.bits >> 1) & 1) != 0;
        HIB_CTL_HIBREQR { bits }
    }
    #[doc = "Bit 3 - RTC Wake-up Enable"]
    #[inline(always)]
    pub fn hib_ctl_rtcwen(&self) -> HIB_CTL_RTCWENR {
        let bits = ((self.bits >> 3) & 1) != 0;
        HIB_CTL_RTCWENR { bits }
    }
    #[doc = "Bit 4 - External Wake Pin Enable"]
    #[inline(always)]
    pub fn hib_ctl_pinwen(&self) -> HIB_CTL_PINWENR {
        let bits = ((self.bits >> 4) & 1) != 0;
        HIB_CTL_PINWENR { bits }
    }
    #[doc = "Bit 6 - Clocking Enable"]
    #[inline(always)]
    pub fn hib_ctl_clk32en(&self) -> HIB_CTL_CLK32ENR {
        let bits = ((self.bits >> 6) & 1) != 0;
        HIB_CTL_CLK32ENR { bits }
    }
    #[doc = "Bit 7 - Power Cut Abort Enable"]
    #[inline(always)]
    pub fn hib_ctl_vabort(&self) -> HIB_CTL_VABORTR {
        let bits = ((self.bits >> 7) & 1) != 0;
        HIB_CTL_VABORTR { bits }
    }
    #[doc = "Bit 8 - VDD Powered"]
    #[inline(always)]
    pub fn hib_ctl_vdd3on(&self) -> HIB_CTL_VDD3ONR {
        let bits = ((self.bits >> 8) & 1) != 0;
        HIB_CTL_VDD3ONR { bits }
    }
    #[doc = "Bit 9 - Wake on Low Battery"]
    #[inline(always)]
    pub fn hib_ctl_batwken(&self) -> HIB_CTL_BATWKENR {
        let bits = ((self.bits >> 9) & 1) != 0;
        HIB_CTL_BATWKENR { bits }
    }
    #[doc = "Bit 10 - Check Battery Status"]
    #[inline(always)]
    pub fn hib_ctl_batchk(&self) -> HIB_CTL_BATCHKR {
        let bits = ((self.bits >> 10) & 1) != 0;
        HIB_CTL_BATCHKR { bits }
    }
    #[doc = "Bits 13:14 - Select for Low-Battery Comparator"]
    #[inline(always)]
    pub fn hib_ctl_vbatsel(&self) -> HIB_CTL_VBATSELR {
        HIB_CTL_VBATSELR::_from(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 16 - Oscillator Bypass"]
    #[inline(always)]
    pub fn hib_ctl_oscbyp(&self) -> HIB_CTL_OSCBYPR {
        let bits = ((self.bits >> 16) & 1) != 0;
        HIB_CTL_OSCBYPR { bits }
    }
    #[doc = "Bit 17 - Oscillator Drive Capability"]
    #[inline(always)]
    pub fn hib_ctl_oscdrv(&self) -> HIB_CTL_OSCDRVR {
        let bits = ((self.bits >> 17) & 1) != 0;
        HIB_CTL_OSCDRVR { bits }
    }
    #[doc = "Bit 19 - Oscillator Select"]
    #[inline(always)]
    pub fn hib_ctl_oscsel(&self) -> HIB_CTL_OSCSELR {
        let bits = ((self.bits >> 19) & 1) != 0;
        HIB_CTL_OSCSELR { bits }
    }
    #[doc = "Bit 30 - GPIO Retention/Clear"]
    #[inline(always)]
    pub fn hib_ctl_retclr(&self) -> HIB_CTL_RETCLRR {
        let bits = ((self.bits >> 30) & 1) != 0;
        HIB_CTL_RETCLRR { bits }
    }
    #[doc = "Bit 31 - Write Complete/Capable"]
    #[inline(always)]
    pub fn hib_ctl_wrc(&self) -> HIB_CTL_WRCR {
        let bits = ((self.bits >> 31) & 1) != 0;
        HIB_CTL_WRCR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - RTC Timer Enable"]
    #[inline(always)]
    pub fn hib_ctl_rtcen(&mut self) -> _HIB_CTL_RTCENW {
        _HIB_CTL_RTCENW { w: self }
    }
    #[doc = "Bit 1 - Hibernation Request"]
    #[inline(always)]
    pub fn hib_ctl_hibreq(&mut self) -> _HIB_CTL_HIBREQW {
        _HIB_CTL_HIBREQW { w: self }
    }
    #[doc = "Bit 3 - RTC Wake-up Enable"]
    #[inline(always)]
    pub fn hib_ctl_rtcwen(&mut self) -> _HIB_CTL_RTCWENW {
        _HIB_CTL_RTCWENW { w: self }
    }
    #[doc = "Bit 4 - External Wake Pin Enable"]
    #[inline(always)]
    pub fn hib_ctl_pinwen(&mut self) -> _HIB_CTL_PINWENW {
        _HIB_CTL_PINWENW { w: self }
    }
    #[doc = "Bit 6 - Clocking Enable"]
    #[inline(always)]
    pub fn hib_ctl_clk32en(&mut self) -> _HIB_CTL_CLK32ENW {
        _HIB_CTL_CLK32ENW { w: self }
    }
    #[doc = "Bit 7 - Power Cut Abort Enable"]
    #[inline(always)]
    pub fn hib_ctl_vabort(&mut self) -> _HIB_CTL_VABORTW {
        _HIB_CTL_VABORTW { w: self }
    }
    #[doc = "Bit 8 - VDD Powered"]
    #[inline(always)]
    pub fn hib_ctl_vdd3on(&mut self) -> _HIB_CTL_VDD3ONW {
        _HIB_CTL_VDD3ONW { w: self }
    }
    #[doc = "Bit 9 - Wake on Low Battery"]
    #[inline(always)]
    pub fn hib_ctl_batwken(&mut self) -> _HIB_CTL_BATWKENW {
        _HIB_CTL_BATWKENW { w: self }
    }
    #[doc = "Bit 10 - Check Battery Status"]
    #[inline(always)]
    pub fn hib_ctl_batchk(&mut self) -> _HIB_CTL_BATCHKW {
        _HIB_CTL_BATCHKW { w: self }
    }
    #[doc = "Bits 13:14 - Select for Low-Battery Comparator"]
    #[inline(always)]
    pub fn hib_ctl_vbatsel(&mut self) -> _HIB_CTL_VBATSELW {
        _HIB_CTL_VBATSELW { w: self }
    }
    #[doc = "Bit 16 - Oscillator Bypass"]
    #[inline(always)]
    pub fn hib_ctl_oscbyp(&mut self) -> _HIB_CTL_OSCBYPW {
        _HIB_CTL_OSCBYPW { w: self }
    }
    #[doc = "Bit 17 - Oscillator Drive Capability"]
    #[inline(always)]
    pub fn hib_ctl_oscdrv(&mut self) -> _HIB_CTL_OSCDRVW {
        _HIB_CTL_OSCDRVW { w: self }
    }
    #[doc = "Bit 19 - Oscillator Select"]
    #[inline(always)]
    pub fn hib_ctl_oscsel(&mut self) -> _HIB_CTL_OSCSELW {
        _HIB_CTL_OSCSELW { w: self }
    }
    #[doc = "Bit 30 - GPIO Retention/Clear"]
    #[inline(always)]
    pub fn hib_ctl_retclr(&mut self) -> _HIB_CTL_RETCLRW {
        _HIB_CTL_RETCLRW { w: self }
    }
    #[doc = "Bit 31 - Write Complete/Capable"]
    #[inline(always)]
    pub fn hib_ctl_wrc(&mut self) -> _HIB_CTL_WRCW {
        _HIB_CTL_WRCW { w: self }
    }
}
