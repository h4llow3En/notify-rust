use std::borrow::Cow;

use dbus::{MessageItem, MessageItemArray, Signature};

//TODO get rid of the util functions

pub fn unwrap_message_int(item: &MessageItem) -> i32 {
    try_unwrap_message_int(item).unwrap_or(0)
}

pub fn try_unwrap_message_int(item: &MessageItem) -> Option<i32> {
    unwrap_message_str(item).parse::<i32>().ok()
}

pub fn unwrap_message_bool(item: &MessageItem) -> bool {
    unwrap_message_str(item).parse::<bool>().unwrap_or(false)
}

pub fn unwrap_message_str(item: &MessageItem) -> String {
    match *item {
        MessageItem::Str(ref value) => value.to_owned(),
        MessageItem::Variant(ref value) => {
            match **value {
                MessageItem::Str(ref value) => value.to_owned(),
                _ => "".to_owned(),
            }
        }
        _ => "".to_owned(),
    }
}

pub fn unwrap_message_string(item: Option<&MessageItem>) -> String {
    let sig_sv = Signature::new("{sv}").unwrap();
    let sig_s = Signature::new("s").unwrap();
    match item {
        Some(&MessageItem::Str(ref value)) => value.clone(),
        Some(&MessageItem::Array(MessageItemArray{v: items, sig: sig_sv})) => format!("DICT   {:?}", items),
        Some(&MessageItem::Array(MessageItemArray{v: items, sig})) => format!("ARRAY  {:?}", items),
        Some(&MessageItem::Array(MessageItemArray{v: items, sig})) => format!("{sig:?} {items:?}", items=items, sig=sig),
        _ => "".to_owned()
    }
}
