use std::{
    os::raw::c_char,
    slice,
    time::{Instant, SystemTime},
};

use tm_rs::{
    api,
    entity::{self, Engine},
    ffi as tm,
    ffi::tm_engine_i,
    ffi::tm_engine_o,
    hash,
    registry::RegistryApi,
};
use tm_rs::{entity::EntityApi, log::LogApi};

static COMPONENT_NAME: &str = "light_distance_component";

unsafe extern "C" fn engine_update(
    inst: *mut tm::tm_engine_o,
    data: *mut tm::tm_engine_update_set_t,
) {
    api::get::<LogApi>().info("Update 2");

    /*for (tm_engine_update_array_t *a = data->arrays; a < data->arrays + data->num_arrays; ++a)
    {
        struct tm_light_component_t *light_component = a->components[1];
        struct tm_graph_component_t *graph_component = a->components[2];

        for (uint32_t i = 0; i < a->n; ++i)
        {
            tm_graph_interpreter_wire_content_t dist_wire = tm_graph_interpreter_api->read_variable(graph_component[i].gr, GRAPH_DIST_HASH);

            if (dist_wire.data != NULL)
            {
                float distance_to_wall = *(float *)dist_wire.data;
                float hue = (sinf(0.4f * distance_to_wall) + 1.0f) / 2.0f;
                TM_LOG("HUE: %f", hue);
                light_component[i].color_rgb = tm_hue_to_rgb(hue);
            }
        }
    }*/
}

unsafe extern "C" fn engine_filter(
    _inst: *mut tm::tm_engine_o,
    components: *const u32,
    num_components: u32,
    mask: *const tm::tm_component_mask_t,
) -> bool {
    if num_components < 2 {
        return false;
    }

    let components = slice::from_raw_parts(components, num_components as usize);

    entity::mask_has_component(mask, components[0])
        && entity::mask_has_component(mask, components[1])
}

unsafe extern "C" fn register_engine(ctx: *mut tm::tm_entity_context_o) {
    assert!(!ctx.is_null());

    let mut entity_api = api::with_ctx::<EntityApi>(ctx);

    let light_component = entity_api.lookup_component(hash(tm::TM_TT_TYPE__LIGHT_COMPONENT));
    let graph_component = entity_api.lookup_component(hash(tm::TM_TT_TYPE__GRAPH_COMPONENT));

    let engine = Engine {
        name: "Light Distance Component",
        disabled: false,
        num_components: 2,
        components: &[light_component, graph_component],
        excludes: &[false],
        writes: &[true, false],
        update: engine_update,
        filter: engine_filter,
    };

    entity_api.register_engine(engine);
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn tm_load_plugin(reg: *mut tm::tm_api_registry_api, load: bool) {
    assert!(!reg.is_null());

    api::register::<LogApi>(reg);
    api::register::<EntityApi>(reg);

    api::get::<LogApi>().info(&format!(
        "Hej {}",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    ));

    reg.add_or_remove_implementation(
        load,
        tm::TM_ENTITY_SIMULATION_INTERFACE_NAME,
        register_engine as _,
    );
}
