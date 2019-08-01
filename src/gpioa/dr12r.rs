#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DR12R {
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
#[doc = "Possible values of the field `GPIO_DR12R_DRV12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_DR12R_DRV12R {
    #[doc = "The corresponding GPIO pin has 12-mA drive. This encoding is only valid if the GPIOPP EDE bit is set and the appropriate GPIOPC EDM bit field is programmed to 0x3"]
    GPIO_DR12R_DRV12_12MA,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl GPIO_DR12R_DRV12R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO_DR12R_DRV12R::GPIO_DR12R_DRV12_12MA => 1,
            GPIO_DR12R_DRV12R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> GPIO_DR12R_DRV12R {
        match value {
            1 => GPIO_DR12R_DRV12R::GPIO_DR12R_DRV12_12MA,
            i => GPIO_DR12R_DRV12R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_DR12R_DRV12_12MA`"]
    #[inline(always)]
    pub fn is_gpio_dr12r_drv12_12ma(&self) -> bool {
        *self == GPIO_DR12R_DRV12R::GPIO_DR12R_DRV12_12MA
    }
}
#[doc = "Values that can be written to the field `GPIO_DR12R_DRV12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_DR12R_DRV12W {
    #[doc = "The corresponding GPIO pin has 12-mA drive. This encoding is only valid if the GPIOPP EDE bit is set and the appropriate GPIOPC EDM bit field is programmed to 0x3"]
    GPIO_DR12R_DRV12_12MA,
}
impl GPIO_DR12R_DRV12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO_DR12R_DRV12W::GPIO_DR12R_DRV12_12MA => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _GPIO_DR12R_DRV12W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_DR12R_DRV12W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_DR12R_DRV12W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The corresponding GPIO pin has 12-mA drive. This encoding is only valid if the GPIOPP EDE bit is set and the appropriate GPIOPC EDM bit field is programmed to 0x3"]
    #[inline(always)]
    pub fn gpio_dr12r_drv12_12ma(self) -> &'a mut W {
        self.variant(GPIO_DR12R_DRV12W::GPIO_DR12R_DRV12_12MA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(255 << 0);
        self.w.bits |= ((value as u32) & 255) << 0;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Output Pad 12-mA Drive Enable"]
    #[inline(always)]
    pub fn gpio_dr12r_drv12(&self) -> GPIO_DR12R_DRV12R {
        GPIO_DR12R_DRV12R::_from(((self.bits >> 0) & 255) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Output Pad 12-mA Drive Enable"]
    #[inline(always)]
    pub fn gpio_dr12r_drv12(&mut self) -> _GPIO_DR12R_DRV12W {
        _GPIO_DR12R_DRV12W { w: self }
    }
}
