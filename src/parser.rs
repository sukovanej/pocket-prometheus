use std::{collections::HashMap, time::SystemTime};

use nom::{
    bytes::complete::take_while,
    character::complete::char,
    combinator::{cut, map},
    error::{context, Error},
    multi::{many_m_n, separated_list0},
    number::complete::double,
    sequence::{preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub labels: HashMap<String, String>,
}

impl Metric {
    fn new(name: &str, value: f64, labels: HashMap<String, String>) -> Metric {
        Metric {
            name: name.to_owned(),
            value,
            labels,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Measurement {
    pub timestamp_ns: u128,
    pub metrics: Vec<Metric>,
}

fn current_timestamp_ns() -> u128 {
    let duration_since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    duration_since_epoch.as_nanos()
}

pub fn parse_metrics(metrics_str: &str) -> Result<Measurement, Error<&str>> {
    let mut metrics = vec![];

    for line in metrics_str.split('\n') {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        let (_, parsed_metric) = parse_metric(line).finish()?;
        metrics.push(parsed_metric);
    }

    let measurement = Measurement {
        timestamp_ns: current_timestamp_ns(),
        metrics,
    };

    Ok(measurement)
}

// nom parser

fn parse_metric(metric: &str) -> IResult<&str, Metric> {
    map(
        tuple((parse_str, many_m_n(0, 1, hash), char(' '), double)),
        |(name, labels, _, value)| {
            let labels = labels
                .first()
                .map(|v| v.to_owned())
                .unwrap_or_else(HashMap::new);
            Metric::new(name, value, labels)
        },
    )(metric)
}

static STRING_ALLOWED_CHARS: [char; 4] = ['_', '.', '+', '-'];

fn parse_str(i: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_alphanumeric() || STRING_ALLOWED_CHARS.contains(&c))(i)
}

fn string(i: &str) -> IResult<&str, &str> {
    context(
        "string",
        preceded(char('\"'), cut(terminated(parse_str, char('\"')))),
    )(i)
}

fn key_value(i: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(parse_str, cut(char('=')), string)(i)
}

fn hash(i: &str) -> IResult<&str, HashMap<String, String>> {
    context(
        "map",
        preceded(
            char('{'),
            cut(terminated(
                map(separated_list0(char(','), key_value), |tuple_vec| {
                    tuple_vec
                        .into_iter()
                        .map(|(k, v)| (String::from(k), String::from(v)))
                        .collect()
                }),
                char('}'),
            )),
        ),
    )(i)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use nom::{error::Error, Finish};

    use crate::parser::{parse_metric, Metric};

    #[test]
    fn test_parse_metric() -> Result<(), Error<&'static str>> {
        let (_, input1) = parse_metric("_cpu_user_seconds_total 5.784335").finish()?;
        let (_, input2) =
            parse_metric("nodejs_heap_space_size_available_bytes{space=\"read_only\"} 0")
                .finish()?;
        let (_, input3) = parse_metric(
            "nodejs_version_info{version=\"v19.7.0\",major=\"19\",minor=\"7\",patch=\"0\"} 1",
        )
        .finish()?;

        assert_eq!(
            input1,
            Metric::new("_cpu_user_seconds_total", 5.784335, HashMap::new())
        );

        assert_eq!(
            input2,
            Metric::new(
                "nodejs_heap_space_size_available_bytes",
                0.0,
                HashMap::from([("space".into(), "read_only".into())])
            )
        );

        assert_eq!(
            input3,
            Metric::new(
                "nodejs_version_info",
                1.0,
                HashMap::from([
                    ("version".into(), "v19.7.0".into()),
                    ("major".into(), "19".into()),
                    ("minor".into(), "7".into()),
                    ("patch".into(), "0".into()),
                ])
            )
        );

        Ok(())
    }
}
