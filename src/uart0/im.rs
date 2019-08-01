#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IM {
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
pub struct UART_IM_RIMIMR {
    bits: bool,
}
impl UART_IM_RIMIMR {
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
pub struct _UART_IM_RIMIMW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_IM_RIMIMW<'a> {
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
pub struct UART_IM_CTSMIMR {
    bits: bool,
}
impl UART_IM_CTSMIMR {
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
pub struct _UART_IM_CTSMIMW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_IM_CTSMIMW<'a> {
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
pub struct UART_IM_DCDMIMR {
    bits: bool,
}
impl UART_IM_DCDMIMR {
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
pub struct _UART_IM_DCDMIMW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_IM_DCDMIMW<'a> {
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
pub struct UART_IM_DSRMIMR {
    bits: bool,
}
impl UART_IM_DSRMIMR {
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
pub struct _UART_IM_DSRMIMW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_IM_DSRMIMW<'a> {
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
pub struct UART_IM_RXIMR {
    bits: bool,
}
impl UART_IM_RXIMR {
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
pub struct _UART_IM_RXIMW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_IM_RXIMW<'a> {
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
pub struct UART_IM_TXIMR {
    bits: bool,
}
impl UART_IM_TXIMR {
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
pub struct _UART_IM_TXIMW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_IM_TXIMW<'a> {
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
pub struct UART_IM_RTIMR {
    bits: bool,
}
impl UART_IM_RTIMR {
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
pub struct _UART_IM_RTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_IM_RTIMW<'a> {
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
pub struct UART_IM_FEIMR {
    bits: bool,
}
impl UART_IM_FEIMR {
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
pub struct _UART_IM_FEIMW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_IM_FEIMW<'a> {
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
pub struct UART_IM_PEIMR {
    bits: bool,
}
impl UART_IM_PEIMR {
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
pub struct _UART_IM_PEIMW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_IM_PEIMW<'a> {
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
pub struct UART_IM_BEIMR {
    bits: bool,
}
impl UART_IM_BEIMR {
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
pub struct _UART_IM_BEIMW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_IM_BEIMW<'a> {
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
pub struct UART_IM_OEIMR {
    bits: bool,
}
impl UART_IM_OEIMR {
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
pub struct _UART_IM_OEIMW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_IM_OEIMW<'a> {
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
#[doc = r"Value of the field"]
pub struct UART_IM_EOTIMR {
    bits: bool,
}
impl UART_IM_EOTIMR {
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
pub struct _UART_IM_EOTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_IM_EOTIMW<'a> {
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
        self.w.bits &= !(1 << 11);
        self.w.bits |= ((value as u32) & 1) << 11;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UART_IM_9BITIMR {
    bits: bool,
}
impl UART_IM_9BITIMR {
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
pub struct _UART_IM_9BITIMW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_IM_9BITIMW<'a> {
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
        self.w.bits &= !(1 << 12);
        self.w.bits |= ((value as u32) & 1) << 12;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UART_IM_DMARXIMR {
    bits: bool,
}
impl UART_IM_DMARXIMR {
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
pub struct _UART_IM_DMARXIMW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_IM_DMARXIMW<'a> {
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
pub struct UART_IM_DMATXIMR {
    bits: bool,
}
impl UART_IM_DMATXIMR {
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
pub struct _UART_IM_DMATXIMW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_IM_DMATXIMW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - UART Ring Indicator Modem Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_rimim(&self) -> UART_IM_RIMIMR {
        let bits = ((self.bits >> 0) & 1) != 0;
        UART_IM_RIMIMR { bits }
    }
    #[doc = "Bit 1 - UART Clear to Send Modem Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_ctsmim(&self) -> UART_IM_CTSMIMR {
        let bits = ((self.bits >> 1) & 1) != 0;
        UART_IM_CTSMIMR { bits }
    }
    #[doc = "Bit 2 - UART Data Carrier Detect Modem Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_dcdmim(&self) -> UART_IM_DCDMIMR {
        let bits = ((self.bits >> 2) & 1) != 0;
        UART_IM_DCDMIMR { bits }
    }
    #[doc = "Bit 3 - UART Data Set Ready Modem Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_dsrmim(&self) -> UART_IM_DSRMIMR {
        let bits = ((self.bits >> 3) & 1) != 0;
        UART_IM_DSRMIMR { bits }
    }
    #[doc = "Bit 4 - UART Receive Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_rxim(&self) -> UART_IM_RXIMR {
        let bits = ((self.bits >> 4) & 1) != 0;
        UART_IM_RXIMR { bits }
    }
    #[doc = "Bit 5 - UART Transmit Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_txim(&self) -> UART_IM_TXIMR {
        let bits = ((self.bits >> 5) & 1) != 0;
        UART_IM_TXIMR { bits }
    }
    #[doc = "Bit 6 - UART Receive Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_rtim(&self) -> UART_IM_RTIMR {
        let bits = ((self.bits >> 6) & 1) != 0;
        UART_IM_RTIMR { bits }
    }
    #[doc = "Bit 7 - UART Framing Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_feim(&self) -> UART_IM_FEIMR {
        let bits = ((self.bits >> 7) & 1) != 0;
        UART_IM_FEIMR { bits }
    }
    #[doc = "Bit 8 - UART Parity Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_peim(&self) -> UART_IM_PEIMR {
        let bits = ((self.bits >> 8) & 1) != 0;
        UART_IM_PEIMR { bits }
    }
    #[doc = "Bit 9 - UART Break Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_beim(&self) -> UART_IM_BEIMR {
        let bits = ((self.bits >> 9) & 1) != 0;
        UART_IM_BEIMR { bits }
    }
    #[doc = "Bit 10 - UART Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_oeim(&self) -> UART_IM_OEIMR {
        let bits = ((self.bits >> 10) & 1) != 0;
        UART_IM_OEIMR { bits }
    }
    #[doc = "Bit 11 - End of Transmission Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_eotim(&self) -> UART_IM_EOTIMR {
        let bits = ((self.bits >> 11) & 1) != 0;
        UART_IM_EOTIMR { bits }
    }
    #[doc = "Bit 12 - 9-Bit Mode Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_9bitim(&self) -> UART_IM_9BITIMR {
        let bits = ((self.bits >> 12) & 1) != 0;
        UART_IM_9BITIMR { bits }
    }
    #[doc = "Bit 16 - Receive DMA Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_dmarxim(&self) -> UART_IM_DMARXIMR {
        let bits = ((self.bits >> 16) & 1) != 0;
        UART_IM_DMARXIMR { bits }
    }
    #[doc = "Bit 17 - Transmit DMA Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_dmatxim(&self) -> UART_IM_DMATXIMR {
        let bits = ((self.bits >> 17) & 1) != 0;
        UART_IM_DMATXIMR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - UART Ring Indicator Modem Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_rimim(&mut self) -> _UART_IM_RIMIMW {
        _UART_IM_RIMIMW { w: self }
    }
    #[doc = "Bit 1 - UART Clear to Send Modem Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_ctsmim(&mut self) -> _UART_IM_CTSMIMW {
        _UART_IM_CTSMIMW { w: self }
    }
    #[doc = "Bit 2 - UART Data Carrier Detect Modem Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_dcdmim(&mut self) -> _UART_IM_DCDMIMW {
        _UART_IM_DCDMIMW { w: self }
    }
    #[doc = "Bit 3 - UART Data Set Ready Modem Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_dsrmim(&mut self) -> _UART_IM_DSRMIMW {
        _UART_IM_DSRMIMW { w: self }
    }
    #[doc = "Bit 4 - UART Receive Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_rxim(&mut self) -> _UART_IM_RXIMW {
        _UART_IM_RXIMW { w: self }
    }
    #[doc = "Bit 5 - UART Transmit Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_txim(&mut self) -> _UART_IM_TXIMW {
        _UART_IM_TXIMW { w: self }
    }
    #[doc = "Bit 6 - UART Receive Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_rtim(&mut self) -> _UART_IM_RTIMW {
        _UART_IM_RTIMW { w: self }
    }
    #[doc = "Bit 7 - UART Framing Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_feim(&mut self) -> _UART_IM_FEIMW {
        _UART_IM_FEIMW { w: self }
    }
    #[doc = "Bit 8 - UART Parity Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_peim(&mut self) -> _UART_IM_PEIMW {
        _UART_IM_PEIMW { w: self }
    }
    #[doc = "Bit 9 - UART Break Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_beim(&mut self) -> _UART_IM_BEIMW {
        _UART_IM_BEIMW { w: self }
    }
    #[doc = "Bit 10 - UART Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_oeim(&mut self) -> _UART_IM_OEIMW {
        _UART_IM_OEIMW { w: self }
    }
    #[doc = "Bit 11 - End of Transmission Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_eotim(&mut self) -> _UART_IM_EOTIMW {
        _UART_IM_EOTIMW { w: self }
    }
    #[doc = "Bit 12 - 9-Bit Mode Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_9bitim(&mut self) -> _UART_IM_9BITIMW {
        _UART_IM_9BITIMW { w: self }
    }
    #[doc = "Bit 16 - Receive DMA Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_dmarxim(&mut self) -> _UART_IM_DMARXIMW {
        _UART_IM_DMARXIMW { w: self }
    }
    #[doc = "Bit 17 - Transmit DMA Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_dmatxim(&mut self) -> _UART_IM_DMATXIMW {
        _UART_IM_DMATXIMW { w: self }
    }
}
