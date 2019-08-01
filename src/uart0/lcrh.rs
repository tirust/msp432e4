#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LCRH {
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
pub struct UART_LCRH_BRKR {
    bits: bool,
}
impl UART_LCRH_BRKR {
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
pub struct _UART_LCRH_BRKW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_LCRH_BRKW<'a> {
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
pub struct UART_LCRH_PENR {
    bits: bool,
}
impl UART_LCRH_PENR {
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
pub struct _UART_LCRH_PENW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_LCRH_PENW<'a> {
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
pub struct UART_LCRH_EPSR {
    bits: bool,
}
impl UART_LCRH_EPSR {
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
pub struct _UART_LCRH_EPSW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_LCRH_EPSW<'a> {
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
pub struct UART_LCRH_STP2R {
    bits: bool,
}
impl UART_LCRH_STP2R {
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
pub struct _UART_LCRH_STP2W<'a> {
    w: &'a mut W,
}
impl<'a> _UART_LCRH_STP2W<'a> {
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
pub struct UART_LCRH_FENR {
    bits: bool,
}
impl UART_LCRH_FENR {
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
pub struct _UART_LCRH_FENW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_LCRH_FENW<'a> {
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
#[doc = "Possible values of the field `UART_LCRH_WLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_LCRH_WLENR {
    #[doc = "5 bits (default)"]
    UART_LCRH_WLEN_5,
    #[doc = "6 bits"]
    UART_LCRH_WLEN_6,
    #[doc = "7 bits"]
    UART_LCRH_WLEN_7,
    #[doc = "8 bits"]
    UART_LCRH_WLEN_8,
}
impl UART_LCRH_WLENR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            UART_LCRH_WLENR::UART_LCRH_WLEN_5 => 0,
            UART_LCRH_WLENR::UART_LCRH_WLEN_6 => 1,
            UART_LCRH_WLENR::UART_LCRH_WLEN_7 => 2,
            UART_LCRH_WLENR::UART_LCRH_WLEN_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> UART_LCRH_WLENR {
        match value {
            0 => UART_LCRH_WLENR::UART_LCRH_WLEN_5,
            1 => UART_LCRH_WLENR::UART_LCRH_WLEN_6,
            2 => UART_LCRH_WLENR::UART_LCRH_WLEN_7,
            3 => UART_LCRH_WLENR::UART_LCRH_WLEN_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UART_LCRH_WLEN_5`"]
    #[inline(always)]
    pub fn is_uart_lcrh_wlen_5(&self) -> bool {
        *self == UART_LCRH_WLENR::UART_LCRH_WLEN_5
    }
    #[doc = "Checks if the value of the field is `UART_LCRH_WLEN_6`"]
    #[inline(always)]
    pub fn is_uart_lcrh_wlen_6(&self) -> bool {
        *self == UART_LCRH_WLENR::UART_LCRH_WLEN_6
    }
    #[doc = "Checks if the value of the field is `UART_LCRH_WLEN_7`"]
    #[inline(always)]
    pub fn is_uart_lcrh_wlen_7(&self) -> bool {
        *self == UART_LCRH_WLENR::UART_LCRH_WLEN_7
    }
    #[doc = "Checks if the value of the field is `UART_LCRH_WLEN_8`"]
    #[inline(always)]
    pub fn is_uart_lcrh_wlen_8(&self) -> bool {
        *self == UART_LCRH_WLENR::UART_LCRH_WLEN_8
    }
}
#[doc = "Values that can be written to the field `UART_LCRH_WLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_LCRH_WLENW {
    #[doc = "5 bits (default)"]
    UART_LCRH_WLEN_5,
    #[doc = "6 bits"]
    UART_LCRH_WLEN_6,
    #[doc = "7 bits"]
    UART_LCRH_WLEN_7,
    #[doc = "8 bits"]
    UART_LCRH_WLEN_8,
}
impl UART_LCRH_WLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            UART_LCRH_WLENW::UART_LCRH_WLEN_5 => 0,
            UART_LCRH_WLENW::UART_LCRH_WLEN_6 => 1,
            UART_LCRH_WLENW::UART_LCRH_WLEN_7 => 2,
            UART_LCRH_WLENW::UART_LCRH_WLEN_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _UART_LCRH_WLENW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_LCRH_WLENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART_LCRH_WLENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "5 bits (default)"]
    #[inline(always)]
    pub fn uart_lcrh_wlen_5(self) -> &'a mut W {
        self.variant(UART_LCRH_WLENW::UART_LCRH_WLEN_5)
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn uart_lcrh_wlen_6(self) -> &'a mut W {
        self.variant(UART_LCRH_WLENW::UART_LCRH_WLEN_6)
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn uart_lcrh_wlen_7(self) -> &'a mut W {
        self.variant(UART_LCRH_WLENW::UART_LCRH_WLEN_7)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn uart_lcrh_wlen_8(self) -> &'a mut W {
        self.variant(UART_LCRH_WLENW::UART_LCRH_WLEN_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 5);
        self.w.bits |= ((value as u32) & 3) << 5;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UART_LCRH_SPSR {
    bits: bool,
}
impl UART_LCRH_SPSR {
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
pub struct _UART_LCRH_SPSW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_LCRH_SPSW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - UART Send Break"]
    #[inline(always)]
    pub fn uart_lcrh_brk(&self) -> UART_LCRH_BRKR {
        let bits = ((self.bits >> 0) & 1) != 0;
        UART_LCRH_BRKR { bits }
    }
    #[doc = "Bit 1 - UART Parity Enable"]
    #[inline(always)]
    pub fn uart_lcrh_pen(&self) -> UART_LCRH_PENR {
        let bits = ((self.bits >> 1) & 1) != 0;
        UART_LCRH_PENR { bits }
    }
    #[doc = "Bit 2 - UART Even Parity Select"]
    #[inline(always)]
    pub fn uart_lcrh_eps(&self) -> UART_LCRH_EPSR {
        let bits = ((self.bits >> 2) & 1) != 0;
        UART_LCRH_EPSR { bits }
    }
    #[doc = "Bit 3 - UART Two Stop Bits Select"]
    #[inline(always)]
    pub fn uart_lcrh_stp2(&self) -> UART_LCRH_STP2R {
        let bits = ((self.bits >> 3) & 1) != 0;
        UART_LCRH_STP2R { bits }
    }
    #[doc = "Bit 4 - UART Enable FIFOs"]
    #[inline(always)]
    pub fn uart_lcrh_fen(&self) -> UART_LCRH_FENR {
        let bits = ((self.bits >> 4) & 1) != 0;
        UART_LCRH_FENR { bits }
    }
    #[doc = "Bits 5:6 - UART Word Length"]
    #[inline(always)]
    pub fn uart_lcrh_wlen(&self) -> UART_LCRH_WLENR {
        UART_LCRH_WLENR::_from(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - UART Stick Parity Select"]
    #[inline(always)]
    pub fn uart_lcrh_sps(&self) -> UART_LCRH_SPSR {
        let bits = ((self.bits >> 7) & 1) != 0;
        UART_LCRH_SPSR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - UART Send Break"]
    #[inline(always)]
    pub fn uart_lcrh_brk(&mut self) -> _UART_LCRH_BRKW {
        _UART_LCRH_BRKW { w: self }
    }
    #[doc = "Bit 1 - UART Parity Enable"]
    #[inline(always)]
    pub fn uart_lcrh_pen(&mut self) -> _UART_LCRH_PENW {
        _UART_LCRH_PENW { w: self }
    }
    #[doc = "Bit 2 - UART Even Parity Select"]
    #[inline(always)]
    pub fn uart_lcrh_eps(&mut self) -> _UART_LCRH_EPSW {
        _UART_LCRH_EPSW { w: self }
    }
    #[doc = "Bit 3 - UART Two Stop Bits Select"]
    #[inline(always)]
    pub fn uart_lcrh_stp2(&mut self) -> _UART_LCRH_STP2W {
        _UART_LCRH_STP2W { w: self }
    }
    #[doc = "Bit 4 - UART Enable FIFOs"]
    #[inline(always)]
    pub fn uart_lcrh_fen(&mut self) -> _UART_LCRH_FENW {
        _UART_LCRH_FENW { w: self }
    }
    #[doc = "Bits 5:6 - UART Word Length"]
    #[inline(always)]
    pub fn uart_lcrh_wlen(&mut self) -> _UART_LCRH_WLENW {
        _UART_LCRH_WLENW { w: self }
    }
    #[doc = "Bit 7 - UART Stick Parity Select"]
    #[inline(always)]
    pub fn uart_lcrh_sps(&mut self) -> _UART_LCRH_SPSW {
        _UART_LCRH_SPSW { w: self }
    }
}
