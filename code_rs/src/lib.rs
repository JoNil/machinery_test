use color_space::{FromColor, Hsv, Rgb};
use tm_rs::{
    add_or_remove_entity_simulation, api,
    component::{ComponentsIterator, Read, Write},
    components::{graph::GraphComponent, light::LightComponent},
    entity,
    entity::EntityApi,
    entity::EntityApiInstance,
    graph_interpreter::GraphInterpreterApi,
    log::LogApi,
    tm_plugin, ComponentMask, Vec3,
};

static COMPONENT_NAME: &str = "light_distance_component";

fn engine_update(
    entity_api: &mut EntityApiInstance,
    components: ComponentsIterator<(Write<LightComponent>, Read<GraphComponent>)>,
) {
    let log = api::get::<LogApi>();

    for (light, graph) in components {
        let mut graph = api::with_ctx::<GraphInterpreterApi>(graph.gr);

        if let Some(distance_to_wall) = graph.read_variable_f32("Dist") {
            let hue = (f32::sin(0.4 * distance_to_wall) + 1.0) / 2.0;
            log.info(&format!("WOHOO: {}", hue));
            let color = Rgb::from_color(&Hsv::new((hue * 360.0) as f64, 1.0, 0.6));
            light.color_rgb = Vec3 {
                x: color.r as f32 / 255.0,
                y: color.g as f32 / 255.0,
                z: color.b as f32 / 255.0,
            };
        }
    }
}

fn engine_filter(components: &[u32], mask: &ComponentMask) -> bool {
    entity::mask_has_component(mask, components[0])
        && entity::mask_has_component(mask, components[1])
}

tm_plugin!(|reg: &mut RegistryApi| {
    api::register::<LogApi>(reg);
    api::register::<EntityApi>(reg);
    api::register::<GraphInterpreterApi>(reg);

    add_or_remove_entity_simulation!(
        reg,
        fn register_light_engine(entity_api: &mut EntityApiInstance) {
            entity_api.register_engine(
                "Light Distance Component",
                engine_update,
                Some(engine_filter),
            );
        }
    );
});
