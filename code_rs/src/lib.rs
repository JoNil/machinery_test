use color_space::{FromColor, Hsv, Rgb};
use std::{convert::TryInto, slice};
use tm_rs::{
    api,
    component::{ComponentsIterator, Read, Write},
    components::{graph::GraphComponent, light::LightComponent},
    entity,
    ffi::tm_api_registry_api,
    ffi::tm_component_mask_t,
    ffi::tm_engine_update_set_t,
    ffi::tm_entity_context_o,
    ffi::tm_vec3_t,
    ffi::TM_ENTITY_SIMULATION_INTERFACE_NAME,
    graph_interpreter::GraphInterpreterApi,
    hash,
    registry::RegistryApi,
};
use tm_rs::{entity::EntityApi, log::LogApi};

static COMPONENT_NAME: &str = "light_distance_component";

fn engine_update(update_set: &mut tm_engine_update_set_t) {
    let log = api::get::<LogApi>();

    let components =
        ComponentsIterator::<(Write<LightComponent>, Read<GraphComponent>)>::new(update_set);

    log.info("Update 4");

    for (light, graph) in components {
        let mut graph = api::with_ctx::<GraphInterpreterApi>(graph.gr);

        let var = graph.read_variable(hash(b"Dist"));

        if !var.data.is_null() {
            let data = unsafe { slice::from_raw_parts(var.data as *mut u8, var.size as usize) };
            let distance_to_wall = f32::from_ne_bytes(data.try_into().unwrap());

            let hue = (f32::sin(0.4 * distance_to_wall) + 1.0) / 2.0;
            log.info(&format!("HUE: {}", hue));
            let color = Rgb::from_color(&Hsv::new((hue * 360.0) as f64, 1.0, 1.0));
            light.color_rgb = tm_vec3_t {
                x: color.r as f32,
                y: color.g as f32,
                z: color.b as f32,
            };
        }
    }
}

fn engine_filter(components: &[u32], mask: &tm_component_mask_t) -> bool {
    entity::mask_has_component(mask, components[0])
        && entity::mask_has_component(mask, components[1])
}

unsafe extern "C" fn register_engine(ctx: *mut tm_entity_context_o) {
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
pub unsafe extern "C" fn tm_load_plugin(reg: *mut tm_api_registry_api, load: bool) {
    assert!(!reg.is_null());

    api::register::<LogApi>(reg);
    api::register::<EntityApi>(reg);
    api::register::<GraphInterpreterApi>(reg);

    reg.add_or_remove_implementation(
        load,
        TM_ENTITY_SIMULATION_INTERFACE_NAME,
        register_engine as _,
    );
}
