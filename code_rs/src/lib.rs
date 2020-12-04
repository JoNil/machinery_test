use color_space::{FromColor, Hsv, Rgb};
use tm_derive::Component;
use tm_rs::{
    add_or_remove_entity_simulation, api,
    component::{ComponentsIterator, Read, Write},
    components::{graph::GraphComponent, light::LightComponent},
    entity,
    entity::EntityApi,
    entity::EntityApiInstanceMut,
    graph_interpreter::GraphInterpreterApi,
    log::LogApi,
    the_truth::TheTruthApi,
    tm_plugin, ComponentMask, Vec3,
};

#[derive(Copy, Clone, Default, Component)]
struct LightDistanceComponent {
    #[property(default = 1.0f)]
    intensity: f32,

    a: f32,
}

fn engine_update(
    entity_api: &mut EntityApiInstanceMut,
    components: ComponentsIterator<(
        Read<LightDistanceComponent>,
        Write<LightComponent>,
        Read<GraphComponent>,
    )>,
) {
    let log = api::get::<LogApi>();

    for (ld, light, graph) in components {
        let mut graph = api::with_ctx_mut::<GraphInterpreterApi>(graph.gr);

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

fn register_light_engine(entity_api: &mut EntityApiInstanceMut) {
    entity_api.register_engine("Light Distance Engine", engine_update, Some(engine_filter));
}

tm_plugin!(|reg: &mut RegistryApi| {
    api::register::<LogApi>(reg);
    api::register::<EntityApi>(reg);
    api::register::<TheTruthApi>(reg);
    api::register::<GraphInterpreterApi>(reg);

    //add_or_remove_component!(reg, LightDistanceComponent);
    add_or_remove_entity_simulation!(reg, register_light_engine);
});
