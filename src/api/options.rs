//! Settings for [`Engine`]'s language options.

use crate::Engine;
use bitflags::bitflags;
#[cfg(feature = "no_std")]
use std::prelude::v1::*;

bitflags! {
    /// Bit-flags containing all language options for the [`Engine`].
    pub struct LangOptions: u8 {
        /// Is `if`-expression allowed?
        const IF_EXPR = 0b_00000001;
        /// Is `switch` expression allowed?
        const SWITCH_EXPR = 0b_00000010;
        /// Is statement-expression allowed?
        const STMT_EXPR = 0b_00000100;
        /// Is anonymous function allowed?
        #[cfg(not(feature = "no_function"))]
        const ANON_FN = 0b_00001000;
        /// Is looping allowed?
        const LOOPING = 0b_00010000;
        /// Is variables shadowing allowed?
        const SHADOW = 0b_00100000;
        /// Strict variables mode?
        const STRICT_VAR = 0b_01000000;
        /// Raise error if an object map property does not exist?
        /// Returns `()` if `false`.
        #[cfg(not(feature = "no_object"))]
        const FAIL_ON_INVALID_MAP_PROPERTY = 0b_10000000;
    }
}

impl LangOptions {
    /// Create a new [`Options`] with default values.
    #[inline(always)]
    pub fn new() -> Self {
        Self::IF_EXPR | Self::SWITCH_EXPR | Self::STMT_EXPR | Self::LOOPING | Self::SHADOW | {
            #[cfg(not(feature = "no_function"))]
            {
                Self::ANON_FN
            }
            #[cfg(feature = "no_function")]
            {
                Self::empty()
            }
        }
    }
}

impl Engine {
    /// Is `if`-expression allowed?
    /// Default is `true`.
    #[inline(always)]
    pub const fn allow_if_expression(&self) -> bool {
        self.options.contains(LangOptions::IF_EXPR)
    }
    /// Set whether `if`-expression is allowed.
    #[inline(always)]
    pub fn set_allow_if_expression(&mut self, enable: bool) {
        self.options.set(LangOptions::IF_EXPR, enable)
    }
    /// Is `switch` expression allowed?
    /// Default is `true`.
    #[inline(always)]
    pub const fn allow_switch_expression(&self) -> bool {
        self.options.contains(LangOptions::SWITCH_EXPR)
    }
    /// Set whether `switch` expression is allowed.
    #[inline(always)]
    pub fn set_allow_switch_expression(&mut self, enable: bool) {
        self.options.set(LangOptions::SWITCH_EXPR, enable);
    }
    /// Is statement-expression allowed?
    /// Default is `true`.
    #[inline(always)]
    pub const fn allow_statement_expression(&self) -> bool {
        self.options.contains(LangOptions::STMT_EXPR)
    }
    /// Set whether statement-expression is allowed.
    #[inline(always)]
    pub fn set_allow_statement_expression(&mut self, enable: bool) {
        self.options.set(LangOptions::STMT_EXPR, enable);
    }
    /// Is anonymous function allowed?
    /// Default is `true`.
    ///
    /// Not available under `no_function`.
    #[cfg(not(feature = "no_function"))]
    #[inline(always)]
    pub const fn allow_anonymous_fn(&self) -> bool {
        self.options.contains(LangOptions::ANON_FN)
    }
    /// Set whether anonymous function is allowed.
    ///
    /// Not available under `no_function`.
    #[cfg(not(feature = "no_function"))]
    #[inline(always)]
    pub fn set_allow_anonymous_fn(&mut self, enable: bool) {
        self.options.set(LangOptions::ANON_FN, enable);
    }
    /// Is looping allowed?
    /// Default is `true`.
    #[inline(always)]
    pub const fn allow_looping(&self) -> bool {
        self.options.contains(LangOptions::LOOPING)
    }
    /// Set whether looping is allowed.
    #[inline(always)]
    pub fn set_allow_looping(&mut self, enable: bool) {
        self.options.set(LangOptions::LOOPING, enable);
    }
    /// Is variables shadowing allowed?
    /// Default is `true`.
    #[inline(always)]
    pub const fn allow_shadowing(&self) -> bool {
        self.options.contains(LangOptions::SHADOW)
    }
    /// Set whether variables shadowing is allowed.
    #[inline(always)]
    pub fn set_allow_shadowing(&mut self, enable: bool) {
        self.options.set(LangOptions::SHADOW, enable);
    }
    /// Is strict variables mode enabled?
    /// Default is `false`.
    #[inline(always)]
    pub const fn strict_variables(&self) -> bool {
        self.options.contains(LangOptions::STRICT_VAR)
    }
    /// Set whether strict variables mode is enabled.
    #[inline(always)]
    pub fn set_strict_variables(&mut self, enable: bool) {
        self.options.set(LangOptions::STRICT_VAR, enable);
    }
    /// Raise error if an object map property does not exist?
    /// Default is `false`.
    ///
    /// Not available under `no_object`.
    #[cfg(not(feature = "no_object"))]
    #[inline(always)]
    pub const fn fail_on_invalid_map_property(&self) -> bool {
        self.options
            .contains(LangOptions::FAIL_ON_INVALID_MAP_PROPERTY)
    }
    /// Set whether to raise error if an object map property does not exist.
    ///
    /// Not available under `no_object`.
    #[cfg(not(feature = "no_object"))]
    #[inline(always)]
    pub fn set_fail_on_invalid_map_property(&mut self, enable: bool) {
        self.options
            .set(LangOptions::FAIL_ON_INVALID_MAP_PROPERTY, enable);
    }
}
