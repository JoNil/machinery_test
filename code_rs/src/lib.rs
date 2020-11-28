use tm_rs::{api, ffi as tm, registry::RegistryApi};
use tm_rs::{entity::EntityApi, log::LogApi};

unsafe extern "C" fn engine_update(
    inst: *mut tm::tm_engine_o,
    data: *mut tm::tm_engine_update_set_t,
) {
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
    inst: *mut tm::tm_engine_o,
    components: *const u32,
    num_components: u32,
    mask: *const tm::tm_component_mask_t,
) -> bool {
    /*tm_entity_mask_has_component(mask, components[0]) &&
    tm_entity_mask_has_component(mask, components[1]) &&
    tm_entity_mask_has_component(mask, components[2])*/
    false
}

unsafe extern "C" fn register_engine(ctx: *mut tm::tm_entity_context_o) {
    //assert!(!ctx.is_null());

    let entity_api = api::with_ctx::<EntityApi>(ctx);

    let light_component = entity_api.lookup_component(tm::TM_TT_TYPE_HASH__LIGHT_COMPONENT);
    let graph_component = entity_api.lookup_component(tm::TM_TT_TYPE_HASH__GRAPH_COMPONENT);

    /*const uint32_t cave_component = tm_entity_api->lookup_component(ctx, TM_TT_TYPE_HASH__CAVE_COMPONENT);
    const uint32_t light_component = tm_entity_api->lookup_component(ctx, TM_TT_TYPE_HASH__LIGHT_COMPONENT);
    const uint32_t graph_component = tm_entity_api->lookup_component(ctx, TM_TT_TYPE_HASH__GRAPH_COMPONENT);

    const tm_engine_i cave_component_engine = {
        .name = "Cave Component",
        .num_components = 3,
        .components = {cave_component, light_component, graph_component},
        .writes = {false, true, false},
        .update = engine_update__cave_component,
        .filter = engine_filter__cave_component,
        .inst = (tm_engine_o *)ctx,
    };
    tm_entity_api->register_engine(ctx, &cave_component_engine);*/

    api::get::<LogApi>().info("Hej4");
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn tm_load_plugin(reg: *mut tm::tm_api_registry_api, load: bool) {
    assert!(!reg.is_null());

    api::register::<LogApi>(reg);
    api::register::<EntityApi>(reg);

    api::get::<LogApi>().info("Hej3");

    reg.add_or_remove_implementation(
        load,
        tm::TM_ENTITY_SIMULATION_INTERFACE_NAME,
        register_engine as _,
    );
}
