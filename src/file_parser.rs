use std::{error::Error, fs::File, io::BufReader, path::Path};

use diesel::SqliteConnection;
use log::debug;

use crate::{
    trace_event::{TraceEvent, TraceEvents},
    tracedb::{
        NewInstantiateClass, NewInstantiateFunction, NewObject, NewParseClass, NewParseTemplate,
        NewSource,
    },
};

#[inline]
fn get_detail(trace_event: &TraceEvent) -> Option<&str> {
    if let Some(args) = &trace_event.args {
        if let Some(detail) = args.get("detail") {
            if detail.is_string() {
                return Some(detail.as_str().unwrap());
            }
        }
    }
    None
}

#[inline]
fn get_duration(trace_event: &TraceEvent) -> i32 {
    trace_event.duration.unwrap().try_into().unwrap()
}

/// Parse the JSON file and store the data into the database
pub fn json_parser(
    path: &Path,
    conn: &mut SqliteConnection,
) -> Result<(), Box<dyn Error + 'static>> {
    let json_file = File::open(path)?;
    let reader = BufReader::new(json_file);
    let trace_events: TraceEvents = serde_json::from_reader(reader)?;

    let object: String = path.with_extension("").as_os_str().to_str().unwrap().into();

    let mut frontend: i32 = 0;
    let mut backend: i32 = 0;
    for trace_event in &trace_events.trace_events {
        match trace_event.name.as_str() {
            "Source" => {
                let path = get_detail(&trace_event).unwrap();
                NewSource::new(path, &object, get_duration(trace_event)).insert(conn)?;
            }
            "InstantiateClass" => {
                let class = get_detail(&trace_event).unwrap();
                NewInstantiateClass::new(class, &object, get_duration(trace_event)).insert(conn)?;
            }
            "InstantiateFunction" => {
                let function = get_detail(&trace_event).unwrap();
                NewInstantiateFunction::new(function, &object, get_duration(trace_event))
                    .insert(conn)?;
            }
            "ParseClass" => {
                let class = get_detail(&trace_event).unwrap();
                NewParseClass::new(class, &object, get_duration(trace_event)).insert(conn)?;
            }
            "ParseTemplate" => {
                let template = get_detail(&trace_event).unwrap();
                NewParseTemplate::new(template, &object, get_duration(trace_event)).insert(conn)?;
            }
            "Total Frontend" => {
                frontend = get_duration(trace_event);
            }
            "Total Backend" => {
                backend = get_duration(trace_event);
            }
            _ => {}
        }
    }

    let total_time = frontend + backend;
    NewObject::new(&object, total_time, frontend, backend).insert(conn)?;
    debug!("COmpleted {}, total compile time {}", object, total_time);

    Ok(())
}
