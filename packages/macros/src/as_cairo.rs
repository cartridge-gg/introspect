use std::collections::HashMap;

use crate::byte_array::parse_bytes_to_cairo_byte_array;
use crate::utils::Quoted;
use indent::{indent_all_by, indent_by};
use starknet_types_core::felt::Felt;

pub trait AsCairo {
    fn as_cairo(&self) -> String;
    fn as_cairo_indented(&self, indent: usize) -> String {
        indent_by(indent, self.as_cairo())
    }
    fn as_cairo_suffixed(&self, suffix: &str) -> String {
        format!("{}{}", self.as_cairo(), suffix)
    }
}

pub trait AsCairoWith<C> {
    fn as_cairo_with(&self, context: &C) -> String;
}

pub trait CollectionsAsCairo<S> {
    fn as_cairo_elements(&self) -> Vec<String>;
    fn as_cairo_block(&self) -> String {
        let elements = self.as_cairo_elements();
        match elements.len() {
            0 => "".to_string(),
            _ => elements.join("\n") + "\n",
        }
    }
    // fn as_cairo_block_indented(&self, indent: usize) -> String {
    //     indent_by(indent, self.as_cairo_block())
    // }
    fn as_cairo_csv_wrapped(&self, prefix: &str, suffix: &str) -> String {
        format!("{}{}{}", prefix, self.as_cairo_csv(), suffix)
    }

    fn as_cairo_block_section(&self) -> String {
        let elements = self.as_cairo_elements();
        match elements.len() {
            0 => "".to_string(),
            _ => format!("\n{}\n", indent_all_by(4, elements.join("\n"))),
        }
    }
    fn as_cairo_csv(&self) -> String {
        self.as_cairo_elements().join(",")
    }
    fn as_cairo_delimited(&self, delimiter: &str) -> String {
        self.as_cairo_elements().join(delimiter)
    }
    fn as_cairo_span(&self) -> String {
        let elements = self.as_cairo_elements();
        format!("[{}].span()", elements.join(","))
    }
    fn as_cairo_array(&self) -> String {
        let elements = self.as_cairo_elements();
        format!("array![{}]", elements.join(","))
    }
    fn as_cairo_fixed_array(&self) -> String {
        let elements = self.as_cairo_elements();
        format!("[{}]", elements.join(","))
    }
    // fn as_cairo_delimited_indented(&self, delimiter: &str, indent: usize) -> String {
    //     indent_by(indent, self.as_cairo_delimited(delimiter))
    // }
}

impl AsCairo for String {
    fn as_cairo(&self) -> String {
        self.clone()
    }
}

impl<T, S> CollectionsAsCairo<S> for T
where
    T: AsRef<[S]>,
    S: AsCairo,
{
    fn as_cairo_elements(&self) -> Vec<String> {
        self.as_ref().iter().map(|e| e.as_cairo()).collect()
    }
}

// impl<T, S, C> CollectionsAsCairo<(S, C)> for T
// where
//     T: AsRef<[(S, C)]>,
//     S: AsCairoWith<C>,
// {
//     fn as_cairo_elements(&self) -> Vec<String> {
//         self.as_ref()
//             .iter()
//             .map(|(e, c)| e.as_cairo_with(c))
//             .collect()
//     }
// }

impl<S, C> CollectionsAsCairo<(S, C)> for HashMap<C, S>
where
    S: AsCairoWith<C>,
{
    fn as_cairo_elements(&self) -> Vec<String> {
        self.iter().map(|(c, s)| s.as_cairo_with(c)).collect()
    }
}

pub fn spanify(elements: Vec<String>) -> String {
    match elements.len() {
        0 => "[].span()".to_string(),
        1 => format!("[{}].span()", elements[0]),
        _ => format!("[\n{}\n].span()", indent_all_by(4, elements.join(",\n"))),
    }
}

impl AsCairo for [u8] {
    fn as_cairo(&self) -> String {
        self.as_cairo_byte_array()
    }
}

impl AsCairo for Felt {
    fn as_cairo(&self) -> String {
        self.to_fixed_hex_string()
    }
}

pub trait AsCairoBytes {
    fn as_cairo_bytes(&self) -> String;
    fn as_cairo_byte_array(&self) -> String {
        self.as_cairo_bytes().quoted()
    }
}

impl<T> AsCairoBytes for T
where
    T: AsRef<[u8]>,
{
    fn as_cairo_bytes(&self) -> String {
        parse_bytes_to_cairo_byte_array(self)
    }
}
