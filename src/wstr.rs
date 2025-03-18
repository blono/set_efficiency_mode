#![allow(dead_code)]
use std::{char::REPLACEMENT_CHARACTER, marker::PhantomData, mem::zeroed};

#[cfg(target_os = "windows")]
use windows::core::{Param, ParamValue, PCWSTR, PWSTR};

pub struct CWSTR<'a> {
    s: PCWSTR,
    phantom: PhantomData<&'a ()>,
}

pub struct WSTR<'a> {
    s: PWSTR,
    phantom: PhantomData<&'a ()>,
}

impl Param<PCWSTR> for CWSTR<'_> {
    unsafe fn param(self) -> ParamValue<PCWSTR> {
        if self.s.is_null() {
            ParamValue::Borrowed(zeroed())
        } else {
            ParamValue::Owned(self.s)
        }
    }
}

impl Param<PWSTR> for WSTR<'_> {
    unsafe fn param(self) -> ParamValue<PWSTR> {
        if self.s.is_null() {
            ParamValue::Borrowed(zeroed())
        } else {
            ParamValue::Owned(self.s)
        }
    }
}

pub trait WStrExt<'a> {
    fn to_cw(&'a self) -> CWSTR<'a>;
    fn to_w(&'a mut self) -> WSTR<'a>;
}

impl<'a> WStrExt<'a> for Vec<u16> {
    fn to_cw(&'a self) -> CWSTR<'a> {
        CWSTR {
            s: PCWSTR::from_raw(self.as_ptr()),
            phantom: PhantomData,
        }
    }

    fn to_w(&'a mut self) -> WSTR<'a> {
        WSTR {
            s: PWSTR::from_raw(self.as_mut_ptr()),
            phantom: PhantomData,
        }
    }
}

impl<'a> From<&'a Vec<u16>> for CWSTR<'a> {
    fn from(v: &'a Vec<u16>) -> Self {
        v.to_cw()
    }
}

impl<'a> From<&'a mut Vec<u16>> for WSTR<'a> {
    fn from(v: &'a mut Vec<u16>) -> Self {
        v.to_w()
    }
}

impl From<CWSTR<'_>> for PCWSTR {
    fn from(s: CWSTR<'_>) -> Self {
        s.s
    }
}

impl From<WSTR<'_>> for PWSTR {
    fn from(s: WSTR<'_>) -> Self {
        s.s
    }
}

impl From<CWSTR<'_>> for Option<PCWSTR> {
    fn from(s: CWSTR<'_>) -> Self {
        if s.s.is_null() {
            None
        } else {
            Some(s.s)
        }
    }
}

impl From<WSTR<'_>> for Option<PWSTR> {
    fn from(s: WSTR<'_>) -> Self {
        if s.s.is_null() {
            None
        } else {
            Some(s.s)
        }
    }
}

pub fn encode_utf16(source: &str) -> Vec<u16> {
    source.encode_utf16().chain(Some(0)).collect()
}

pub fn decode_utf16(source: &[u16]) -> String {
    std::char::decode_utf16(source.iter().cloned())
        .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
        .collect()
}
