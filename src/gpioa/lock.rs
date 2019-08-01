#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LOCK {
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
#[doc = "Possible values of the field `GPIO_LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_LOCKR {
    #[doc = "The GPIOCR register is unlocked and may be modified"]
    GPIO_LOCK_UNLOCKED,
    #[doc = "The GPIOCR register is locked and may not be modified"]
    GPIO_LOCK_LOCKED,
    #[doc = r"Reserved"]
    _Reserved(u32),
}
impl GPIO_LOCKR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        match *self {
            GPIO_LOCKR::GPIO_LOCK_UNLOCKED => 0,
            GPIO_LOCKR::GPIO_LOCK_LOCKED => 1,
            GPIO_LOCKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u32) -> GPIO_LOCKR {
        match value {
            0 => GPIO_LOCKR::GPIO_LOCK_UNLOCKED,
            1 => GPIO_LOCKR::GPIO_LOCK_LOCKED,
            i => GPIO_LOCKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_LOCK_UNLOCKED`"]
    #[inline(always)]
    pub fn is_gpio_lock_unlocked(&self) -> bool {
        *self == GPIO_LOCKR::GPIO_LOCK_UNLOCKED
    }
    #[doc = "Checks if the value of the field is `GPIO_LOCK_LOCKED`"]
    #[inline(always)]
    pub fn is_gpio_lock_locked(&self) -> bool {
        *self == GPIO_LOCKR::GPIO_LOCK_LOCKED
    }
}
#[doc = "Values that can be written to the field `GPIO_LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_LOCKW {
    #[doc = "The GPIOCR register is unlocked and may be modified"]
    GPIO_LOCK_UNLOCKED,
    #[doc = "The GPIOCR register is locked and may not be modified"]
    GPIO_LOCK_LOCKED,
}
impl GPIO_LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u32 {
        match *self {
            GPIO_LOCKW::GPIO_LOCK_UNLOCKED => 0,
            GPIO_LOCKW::GPIO_LOCK_LOCKED => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _GPIO_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_LOCKW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_LOCKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The GPIOCR register is unlocked and may be modified"]
    #[inline(always)]
    pub fn gpio_lock_unlocked(self) -> &'a mut W {
        self.variant(GPIO_LOCKW::GPIO_LOCK_UNLOCKED)
    }
    #[doc = "The GPIOCR register is locked and may not be modified"]
    #[inline(always)]
    pub fn gpio_lock_locked(self) -> &'a mut W {
        self.variant(GPIO_LOCKW::GPIO_LOCK_LOCKED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits &= !(4294967295 << 0);
        self.w.bits |= ((value as u32) & 4294967295) << 0;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - GPIO Lock"]
    #[inline(always)]
    pub fn gpio_lock(&self) -> GPIO_LOCKR {
        GPIO_LOCKR::_from(((self.bits >> 0) & 4294967295) as u32)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:31 - GPIO Lock"]
    #[inline(always)]
    pub fn gpio_lock(&mut self) -> _GPIO_LOCKW {
        _GPIO_LOCKW { w: self }
    }
}
