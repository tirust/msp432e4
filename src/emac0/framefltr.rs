#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FRAMEFLTR {
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
pub struct EMAC_FRAMEFLTR_PRR {
    bits: bool,
}
impl EMAC_FRAMEFLTR_PRR {
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
pub struct _EMAC_FRAMEFLTR_PRW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_FRAMEFLTR_PRW<'a> {
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
pub struct EMAC_FRAMEFLTR_HUCR {
    bits: bool,
}
impl EMAC_FRAMEFLTR_HUCR {
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
pub struct _EMAC_FRAMEFLTR_HUCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_FRAMEFLTR_HUCW<'a> {
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
pub struct EMAC_FRAMEFLTR_HMCR {
    bits: bool,
}
impl EMAC_FRAMEFLTR_HMCR {
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
pub struct _EMAC_FRAMEFLTR_HMCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_FRAMEFLTR_HMCW<'a> {
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
pub struct EMAC_FRAMEFLTR_DAIFR {
    bits: bool,
}
impl EMAC_FRAMEFLTR_DAIFR {
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
pub struct _EMAC_FRAMEFLTR_DAIFW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_FRAMEFLTR_DAIFW<'a> {
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
pub struct EMAC_FRAMEFLTR_PMR {
    bits: bool,
}
impl EMAC_FRAMEFLTR_PMR {
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
pub struct _EMAC_FRAMEFLTR_PMW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_FRAMEFLTR_PMW<'a> {
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
pub struct EMAC_FRAMEFLTR_DBFR {
    bits: bool,
}
impl EMAC_FRAMEFLTR_DBFR {
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
pub struct _EMAC_FRAMEFLTR_DBFW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_FRAMEFLTR_DBFW<'a> {
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
#[doc = "Possible values of the field `EMAC_FRAMEFLTR_PCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_FRAMEFLTR_PCFR {
    #[doc = "The MAC filters all control frames from reaching application"]
    EMAC_FRAMEFLTR_PCF_ALL,
    #[doc = "MAC forwards all control frames except PAUSE control frames to application even if they fail the address filter"]
    EMAC_FRAMEFLTR_PCF_PAUSE,
    #[doc = "MAC forwards all control frames to application even if they fail the address Filter"]
    EMAC_FRAMEFLTR_PCF_NONE,
    #[doc = "MAC forwards control frames that pass the address Filter"]
    EMAC_FRAMEFLTR_PCF_ADDR,
}
impl EMAC_FRAMEFLTR_PCFR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EMAC_FRAMEFLTR_PCFR::EMAC_FRAMEFLTR_PCF_ALL => 0,
            EMAC_FRAMEFLTR_PCFR::EMAC_FRAMEFLTR_PCF_PAUSE => 1,
            EMAC_FRAMEFLTR_PCFR::EMAC_FRAMEFLTR_PCF_NONE => 2,
            EMAC_FRAMEFLTR_PCFR::EMAC_FRAMEFLTR_PCF_ADDR => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EMAC_FRAMEFLTR_PCFR {
        match value {
            0 => EMAC_FRAMEFLTR_PCFR::EMAC_FRAMEFLTR_PCF_ALL,
            1 => EMAC_FRAMEFLTR_PCFR::EMAC_FRAMEFLTR_PCF_PAUSE,
            2 => EMAC_FRAMEFLTR_PCFR::EMAC_FRAMEFLTR_PCF_NONE,
            3 => EMAC_FRAMEFLTR_PCFR::EMAC_FRAMEFLTR_PCF_ADDR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_FRAMEFLTR_PCF_ALL`"]
    #[inline(always)]
    pub fn is_emac_framefltr_pcf_all(&self) -> bool {
        *self == EMAC_FRAMEFLTR_PCFR::EMAC_FRAMEFLTR_PCF_ALL
    }
    #[doc = "Checks if the value of the field is `EMAC_FRAMEFLTR_PCF_PAUSE`"]
    #[inline(always)]
    pub fn is_emac_framefltr_pcf_pause(&self) -> bool {
        *self == EMAC_FRAMEFLTR_PCFR::EMAC_FRAMEFLTR_PCF_PAUSE
    }
    #[doc = "Checks if the value of the field is `EMAC_FRAMEFLTR_PCF_NONE`"]
    #[inline(always)]
    pub fn is_emac_framefltr_pcf_none(&self) -> bool {
        *self == EMAC_FRAMEFLTR_PCFR::EMAC_FRAMEFLTR_PCF_NONE
    }
    #[doc = "Checks if the value of the field is `EMAC_FRAMEFLTR_PCF_ADDR`"]
    #[inline(always)]
    pub fn is_emac_framefltr_pcf_addr(&self) -> bool {
        *self == EMAC_FRAMEFLTR_PCFR::EMAC_FRAMEFLTR_PCF_ADDR
    }
}
#[doc = "Values that can be written to the field `EMAC_FRAMEFLTR_PCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_FRAMEFLTR_PCFW {
    #[doc = "The MAC filters all control frames from reaching application"]
    EMAC_FRAMEFLTR_PCF_ALL,
    #[doc = "MAC forwards all control frames except PAUSE control frames to application even if they fail the address filter"]
    EMAC_FRAMEFLTR_PCF_PAUSE,
    #[doc = "MAC forwards all control frames to application even if they fail the address Filter"]
    EMAC_FRAMEFLTR_PCF_NONE,
    #[doc = "MAC forwards control frames that pass the address Filter"]
    EMAC_FRAMEFLTR_PCF_ADDR,
}
impl EMAC_FRAMEFLTR_PCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMAC_FRAMEFLTR_PCFW::EMAC_FRAMEFLTR_PCF_ALL => 0,
            EMAC_FRAMEFLTR_PCFW::EMAC_FRAMEFLTR_PCF_PAUSE => 1,
            EMAC_FRAMEFLTR_PCFW::EMAC_FRAMEFLTR_PCF_NONE => 2,
            EMAC_FRAMEFLTR_PCFW::EMAC_FRAMEFLTR_PCF_ADDR => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_FRAMEFLTR_PCFW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_FRAMEFLTR_PCFW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMAC_FRAMEFLTR_PCFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The MAC filters all control frames from reaching application"]
    #[inline(always)]
    pub fn emac_framefltr_pcf_all(self) -> &'a mut W {
        self.variant(EMAC_FRAMEFLTR_PCFW::EMAC_FRAMEFLTR_PCF_ALL)
    }
    #[doc = "MAC forwards all control frames except PAUSE control frames to application even if they fail the address filter"]
    #[inline(always)]
    pub fn emac_framefltr_pcf_pause(self) -> &'a mut W {
        self.variant(EMAC_FRAMEFLTR_PCFW::EMAC_FRAMEFLTR_PCF_PAUSE)
    }
    #[doc = "MAC forwards all control frames to application even if they fail the address Filter"]
    #[inline(always)]
    pub fn emac_framefltr_pcf_none(self) -> &'a mut W {
        self.variant(EMAC_FRAMEFLTR_PCFW::EMAC_FRAMEFLTR_PCF_NONE)
    }
    #[doc = "MAC forwards control frames that pass the address Filter"]
    #[inline(always)]
    pub fn emac_framefltr_pcf_addr(self) -> &'a mut W {
        self.variant(EMAC_FRAMEFLTR_PCFW::EMAC_FRAMEFLTR_PCF_ADDR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 6);
        self.w.bits |= ((value as u32) & 3) << 6;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_FRAMEFLTR_SAIFR {
    bits: bool,
}
impl EMAC_FRAMEFLTR_SAIFR {
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
pub struct _EMAC_FRAMEFLTR_SAIFW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_FRAMEFLTR_SAIFW<'a> {
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
pub struct EMAC_FRAMEFLTR_SAFR {
    bits: bool,
}
impl EMAC_FRAMEFLTR_SAFR {
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
pub struct _EMAC_FRAMEFLTR_SAFW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_FRAMEFLTR_SAFW<'a> {
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
pub struct EMAC_FRAMEFLTR_HPFR {
    bits: bool,
}
impl EMAC_FRAMEFLTR_HPFR {
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
pub struct _EMAC_FRAMEFLTR_HPFW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_FRAMEFLTR_HPFW<'a> {
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
pub struct EMAC_FRAMEFLTR_VTFER {
    bits: bool,
}
impl EMAC_FRAMEFLTR_VTFER {
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
pub struct _EMAC_FRAMEFLTR_VTFEW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_FRAMEFLTR_VTFEW<'a> {
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
pub struct EMAC_FRAMEFLTR_RAR {
    bits: bool,
}
impl EMAC_FRAMEFLTR_RAR {
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
pub struct _EMAC_FRAMEFLTR_RAW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_FRAMEFLTR_RAW<'a> {
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
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    pub fn emac_framefltr_pr(&self) -> EMAC_FRAMEFLTR_PRR {
        let bits = ((self.bits >> 0) & 1) != 0;
        EMAC_FRAMEFLTR_PRR { bits }
    }
    #[doc = "Bit 1 - Hash Unicast"]
    #[inline(always)]
    pub fn emac_framefltr_huc(&self) -> EMAC_FRAMEFLTR_HUCR {
        let bits = ((self.bits >> 1) & 1) != 0;
        EMAC_FRAMEFLTR_HUCR { bits }
    }
    #[doc = "Bit 2 - Hash Multicast"]
    #[inline(always)]
    pub fn emac_framefltr_hmc(&self) -> EMAC_FRAMEFLTR_HMCR {
        let bits = ((self.bits >> 2) & 1) != 0;
        EMAC_FRAMEFLTR_HMCR { bits }
    }
    #[doc = "Bit 3 - Destination Address (DA) Inverse Filtering"]
    #[inline(always)]
    pub fn emac_framefltr_daif(&self) -> EMAC_FRAMEFLTR_DAIFR {
        let bits = ((self.bits >> 3) & 1) != 0;
        EMAC_FRAMEFLTR_DAIFR { bits }
    }
    #[doc = "Bit 4 - Pass All Multicast"]
    #[inline(always)]
    pub fn emac_framefltr_pm(&self) -> EMAC_FRAMEFLTR_PMR {
        let bits = ((self.bits >> 4) & 1) != 0;
        EMAC_FRAMEFLTR_PMR { bits }
    }
    #[doc = "Bit 5 - Disable Broadcast Frames"]
    #[inline(always)]
    pub fn emac_framefltr_dbf(&self) -> EMAC_FRAMEFLTR_DBFR {
        let bits = ((self.bits >> 5) & 1) != 0;
        EMAC_FRAMEFLTR_DBFR { bits }
    }
    #[doc = "Bits 6:7 - Pass Control Frames"]
    #[inline(always)]
    pub fn emac_framefltr_pcf(&self) -> EMAC_FRAMEFLTR_PCFR {
        EMAC_FRAMEFLTR_PCFR::_from(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Source Address (SA) Inverse Filtering"]
    #[inline(always)]
    pub fn emac_framefltr_saif(&self) -> EMAC_FRAMEFLTR_SAIFR {
        let bits = ((self.bits >> 8) & 1) != 0;
        EMAC_FRAMEFLTR_SAIFR { bits }
    }
    #[doc = "Bit 9 - Source Address Filter Enable"]
    #[inline(always)]
    pub fn emac_framefltr_saf(&self) -> EMAC_FRAMEFLTR_SAFR {
        let bits = ((self.bits >> 9) & 1) != 0;
        EMAC_FRAMEFLTR_SAFR { bits }
    }
    #[doc = "Bit 10 - Hash or Perfect Filter"]
    #[inline(always)]
    pub fn emac_framefltr_hpf(&self) -> EMAC_FRAMEFLTR_HPFR {
        let bits = ((self.bits >> 10) & 1) != 0;
        EMAC_FRAMEFLTR_HPFR { bits }
    }
    #[doc = "Bit 16 - VLAN Tag Filter Enable"]
    #[inline(always)]
    pub fn emac_framefltr_vtfe(&self) -> EMAC_FRAMEFLTR_VTFER {
        let bits = ((self.bits >> 16) & 1) != 0;
        EMAC_FRAMEFLTR_VTFER { bits }
    }
    #[doc = "Bit 31 - Receive All"]
    #[inline(always)]
    pub fn emac_framefltr_ra(&self) -> EMAC_FRAMEFLTR_RAR {
        let bits = ((self.bits >> 31) & 1) != 0;
        EMAC_FRAMEFLTR_RAR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    pub fn emac_framefltr_pr(&mut self) -> _EMAC_FRAMEFLTR_PRW {
        _EMAC_FRAMEFLTR_PRW { w: self }
    }
    #[doc = "Bit 1 - Hash Unicast"]
    #[inline(always)]
    pub fn emac_framefltr_huc(&mut self) -> _EMAC_FRAMEFLTR_HUCW {
        _EMAC_FRAMEFLTR_HUCW { w: self }
    }
    #[doc = "Bit 2 - Hash Multicast"]
    #[inline(always)]
    pub fn emac_framefltr_hmc(&mut self) -> _EMAC_FRAMEFLTR_HMCW {
        _EMAC_FRAMEFLTR_HMCW { w: self }
    }
    #[doc = "Bit 3 - Destination Address (DA) Inverse Filtering"]
    #[inline(always)]
    pub fn emac_framefltr_daif(&mut self) -> _EMAC_FRAMEFLTR_DAIFW {
        _EMAC_FRAMEFLTR_DAIFW { w: self }
    }
    #[doc = "Bit 4 - Pass All Multicast"]
    #[inline(always)]
    pub fn emac_framefltr_pm(&mut self) -> _EMAC_FRAMEFLTR_PMW {
        _EMAC_FRAMEFLTR_PMW { w: self }
    }
    #[doc = "Bit 5 - Disable Broadcast Frames"]
    #[inline(always)]
    pub fn emac_framefltr_dbf(&mut self) -> _EMAC_FRAMEFLTR_DBFW {
        _EMAC_FRAMEFLTR_DBFW { w: self }
    }
    #[doc = "Bits 6:7 - Pass Control Frames"]
    #[inline(always)]
    pub fn emac_framefltr_pcf(&mut self) -> _EMAC_FRAMEFLTR_PCFW {
        _EMAC_FRAMEFLTR_PCFW { w: self }
    }
    #[doc = "Bit 8 - Source Address (SA) Inverse Filtering"]
    #[inline(always)]
    pub fn emac_framefltr_saif(&mut self) -> _EMAC_FRAMEFLTR_SAIFW {
        _EMAC_FRAMEFLTR_SAIFW { w: self }
    }
    #[doc = "Bit 9 - Source Address Filter Enable"]
    #[inline(always)]
    pub fn emac_framefltr_saf(&mut self) -> _EMAC_FRAMEFLTR_SAFW {
        _EMAC_FRAMEFLTR_SAFW { w: self }
    }
    #[doc = "Bit 10 - Hash or Perfect Filter"]
    #[inline(always)]
    pub fn emac_framefltr_hpf(&mut self) -> _EMAC_FRAMEFLTR_HPFW {
        _EMAC_FRAMEFLTR_HPFW { w: self }
    }
    #[doc = "Bit 16 - VLAN Tag Filter Enable"]
    #[inline(always)]
    pub fn emac_framefltr_vtfe(&mut self) -> _EMAC_FRAMEFLTR_VTFEW {
        _EMAC_FRAMEFLTR_VTFEW { w: self }
    }
    #[doc = "Bit 31 - Receive All"]
    #[inline(always)]
    pub fn emac_framefltr_ra(&mut self) -> _EMAC_FRAMEFLTR_RAW {
        _EMAC_FRAMEFLTR_RAW { w: self }
    }
}
