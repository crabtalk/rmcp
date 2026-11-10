use rmcp::model::{InitializeResult, ServerCapabilities};
use serde_json::{self, json};

#[test]
fn unknown_capabilities_round_trip_at_the_top_level() {
    let wire = json!({"tools":{},"events":{},"vendor.feature":{"version":1}});
    let capabilities: ServerCapabilities = serde_json::from_value(wire.clone()).unwrap();
    assert!(capabilities.tools.is_some());
    assert_eq!(capabilities.custom["events"], json!({}));
    let initialized = serde_json::to_value(InitializeResult::new(capabilities.clone())).unwrap();
    assert_eq!(initialized["capabilities"], wire);
    assert_eq!(serde_json::to_value(capabilities).unwrap(), wire);
}

#[cfg(feature = "server")]
#[test]
fn capability_builders_keep_the_existing_wire_shape() {
    let mut capabilities = ServerCapabilities::builder().enable_tools().build();
    assert!(capabilities.custom.is_empty());
    assert_eq!(
        serde_json::to_value(&capabilities).unwrap(),
        json!({"tools":{}})
    );
    capabilities.custom.insert("events".into(), json!({}));
    let result = InitializeResult::new(capabilities);
    let wire = serde_json::to_value(result).unwrap();
    assert_eq!(wire["capabilities"]["events"], json!({}));
    assert!(wire["capabilities"].get("custom").is_none());
}
