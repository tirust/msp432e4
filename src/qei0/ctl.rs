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
pub struct QEI_CTL_ENABLER {
    bits: bool,
}
impl QEI_CTL_ENABLER {
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
pub struct _QEI_CTL_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_CTL_ENABLEW<'a> {
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
pub struct QEI_CTL_SWAPR {
    bits: bool,
}
impl QEI_CTL_SWAPR {
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
pub struct _QEI_CTL_SWAPW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_CTL_SWAPW<'a> {
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
pub struct QEI_CTL_SIGMODER {
    bits: bool,
}
impl QEI_CTL_SIGMODER {
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
pub struct _QEI_CTL_SIGMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_CTL_SIGMODEW<'a> {
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
pub struct QEI_CTL_CAPMODER {
    bits: bool,
}
impl QEI_CTL_CAPMODER {
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
pub struct _QEI_CTL_CAPMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_CTL_CAPMODEW<'a> {
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
pub struct QEI_CTL_RESMODER {
    bits: bool,
}
impl QEI_CTL_RESMODER {
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
pub struct _QEI_CTL_RESMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_CTL_RESMODEW<'a> {
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
pub struct QEI_CTL_VELENR {
    bits: bool,
}
impl QEI_CTL_VELENR {
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
pub struct _QEI_CTL_VELENW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_CTL_VELENW<'a> {
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
#[doc = "Possible values of the field `QEI_CTL_VELDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QEI_CTL_VELDIVR {
    #[doc = "QEI clock /1"]
    QEI_CTL_VELDIV_1,
    #[doc = "QEI clock /2"]
    QEI_CTL_VELDIV_2,
    #[doc = "QEI clock /4"]
    QEI_CTL_VELDIV_4,
    #[doc = "QEI clock /8"]
    QEI_CTL_VELDIV_8,
    #[doc = "QEI clock /16"]
    QEI_CTL_VELDIV_16,
    #[doc = "QEI clock /32"]
    QEI_CTL_VELDIV_32,
    #[doc = "QEI clock /64"]
    QEI_CTL_VELDIV_64,
    #[doc = "QEI clock /128"]
    QEI_CTL_VELDIV_128,
}
impl QEI_CTL_VELDIVR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            QEI_CTL_VELDIVR::QEI_CTL_VELDIV_1 => 0,
            QEI_CTL_VELDIVR::QEI_CTL_VELDIV_2 => 1,
            QEI_CTL_VELDIVR::QEI_CTL_VELDIV_4 => 2,
            QEI_CTL_VELDIVR::QEI_CTL_VELDIV_8 => 3,
            QEI_CTL_VELDIVR::QEI_CTL_VELDIV_16 => 4,
            QEI_CTL_VELDIVR::QEI_CTL_VELDIV_32 => 5,
            QEI_CTL_VELDIVR::QEI_CTL_VELDIV_64 => 6,
            QEI_CTL_VELDIVR::QEI_CTL_VELDIV_128 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> QEI_CTL_VELDIVR {
        match value {
            0 => QEI_CTL_VELDIVR::QEI_CTL_VELDIV_1,
            1 => QEI_CTL_VELDIVR::QEI_CTL_VELDIV_2,
            2 => QEI_CTL_VELDIVR::QEI_CTL_VELDIV_4,
            3 => QEI_CTL_VELDIVR::QEI_CTL_VELDIV_8,
            4 => QEI_CTL_VELDIVR::QEI_CTL_VELDIV_16,
            5 => QEI_CTL_VELDIVR::QEI_CTL_VELDIV_32,
            6 => QEI_CTL_VELDIVR::QEI_CTL_VELDIV_64,
            7 => QEI_CTL_VELDIVR::QEI_CTL_VELDIV_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_1`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_1(&self) -> bool {
        *self == QEI_CTL_VELDIVR::QEI_CTL_VELDIV_1
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_2`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_2(&self) -> bool {
        *self == QEI_CTL_VELDIVR::QEI_CTL_VELDIV_2
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_4`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_4(&self) -> bool {
        *self == QEI_CTL_VELDIVR::QEI_CTL_VELDIV_4
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_8`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_8(&self) -> bool {
        *self == QEI_CTL_VELDIVR::QEI_CTL_VELDIV_8
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_16`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_16(&self) -> bool {
        *self == QEI_CTL_VELDIVR::QEI_CTL_VELDIV_16
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_32`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_32(&self) -> bool {
        *self == QEI_CTL_VELDIVR::QEI_CTL_VELDIV_32
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_64`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_64(&self) -> bool {
        *self == QEI_CTL_VELDIVR::QEI_CTL_VELDIV_64
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_128`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_128(&self) -> bool {
        *self == QEI_CTL_VELDIVR::QEI_CTL_VELDIV_128
    }
}
#[doc = "Values that can be written to the field `QEI_CTL_VELDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QEI_CTL_VELDIVW {
    #[doc = "QEI clock /1"]
    QEI_CTL_VELDIV_1,
    #[doc = "QEI clock /2"]
    QEI_CTL_VELDIV_2,
    #[doc = "QEI clock /4"]
    QEI_CTL_VELDIV_4,
    #[doc = "QEI clock /8"]
    QEI_CTL_VELDIV_8,
    #[doc = "QEI clock /16"]
    QEI_CTL_VELDIV_16,
    #[doc = "QEI clock /32"]
    QEI_CTL_VELDIV_32,
    #[doc = "QEI clock /64"]
    QEI_CTL_VELDIV_64,
    #[doc = "QEI clock /128"]
    QEI_CTL_VELDIV_128,
}
impl QEI_CTL_VELDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            QEI_CTL_VELDIVW::QEI_CTL_VELDIV_1 => 0,
            QEI_CTL_VELDIVW::QEI_CTL_VELDIV_2 => 1,
            QEI_CTL_VELDIVW::QEI_CTL_VELDIV_4 => 2,
            QEI_CTL_VELDIVW::QEI_CTL_VELDIV_8 => 3,
            QEI_CTL_VELDIVW::QEI_CTL_VELDIV_16 => 4,
            QEI_CTL_VELDIVW::QEI_CTL_VELDIV_32 => 5,
            QEI_CTL_VELDIVW::QEI_CTL_VELDIV_64 => 6,
            QEI_CTL_VELDIVW::QEI_CTL_VELDIV_128 => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _QEI_CTL_VELDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_CTL_VELDIVW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QEI_CTL_VELDIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "QEI clock /1"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_1(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIVW::QEI_CTL_VELDIV_1)
    }
    #[doc = "QEI clock /2"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_2(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIVW::QEI_CTL_VELDIV_2)
    }
    #[doc = "QEI clock /4"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_4(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIVW::QEI_CTL_VELDIV_4)
    }
    #[doc = "QEI clock /8"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_8(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIVW::QEI_CTL_VELDIV_8)
    }
    #[doc = "QEI clock /16"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_16(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIVW::QEI_CTL_VELDIV_16)
    }
    #[doc = "QEI clock /32"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_32(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIVW::QEI_CTL_VELDIV_32)
    }
    #[doc = "QEI clock /64"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_64(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIVW::QEI_CTL_VELDIV_64)
    }
    #[doc = "QEI clock /128"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_128(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIVW::QEI_CTL_VELDIV_128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 6);
        self.w.bits |= ((value as u32) & 7) << 6;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct QEI_CTL_INVAR {
    bits: bool,
}
impl QEI_CTL_INVAR {
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
pub struct _QEI_CTL_INVAW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_CTL_INVAW<'a> {
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
pub struct QEI_CTL_INVBR {
    bits: bool,
}
impl QEI_CTL_INVBR {
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
pub struct _QEI_CTL_INVBW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_CTL_INVBW<'a> {
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
pub struct QEI_CTL_INVIR {
    bits: bool,
}
impl QEI_CTL_INVIR {
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
pub struct _QEI_CTL_INVIW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_CTL_INVIW<'a> {
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
pub struct QEI_CTL_STALLENR {
    bits: bool,
}
impl QEI_CTL_STALLENR {
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
pub struct _QEI_CTL_STALLENW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_CTL_STALLENW<'a> {
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
pub struct QEI_CTL_FILTENR {
    bits: bool,
}
impl QEI_CTL_FILTENR {
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
pub struct _QEI_CTL_FILTENW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_CTL_FILTENW<'a> {
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
        self.w.bits &= !(1 << 13);
        self.w.bits |= ((value as u32) & 1) << 13;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct QEI_CTL_FILTCNTR {
    bits: u8,
}
impl QEI_CTL_FILTCNTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _QEI_CTL_FILTCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_CTL_FILTCNTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 16);
        self.w.bits |= ((value as u32) & 15) << 16;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable QEI"]
    #[inline(always)]
    pub fn qei_ctl_enable(&self) -> QEI_CTL_ENABLER {
        let bits = ((self.bits >> 0) & 1) != 0;
        QEI_CTL_ENABLER { bits }
    }
    #[doc = "Bit 1 - Swap Signals"]
    #[inline(always)]
    pub fn qei_ctl_swap(&self) -> QEI_CTL_SWAPR {
        let bits = ((self.bits >> 1) & 1) != 0;
        QEI_CTL_SWAPR { bits }
    }
    #[doc = "Bit 2 - Signal Mode"]
    #[inline(always)]
    pub fn qei_ctl_sigmode(&self) -> QEI_CTL_SIGMODER {
        let bits = ((self.bits >> 2) & 1) != 0;
        QEI_CTL_SIGMODER { bits }
    }
    #[doc = "Bit 3 - Capture Mode"]
    #[inline(always)]
    pub fn qei_ctl_capmode(&self) -> QEI_CTL_CAPMODER {
        let bits = ((self.bits >> 3) & 1) != 0;
        QEI_CTL_CAPMODER { bits }
    }
    #[doc = "Bit 4 - Reset Mode"]
    #[inline(always)]
    pub fn qei_ctl_resmode(&self) -> QEI_CTL_RESMODER {
        let bits = ((self.bits >> 4) & 1) != 0;
        QEI_CTL_RESMODER { bits }
    }
    #[doc = "Bit 5 - Capture Velocity"]
    #[inline(always)]
    pub fn qei_ctl_velen(&self) -> QEI_CTL_VELENR {
        let bits = ((self.bits >> 5) & 1) != 0;
        QEI_CTL_VELENR { bits }
    }
    #[doc = "Bits 6:8 - Predivide Velocity"]
    #[inline(always)]
    pub fn qei_ctl_veldiv(&self) -> QEI_CTL_VELDIVR {
        QEI_CTL_VELDIVR::_from(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - Invert PhA"]
    #[inline(always)]
    pub fn qei_ctl_inva(&self) -> QEI_CTL_INVAR {
        let bits = ((self.bits >> 9) & 1) != 0;
        QEI_CTL_INVAR { bits }
    }
    #[doc = "Bit 10 - Invert PhB"]
    #[inline(always)]
    pub fn qei_ctl_invb(&self) -> QEI_CTL_INVBR {
        let bits = ((self.bits >> 10) & 1) != 0;
        QEI_CTL_INVBR { bits }
    }
    #[doc = "Bit 11 - Invert Index Pulse"]
    #[inline(always)]
    pub fn qei_ctl_invi(&self) -> QEI_CTL_INVIR {
        let bits = ((self.bits >> 11) & 1) != 0;
        QEI_CTL_INVIR { bits }
    }
    #[doc = "Bit 12 - Stall QEI"]
    #[inline(always)]
    pub fn qei_ctl_stallen(&self) -> QEI_CTL_STALLENR {
        let bits = ((self.bits >> 12) & 1) != 0;
        QEI_CTL_STALLENR { bits }
    }
    #[doc = "Bit 13 - Enable Input Filter"]
    #[inline(always)]
    pub fn qei_ctl_filten(&self) -> QEI_CTL_FILTENR {
        let bits = ((self.bits >> 13) & 1) != 0;
        QEI_CTL_FILTENR { bits }
    }
    #[doc = "Bits 16:19 - Input Filter Prescale Count"]
    #[inline(always)]
    pub fn qei_ctl_filtcnt(&self) -> QEI_CTL_FILTCNTR {
        let bits = ((self.bits >> 16) & 15) as u8;
        QEI_CTL_FILTCNTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enable QEI"]
    #[inline(always)]
    pub fn qei_ctl_enable(&mut self) -> _QEI_CTL_ENABLEW {
        _QEI_CTL_ENABLEW { w: self }
    }
    #[doc = "Bit 1 - Swap Signals"]
    #[inline(always)]
    pub fn qei_ctl_swap(&mut self) -> _QEI_CTL_SWAPW {
        _QEI_CTL_SWAPW { w: self }
    }
    #[doc = "Bit 2 - Signal Mode"]
    #[inline(always)]
    pub fn qei_ctl_sigmode(&mut self) -> _QEI_CTL_SIGMODEW {
        _QEI_CTL_SIGMODEW { w: self }
    }
    #[doc = "Bit 3 - Capture Mode"]
    #[inline(always)]
    pub fn qei_ctl_capmode(&mut self) -> _QEI_CTL_CAPMODEW {
        _QEI_CTL_CAPMODEW { w: self }
    }
    #[doc = "Bit 4 - Reset Mode"]
    #[inline(always)]
    pub fn qei_ctl_resmode(&mut self) -> _QEI_CTL_RESMODEW {
        _QEI_CTL_RESMODEW { w: self }
    }
    #[doc = "Bit 5 - Capture Velocity"]
    #[inline(always)]
    pub fn qei_ctl_velen(&mut self) -> _QEI_CTL_VELENW {
        _QEI_CTL_VELENW { w: self }
    }
    #[doc = "Bits 6:8 - Predivide Velocity"]
    #[inline(always)]
    pub fn qei_ctl_veldiv(&mut self) -> _QEI_CTL_VELDIVW {
        _QEI_CTL_VELDIVW { w: self }
    }
    #[doc = "Bit 9 - Invert PhA"]
    #[inline(always)]
    pub fn qei_ctl_inva(&mut self) -> _QEI_CTL_INVAW {
        _QEI_CTL_INVAW { w: self }
    }
    #[doc = "Bit 10 - Invert PhB"]
    #[inline(always)]
    pub fn qei_ctl_invb(&mut self) -> _QEI_CTL_INVBW {
        _QEI_CTL_INVBW { w: self }
    }
    #[doc = "Bit 11 - Invert Index Pulse"]
    #[inline(always)]
    pub fn qei_ctl_invi(&mut self) -> _QEI_CTL_INVIW {
        _QEI_CTL_INVIW { w: self }
    }
    #[doc = "Bit 12 - Stall QEI"]
    #[inline(always)]
    pub fn qei_ctl_stallen(&mut self) -> _QEI_CTL_STALLENW {
        _QEI_CTL_STALLENW { w: self }
    }
    #[doc = "Bit 13 - Enable Input Filter"]
    #[inline(always)]
    pub fn qei_ctl_filten(&mut self) -> _QEI_CTL_FILTENW {
        _QEI_CTL_FILTENW { w: self }
    }
    #[doc = "Bits 16:19 - Input Filter Prescale Count"]
    #[inline(always)]
    pub fn qei_ctl_filtcnt(&mut self) -> _QEI_CTL_FILTCNTW {
        _QEI_CTL_FILTCNTW { w: self }
    }
}
