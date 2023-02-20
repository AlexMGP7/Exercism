extern crate enum_iterator;
use enum_iterator::{Sequence, all};

#[derive(Debug, PartialEq, Sequence)]
pub enum ResistorColor {
    Black, Brown, Red, Orange, Yellow, Green, Blue, Violet, Grey, White,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    //unimplemented!("convert color {color:?} into a numerical representation")
    use ResistorColor::*;
    match color {
        Black => 0, Brown => 1, Red => 2, Orange => 3, Yellow => 4,
        Green => 5, Blue => 6, Violet => 7, Grey => 8, White => 9,
    }
}

pub fn value_to_color_string(value: u32) -> String {
    //unimplemented!("convert the value {value} into a string representation of color")
    match value {
        0 => "Black".to_string(),
        6 => "Blue".to_string(),
        1 => "Brown".to_string(),
        5 => "Green".to_string(),
        8 => "Grey".to_string(),
        3 => "Orange".to_string(),
        2 => "Red".to_string(),
        7 => "Violet".to_string(),
        9 => "White".to_string(),
        4 => "Yellow".to_string(),
        _ => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect()
}

