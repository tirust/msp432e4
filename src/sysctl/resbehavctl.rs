#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RESBEHAVCTL {
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
#[doc = "Possible values of the field `SYSCTL_RESBEHAVCTL_EXTRES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_RESBEHAVCTL_EXTRESR {
    #[doc = "External RST assertion issues a system reset. The application starts within 10 us"]
    SYSCTL_RESBEHAVCTL_EXTRES_SYSRST,
    #[doc = "External RST assertion issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    SYSCTL_RESBEHAVCTL_EXTRES_POR,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_RESBEHAVCTL_EXTRESR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_RESBEHAVCTL_EXTRESR::SYSCTL_RESBEHAVCTL_EXTRES_SYSRST => 2,
            SYSCTL_RESBEHAVCTL_EXTRESR::SYSCTL_RESBEHAVCTL_EXTRES_POR => 3,
            SYSCTL_RESBEHAVCTL_EXTRESR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_RESBEHAVCTL_EXTRESR {
        match value {
            2 => SYSCTL_RESBEHAVCTL_EXTRESR::SYSCTL_RESBEHAVCTL_EXTRES_SYSRST,
            3 => SYSCTL_RESBEHAVCTL_EXTRESR::SYSCTL_RESBEHAVCTL_EXTRES_POR,
            i => SYSCTL_RESBEHAVCTL_EXTRESR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RESBEHAVCTL_EXTRES_SYSRST`"]
    #[inline(always)]
    pub fn is_sysctl_resbehavctl_extres_sysrst(&self) -> bool {
        *self == SYSCTL_RESBEHAVCTL_EXTRESR::SYSCTL_RESBEHAVCTL_EXTRES_SYSRST
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RESBEHAVCTL_EXTRES_POR`"]
    #[inline(always)]
    pub fn is_sysctl_resbehavctl_extres_por(&self) -> bool {
        *self == SYSCTL_RESBEHAVCTL_EXTRESR::SYSCTL_RESBEHAVCTL_EXTRES_POR
    }
}
#[doc = "Values that can be written to the field `SYSCTL_RESBEHAVCTL_EXTRES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_RESBEHAVCTL_EXTRESW {
    #[doc = "External RST assertion issues a system reset. The application starts within 10 us"]
    SYSCTL_RESBEHAVCTL_EXTRES_SYSRST,
    #[doc = "External RST assertion issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    SYSCTL_RESBEHAVCTL_EXTRES_POR,
}
impl SYSCTL_RESBEHAVCTL_EXTRESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_RESBEHAVCTL_EXTRESW::SYSCTL_RESBEHAVCTL_EXTRES_SYSRST => 2,
            SYSCTL_RESBEHAVCTL_EXTRESW::SYSCTL_RESBEHAVCTL_EXTRES_POR => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_RESBEHAVCTL_EXTRESW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RESBEHAVCTL_EXTRESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_RESBEHAVCTL_EXTRESW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "External RST assertion issues a system reset. The application starts within 10 us"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_extres_sysrst(self) -> &'a mut W {
        self.variant(SYSCTL_RESBEHAVCTL_EXTRESW::SYSCTL_RESBEHAVCTL_EXTRES_SYSRST)
    }
    #[doc = "External RST assertion issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_extres_por(self) -> &'a mut W {
        self.variant(SYSCTL_RESBEHAVCTL_EXTRESW::SYSCTL_RESBEHAVCTL_EXTRES_POR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `SYSCTL_RESBEHAVCTL_BOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_RESBEHAVCTL_BORR {
    #[doc = "Brown Out Reset issues system reset. The application starts within 10 us"]
    SYSCTL_RESBEHAVCTL_BOR_SYSRST,
    #[doc = "Brown Out Reset issues a simulated POR sequence. The application starts less than 500 us after deassertion (Default)"]
    SYSCTL_RESBEHAVCTL_BOR_POR,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_RESBEHAVCTL_BORR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_RESBEHAVCTL_BORR::SYSCTL_RESBEHAVCTL_BOR_SYSRST => 2,
            SYSCTL_RESBEHAVCTL_BORR::SYSCTL_RESBEHAVCTL_BOR_POR => 3,
            SYSCTL_RESBEHAVCTL_BORR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_RESBEHAVCTL_BORR {
        match value {
            2 => SYSCTL_RESBEHAVCTL_BORR::SYSCTL_RESBEHAVCTL_BOR_SYSRST,
            3 => SYSCTL_RESBEHAVCTL_BORR::SYSCTL_RESBEHAVCTL_BOR_POR,
            i => SYSCTL_RESBEHAVCTL_BORR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RESBEHAVCTL_BOR_SYSRST`"]
    #[inline(always)]
    pub fn is_sysctl_resbehavctl_bor_sysrst(&self) -> bool {
        *self == SYSCTL_RESBEHAVCTL_BORR::SYSCTL_RESBEHAVCTL_BOR_SYSRST
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RESBEHAVCTL_BOR_POR`"]
    #[inline(always)]
    pub fn is_sysctl_resbehavctl_bor_por(&self) -> bool {
        *self == SYSCTL_RESBEHAVCTL_BORR::SYSCTL_RESBEHAVCTL_BOR_POR
    }
}
#[doc = "Values that can be written to the field `SYSCTL_RESBEHAVCTL_BOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_RESBEHAVCTL_BORW {
    #[doc = "Brown Out Reset issues system reset. The application starts within 10 us"]
    SYSCTL_RESBEHAVCTL_BOR_SYSRST,
    #[doc = "Brown Out Reset issues a simulated POR sequence. The application starts less than 500 us after deassertion (Default)"]
    SYSCTL_RESBEHAVCTL_BOR_POR,
}
impl SYSCTL_RESBEHAVCTL_BORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_RESBEHAVCTL_BORW::SYSCTL_RESBEHAVCTL_BOR_SYSRST => 2,
            SYSCTL_RESBEHAVCTL_BORW::SYSCTL_RESBEHAVCTL_BOR_POR => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_RESBEHAVCTL_BORW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RESBEHAVCTL_BORW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_RESBEHAVCTL_BORW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Brown Out Reset issues system reset. The application starts within 10 us"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_bor_sysrst(self) -> &'a mut W {
        self.variant(SYSCTL_RESBEHAVCTL_BORW::SYSCTL_RESBEHAVCTL_BOR_SYSRST)
    }
    #[doc = "Brown Out Reset issues a simulated POR sequence. The application starts less than 500 us after deassertion (Default)"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_bor_por(self) -> &'a mut W {
        self.variant(SYSCTL_RESBEHAVCTL_BORW::SYSCTL_RESBEHAVCTL_BOR_POR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 2);
        self.w.bits |= ((value as u32) & 3) << 2;
        self.w
    }
}
#[doc = "Possible values of the field `SYSCTL_RESBEHAVCTL_WDOG0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_RESBEHAVCTL_WDOG0R {
    #[doc = "Watchdog 0 issues a system reset. The application starts within 10 us"]
    SYSCTL_RESBEHAVCTL_WDOG0_SYSRST,
    #[doc = "Watchdog 0 issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    SYSCTL_RESBEHAVCTL_WDOG0_POR,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_RESBEHAVCTL_WDOG0R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_RESBEHAVCTL_WDOG0R::SYSCTL_RESBEHAVCTL_WDOG0_SYSRST => 2,
            SYSCTL_RESBEHAVCTL_WDOG0R::SYSCTL_RESBEHAVCTL_WDOG0_POR => 3,
            SYSCTL_RESBEHAVCTL_WDOG0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_RESBEHAVCTL_WDOG0R {
        match value {
            2 => SYSCTL_RESBEHAVCTL_WDOG0R::SYSCTL_RESBEHAVCTL_WDOG0_SYSRST,
            3 => SYSCTL_RESBEHAVCTL_WDOG0R::SYSCTL_RESBEHAVCTL_WDOG0_POR,
            i => SYSCTL_RESBEHAVCTL_WDOG0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RESBEHAVCTL_WDOG0_SYSRST`"]
    #[inline(always)]
    pub fn is_sysctl_resbehavctl_wdog0_sysrst(&self) -> bool {
        *self == SYSCTL_RESBEHAVCTL_WDOG0R::SYSCTL_RESBEHAVCTL_WDOG0_SYSRST
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RESBEHAVCTL_WDOG0_POR`"]
    #[inline(always)]
    pub fn is_sysctl_resbehavctl_wdog0_por(&self) -> bool {
        *self == SYSCTL_RESBEHAVCTL_WDOG0R::SYSCTL_RESBEHAVCTL_WDOG0_POR
    }
}
#[doc = "Values that can be written to the field `SYSCTL_RESBEHAVCTL_WDOG0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_RESBEHAVCTL_WDOG0W {
    #[doc = "Watchdog 0 issues a system reset. The application starts within 10 us"]
    SYSCTL_RESBEHAVCTL_WDOG0_SYSRST,
    #[doc = "Watchdog 0 issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    SYSCTL_RESBEHAVCTL_WDOG0_POR,
}
impl SYSCTL_RESBEHAVCTL_WDOG0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_RESBEHAVCTL_WDOG0W::SYSCTL_RESBEHAVCTL_WDOG0_SYSRST => 2,
            SYSCTL_RESBEHAVCTL_WDOG0W::SYSCTL_RESBEHAVCTL_WDOG0_POR => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_RESBEHAVCTL_WDOG0W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RESBEHAVCTL_WDOG0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_RESBEHAVCTL_WDOG0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Watchdog 0 issues a system reset. The application starts within 10 us"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_wdog0_sysrst(self) -> &'a mut W {
        self.variant(SYSCTL_RESBEHAVCTL_WDOG0W::SYSCTL_RESBEHAVCTL_WDOG0_SYSRST)
    }
    #[doc = "Watchdog 0 issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_wdog0_por(self) -> &'a mut W {
        self.variant(SYSCTL_RESBEHAVCTL_WDOG0W::SYSCTL_RESBEHAVCTL_WDOG0_POR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
        self.w
    }
}
#[doc = "Possible values of the field `SYSCTL_RESBEHAVCTL_WDOG1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_RESBEHAVCTL_WDOG1R {
    #[doc = "Watchdog 1 issues a system reset. The application starts within 10 us"]
    SYSCTL_RESBEHAVCTL_WDOG1_SYSRST,
    #[doc = "Watchdog 1 issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    SYSCTL_RESBEHAVCTL_WDOG1_POR,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_RESBEHAVCTL_WDOG1R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_RESBEHAVCTL_WDOG1R::SYSCTL_RESBEHAVCTL_WDOG1_SYSRST => 2,
            SYSCTL_RESBEHAVCTL_WDOG1R::SYSCTL_RESBEHAVCTL_WDOG1_POR => 3,
            SYSCTL_RESBEHAVCTL_WDOG1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_RESBEHAVCTL_WDOG1R {
        match value {
            2 => SYSCTL_RESBEHAVCTL_WDOG1R::SYSCTL_RESBEHAVCTL_WDOG1_SYSRST,
            3 => SYSCTL_RESBEHAVCTL_WDOG1R::SYSCTL_RESBEHAVCTL_WDOG1_POR,
            i => SYSCTL_RESBEHAVCTL_WDOG1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RESBEHAVCTL_WDOG1_SYSRST`"]
    #[inline(always)]
    pub fn is_sysctl_resbehavctl_wdog1_sysrst(&self) -> bool {
        *self == SYSCTL_RESBEHAVCTL_WDOG1R::SYSCTL_RESBEHAVCTL_WDOG1_SYSRST
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RESBEHAVCTL_WDOG1_POR`"]
    #[inline(always)]
    pub fn is_sysctl_resbehavctl_wdog1_por(&self) -> bool {
        *self == SYSCTL_RESBEHAVCTL_WDOG1R::SYSCTL_RESBEHAVCTL_WDOG1_POR
    }
}
#[doc = "Values that can be written to the field `SYSCTL_RESBEHAVCTL_WDOG1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_RESBEHAVCTL_WDOG1W {
    #[doc = "Watchdog 1 issues a system reset. The application starts within 10 us"]
    SYSCTL_RESBEHAVCTL_WDOG1_SYSRST,
    #[doc = "Watchdog 1 issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    SYSCTL_RESBEHAVCTL_WDOG1_POR,
}
impl SYSCTL_RESBEHAVCTL_WDOG1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_RESBEHAVCTL_WDOG1W::SYSCTL_RESBEHAVCTL_WDOG1_SYSRST => 2,
            SYSCTL_RESBEHAVCTL_WDOG1W::SYSCTL_RESBEHAVCTL_WDOG1_POR => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_RESBEHAVCTL_WDOG1W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RESBEHAVCTL_WDOG1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_RESBEHAVCTL_WDOG1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Watchdog 1 issues a system reset. The application starts within 10 us"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_wdog1_sysrst(self) -> &'a mut W {
        self.variant(SYSCTL_RESBEHAVCTL_WDOG1W::SYSCTL_RESBEHAVCTL_WDOG1_SYSRST)
    }
    #[doc = "Watchdog 1 issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_wdog1_por(self) -> &'a mut W {
        self.variant(SYSCTL_RESBEHAVCTL_WDOG1W::SYSCTL_RESBEHAVCTL_WDOG1_POR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 6);
        self.w.bits |= ((value as u32) & 3) << 6;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - External RST Pin Operation"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_extres(&self) -> SYSCTL_RESBEHAVCTL_EXTRESR {
        SYSCTL_RESBEHAVCTL_EXTRESR::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bits 2:3 - BOR Reset operation"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_bor(&self) -> SYSCTL_RESBEHAVCTL_BORR {
        SYSCTL_RESBEHAVCTL_BORR::_from(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Watchdog 0 Reset Operation"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_wdog0(&self) -> SYSCTL_RESBEHAVCTL_WDOG0R {
        SYSCTL_RESBEHAVCTL_WDOG0R::_from(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Watchdog 1 Reset Operation"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_wdog1(&self) -> SYSCTL_RESBEHAVCTL_WDOG1R {
        SYSCTL_RESBEHAVCTL_WDOG1R::_from(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - External RST Pin Operation"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_extres(&mut self) -> _SYSCTL_RESBEHAVCTL_EXTRESW {
        _SYSCTL_RESBEHAVCTL_EXTRESW { w: self }
    }
    #[doc = "Bits 2:3 - BOR Reset operation"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_bor(&mut self) -> _SYSCTL_RESBEHAVCTL_BORW {
        _SYSCTL_RESBEHAVCTL_BORW { w: self }
    }
    #[doc = "Bits 4:5 - Watchdog 0 Reset Operation"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_wdog0(&mut self) -> _SYSCTL_RESBEHAVCTL_WDOG0W {
        _SYSCTL_RESBEHAVCTL_WDOG0W { w: self }
    }
    #[doc = "Bits 6:7 - Watchdog 1 Reset Operation"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_wdog1(&mut self) -> _SYSCTL_RESBEHAVCTL_WDOG1W {
        _SYSCTL_RESBEHAVCTL_WDOG1W { w: self }
    }
}
