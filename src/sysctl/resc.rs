#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RESC {
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
pub struct SYSCTL_RESC_EXTR {
    bits: bool,
}
impl SYSCTL_RESC_EXTR {
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
pub struct _SYSCTL_RESC_EXTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RESC_EXTW<'a> {
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
pub struct SYSCTL_RESC_PORR {
    bits: bool,
}
impl SYSCTL_RESC_PORR {
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
pub struct _SYSCTL_RESC_PORW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RESC_PORW<'a> {
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
pub struct SYSCTL_RESC_BORR {
    bits: bool,
}
impl SYSCTL_RESC_BORR {
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
pub struct _SYSCTL_RESC_BORW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RESC_BORW<'a> {
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
pub struct SYSCTL_RESC_WDT0R {
    bits: bool,
}
impl SYSCTL_RESC_WDT0R {
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
pub struct _SYSCTL_RESC_WDT0W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RESC_WDT0W<'a> {
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
pub struct SYSCTL_RESC_SWR {
    bits: bool,
}
impl SYSCTL_RESC_SWR {
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
pub struct _SYSCTL_RESC_SWW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RESC_SWW<'a> {
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
pub struct SYSCTL_RESC_WDT1R {
    bits: bool,
}
impl SYSCTL_RESC_WDT1R {
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
pub struct _SYSCTL_RESC_WDT1W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RESC_WDT1W<'a> {
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
pub struct SYSCTL_RESC_HIBR {
    bits: bool,
}
impl SYSCTL_RESC_HIBR {
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
pub struct _SYSCTL_RESC_HIBW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RESC_HIBW<'a> {
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
pub struct SYSCTL_RESC_HSSRR {
    bits: bool,
}
impl SYSCTL_RESC_HSSRR {
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
pub struct _SYSCTL_RESC_HSSRW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RESC_HSSRW<'a> {
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
pub struct SYSCTL_RESC_MOSCFAILR {
    bits: bool,
}
impl SYSCTL_RESC_MOSCFAILR {
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
pub struct _SYSCTL_RESC_MOSCFAILW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RESC_MOSCFAILW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - External Reset"]
    #[inline(always)]
    pub fn sysctl_resc_ext(&self) -> SYSCTL_RESC_EXTR {
        let bits = ((self.bits >> 0) & 1) != 0;
        SYSCTL_RESC_EXTR { bits }
    }
    #[doc = "Bit 1 - Power-On Reset"]
    #[inline(always)]
    pub fn sysctl_resc_por(&self) -> SYSCTL_RESC_PORR {
        let bits = ((self.bits >> 1) & 1) != 0;
        SYSCTL_RESC_PORR { bits }
    }
    #[doc = "Bit 2 - Brown-Out Reset"]
    #[inline(always)]
    pub fn sysctl_resc_bor(&self) -> SYSCTL_RESC_BORR {
        let bits = ((self.bits >> 2) & 1) != 0;
        SYSCTL_RESC_BORR { bits }
    }
    #[doc = "Bit 3 - Watchdog Timer 0 Reset"]
    #[inline(always)]
    pub fn sysctl_resc_wdt0(&self) -> SYSCTL_RESC_WDT0R {
        let bits = ((self.bits >> 3) & 1) != 0;
        SYSCTL_RESC_WDT0R { bits }
    }
    #[doc = "Bit 4 - Software Reset"]
    #[inline(always)]
    pub fn sysctl_resc_sw(&self) -> SYSCTL_RESC_SWR {
        let bits = ((self.bits >> 4) & 1) != 0;
        SYSCTL_RESC_SWR { bits }
    }
    #[doc = "Bit 5 - Watchdog Timer 1 Reset"]
    #[inline(always)]
    pub fn sysctl_resc_wdt1(&self) -> SYSCTL_RESC_WDT1R {
        let bits = ((self.bits >> 5) & 1) != 0;
        SYSCTL_RESC_WDT1R { bits }
    }
    #[doc = "Bit 6 - HIB Reset"]
    #[inline(always)]
    pub fn sysctl_resc_hib(&self) -> SYSCTL_RESC_HIBR {
        let bits = ((self.bits >> 6) & 1) != 0;
        SYSCTL_RESC_HIBR { bits }
    }
    #[doc = "Bit 12 - HSSR Reset"]
    #[inline(always)]
    pub fn sysctl_resc_hssr(&self) -> SYSCTL_RESC_HSSRR {
        let bits = ((self.bits >> 12) & 1) != 0;
        SYSCTL_RESC_HSSRR { bits }
    }
    #[doc = "Bit 16 - MOSC Failure Reset"]
    #[inline(always)]
    pub fn sysctl_resc_moscfail(&self) -> SYSCTL_RESC_MOSCFAILR {
        let bits = ((self.bits >> 16) & 1) != 0;
        SYSCTL_RESC_MOSCFAILR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - External Reset"]
    #[inline(always)]
    pub fn sysctl_resc_ext(&mut self) -> _SYSCTL_RESC_EXTW {
        _SYSCTL_RESC_EXTW { w: self }
    }
    #[doc = "Bit 1 - Power-On Reset"]
    #[inline(always)]
    pub fn sysctl_resc_por(&mut self) -> _SYSCTL_RESC_PORW {
        _SYSCTL_RESC_PORW { w: self }
    }
    #[doc = "Bit 2 - Brown-Out Reset"]
    #[inline(always)]
    pub fn sysctl_resc_bor(&mut self) -> _SYSCTL_RESC_BORW {
        _SYSCTL_RESC_BORW { w: self }
    }
    #[doc = "Bit 3 - Watchdog Timer 0 Reset"]
    #[inline(always)]
    pub fn sysctl_resc_wdt0(&mut self) -> _SYSCTL_RESC_WDT0W {
        _SYSCTL_RESC_WDT0W { w: self }
    }
    #[doc = "Bit 4 - Software Reset"]
    #[inline(always)]
    pub fn sysctl_resc_sw(&mut self) -> _SYSCTL_RESC_SWW {
        _SYSCTL_RESC_SWW { w: self }
    }
    #[doc = "Bit 5 - Watchdog Timer 1 Reset"]
    #[inline(always)]
    pub fn sysctl_resc_wdt1(&mut self) -> _SYSCTL_RESC_WDT1W {
        _SYSCTL_RESC_WDT1W { w: self }
    }
    #[doc = "Bit 6 - HIB Reset"]
    #[inline(always)]
    pub fn sysctl_resc_hib(&mut self) -> _SYSCTL_RESC_HIBW {
        _SYSCTL_RESC_HIBW { w: self }
    }
    #[doc = "Bit 12 - HSSR Reset"]
    #[inline(always)]
    pub fn sysctl_resc_hssr(&mut self) -> _SYSCTL_RESC_HSSRW {
        _SYSCTL_RESC_HSSRW { w: self }
    }
    #[doc = "Bit 16 - MOSC Failure Reset"]
    #[inline(always)]
    pub fn sysctl_resc_moscfail(&mut self) -> _SYSCTL_RESC_MOSCFAILW {
        _SYSCTL_RESC_MOSCFAILW { w: self }
    }
}
