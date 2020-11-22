use tm_rs::log::LogApi;
use tm_rs::{api, ffi as tm};

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn tm_load_plugin(reg: *mut tm::tm_api_registry_api, load: bool) {
    assert!(!reg.is_null());

    api::register::<LogApi>(reg);

    api::get::<LogApi>().print_error("Hej 2");
}
