#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `CRC_CTRL_TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_CTRL_TYPER {
    #[doc = "Polynomial 0x8005"]
    CRC_CTRL_TYPE_P8055,
    #[doc = "Polynomial 0x1021"]
    CRC_CTRL_TYPE_P1021,
    #[doc = "Polynomial 0x4C11DB7"]
    CRC_CTRL_TYPE_P4C11DB7,
    #[doc = "Polynomial 0x1EDC6F41"]
    CRC_CTRL_TYPE_P1EDC6F41,
    #[doc = "TCP checksum"]
    CRC_CTRL_TYPE_TCPCHKSUM,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl CRC_CTRL_TYPER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            CRC_CTRL_TYPER::CRC_CTRL_TYPE_P8055 => 0,
            CRC_CTRL_TYPER::CRC_CTRL_TYPE_P1021 => 1,
            CRC_CTRL_TYPER::CRC_CTRL_TYPE_P4C11DB7 => 2,
            CRC_CTRL_TYPER::CRC_CTRL_TYPE_P1EDC6F41 => 3,
            CRC_CTRL_TYPER::CRC_CTRL_TYPE_TCPCHKSUM => 8,
            CRC_CTRL_TYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> CRC_CTRL_TYPER {
        match value {
            0 => CRC_CTRL_TYPER::CRC_CTRL_TYPE_P8055,
            1 => CRC_CTRL_TYPER::CRC_CTRL_TYPE_P1021,
            2 => CRC_CTRL_TYPER::CRC_CTRL_TYPE_P4C11DB7,
            3 => CRC_CTRL_TYPER::CRC_CTRL_TYPE_P1EDC6F41,
            8 => CRC_CTRL_TYPER::CRC_CTRL_TYPE_TCPCHKSUM,
            i => CRC_CTRL_TYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_TYPE_P8055`"]
    #[inline(always)]
    pub fn is_crc_ctrl_type_p8055(&self) -> bool {
        *self == CRC_CTRL_TYPER::CRC_CTRL_TYPE_P8055
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_TYPE_P1021`"]
    #[inline(always)]
    pub fn is_crc_ctrl_type_p1021(&self) -> bool {
        *self == CRC_CTRL_TYPER::CRC_CTRL_TYPE_P1021
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_TYPE_P4C11DB7`"]
    #[inline(always)]
    pub fn is_crc_ctrl_type_p4c11db7(&self) -> bool {
        *self == CRC_CTRL_TYPER::CRC_CTRL_TYPE_P4C11DB7
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_TYPE_P1EDC6F41`"]
    #[inline(always)]
    pub fn is_crc_ctrl_type_p1edc6f41(&self) -> bool {
        *self == CRC_CTRL_TYPER::CRC_CTRL_TYPE_P1EDC6F41
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_TYPE_TCPCHKSUM`"]
    #[inline(always)]
    pub fn is_crc_ctrl_type_tcpchksum(&self) -> bool {
        *self == CRC_CTRL_TYPER::CRC_CTRL_TYPE_TCPCHKSUM
    }
}
#[doc = "Values that can be written to the field `CRC_CTRL_TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_CTRL_TYPEW {
    #[doc = "Polynomial 0x8005"]
    CRC_CTRL_TYPE_P8055,
    #[doc = "Polynomial 0x1021"]
    CRC_CTRL_TYPE_P1021,
    #[doc = "Polynomial 0x4C11DB7"]
    CRC_CTRL_TYPE_P4C11DB7,
    #[doc = "Polynomial 0x1EDC6F41"]
    CRC_CTRL_TYPE_P1EDC6F41,
    #[doc = "TCP checksum"]
    CRC_CTRL_TYPE_TCPCHKSUM,
}
impl CRC_CTRL_TYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CRC_CTRL_TYPEW::CRC_CTRL_TYPE_P8055 => 0,
            CRC_CTRL_TYPEW::CRC_CTRL_TYPE_P1021 => 1,
            CRC_CTRL_TYPEW::CRC_CTRL_TYPE_P4C11DB7 => 2,
            CRC_CTRL_TYPEW::CRC_CTRL_TYPE_P1EDC6F41 => 3,
            CRC_CTRL_TYPEW::CRC_CTRL_TYPE_TCPCHKSUM => 8,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CRC_CTRL_TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_CTRL_TYPEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_CTRL_TYPEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Polynomial 0x8005"]
    #[inline(always)]
    pub fn crc_ctrl_type_p8055(self) -> &'a mut W {
        self.variant(CRC_CTRL_TYPEW::CRC_CTRL_TYPE_P8055)
    }
    #[doc = "Polynomial 0x1021"]
    #[inline(always)]
    pub fn crc_ctrl_type_p1021(self) -> &'a mut W {
        self.variant(CRC_CTRL_TYPEW::CRC_CTRL_TYPE_P1021)
    }
    #[doc = "Polynomial 0x4C11DB7"]
    #[inline(always)]
    pub fn crc_ctrl_type_p4c11db7(self) -> &'a mut W {
        self.variant(CRC_CTRL_TYPEW::CRC_CTRL_TYPE_P4C11DB7)
    }
    #[doc = "Polynomial 0x1EDC6F41"]
    #[inline(always)]
    pub fn crc_ctrl_type_p1edc6f41(self) -> &'a mut W {
        self.variant(CRC_CTRL_TYPEW::CRC_CTRL_TYPE_P1EDC6F41)
    }
    #[doc = "TCP checksum"]
    #[inline(always)]
    pub fn crc_ctrl_type_tcpchksum(self) -> &'a mut W {
        self.variant(CRC_CTRL_TYPEW::CRC_CTRL_TYPE_TCPCHKSUM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `CRC_CTRL_ENDIAN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_CTRL_ENDIANR {
    #[doc = "Configuration unchanged. (B3, B2, B1, B0)"]
    CRC_CTRL_ENDIAN_SBHW,
    #[doc = "Bytes are swapped in half-words but half-words are not swapped (B2, B3, B0, B1)"]
    CRC_CTRL_ENDIAN_SHW,
    #[doc = "Half-words are swapped but bytes are not swapped in half-word. (B1, B0, B3, B2)"]
    CRC_CTRL_ENDIAN_SHWNB,
    #[doc = "Bytes are swapped in half-words and half-words are swapped. (B0, B1, B2, B3)"]
    CRC_CTRL_ENDIAN_SBSW,
}
impl CRC_CTRL_ENDIANR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            CRC_CTRL_ENDIANR::CRC_CTRL_ENDIAN_SBHW => 0,
            CRC_CTRL_ENDIANR::CRC_CTRL_ENDIAN_SHW => 1,
            CRC_CTRL_ENDIANR::CRC_CTRL_ENDIAN_SHWNB => 2,
            CRC_CTRL_ENDIANR::CRC_CTRL_ENDIAN_SBSW => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> CRC_CTRL_ENDIANR {
        match value {
            0 => CRC_CTRL_ENDIANR::CRC_CTRL_ENDIAN_SBHW,
            1 => CRC_CTRL_ENDIANR::CRC_CTRL_ENDIAN_SHW,
            2 => CRC_CTRL_ENDIANR::CRC_CTRL_ENDIAN_SHWNB,
            3 => CRC_CTRL_ENDIANR::CRC_CTRL_ENDIAN_SBSW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_ENDIAN_SBHW`"]
    #[inline(always)]
    pub fn is_crc_ctrl_endian_sbhw(&self) -> bool {
        *self == CRC_CTRL_ENDIANR::CRC_CTRL_ENDIAN_SBHW
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_ENDIAN_SHW`"]
    #[inline(always)]
    pub fn is_crc_ctrl_endian_shw(&self) -> bool {
        *self == CRC_CTRL_ENDIANR::CRC_CTRL_ENDIAN_SHW
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_ENDIAN_SHWNB`"]
    #[inline(always)]
    pub fn is_crc_ctrl_endian_shwnb(&self) -> bool {
        *self == CRC_CTRL_ENDIANR::CRC_CTRL_ENDIAN_SHWNB
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_ENDIAN_SBSW`"]
    #[inline(always)]
    pub fn is_crc_ctrl_endian_sbsw(&self) -> bool {
        *self == CRC_CTRL_ENDIANR::CRC_CTRL_ENDIAN_SBSW
    }
}
#[doc = "Values that can be written to the field `CRC_CTRL_ENDIAN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_CTRL_ENDIANW {
    #[doc = "Configuration unchanged. (B3, B2, B1, B0)"]
    CRC_CTRL_ENDIAN_SBHW,
    #[doc = "Bytes are swapped in half-words but half-words are not swapped (B2, B3, B0, B1)"]
    CRC_CTRL_ENDIAN_SHW,
    #[doc = "Half-words are swapped but bytes are not swapped in half-word. (B1, B0, B3, B2)"]
    CRC_CTRL_ENDIAN_SHWNB,
    #[doc = "Bytes are swapped in half-words and half-words are swapped. (B0, B1, B2, B3)"]
    CRC_CTRL_ENDIAN_SBSW,
}
impl CRC_CTRL_ENDIANW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CRC_CTRL_ENDIANW::CRC_CTRL_ENDIAN_SBHW => 0,
            CRC_CTRL_ENDIANW::CRC_CTRL_ENDIAN_SHW => 1,
            CRC_CTRL_ENDIANW::CRC_CTRL_ENDIAN_SHWNB => 2,
            CRC_CTRL_ENDIANW::CRC_CTRL_ENDIAN_SBSW => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CRC_CTRL_ENDIANW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_CTRL_ENDIANW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_CTRL_ENDIANW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Configuration unchanged. (B3, B2, B1, B0)"]
    #[inline(always)]
    pub fn crc_ctrl_endian_sbhw(self) -> &'a mut W {
        self.variant(CRC_CTRL_ENDIANW::CRC_CTRL_ENDIAN_SBHW)
    }
    #[doc = "Bytes are swapped in half-words but half-words are not swapped (B2, B3, B0, B1)"]
    #[inline(always)]
    pub fn crc_ctrl_endian_shw(self) -> &'a mut W {
        self.variant(CRC_CTRL_ENDIANW::CRC_CTRL_ENDIAN_SHW)
    }
    #[doc = "Half-words are swapped but bytes are not swapped in half-word. (B1, B0, B3, B2)"]
    #[inline(always)]
    pub fn crc_ctrl_endian_shwnb(self) -> &'a mut W {
        self.variant(CRC_CTRL_ENDIANW::CRC_CTRL_ENDIAN_SHWNB)
    }
    #[doc = "Bytes are swapped in half-words and half-words are swapped. (B0, B1, B2, B3)"]
    #[inline(always)]
    pub fn crc_ctrl_endian_sbsw(self) -> &'a mut W {
        self.variant(CRC_CTRL_ENDIANW::CRC_CTRL_ENDIAN_SBSW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct CRC_CTRL_BRR {
    bits: bool,
}
impl CRC_CTRL_BRR {
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
pub struct _CRC_CTRL_BRW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_CTRL_BRW<'a> {
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
pub struct CRC_CTRL_OBRR {
    bits: bool,
}
impl CRC_CTRL_OBRR {
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
pub struct _CRC_CTRL_OBRW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_CTRL_OBRW<'a> {
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
pub struct CRC_CTRL_RESINVR {
    bits: bool,
}
impl CRC_CTRL_RESINVR {
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
pub struct _CRC_CTRL_RESINVW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_CTRL_RESINVW<'a> {
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
pub struct CRC_CTRL_SIZER {
    bits: bool,
}
impl CRC_CTRL_SIZER {
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
pub struct _CRC_CTRL_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_CTRL_SIZEW<'a> {
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
#[doc = "Possible values of the field `CRC_CTRL_INIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_CTRL_INITR {
    #[doc = "Use the CRCSEED register context as the starting value"]
    CRC_CTRL_INIT_SEED,
    #[doc = "Initialize to all '0s'"]
    CRC_CTRL_INIT_0,
    #[doc = "Initialize to all '1s'"]
    CRC_CTRL_INIT_1,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl CRC_CTRL_INITR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            CRC_CTRL_INITR::CRC_CTRL_INIT_SEED => 0,
            CRC_CTRL_INITR::CRC_CTRL_INIT_0 => 2,
            CRC_CTRL_INITR::CRC_CTRL_INIT_1 => 3,
            CRC_CTRL_INITR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> CRC_CTRL_INITR {
        match value {
            0 => CRC_CTRL_INITR::CRC_CTRL_INIT_SEED,
            2 => CRC_CTRL_INITR::CRC_CTRL_INIT_0,
            3 => CRC_CTRL_INITR::CRC_CTRL_INIT_1,
            i => CRC_CTRL_INITR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_INIT_SEED`"]
    #[inline(always)]
    pub fn is_crc_ctrl_init_seed(&self) -> bool {
        *self == CRC_CTRL_INITR::CRC_CTRL_INIT_SEED
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_INIT_0`"]
    #[inline(always)]
    pub fn is_crc_ctrl_init_0(&self) -> bool {
        *self == CRC_CTRL_INITR::CRC_CTRL_INIT_0
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_INIT_1`"]
    #[inline(always)]
    pub fn is_crc_ctrl_init_1(&self) -> bool {
        *self == CRC_CTRL_INITR::CRC_CTRL_INIT_1
    }
}
#[doc = "Values that can be written to the field `CRC_CTRL_INIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_CTRL_INITW {
    #[doc = "Use the CRCSEED register context as the starting value"]
    CRC_CTRL_INIT_SEED,
    #[doc = "Initialize to all '0s'"]
    CRC_CTRL_INIT_0,
    #[doc = "Initialize to all '1s'"]
    CRC_CTRL_INIT_1,
}
impl CRC_CTRL_INITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CRC_CTRL_INITW::CRC_CTRL_INIT_SEED => 0,
            CRC_CTRL_INITW::CRC_CTRL_INIT_0 => 2,
            CRC_CTRL_INITW::CRC_CTRL_INIT_1 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CRC_CTRL_INITW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_CTRL_INITW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_CTRL_INITW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Use the CRCSEED register context as the starting value"]
    #[inline(always)]
    pub fn crc_ctrl_init_seed(self) -> &'a mut W {
        self.variant(CRC_CTRL_INITW::CRC_CTRL_INIT_SEED)
    }
    #[doc = "Initialize to all '0s'"]
    #[inline(always)]
    pub fn crc_ctrl_init_0(self) -> &'a mut W {
        self.variant(CRC_CTRL_INITW::CRC_CTRL_INIT_0)
    }
    #[doc = "Initialize to all '1s'"]
    #[inline(always)]
    pub fn crc_ctrl_init_1(self) -> &'a mut W {
        self.variant(CRC_CTRL_INITW::CRC_CTRL_INIT_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 13);
        self.w.bits |= ((value as u32) & 3) << 13;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Operation Type"]
    #[inline(always)]
    pub fn crc_ctrl_type(&self) -> CRC_CTRL_TYPER {
        CRC_CTRL_TYPER::_from(((self.bits >> 0) & 15) as u8)
    }
    #[doc = "Bits 4:5 - Endian Control"]
    #[inline(always)]
    pub fn crc_ctrl_endian(&self) -> CRC_CTRL_ENDIANR {
        CRC_CTRL_ENDIANR::_from(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Bit reverse enable"]
    #[inline(always)]
    pub fn crc_ctrl_br(&self) -> CRC_CTRL_BRR {
        let bits = ((self.bits >> 7) & 1) != 0;
        CRC_CTRL_BRR { bits }
    }
    #[doc = "Bit 8 - Output Reverse Enable"]
    #[inline(always)]
    pub fn crc_ctrl_obr(&self) -> CRC_CTRL_OBRR {
        let bits = ((self.bits >> 8) & 1) != 0;
        CRC_CTRL_OBRR { bits }
    }
    #[doc = "Bit 9 - Result Inverse Enable"]
    #[inline(always)]
    pub fn crc_ctrl_resinv(&self) -> CRC_CTRL_RESINVR {
        let bits = ((self.bits >> 9) & 1) != 0;
        CRC_CTRL_RESINVR { bits }
    }
    #[doc = "Bit 12 - Input Data Size"]
    #[inline(always)]
    pub fn crc_ctrl_size(&self) -> CRC_CTRL_SIZER {
        let bits = ((self.bits >> 12) & 1) != 0;
        CRC_CTRL_SIZER { bits }
    }
    #[doc = "Bits 13:14 - CRC Initialization"]
    #[inline(always)]
    pub fn crc_ctrl_init(&self) -> CRC_CTRL_INITR {
        CRC_CTRL_INITR::_from(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Operation Type"]
    #[inline(always)]
    pub fn crc_ctrl_type(&mut self) -> _CRC_CTRL_TYPEW {
        _CRC_CTRL_TYPEW { w: self }
    }
    #[doc = "Bits 4:5 - Endian Control"]
    #[inline(always)]
    pub fn crc_ctrl_endian(&mut self) -> _CRC_CTRL_ENDIANW {
        _CRC_CTRL_ENDIANW { w: self }
    }
    #[doc = "Bit 7 - Bit reverse enable"]
    #[inline(always)]
    pub fn crc_ctrl_br(&mut self) -> _CRC_CTRL_BRW {
        _CRC_CTRL_BRW { w: self }
    }
    #[doc = "Bit 8 - Output Reverse Enable"]
    #[inline(always)]
    pub fn crc_ctrl_obr(&mut self) -> _CRC_CTRL_OBRW {
        _CRC_CTRL_OBRW { w: self }
    }
    #[doc = "Bit 9 - Result Inverse Enable"]
    #[inline(always)]
    pub fn crc_ctrl_resinv(&mut self) -> _CRC_CTRL_RESINVW {
        _CRC_CTRL_RESINVW { w: self }
    }
    #[doc = "Bit 12 - Input Data Size"]
    #[inline(always)]
    pub fn crc_ctrl_size(&mut self) -> _CRC_CTRL_SIZEW {
        _CRC_CTRL_SIZEW { w: self }
    }
    #[doc = "Bits 13:14 - CRC Initialization"]
    #[inline(always)]
    pub fn crc_ctrl_init(&mut self) -> _CRC_CTRL_INITW {
        _CRC_CTRL_INITW { w: self }
    }
}
