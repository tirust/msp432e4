#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMSTCTRL {
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
pub struct EMAC_TIMSTCTRL_TSENR {
    bits: bool,
}
impl EMAC_TIMSTCTRL_TSENR {
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
pub struct _EMAC_TIMSTCTRL_TSENW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_TIMSTCTRL_TSENW<'a> {
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
pub struct EMAC_TIMSTCTRL_TSFCUPDTR {
    bits: bool,
}
impl EMAC_TIMSTCTRL_TSFCUPDTR {
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
pub struct _EMAC_TIMSTCTRL_TSFCUPDTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_TIMSTCTRL_TSFCUPDTW<'a> {
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
pub struct EMAC_TIMSTCTRL_TSINITR {
    bits: bool,
}
impl EMAC_TIMSTCTRL_TSINITR {
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
pub struct _EMAC_TIMSTCTRL_TSINITW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_TIMSTCTRL_TSINITW<'a> {
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
pub struct EMAC_TIMSTCTRL_TSUPDTR {
    bits: bool,
}
impl EMAC_TIMSTCTRL_TSUPDTR {
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
pub struct _EMAC_TIMSTCTRL_TSUPDTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_TIMSTCTRL_TSUPDTW<'a> {
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
pub struct EMAC_TIMSTCTRL_INTTRIGR {
    bits: bool,
}
impl EMAC_TIMSTCTRL_INTTRIGR {
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
pub struct _EMAC_TIMSTCTRL_INTTRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_TIMSTCTRL_INTTRIGW<'a> {
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
pub struct EMAC_TIMSTCTRL_ADDREGUPR {
    bits: bool,
}
impl EMAC_TIMSTCTRL_ADDREGUPR {
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
pub struct _EMAC_TIMSTCTRL_ADDREGUPW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_TIMSTCTRL_ADDREGUPW<'a> {
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
pub struct EMAC_TIMSTCTRL_ALLFR {
    bits: bool,
}
impl EMAC_TIMSTCTRL_ALLFR {
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
pub struct _EMAC_TIMSTCTRL_ALLFW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_TIMSTCTRL_ALLFW<'a> {
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
pub struct EMAC_TIMSTCTRL_DGTLBINR {
    bits: bool,
}
impl EMAC_TIMSTCTRL_DGTLBINR {
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
pub struct _EMAC_TIMSTCTRL_DGTLBINW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_TIMSTCTRL_DGTLBINW<'a> {
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
pub struct EMAC_TIMSTCTRL_PTPVER2R {
    bits: bool,
}
impl EMAC_TIMSTCTRL_PTPVER2R {
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
pub struct _EMAC_TIMSTCTRL_PTPVER2W<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_TIMSTCTRL_PTPVER2W<'a> {
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
pub struct EMAC_TIMSTCTRL_PTPETHR {
    bits: bool,
}
impl EMAC_TIMSTCTRL_PTPETHR {
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
pub struct _EMAC_TIMSTCTRL_PTPETHW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_TIMSTCTRL_PTPETHW<'a> {
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
pub struct EMAC_TIMSTCTRL_PTPIPV6R {
    bits: bool,
}
impl EMAC_TIMSTCTRL_PTPIPV6R {
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
pub struct _EMAC_TIMSTCTRL_PTPIPV6W<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_TIMSTCTRL_PTPIPV6W<'a> {
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
pub struct EMAC_TIMSTCTRL_PTPIPV4R {
    bits: bool,
}
impl EMAC_TIMSTCTRL_PTPIPV4R {
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
pub struct _EMAC_TIMSTCTRL_PTPIPV4W<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_TIMSTCTRL_PTPIPV4W<'a> {
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
pub struct EMAC_TIMSTCTRL_TSEVNTR {
    bits: bool,
}
impl EMAC_TIMSTCTRL_TSEVNTR {
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
pub struct _EMAC_TIMSTCTRL_TSEVNTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_TIMSTCTRL_TSEVNTW<'a> {
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
pub struct EMAC_TIMSTCTRL_TSMASTR {
    bits: bool,
}
impl EMAC_TIMSTCTRL_TSMASTR {
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
pub struct _EMAC_TIMSTCTRL_TSMASTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_TIMSTCTRL_TSMASTW<'a> {
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
        self.w.bits &= !(1 << 15);
        self.w.bits |= ((value as u32) & 1) << 15;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_TIMSTCTRL_SELPTPR {
    bits: u8,
}
impl EMAC_TIMSTCTRL_SELPTPR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_TIMSTCTRL_SELPTPW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_TIMSTCTRL_SELPTPW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 16);
        self.w.bits |= ((value as u32) & 3) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_TIMSTCTRL_PTPFLTRR {
    bits: bool,
}
impl EMAC_TIMSTCTRL_PTPFLTRR {
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
pub struct _EMAC_TIMSTCTRL_PTPFLTRW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_TIMSTCTRL_PTPFLTRW<'a> {
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
        self.w.bits &= !(1 << 18);
        self.w.bits |= ((value as u32) & 1) << 18;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Timestamp Enable"]
    #[inline(always)]
    pub fn emac_timstctrl_tsen(&self) -> EMAC_TIMSTCTRL_TSENR {
        let bits = ((self.bits >> 0) & 1) != 0;
        EMAC_TIMSTCTRL_TSENR { bits }
    }
    #[doc = "Bit 1 - Timestamp Fine or Coarse Update"]
    #[inline(always)]
    pub fn emac_timstctrl_tsfcupdt(&self) -> EMAC_TIMSTCTRL_TSFCUPDTR {
        let bits = ((self.bits >> 1) & 1) != 0;
        EMAC_TIMSTCTRL_TSFCUPDTR { bits }
    }
    #[doc = "Bit 2 - Timestamp Initialize"]
    #[inline(always)]
    pub fn emac_timstctrl_tsinit(&self) -> EMAC_TIMSTCTRL_TSINITR {
        let bits = ((self.bits >> 2) & 1) != 0;
        EMAC_TIMSTCTRL_TSINITR { bits }
    }
    #[doc = "Bit 3 - Timestamp Update"]
    #[inline(always)]
    pub fn emac_timstctrl_tsupdt(&self) -> EMAC_TIMSTCTRL_TSUPDTR {
        let bits = ((self.bits >> 3) & 1) != 0;
        EMAC_TIMSTCTRL_TSUPDTR { bits }
    }
    #[doc = "Bit 4 - Timestamp Interrupt Trigger Enable"]
    #[inline(always)]
    pub fn emac_timstctrl_inttrig(&self) -> EMAC_TIMSTCTRL_INTTRIGR {
        let bits = ((self.bits >> 4) & 1) != 0;
        EMAC_TIMSTCTRL_INTTRIGR { bits }
    }
    #[doc = "Bit 5 - Addend Register Update"]
    #[inline(always)]
    pub fn emac_timstctrl_addregup(&self) -> EMAC_TIMSTCTRL_ADDREGUPR {
        let bits = ((self.bits >> 5) & 1) != 0;
        EMAC_TIMSTCTRL_ADDREGUPR { bits }
    }
    #[doc = "Bit 8 - Enable Timestamp For All Frames"]
    #[inline(always)]
    pub fn emac_timstctrl_allf(&self) -> EMAC_TIMSTCTRL_ALLFR {
        let bits = ((self.bits >> 8) & 1) != 0;
        EMAC_TIMSTCTRL_ALLFR { bits }
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control"]
    #[inline(always)]
    pub fn emac_timstctrl_dgtlbin(&self) -> EMAC_TIMSTCTRL_DGTLBINR {
        let bits = ((self.bits >> 9) & 1) != 0;
        EMAC_TIMSTCTRL_DGTLBINR { bits }
    }
    #[doc = "Bit 10 - Enable PTP Packet Processing For Version 2 Format"]
    #[inline(always)]
    pub fn emac_timstctrl_ptpver2(&self) -> EMAC_TIMSTCTRL_PTPVER2R {
        let bits = ((self.bits >> 10) & 1) != 0;
        EMAC_TIMSTCTRL_PTPVER2R { bits }
    }
    #[doc = "Bit 11 - Enable Processing of PTP Over Ethernet Frames"]
    #[inline(always)]
    pub fn emac_timstctrl_ptpeth(&self) -> EMAC_TIMSTCTRL_PTPETHR {
        let bits = ((self.bits >> 11) & 1) != 0;
        EMAC_TIMSTCTRL_PTPETHR { bits }
    }
    #[doc = "Bit 12 - Enable Processing of PTP Frames Sent Over IPv6-UDP"]
    #[inline(always)]
    pub fn emac_timstctrl_ptpipv6(&self) -> EMAC_TIMSTCTRL_PTPIPV6R {
        let bits = ((self.bits >> 12) & 1) != 0;
        EMAC_TIMSTCTRL_PTPIPV6R { bits }
    }
    #[doc = "Bit 13 - Enable Processing of PTP Frames Sent over IPv4-UDP"]
    #[inline(always)]
    pub fn emac_timstctrl_ptpipv4(&self) -> EMAC_TIMSTCTRL_PTPIPV4R {
        let bits = ((self.bits >> 13) & 1) != 0;
        EMAC_TIMSTCTRL_PTPIPV4R { bits }
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages"]
    #[inline(always)]
    pub fn emac_timstctrl_tsevnt(&self) -> EMAC_TIMSTCTRL_TSEVNTR {
        let bits = ((self.bits >> 14) & 1) != 0;
        EMAC_TIMSTCTRL_TSEVNTR { bits }
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master"]
    #[inline(always)]
    pub fn emac_timstctrl_tsmast(&self) -> EMAC_TIMSTCTRL_TSMASTR {
        let bits = ((self.bits >> 15) & 1) != 0;
        EMAC_TIMSTCTRL_TSMASTR { bits }
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots"]
    #[inline(always)]
    pub fn emac_timstctrl_selptp(&self) -> EMAC_TIMSTCTRL_SELPTPR {
        let bits = ((self.bits >> 16) & 3) as u8;
        EMAC_TIMSTCTRL_SELPTPR { bits }
    }
    #[doc = "Bit 18 - Enable MAC address for PTP Frame Filtering"]
    #[inline(always)]
    pub fn emac_timstctrl_ptpfltr(&self) -> EMAC_TIMSTCTRL_PTPFLTRR {
        let bits = ((self.bits >> 18) & 1) != 0;
        EMAC_TIMSTCTRL_PTPFLTRR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Timestamp Enable"]
    #[inline(always)]
    pub fn emac_timstctrl_tsen(&mut self) -> _EMAC_TIMSTCTRL_TSENW {
        _EMAC_TIMSTCTRL_TSENW { w: self }
    }
    #[doc = "Bit 1 - Timestamp Fine or Coarse Update"]
    #[inline(always)]
    pub fn emac_timstctrl_tsfcupdt(&mut self) -> _EMAC_TIMSTCTRL_TSFCUPDTW {
        _EMAC_TIMSTCTRL_TSFCUPDTW { w: self }
    }
    #[doc = "Bit 2 - Timestamp Initialize"]
    #[inline(always)]
    pub fn emac_timstctrl_tsinit(&mut self) -> _EMAC_TIMSTCTRL_TSINITW {
        _EMAC_TIMSTCTRL_TSINITW { w: self }
    }
    #[doc = "Bit 3 - Timestamp Update"]
    #[inline(always)]
    pub fn emac_timstctrl_tsupdt(&mut self) -> _EMAC_TIMSTCTRL_TSUPDTW {
        _EMAC_TIMSTCTRL_TSUPDTW { w: self }
    }
    #[doc = "Bit 4 - Timestamp Interrupt Trigger Enable"]
    #[inline(always)]
    pub fn emac_timstctrl_inttrig(&mut self) -> _EMAC_TIMSTCTRL_INTTRIGW {
        _EMAC_TIMSTCTRL_INTTRIGW { w: self }
    }
    #[doc = "Bit 5 - Addend Register Update"]
    #[inline(always)]
    pub fn emac_timstctrl_addregup(&mut self) -> _EMAC_TIMSTCTRL_ADDREGUPW {
        _EMAC_TIMSTCTRL_ADDREGUPW { w: self }
    }
    #[doc = "Bit 8 - Enable Timestamp For All Frames"]
    #[inline(always)]
    pub fn emac_timstctrl_allf(&mut self) -> _EMAC_TIMSTCTRL_ALLFW {
        _EMAC_TIMSTCTRL_ALLFW { w: self }
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control"]
    #[inline(always)]
    pub fn emac_timstctrl_dgtlbin(&mut self) -> _EMAC_TIMSTCTRL_DGTLBINW {
        _EMAC_TIMSTCTRL_DGTLBINW { w: self }
    }
    #[doc = "Bit 10 - Enable PTP Packet Processing For Version 2 Format"]
    #[inline(always)]
    pub fn emac_timstctrl_ptpver2(&mut self) -> _EMAC_TIMSTCTRL_PTPVER2W {
        _EMAC_TIMSTCTRL_PTPVER2W { w: self }
    }
    #[doc = "Bit 11 - Enable Processing of PTP Over Ethernet Frames"]
    #[inline(always)]
    pub fn emac_timstctrl_ptpeth(&mut self) -> _EMAC_TIMSTCTRL_PTPETHW {
        _EMAC_TIMSTCTRL_PTPETHW { w: self }
    }
    #[doc = "Bit 12 - Enable Processing of PTP Frames Sent Over IPv6-UDP"]
    #[inline(always)]
    pub fn emac_timstctrl_ptpipv6(&mut self) -> _EMAC_TIMSTCTRL_PTPIPV6W {
        _EMAC_TIMSTCTRL_PTPIPV6W { w: self }
    }
    #[doc = "Bit 13 - Enable Processing of PTP Frames Sent over IPv4-UDP"]
    #[inline(always)]
    pub fn emac_timstctrl_ptpipv4(&mut self) -> _EMAC_TIMSTCTRL_PTPIPV4W {
        _EMAC_TIMSTCTRL_PTPIPV4W { w: self }
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages"]
    #[inline(always)]
    pub fn emac_timstctrl_tsevnt(&mut self) -> _EMAC_TIMSTCTRL_TSEVNTW {
        _EMAC_TIMSTCTRL_TSEVNTW { w: self }
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master"]
    #[inline(always)]
    pub fn emac_timstctrl_tsmast(&mut self) -> _EMAC_TIMSTCTRL_TSMASTW {
        _EMAC_TIMSTCTRL_TSMASTW { w: self }
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots"]
    #[inline(always)]
    pub fn emac_timstctrl_selptp(&mut self) -> _EMAC_TIMSTCTRL_SELPTPW {
        _EMAC_TIMSTCTRL_SELPTPW { w: self }
    }
    #[doc = "Bit 18 - Enable MAC address for PTP Frame Filtering"]
    #[inline(always)]
    pub fn emac_timstctrl_ptpfltr(&mut self) -> _EMAC_TIMSTCTRL_PTPFLTRW {
        _EMAC_TIMSTCTRL_PTPFLTRW { w: self }
    }
}
