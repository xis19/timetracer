/// TraceEvent format
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum EventType {
    /// Duration event
    /// Begin
    #[serde(alias="B")]
    B,
    /// End
    #[serde(alias="E")]
    E,
    /// Complete event
    #[serde(alias="X")]
    X,
    /// Instant event
    #[serde(alias="i")]
    #[allow(non_camel_case_types)]
    i,
    #[serde(alias="I")]
    #[deprecated]
    I,
    /// Counter event
    #[serde(alias="C")]
    C,
    /// Async event
    /// Nestable start
    #[serde(alias="b")]
    #[allow(non_camel_case_types)]
    b,
    /// Nestable instant
    #[serde(alias="n")]
    #[allow(non_camel_case_types)]
    n,
    /// Nestable end
    #[serde(alias="e")]
    #[allow(non_camel_case_types)]
    e,
    /// Start
    #[serde(alias="S")]
    #[deprecated]
    S,
    /// Step into
    #[serde(alias="T")]
    #[deprecated]
    T,
    /// Step past
    #[serde(alias="p")]
    #[deprecated]
    #[allow(non_camel_case_types)]
    p,
    /// End
    #[serde(alias="F")]
    #[deprecated]
    F,
    /// Flow event
    /// Start
    #[serde(alias="s")]
    #[allow(non_camel_case_types)]
    s,
    /// Step
    #[serde(alias="t")]
    #[allow(non_camel_case_types)]
    t,
    /// End
    #[serde(alias="f")]
    #[allow(non_camel_case_types)]
    f,
    /// Sample event
    #[serde(alias="P")]
    P,
    /// Object event
    /// Created
    #[serde(alias="N")]
    N,
    /// Snapshot
    #[serde(alias="O")]
    O,
    /// Destroyed
    #[serde(alias="D")]
    D,
    /// Meteadata event
    #[serde(alias="M")]
    M,
    /// Memory Dump event
    /// Global
    #[serde(alias="V")]
    V,
    /// Process
    #[serde(alias="v")]
    #[allow(non_camel_case_types)]
    v,
    /// Mark event
    #[serde(alias="R")]
    R,
    /// Clock sync event
    #[serde(alias="c")]
    #[allow(non_camel_case_types)]
    c,
    /// Context event
    #[serde(alias="(")]
    LParenthesis,
    #[serde(alias=")")]
    RPparenthesis,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TraceEvent {
    pub name: String,
    #[serde(alias = "cat")]
    pub category: Option<String>,
    #[serde(alias = "ph")]
    pub phase: EventType,
    #[serde(alias = "ts")]
    pub timestamp: u64,
    pub pid: u64,
    pub tid: u64,
    #[serde(alias = "dur")]
    pub duration: Option<u64>,
    pub args: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TraceEvents {
    #[serde(alias = "traceEvents")]
    pub trace_events: Vec<TraceEvent>,
    #[serde(alias = "displayTimeUnit")]
    pub display_time_unit: Option<String>,
    #[serde(alias = "systemTraceEvents")]
    pub system_trace_events: Option<String>,
}

#[cfg(test)]
mod test {
    use crate::trace_event::EventType;

    use super::TraceEvents;

    #[test]
    fn test_deserialize_trace_event() {
        let result = serde_json::from_str::<TraceEvents>(
            r#"
{
    "traceEvents": [
        {
            "pid": 2607,
            "tid": 2607,
            "ph": "X",
            "ts": 1502,
            "dur": 689,
            "name": "Source",
            "args": {
                "detail": "/usr/include/features.h"
            }
        }
    ],
    "displayTimeUnit": "ns"
}"#,
        )
        .unwrap();
        assert_eq!(result.trace_events.len(), 1);

        let event = &result.trace_events[0];
        assert_eq!(event.pid, 2607);
        assert_eq!(event.tid, 2607);
        assert_eq!(event.phase, EventType::X);
        assert_eq!(event.timestamp, 1502);
        assert_eq!(event.duration.unwrap(), 689);
        assert_eq!(event.name, "Source");

        let args = &event.args;
        assert_eq!(
            args.as_ref().unwrap().get("detail").unwrap(),
            "/usr/include/features.h"
        );
    }
}
