use chrono::NaiveDate;
use nom::combinator::map;
use nom::multi::many_till;
use nom::{
    character::streaming::*,
    bytes::streaming::*,
};
use nom::{IResult, sequence::*};
use serde::Deserialize;

pub fn strip_yaml(contents: &str) -> (Option<IndexConfig>, &str) {
    let seperator = "---";
    let seperator_with_spaces = || {
        tuple((
            multispace0,
            tag(seperator),
            multispace0,
        ))
    };

    let mut header_parser = preceded(
        seperator_with_spaces(),
        map(
            many_till(anychar, seperator_with_spaces()),
            |(chars, _sep)| chars.into_iter().collect(),
        ),
    );

    return (header_parser(contents) as IResult<&str, String>)
        .ok()
        .map(|(rest, header)| {
            (serde_yaml::from_str(&header).ok(), rest)
        })
        .unwrap_or((None, contents));
}


#[derive(Debug, Deserialize)]
pub struct IndexConfig {
		pub title: Option<String>,
		pub date: Option<NaiveDate>,
		pub is_draft: Option<bool>
}
