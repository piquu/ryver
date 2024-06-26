use core::fmt;

use calamine::{Data, Range};

#[derive(Clone)]
pub enum Values {
    Nil,
    String(String),
    Number(String),
    Boolean(bool),
}

impl fmt::Display for Values {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Values::Nil => write!(f, "Nil"),
            Values::String(s) => write!(f, "{}", s),
            Values::Number(n) => write!(f, "{}", n),
            Values::Boolean(b) => write!(f, "{}", b),
        }
    }
}

pub struct Sheet {
    pub name: String,
    pub types: Vec<(String, String)>,
    pub sheet: Vec<Vec<Values>>,
}

impl Sheet {
    pub fn excel(name: String, range: Range<Data>) -> Self {
        let mut types = vec![];
        let mut sheet = vec![];

        for row in 0..range.get_size().1 {
            types.push((
                match range.get((0, row)).unwrap() {
                    Data::Int(i) => i.to_string(),
                    Data::Float(f) => f.to_string(),
                    Data::String(s) => s.to_owned(),
                    Data::Bool(_) => panic!("key at {} is a boolean", row + 1),
                    Data::Empty => panic!("key at {} is nil", row + 1),

                    Data::Error(e) => panic!("{}", e),

                    Data::DateTime(d) => d.to_string(),
                    Data::DateTimeIso(d) | Data::DurationIso(d) => d.to_owned(),
                },
                match range.get((1, row)).unwrap() {
                    Data::Int(_) | Data::Float(_) => "number".to_owned(),
                    Data::String(_) => "string".to_owned(),
                    Data::Bool(_) => "boolean".to_owned(),
                    Data::Empty => "nil".to_owned(),

                    Data::Error(e) => panic!("{}", e),

                    Data::DateTime(_) | Data::DateTimeIso(_) | Data::DurationIso(_) => {
                        "string".to_owned()
                    }
                },
            ));
        }

        for column in 1..range.get_size().0 {
            let mut sheet_row = vec![];
            for row in 0..range.get_size().1 {
                sheet_row.push(match range.get((column, row)).unwrap() {
                    Data::Int(i) => Values::Number(i.to_string()),
                    Data::Float(f) => Values::Number(f.to_string()),
                    Data::String(s) => Values::String(s.to_owned()),
                    Data::Bool(b) => Values::Boolean(b.to_owned()),
                    Data::Empty => Values::Nil,

                    Data::Error(e) => panic!("{e}"),

                    Data::DateTime(d) => Values::String(d.to_string()),
                    Data::DateTimeIso(d) | Data::DurationIso(d) => Values::String(d.to_owned()),
                })
            }
            sheet.push(sheet_row);
        }

        Self { name, types, sheet }
    }

    pub fn csv(tabs: bool, name: String, csv: String) -> Self {
        todo!()
    }
}