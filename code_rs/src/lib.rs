use std::ptr;
use tm_rs::ffi as tm;
use tm_rs::registry::RegistryApi;

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn tm_load_plugin(reg: *mut tm::tm_api_registry_api, load: bool) {
    assert!(!reg.is_null());

    let tm_entity_api = reg.get(tm::TM_ENTITY_API_NAME) as *mut tm::tm_entity_api;
    let tm_the_truth_api = reg.get(tm::TM_THE_TRUTH_API_NAME) as *mut tm::tm_the_truth_api;
    let tm_graph_interpreter_api =
        reg.get(tm::TM_GRAPH_INTERPRETER_API_NAME) as *mut tm::tm_graph_interpreter_api;
    let tm_graph_component_api =
        reg.get(tm::TM_GRAPH_COMPONENT_API_NAME) as *mut tm::tm_graph_component_api;
    let tm_logger_api = reg.get(tm::TM_LOGGER_API_NAME) as *mut tm::tm_logger_api;

    (*tm_logger_api).print.unwrap()(
        tm::tm_log_type_TM_LOG_TYPE_INFO,
        b"Hej\0".as_ptr() as *const _,
    );
}
