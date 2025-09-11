use std::{borrow::Cow, fmt::Write};

use crate::index::IdType;

#[macro_export]
macro_rules! os_struct {
    ($pub:vis struct $struct_name:ident {
        $(
            $( #[rename = $field_rename:expr] )?
            $field_name:ident: $field_typ:ty,
        )*

        $(#[content_fields] {
            $(
                $( #[rename = $content_field_rename:expr] )?
                $content_field_name:ident: $content_field_type:ty
            ),+
        })?

        $(#[children] {
            $(
                $( #[rename = $child_rename:expr] )?
                $child_name:ident: $child_type:ty
            ),+
        })?

        $(#[content] $( #[rename = $content_rename:expr] )? $content_name:ident: $content_type: ty)?
    }) => {
        $pub struct $struct_name {
            $( $pub $field_name: $field_typ, )*
            $( $( $pub $content_field_name: $content_field_type, )+ )?
            $( $( $pub $child_name: $child_type, )+ )?
            $( $pub $content_name: $content_type )?
        }

        impl $crate::http::opensubsonic::xml::XMLCompatible for $struct_name {
            #[allow(clippy::vec_init_then_push)]
            fn to_xml_element(&self) -> $crate::http::opensubsonic::xml::XMLElement {
                #[allow(unused_imports)]
                use $crate::http::opensubsonic::xml::{
                    XMLChildren,
                    XMLOptionalChildren,
                    XMLOptionalField,
                    XMLElement,
                    XMLFieldValue,
                    camel_case
                };

                #[allow(unused_mut)]
                let mut fields = vec![];

                $(
                    if let Some(child) = XMLOptionalField::to_option(&self.$field_name) {
                        fields.push((
                            camel_case(([$($field_rename,)? stringify!($field_name)])[0]),
                            XMLFieldValue::from(child.clone())
                        ));
                    }
                )*

                #[allow(unused_mut)]
                let mut content_fields = vec![];

                $($(
                    if let Some(child) = XMLOptionalField::to_option(&self.$content_field_name) {
                        content_fields.push((
                            camel_case(([$($content_field_rename,)? stringify!($content_field_name)])[0]),
                            XMLFieldValue::from(child.clone())
                        ));
                    }
                )*)?

                #[allow(unused_mut)]
                let mut children = vec![];

                $($(
                    if let Some(sub_children) = XMLOptionalChildren::to_option(&self.$child_name) {
                        children.push((
                            camel_case(([$($child_rename,)? stringify!($child_name)])[0]),
                            sub_children.to_owned()
                        ));
                    }
                )* )?

                #[allow(unused_mut, unused_assignments)]
                let mut content = None;

                $(
                    content = Some((
                        camel_case(([$($content_rename,)? stringify!($content_name)])[0]),
                        XMLFieldValue::from(self.$content_name.clone())
                    ));
                )?

                XMLElement {
                    fields,
                    content_fields,
                    children,
                    content
                }
            }
        }
    };
}

pub struct XMLElement {
    pub fields: Vec<(Cow<'static, str>, XMLFieldValue)>,
    pub content_fields: Vec<(Cow<'static, str>, XMLFieldValue)>,
    pub children: Vec<(Cow<'static, str>, XMLChildren)>,
    pub content: Option<(Cow<'static, str>, XMLFieldValue)>,
}

impl XMLElement {
    pub fn serialize_to_xml(&self, tag_name: &str, out: &mut impl Write) -> std::fmt::Result {
        let Self {
            fields,
            content_fields,
            children,
            content,
        } = self;

        out.write_char('<')?;
        out.write_str(tag_name)?;

        for (name, value) in fields {
            out.write_char(' ')?;
            out.write_str(name)?;
            out.write_str("=\"")?;
            value.serialize_to_xml(out)?;
            out.write_char('"')?;
        }

        if children.is_empty() && content_fields.is_empty() && content.is_none() {
            out.write_str("/>")?;
        } else {
            out.write_char('>')?;

            if let Some((_, content)) = content {
                content.serialize_to_xml(out)?;
            }

            for (tag_name, child) in content_fields {
                out.write_char('<')?;
                out.write_str(tag_name)?;
                out.write_char('>')?;
                child.serialize_to_xml(out)?;
                out.write_str("</")?;
                out.write_str(tag_name)?;
                out.write_char('>')?;
            }

            for (tag_name, children) in children {
                match children {
                    XMLChildren::Single(child) => {
                        child.serialize_to_xml(tag_name, out)?;
                    }

                    XMLChildren::Multiple(children) => {
                        for child in children {
                            child.serialize_to_xml(tag_name, out)?;
                        }
                    }
                }
            }

            out.write_str("</")?;
            out.write_str(tag_name)?;
            out.write_char('>')?;
        }

        Ok(())
    }

    pub fn serialize_to_json(&self, out: &mut impl Write) -> std::fmt::Result {
        let Self {
            fields,
            content_fields,
            children,
            content,
        } = self;

        out.write_char('{')?;

        for (i, (key, value)) in fields.iter().enumerate() {
            if i > 0 {
                out.write_char(',')?;
            }

            out.write_char('"')?;
            out.write_str(&escape_json_str(key))?;
            out.write_str("\":")?;
            value.serialize_to_json(out)?;
        }

        for (i, (key, value)) in content_fields.iter().enumerate() {
            if i + fields.len() > 0 {
                out.write_char(',')?;
            }

            out.write_char('"')?;
            out.write_str(&escape_json_str(key))?;
            out.write_str("\":")?;
            value.serialize_to_json(out)?;
        }

        for (i, (key, children)) in children.iter().enumerate() {
            if i + fields.len() + content_fields.len() > 0 {
                out.write_char(',')?;
            }

            match children {
                XMLChildren::Single(child) => {
                    out.write_char('"')?;
                    out.write_str(&escape_json_str(key))?;
                    out.write_str("\":")?;
                    child.serialize_to_json(out)?;
                }

                XMLChildren::Multiple(children) => {
                    out.write_char('"')?;
                    out.write_str(&escape_json_str(key))?;
                    out.write_str("\":[")?;

                    for (i, value) in children.iter().enumerate() {
                        if i > 0 {
                            out.write_char(',')?;
                        }

                        value.serialize_to_json(out)?;
                    }

                    out.write_char(']')?;
                }
            }
        }

        if let Some((key, value)) = content {
            if fields.len() + content_fields.len() + children.len() > 0 {
                out.write_char(',')?;
            }

            out.write_char('"')?;
            out.write_str(&escape_json_str(key))?;
            out.write_str("\":")?;
            value.serialize_to_json(out)?;
        }

        out.write_char('}')?;

        Ok(())
    }
}

pub enum XMLFieldValue {
    Bool(bool),
    SignedInt(i64),
    UnsignedInt(u64),
    Float(f64),
    StaticString(&'static str),
    String(String),
}

impl XMLFieldValue {
    fn serialize_to_xml(&self, out: &mut impl Write) -> std::fmt::Result {
        match self {
            XMLFieldValue::Bool(bool) => write!(out, "{bool}"),
            XMLFieldValue::SignedInt(int) => write!(out, "{int}"),
            XMLFieldValue::UnsignedInt(int) => write!(out, "{int}"),
            XMLFieldValue::Float(float) => write!(out, "{float}"),
            XMLFieldValue::StaticString(str) => out.write_str(&escape_xml_str(str)),
            XMLFieldValue::String(str) => out.write_str(&escape_xml_str(str)),
        }
    }

    fn serialize_to_json(&self, out: &mut impl Write) -> std::fmt::Result {
        match self {
            XMLFieldValue::Bool(bool) => write!(out, "{bool}"),
            XMLFieldValue::SignedInt(int) => write!(out, "{int}"),
            XMLFieldValue::UnsignedInt(int) => write!(out, "{int}"),
            XMLFieldValue::Float(float) => write!(out, "{float}"),
            XMLFieldValue::StaticString(str) => write!(out, "\"{}\"", escape_json_str(str)),
            XMLFieldValue::String(str) => write!(out, "\"{}\"", escape_json_str(str)),
        }
    }
}

impl From<bool> for XMLFieldValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i64> for XMLFieldValue {
    fn from(value: i64) -> Self {
        Self::SignedInt(value)
    }
}

impl From<i32> for XMLFieldValue {
    fn from(value: i32) -> Self {
        Self::SignedInt(value.into())
    }
}

impl From<i16> for XMLFieldValue {
    fn from(value: i16) -> Self {
        Self::SignedInt(value.into())
    }
}

impl From<i8> for XMLFieldValue {
    fn from(value: i8) -> Self {
        Self::SignedInt(value.into())
    }
}

impl From<u64> for XMLFieldValue {
    fn from(value: u64) -> Self {
        Self::UnsignedInt(value)
    }
}

impl From<u32> for XMLFieldValue {
    fn from(value: u32) -> Self {
        Self::UnsignedInt(value.into())
    }
}

impl From<u16> for XMLFieldValue {
    fn from(value: u16) -> Self {
        Self::UnsignedInt(value.into())
    }
}

impl From<u8> for XMLFieldValue {
    fn from(value: u8) -> Self {
        Self::UnsignedInt(value.into())
    }
}

impl From<usize> for XMLFieldValue {
    fn from(value: usize) -> Self {
        Self::UnsignedInt(value.try_into().unwrap())
    }
}

impl From<f32> for XMLFieldValue {
    fn from(value: f32) -> Self {
        Self::Float(value.into())
    }
}

impl From<f64> for XMLFieldValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<&'static str> for XMLFieldValue {
    fn from(value: &'static str) -> Self {
        Self::StaticString(value)
    }
}

impl From<String> for XMLFieldValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl<T: IdType> From<T> for XMLFieldValue {
    fn from(value: T) -> Self {
        XMLFieldValue::String(value.encode())
    }
}

pub trait XMLOptionalField<T> {
    fn to_option(&self) -> Option<&T>;
}

impl<T: Into<XMLFieldValue>> XMLOptionalField<T> for T {
    fn to_option(&self) -> Option<&T> {
        Some(self)
    }
}

impl<T: Into<XMLFieldValue>> XMLOptionalField<T> for Option<T> {
    fn to_option(&self) -> Option<&T> {
        self.as_ref()
    }
}

pub enum XMLChildrenRef<'a, T: XMLCompatible> {
    Single(&'a T),
    Multiple(&'a [T]),
}

impl<'a, T: XMLCompatible> XMLChildrenRef<'a, T> {
    pub fn to_owned(&self) -> XMLChildren {
        match self {
            XMLChildrenRef::Single(item) => XMLChildren::Single(item.to_xml_element()),
            XMLChildrenRef::Multiple(items) => {
                XMLChildren::Multiple(items.iter().map(|item| item.to_xml_element()).collect())
            }
        }
    }
}

pub enum XMLChildren {
    Single(XMLElement),
    Multiple(Vec<XMLElement>),
}

pub trait XMLOptionalChildren<T: XMLCompatible> {
    fn to_option<'a>(&'a self) -> Option<XMLChildrenRef<'a, T>>;
}

impl<T: XMLCompatible> XMLOptionalChildren<T> for T {
    fn to_option<'a>(&'a self) -> Option<XMLChildrenRef<'a, T>> {
        Some(XMLChildrenRef::Single(self))
    }
}

impl<T: XMLCompatible> XMLOptionalChildren<T> for Option<T> {
    fn to_option<'a>(&'a self) -> Option<XMLChildrenRef<'a, T>> {
        self.as_ref().map(XMLChildrenRef::Single)
    }
}

impl<T: XMLCompatible> XMLOptionalChildren<T> for Vec<T> {
    fn to_option<'a>(&'a self) -> Option<XMLChildrenRef<'a, T>> {
        Some(XMLChildrenRef::Multiple(self))
    }
}

impl<T: XMLCompatible> XMLOptionalChildren<T> for Option<Vec<T>> {
    fn to_option<'a>(&'a self) -> Option<XMLChildrenRef<'a, T>> {
        self.as_deref().map(XMLChildrenRef::Multiple)
    }
}

pub trait XMLCompatible {
    fn to_xml_element(&self) -> XMLElement
    where
        Self: Sized;
}

pub fn camel_case<'a>(input: &'a str) -> Cow<'a, str> {
    if !input.contains('-') && !input.contains('_') {
        return Cow::Borrowed(input);
    }

    let mut out = String::with_capacity(input.len());

    let mut chars = input.chars();
    let mut next_chars = input.chars().skip(1);

    while let Some(c) = chars.next() {
        let next_c = next_chars.next();

        match c {
            '-' | '_' => match next_c {
                Some(next_c) => {
                    chars.next();
                    next_chars.next();

                    for char in next_c.to_uppercase() {
                        out.push(char);
                    }
                }

                None => break,
            },

            _ => out.push(c),
        }
    }

    Cow::Owned(out)
}

fn escape_xml_str(str: &str) -> String {
    str.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_json_str(str: &str) -> String {
    str.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\u{8}', "\\b")
        .replace('\u{C}', "\\f")
        .replace('\n', "\\n")
        .replace('\u{A}', "\\r")
        .replace('\t', "\\t")
}
