use std::fmt;

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub priority: Option<Priority>,
    pub projects: Vec<String>,
    pub contexts: Vec<String>,
    pub due_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub end_date: Option<NaiveDate>,
    pub is_done: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    A,
    B,
    C,
}

// カスタムエラー型(PriorityParseError)を定義
#[derive(Debug)]
pub enum PriorityParseError {
    InvalidPriority(String),
}

// Displayを実装して、エラー出力フォーマットを定義
impl std::fmt::Display for PriorityParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PriorityParseError::InvalidPriority(pri) => {
                writeln!(f, "Unrecognized priority variant: {}", pri)
            }
        }
    }
}

// std::error::Errorを実装
impl std::error::Error for PriorityParseError {}

// 実際にParseしてエラーの場合は、定義したPriorityParseErrorを返す。
pub fn parse_priority_to_enum(pri: &str) -> Result<Priority, PriorityParseError> {
    match pri {
        "A" | "a" => Ok(Priority::A),
        "B" | "b" => Ok(Priority::B),
        "C" | "c" => Ok(Priority::C),
        other => Err(PriorityParseError::InvalidPriority(other.to_string())),
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.id)?;
        if self.is_done {
            write!(f, "x")?;
        }
        if let Some(pri) = &self.priority {
            write!(f, "({:?}) ", pri)?;
        }
        if let Some(end) = self.end_date {
            write!(f, "{} ", NaiveDate::format(&end, "%Y-%m-%d"))?;
        }

        write!(
            f,
            "{} ",
            NaiveDateTime::format(&self.created_at, "%Y-%m-%d")
        )?;

        write!(f, "{} ", self.title)?;

        let sep = String::from(",");
        let projects = self.projects.join(&sep);
        write!(f, "{} ", projects)?;

        let contexts = self.contexts.join(&sep);
        write!(f, "{} ", contexts)?;

        if let Some(due) = self.due_date {
            writeln!(f, "due:{} ", NaiveDate::format(&due, "%Y-%m-%d"))?;
        }

        Ok(())
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self)?;
        Ok(())
    }
}
