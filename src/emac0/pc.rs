#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PC {
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
pub struct EMAC_PC_PHYHOLDR {
    bits: bool,
}
impl EMAC_PC_PHYHOLDR {
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
pub struct _EMAC_PC_PHYHOLDW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_PHYHOLDW<'a> {
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
#[doc = "Possible values of the field `EMAC_PC_ANMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_PC_ANMODER {
    #[doc = "When ANEN = 0x0, the mode is 10Base-T, Half-Duplex"]
    EMAC_PC_ANMODE_10HD,
    #[doc = "When ANEN = 0x0, the mode is 10Base-T, Full-Duplex"]
    EMAC_PC_ANMODE_10FD,
    #[doc = "When ANEN = 0x0, the mode is 100Base-TX, Half-Duplex"]
    EMAC_PC_ANMODE_100HD,
    #[doc = "When ANEN = 0x0, the mode is 100Base-TX, Full-Duplex"]
    EMAC_PC_ANMODE_100FD,
}
impl EMAC_PC_ANMODER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EMAC_PC_ANMODER::EMAC_PC_ANMODE_10HD => 0,
            EMAC_PC_ANMODER::EMAC_PC_ANMODE_10FD => 1,
            EMAC_PC_ANMODER::EMAC_PC_ANMODE_100HD => 2,
            EMAC_PC_ANMODER::EMAC_PC_ANMODE_100FD => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EMAC_PC_ANMODER {
        match value {
            0 => EMAC_PC_ANMODER::EMAC_PC_ANMODE_10HD,
            1 => EMAC_PC_ANMODER::EMAC_PC_ANMODE_10FD,
            2 => EMAC_PC_ANMODER::EMAC_PC_ANMODE_100HD,
            3 => EMAC_PC_ANMODER::EMAC_PC_ANMODE_100FD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_PC_ANMODE_10HD`"]
    #[inline(always)]
    pub fn is_emac_pc_anmode_10hd(&self) -> bool {
        *self == EMAC_PC_ANMODER::EMAC_PC_ANMODE_10HD
    }
    #[doc = "Checks if the value of the field is `EMAC_PC_ANMODE_10FD`"]
    #[inline(always)]
    pub fn is_emac_pc_anmode_10fd(&self) -> bool {
        *self == EMAC_PC_ANMODER::EMAC_PC_ANMODE_10FD
    }
    #[doc = "Checks if the value of the field is `EMAC_PC_ANMODE_100HD`"]
    #[inline(always)]
    pub fn is_emac_pc_anmode_100hd(&self) -> bool {
        *self == EMAC_PC_ANMODER::EMAC_PC_ANMODE_100HD
    }
    #[doc = "Checks if the value of the field is `EMAC_PC_ANMODE_100FD`"]
    #[inline(always)]
    pub fn is_emac_pc_anmode_100fd(&self) -> bool {
        *self == EMAC_PC_ANMODER::EMAC_PC_ANMODE_100FD
    }
}
#[doc = "Values that can be written to the field `EMAC_PC_ANMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_PC_ANMODEW {
    #[doc = "When ANEN = 0x0, the mode is 10Base-T, Half-Duplex"]
    EMAC_PC_ANMODE_10HD,
    #[doc = "When ANEN = 0x0, the mode is 10Base-T, Full-Duplex"]
    EMAC_PC_ANMODE_10FD,
    #[doc = "When ANEN = 0x0, the mode is 100Base-TX, Half-Duplex"]
    EMAC_PC_ANMODE_100HD,
    #[doc = "When ANEN = 0x0, the mode is 100Base-TX, Full-Duplex"]
    EMAC_PC_ANMODE_100FD,
}
impl EMAC_PC_ANMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMAC_PC_ANMODEW::EMAC_PC_ANMODE_10HD => 0,
            EMAC_PC_ANMODEW::EMAC_PC_ANMODE_10FD => 1,
            EMAC_PC_ANMODEW::EMAC_PC_ANMODE_100HD => 2,
            EMAC_PC_ANMODEW::EMAC_PC_ANMODE_100FD => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_PC_ANMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_ANMODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMAC_PC_ANMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "When ANEN = 0x0, the mode is 10Base-T, Half-Duplex"]
    #[inline(always)]
    pub fn emac_pc_anmode_10hd(self) -> &'a mut W {
        self.variant(EMAC_PC_ANMODEW::EMAC_PC_ANMODE_10HD)
    }
    #[doc = "When ANEN = 0x0, the mode is 10Base-T, Full-Duplex"]
    #[inline(always)]
    pub fn emac_pc_anmode_10fd(self) -> &'a mut W {
        self.variant(EMAC_PC_ANMODEW::EMAC_PC_ANMODE_10FD)
    }
    #[doc = "When ANEN = 0x0, the mode is 100Base-TX, Half-Duplex"]
    #[inline(always)]
    pub fn emac_pc_anmode_100hd(self) -> &'a mut W {
        self.variant(EMAC_PC_ANMODEW::EMAC_PC_ANMODE_100HD)
    }
    #[doc = "When ANEN = 0x0, the mode is 100Base-TX, Full-Duplex"]
    #[inline(always)]
    pub fn emac_pc_anmode_100fd(self) -> &'a mut W {
        self.variant(EMAC_PC_ANMODEW::EMAC_PC_ANMODE_100FD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 1);
        self.w.bits |= ((value as u32) & 3) << 1;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_PC_ANENR {
    bits: bool,
}
impl EMAC_PC_ANENR {
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
pub struct _EMAC_PC_ANENW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_ANENW<'a> {
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
pub struct EMAC_PC_FASTANSELR {
    bits: u8,
}
impl EMAC_PC_FASTANSELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_PC_FASTANSELW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_FASTANSELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_PC_FASTANENR {
    bits: bool,
}
impl EMAC_PC_FASTANENR {
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
pub struct _EMAC_PC_FASTANENW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_FASTANENW<'a> {
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
pub struct EMAC_PC_EXTFDR {
    bits: bool,
}
impl EMAC_PC_EXTFDR {
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
pub struct _EMAC_PC_EXTFDW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_EXTFDW<'a> {
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
pub struct EMAC_PC_FASTLUPDR {
    bits: bool,
}
impl EMAC_PC_FASTLUPDR {
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
pub struct _EMAC_PC_FASTLUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_FASTLUPDW<'a> {
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
pub struct EMAC_PC_FASTRXDVR {
    bits: bool,
}
impl EMAC_PC_FASTRXDVR {
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
pub struct _EMAC_PC_FASTRXDVW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_FASTRXDVW<'a> {
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
pub struct EMAC_PC_MDIXENR {
    bits: bool,
}
impl EMAC_PC_MDIXENR {
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
pub struct _EMAC_PC_MDIXENW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_MDIXENW<'a> {
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
pub struct EMAC_PC_FASTMDIXR {
    bits: bool,
}
impl EMAC_PC_FASTMDIXR {
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
pub struct _EMAC_PC_FASTMDIXW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_FASTMDIXW<'a> {
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
pub struct EMAC_PC_RBSTMDIXR {
    bits: bool,
}
impl EMAC_PC_RBSTMDIXR {
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
pub struct _EMAC_PC_RBSTMDIXW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_RBSTMDIXW<'a> {
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
pub struct EMAC_PC_MDISWAPR {
    bits: bool,
}
impl EMAC_PC_MDISWAPR {
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
pub struct _EMAC_PC_MDISWAPW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_MDISWAPW<'a> {
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
pub struct EMAC_PC_POLSWAPR {
    bits: bool,
}
impl EMAC_PC_POLSWAPR {
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
pub struct _EMAC_PC_POLSWAPW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_POLSWAPW<'a> {
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
        self.w.bits &= !(1 << 14);
        self.w.bits |= ((value as u32) & 1) << 14;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_PC_FASTLDMODER {
    bits: u8,
}
impl EMAC_PC_FASTLDMODER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_PC_FASTLDMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_FASTLDMODEW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(31 << 15);
        self.w.bits |= ((value as u32) & 31) << 15;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_PC_TDRRUNR {
    bits: bool,
}
impl EMAC_PC_TDRRUNR {
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
pub struct _EMAC_PC_TDRRUNW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_TDRRUNW<'a> {
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
        self.w.bits &= !(1 << 20);
        self.w.bits |= ((value as u32) & 1) << 20;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_PC_LRRR {
    bits: bool,
}
impl EMAC_PC_LRRR {
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
pub struct _EMAC_PC_LRRW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_LRRW<'a> {
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
        self.w.bits &= !(1 << 21);
        self.w.bits |= ((value as u32) & 1) << 21;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_PC_ISOMIILLR {
    bits: bool,
}
impl EMAC_PC_ISOMIILLR {
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
pub struct _EMAC_PC_ISOMIILLW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_ISOMIILLW<'a> {
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
        self.w.bits &= !(1 << 22);
        self.w.bits |= ((value as u32) & 1) << 22;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_PC_RXERIDLER {
    bits: bool,
}
impl EMAC_PC_RXERIDLER {
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
pub struct _EMAC_PC_RXERIDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_RXERIDLEW<'a> {
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
        self.w.bits &= !(1 << 23);
        self.w.bits |= ((value as u32) & 1) << 23;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_PC_NIBDETDISR {
    bits: bool,
}
impl EMAC_PC_NIBDETDISR {
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
pub struct _EMAC_PC_NIBDETDISW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_NIBDETDISW<'a> {
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
        self.w.bits &= !(1 << 24);
        self.w.bits |= ((value as u32) & 1) << 24;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_PC_DIGRESTARTR {
    bits: bool,
}
impl EMAC_PC_DIGRESTARTR {
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
pub struct _EMAC_PC_DIGRESTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_DIGRESTARTW<'a> {
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
        self.w.bits &= !(1 << 25);
        self.w.bits |= ((value as u32) & 1) << 25;
        self.w
    }
}
#[doc = "Possible values of the field `EMAC_PC_PINTFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_PC_PINTFSR {
    #[doc = "MII (default) Used for internal PHY or external PHY connected via MII"]
    EMAC_PC_PINTFS_IMII,
    #[doc = "RMII: Used for external PHY connected via RMII"]
    EMAC_PC_PINTFS_RMII,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl EMAC_PC_PINTFSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EMAC_PC_PINTFSR::EMAC_PC_PINTFS_IMII => 0,
            EMAC_PC_PINTFSR::EMAC_PC_PINTFS_RMII => 4,
            EMAC_PC_PINTFSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EMAC_PC_PINTFSR {
        match value {
            0 => EMAC_PC_PINTFSR::EMAC_PC_PINTFS_IMII,
            4 => EMAC_PC_PINTFSR::EMAC_PC_PINTFS_RMII,
            i => EMAC_PC_PINTFSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_PC_PINTFS_IMII`"]
    #[inline(always)]
    pub fn is_emac_pc_pintfs_imii(&self) -> bool {
        *self == EMAC_PC_PINTFSR::EMAC_PC_PINTFS_IMII
    }
    #[doc = "Checks if the value of the field is `EMAC_PC_PINTFS_RMII`"]
    #[inline(always)]
    pub fn is_emac_pc_pintfs_rmii(&self) -> bool {
        *self == EMAC_PC_PINTFSR::EMAC_PC_PINTFS_RMII
    }
}
#[doc = "Values that can be written to the field `EMAC_PC_PINTFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_PC_PINTFSW {
    #[doc = "MII (default) Used for internal PHY or external PHY connected via MII"]
    EMAC_PC_PINTFS_IMII,
    #[doc = "RMII: Used for external PHY connected via RMII"]
    EMAC_PC_PINTFS_RMII,
}
impl EMAC_PC_PINTFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMAC_PC_PINTFSW::EMAC_PC_PINTFS_IMII => 0,
            EMAC_PC_PINTFSW::EMAC_PC_PINTFS_RMII => 4,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_PC_PINTFSW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_PINTFSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMAC_PC_PINTFSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "MII (default) Used for internal PHY or external PHY connected via MII"]
    #[inline(always)]
    pub fn emac_pc_pintfs_imii(self) -> &'a mut W {
        self.variant(EMAC_PC_PINTFSW::EMAC_PC_PINTFS_IMII)
    }
    #[doc = "RMII: Used for external PHY connected via RMII"]
    #[inline(always)]
    pub fn emac_pc_pintfs_rmii(self) -> &'a mut W {
        self.variant(EMAC_PC_PINTFSW::EMAC_PC_PINTFS_RMII)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 28);
        self.w.bits |= ((value as u32) & 7) << 28;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_PC_PHYEXTR {
    bits: bool,
}
impl EMAC_PC_PHYEXTR {
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
pub struct _EMAC_PC_PHYEXTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PC_PHYEXTW<'a> {
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
    #[doc = "Bit 0 - Ethernet PHY Hold"]
    #[inline(always)]
    pub fn emac_pc_phyhold(&self) -> EMAC_PC_PHYHOLDR {
        let bits = ((self.bits >> 0) & 1) != 0;
        EMAC_PC_PHYHOLDR { bits }
    }
    #[doc = "Bits 1:2 - Auto Negotiation Mode"]
    #[inline(always)]
    pub fn emac_pc_anmode(&self) -> EMAC_PC_ANMODER {
        EMAC_PC_ANMODER::_from(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Auto Negotiation Enable"]
    #[inline(always)]
    pub fn emac_pc_anen(&self) -> EMAC_PC_ANENR {
        let bits = ((self.bits >> 3) & 1) != 0;
        EMAC_PC_ANENR { bits }
    }
    #[doc = "Bits 4:5 - Fast Auto Negotiation Select"]
    #[inline(always)]
    pub fn emac_pc_fastansel(&self) -> EMAC_PC_FASTANSELR {
        let bits = ((self.bits >> 4) & 3) as u8;
        EMAC_PC_FASTANSELR { bits }
    }
    #[doc = "Bit 6 - Fast Auto Negotiation Enable"]
    #[inline(always)]
    pub fn emac_pc_fastanen(&self) -> EMAC_PC_FASTANENR {
        let bits = ((self.bits >> 6) & 1) != 0;
        EMAC_PC_FASTANENR { bits }
    }
    #[doc = "Bit 7 - Extended Full Duplex Ability"]
    #[inline(always)]
    pub fn emac_pc_extfd(&self) -> EMAC_PC_EXTFDR {
        let bits = ((self.bits >> 7) & 1) != 0;
        EMAC_PC_EXTFDR { bits }
    }
    #[doc = "Bit 8 - FAST Link-Up in Parallel Detect"]
    #[inline(always)]
    pub fn emac_pc_fastlupd(&self) -> EMAC_PC_FASTLUPDR {
        let bits = ((self.bits >> 8) & 1) != 0;
        EMAC_PC_FASTLUPDR { bits }
    }
    #[doc = "Bit 9 - Fast RXDV Detection"]
    #[inline(always)]
    pub fn emac_pc_fastrxdv(&self) -> EMAC_PC_FASTRXDVR {
        let bits = ((self.bits >> 9) & 1) != 0;
        EMAC_PC_FASTRXDVR { bits }
    }
    #[doc = "Bit 10 - MDIX Enable"]
    #[inline(always)]
    pub fn emac_pc_mdixen(&self) -> EMAC_PC_MDIXENR {
        let bits = ((self.bits >> 10) & 1) != 0;
        EMAC_PC_MDIXENR { bits }
    }
    #[doc = "Bit 11 - Fast Auto MDI-X"]
    #[inline(always)]
    pub fn emac_pc_fastmdix(&self) -> EMAC_PC_FASTMDIXR {
        let bits = ((self.bits >> 11) & 1) != 0;
        EMAC_PC_FASTMDIXR { bits }
    }
    #[doc = "Bit 12 - Robust Auto MDI-X"]
    #[inline(always)]
    pub fn emac_pc_rbstmdix(&self) -> EMAC_PC_RBSTMDIXR {
        let bits = ((self.bits >> 12) & 1) != 0;
        EMAC_PC_RBSTMDIXR { bits }
    }
    #[doc = "Bit 13 - MDI Swap"]
    #[inline(always)]
    pub fn emac_pc_mdiswap(&self) -> EMAC_PC_MDISWAPR {
        let bits = ((self.bits >> 13) & 1) != 0;
        EMAC_PC_MDISWAPR { bits }
    }
    #[doc = "Bit 14 - Polarity Swap"]
    #[inline(always)]
    pub fn emac_pc_polswap(&self) -> EMAC_PC_POLSWAPR {
        let bits = ((self.bits >> 14) & 1) != 0;
        EMAC_PC_POLSWAPR { bits }
    }
    #[doc = "Bits 15:19 - Fast Link Down Mode"]
    #[inline(always)]
    pub fn emac_pc_fastldmode(&self) -> EMAC_PC_FASTLDMODER {
        let bits = ((self.bits >> 15) & 31) as u8;
        EMAC_PC_FASTLDMODER { bits }
    }
    #[doc = "Bit 20 - TDR Auto Run"]
    #[inline(always)]
    pub fn emac_pc_tdrrun(&self) -> EMAC_PC_TDRRUNR {
        let bits = ((self.bits >> 20) & 1) != 0;
        EMAC_PC_TDRRUNR { bits }
    }
    #[doc = "Bit 21 - Link Loss Recovery"]
    #[inline(always)]
    pub fn emac_pc_lrr(&self) -> EMAC_PC_LRRR {
        let bits = ((self.bits >> 21) & 1) != 0;
        EMAC_PC_LRRR { bits }
    }
    #[doc = "Bit 22 - Isolate MII in Link Loss"]
    #[inline(always)]
    pub fn emac_pc_isomiill(&self) -> EMAC_PC_ISOMIILLR {
        let bits = ((self.bits >> 22) & 1) != 0;
        EMAC_PC_ISOMIILLR { bits }
    }
    #[doc = "Bit 23 - RXER Detection During Idle"]
    #[inline(always)]
    pub fn emac_pc_rxeridle(&self) -> EMAC_PC_RXERIDLER {
        let bits = ((self.bits >> 23) & 1) != 0;
        EMAC_PC_RXERIDLER { bits }
    }
    #[doc = "Bit 24 - Odd Nibble TXER Detection Disable"]
    #[inline(always)]
    pub fn emac_pc_nibdetdis(&self) -> EMAC_PC_NIBDETDISR {
        let bits = ((self.bits >> 24) & 1) != 0;
        EMAC_PC_NIBDETDISR { bits }
    }
    #[doc = "Bit 25 - PHY Soft Restart"]
    #[inline(always)]
    pub fn emac_pc_digrestart(&self) -> EMAC_PC_DIGRESTARTR {
        let bits = ((self.bits >> 25) & 1) != 0;
        EMAC_PC_DIGRESTARTR { bits }
    }
    #[doc = "Bits 28:30 - Ethernet Interface Select"]
    #[inline(always)]
    pub fn emac_pc_pintfs(&self) -> EMAC_PC_PINTFSR {
        EMAC_PC_PINTFSR::_from(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - PHY Select"]
    #[inline(always)]
    pub fn emac_pc_phyext(&self) -> EMAC_PC_PHYEXTR {
        let bits = ((self.bits >> 31) & 1) != 0;
        EMAC_PC_PHYEXTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Ethernet PHY Hold"]
    #[inline(always)]
    pub fn emac_pc_phyhold(&mut self) -> _EMAC_PC_PHYHOLDW {
        _EMAC_PC_PHYHOLDW { w: self }
    }
    #[doc = "Bits 1:2 - Auto Negotiation Mode"]
    #[inline(always)]
    pub fn emac_pc_anmode(&mut self) -> _EMAC_PC_ANMODEW {
        _EMAC_PC_ANMODEW { w: self }
    }
    #[doc = "Bit 3 - Auto Negotiation Enable"]
    #[inline(always)]
    pub fn emac_pc_anen(&mut self) -> _EMAC_PC_ANENW {
        _EMAC_PC_ANENW { w: self }
    }
    #[doc = "Bits 4:5 - Fast Auto Negotiation Select"]
    #[inline(always)]
    pub fn emac_pc_fastansel(&mut self) -> _EMAC_PC_FASTANSELW {
        _EMAC_PC_FASTANSELW { w: self }
    }
    #[doc = "Bit 6 - Fast Auto Negotiation Enable"]
    #[inline(always)]
    pub fn emac_pc_fastanen(&mut self) -> _EMAC_PC_FASTANENW {
        _EMAC_PC_FASTANENW { w: self }
    }
    #[doc = "Bit 7 - Extended Full Duplex Ability"]
    #[inline(always)]
    pub fn emac_pc_extfd(&mut self) -> _EMAC_PC_EXTFDW {
        _EMAC_PC_EXTFDW { w: self }
    }
    #[doc = "Bit 8 - FAST Link-Up in Parallel Detect"]
    #[inline(always)]
    pub fn emac_pc_fastlupd(&mut self) -> _EMAC_PC_FASTLUPDW {
        _EMAC_PC_FASTLUPDW { w: self }
    }
    #[doc = "Bit 9 - Fast RXDV Detection"]
    #[inline(always)]
    pub fn emac_pc_fastrxdv(&mut self) -> _EMAC_PC_FASTRXDVW {
        _EMAC_PC_FASTRXDVW { w: self }
    }
    #[doc = "Bit 10 - MDIX Enable"]
    #[inline(always)]
    pub fn emac_pc_mdixen(&mut self) -> _EMAC_PC_MDIXENW {
        _EMAC_PC_MDIXENW { w: self }
    }
    #[doc = "Bit 11 - Fast Auto MDI-X"]
    #[inline(always)]
    pub fn emac_pc_fastmdix(&mut self) -> _EMAC_PC_FASTMDIXW {
        _EMAC_PC_FASTMDIXW { w: self }
    }
    #[doc = "Bit 12 - Robust Auto MDI-X"]
    #[inline(always)]
    pub fn emac_pc_rbstmdix(&mut self) -> _EMAC_PC_RBSTMDIXW {
        _EMAC_PC_RBSTMDIXW { w: self }
    }
    #[doc = "Bit 13 - MDI Swap"]
    #[inline(always)]
    pub fn emac_pc_mdiswap(&mut self) -> _EMAC_PC_MDISWAPW {
        _EMAC_PC_MDISWAPW { w: self }
    }
    #[doc = "Bit 14 - Polarity Swap"]
    #[inline(always)]
    pub fn emac_pc_polswap(&mut self) -> _EMAC_PC_POLSWAPW {
        _EMAC_PC_POLSWAPW { w: self }
    }
    #[doc = "Bits 15:19 - Fast Link Down Mode"]
    #[inline(always)]
    pub fn emac_pc_fastldmode(&mut self) -> _EMAC_PC_FASTLDMODEW {
        _EMAC_PC_FASTLDMODEW { w: self }
    }
    #[doc = "Bit 20 - TDR Auto Run"]
    #[inline(always)]
    pub fn emac_pc_tdrrun(&mut self) -> _EMAC_PC_TDRRUNW {
        _EMAC_PC_TDRRUNW { w: self }
    }
    #[doc = "Bit 21 - Link Loss Recovery"]
    #[inline(always)]
    pub fn emac_pc_lrr(&mut self) -> _EMAC_PC_LRRW {
        _EMAC_PC_LRRW { w: self }
    }
    #[doc = "Bit 22 - Isolate MII in Link Loss"]
    #[inline(always)]
    pub fn emac_pc_isomiill(&mut self) -> _EMAC_PC_ISOMIILLW {
        _EMAC_PC_ISOMIILLW { w: self }
    }
    #[doc = "Bit 23 - RXER Detection During Idle"]
    #[inline(always)]
    pub fn emac_pc_rxeridle(&mut self) -> _EMAC_PC_RXERIDLEW {
        _EMAC_PC_RXERIDLEW { w: self }
    }
    #[doc = "Bit 24 - Odd Nibble TXER Detection Disable"]
    #[inline(always)]
    pub fn emac_pc_nibdetdis(&mut self) -> _EMAC_PC_NIBDETDISW {
        _EMAC_PC_NIBDETDISW { w: self }
    }
    #[doc = "Bit 25 - PHY Soft Restart"]
    #[inline(always)]
    pub fn emac_pc_digrestart(&mut self) -> _EMAC_PC_DIGRESTARTW {
        _EMAC_PC_DIGRESTARTW { w: self }
    }
    #[doc = "Bits 28:30 - Ethernet Interface Select"]
    #[inline(always)]
    pub fn emac_pc_pintfs(&mut self) -> _EMAC_PC_PINTFSW {
        _EMAC_PC_PINTFSW { w: self }
    }
    #[doc = "Bit 31 - PHY Select"]
    #[inline(always)]
    pub fn emac_pc_phyext(&mut self) -> _EMAC_PC_PHYEXTW {
        _EMAC_PC_PHYEXTW { w: self }
    }
}
