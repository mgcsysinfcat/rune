//! Character and string utilities.
use crate::core::{
    gc::Context,
    object::{Gc, Object, OptionalFlag, int_to_char},
};
use anyhow::Result;
use rune_macros::defun;

#[defun]
fn unibyte_string(bytes: &[Gc<i64>]) -> Result<Vec<u8>> {
    let unibyte: Result<Vec<u8>, _> = bytes.iter().map(|x| u8::try_from(x.untag())).collect();
    Ok(unibyte?)
}

#[defun]
fn max_char(unicode: OptionalFlag) -> usize {
    if unicode.is_some() { std::char::MAX as usize } else { 0x3F_FFFF }
}

#[defun]
fn characterp(obj: Object) -> bool {
    char::try_from(obj).is_ok()
}

#[defun]
fn string(characters: &[Gc<i64>]) -> Result<String> {
    let string: Result<_, _> = characters.iter().map(|x| int_to_char(x.untag())).collect();
    Ok(string?)
}

#[defun]
fn make_string<'ob>(
    length: usize,
    init: usize,
    multibyte: OptionalFlag,
    cx: &'ob Context,
) -> Result<Object<'ob>> {
    if multibyte.is_some() {
        let chr = int_to_char(i64::try_from(init)?)?;
        let size = chr.len_utf8();
        let mut string = cx.string_with_capacity(length * size);
        for _ in 0..length {
            string.push(chr);
        }
        Ok(cx.add(string))
    } else {
        let chr = u8::try_from(init)?;
        let string: Vec<_> = (0..length).map(|_| chr).collect();
        Ok(cx.add(string))
    }
}
