use std::time::SystemTime;
use tm_rs::{
    api,
    component::{ComponentsIterator, GraphComponent, LightComponent, Read, Write},
    entity, ffi as tm,
    ffi::tm_engine_update_set_t,
    registry::RegistryApi,
};
use tm_rs::{entity::EntityApi, log::LogApi};

static COMPONENT_NAME: &str = "light_distance_component";

fn engine_update(update_set: &mut tm_engine_update_set_t) {
    let components =
        ComponentsIterator::<(Write<LightComponent>, Read<GraphComponent>)>::new(update_set);

    api::get::<LogApi>().info("Update 2");

    for (light, graph) in components {
        api::get::<LogApi>().info(&format!("Light: {}", light.color_rgb.x));
    }

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

fn engine_filter(components: &[u32], mask: &tm::tm_component_mask_t) -> bool {
    entity::mask_has_component(mask, components[0])
        && entity::mask_has_component(mask, components[1])
}

unsafe extern "C" fn register_engine(ctx: *mut tm::tm_entity_context_o) {
    assert!(!ctx.is_null());

    let mut entity_api = api::with_ctx::<EntityApi>(ctx);

    entity_api.register_engine::<(Write<LightComponent>, Read<GraphComponent>)>(
        "Light Distance Component",
        engine_update,
        Some(engine_filter),
    );
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
