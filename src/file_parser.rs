use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path};

use diesel::{prelude::*, upsert::excluded, SqliteConnection};
use log::debug;

use crate::schema::{
    instantiate_class, instantiate_function, objects, parse_class, parse_template, source,
};
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

macro_rules! insert_records {
    ($table:ident, $key:ident, $vec:expr, $conn:expr) => {
        for __record in $vec.iter() {
            diesel::insert_into($table::table)
                .values(__record)
                .on_conflict($table::$key)
                .do_update()
                .set((
                    $table::count.eq($table::count + excluded($table::count)),
                    $table::duration.eq($table::duration + excluded($table::duration)),
                ))
                .execute($conn)?;
        }
    };
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

    let mut source_records = Vec::<NewSource>::new();
    let mut instantiate_class_records = Vec::<NewInstantiateClass>::new();
    let mut instantiate_function_records = Vec::<NewInstantiateFunction>::new();
    let mut parse_class_records = Vec::<NewParseClass>::new();
    let mut parse_template_records = Vec::<NewParseTemplate>::new();

    for trace_event in &trace_events.trace_events {
        match trace_event.name.as_str() {
            "Source" => {
                let path_result = get_detail(&trace_event);
                if let Some(path) = path_result {
                    source_records.push(NewSource::new(path, get_duration(&trace_event)));
                }
            }
            "InstantiateClass" => {
                let class_result = get_detail(&trace_event);
                if let Some(class) = class_result {
                    instantiate_class_records
                        .push(NewInstantiateClass::new(class, get_duration(&trace_event)));
                }
            }
            "InstantiateFunction" => {
                let function_result = get_detail(&trace_event);
                if let Some(function) = function_result {
                    instantiate_function_records.push(NewInstantiateFunction::new(
                        function,
                        get_duration(&trace_event),
                    ));
                }
            }
            "ParseClass" => {
                let class_result = get_detail(&trace_event);
                if let Some(class) = class_result {
                    parse_class_records.push(NewParseClass::new(class, get_duration(&trace_event)));
                }
            }
            "ParseTemplate" => {
                let template_result = get_detail(&trace_event);
                if let Some(template) = template_result {
                    parse_template_records
                        .push(NewParseTemplate::new(template, get_duration(&trace_event)));
                }
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
    debug!(
        "Parse completed {}, total compile time {}",
        object, total_time
    );

    NewObject::new(&object, total_time, frontend, backend).insert(conn)?;

    insert_records!(source, path, source_records, conn);
    debug!("Persistence source complete");

    insert_records!(instantiate_class, name, instantiate_class_records, conn);
    debug!("Persistence instantiate_class complete");

    insert_records!(
        instantiate_function,
        name,
        instantiate_function_records,
        conn
    );
    debug!("Persistence instantiate_function complete");

    insert_records!(parse_class, name, parse_class_records, conn);
    debug!("Persistence parse_class complete");

    insert_records!(parse_template, name, parse_template_records, conn);
    debug!("Persistence parse_template complete");

    Ok(())
}
