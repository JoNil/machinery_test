use color_space::{FromColor, Hsv, Rgb};
use tm_rs::{
    add_or_remove_entity_simulation, api,
    component::{ComponentsIterator, Read, Write},
    components::{graph::GraphComponent, light::LightComponent},
    entity,
    ffi::tm_component_mask_t,
    ffi::tm_engine_update_set_t,
    ffi::tm_vec3_t,
    graph_interpreter::GraphInterpreterApi,
    tm_plugin,
};
use tm_rs::{entity::EntityApi, log::LogApi};

static COMPONENT_NAME: &str = "light_distance_component";

fn engine_update(update_set: &mut tm_engine_update_set_t) {
    let log = api::get::<LogApi>();

    let components =
        ComponentsIterator::<(Write<LightComponent>, Read<GraphComponent>)>::new(update_set);

    for (light, graph) in components {
        let mut graph = api::with_ctx::<GraphInterpreterApi>(graph.gr);

        if let Some(distance_to_wall) = graph.read_variable_f32("Dist") {
            let hue = (f32::sin(0.4 * distance_to_wall) + 1.0) / 2.0;
            log.info(&format!("H: {}", hue));
            let color = Rgb::from_color(&Hsv::new((hue * 360.0) as f64, 1.0, 0.6));
            light.color_rgb = tm_vec3_t {
                x: color.r as f32 / 255.0,
                y: color.g as f32 / 255.0,
                z: color.b as f32 / 255.0,
            };
        }
    }
}

fn engine_filter(components: &[u32], mask: &tm_component_mask_t) -> bool {
    entity::mask_has_component(mask, components[0])
        && entity::mask_has_component(mask, components[1])
}

tm_plugin!(|reg: &mut RegistryApi| {
    api::register::<LogApi>(reg);
    api::register::<EntityApi>(reg);
    api::register::<GraphInterpreterApi>(reg);

    add_or_remove_entity_simulation!(reg, |entity_api: &mut EntityApiInstance| {
        entity_api.register_engine::<(Write<LightComponent>, Read<GraphComponent>)>(
            "Light Distance Component",
            engine_update,
            Some(engine_filter),
        );
    });
});
