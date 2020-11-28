static struct tm_entity_api *tm_entity_api;
static struct tm_the_truth_api *tm_the_truth_api;
static struct tm_graph_interpreter_api *tm_graph_interpreter_api;
static struct tm_graph_component_api *tm_graph_component_api;
static struct tm_logger_api *tm_logger_api;

#include <stdlib.h>

#include <plugins/the_machinery_shared/component_interfaces/editor_ui_interface.h>

#include <plugins/entity/entity.h>
#include <plugins/graph_interpreter/graph_component.h>
#include <plugins/graph_interpreter/graph_interpreter.h>
#include <plugins/default_render_pipe/light_component.h>

#include <foundation/api_registry.h>
#include <foundation/color_spaces.inl>
#include <foundation/log.h>
#include <foundation/macros.h>
#include <foundation/math.inl>
#include <foundation/the_truth.h>

#define TM_TT_TYPE__CAVE_COMPONENT "tm_cave_component"
#define TM_TT_TYPE_HASH__CAVE_COMPONENT TM_STATIC_HASH("tm_cave_component", 0x777acd9d836b08a6ULL)

#define GRAPH_DIST_HASH TM_STATIC_HASH("Dist", 0xd7882eed6b5209abULL)

enum
{
    TM_TT_PROP__CAVE_COMPONENT__FREQUENCY, // float
    TM_TT_PROP__CAVE_COMPONENT__AMPLITUDE, // float
};

struct tm_cave_component_t
{
    float y0;
    float frequency;
    float amplitude;
};

static tm_ci_editor_ui_i *editor_aspect = &(tm_ci_editor_ui_i){0};

static void truth__create_types(struct tm_the_truth_o *tt)
{
    tm_the_truth_property_definition_t cave_component_properties[] = {
        [TM_TT_PROP__CAVE_COMPONENT__FREQUENCY] = {"frequency", TM_THE_TRUTH_PROPERTY_TYPE_FLOAT},
        [TM_TT_PROP__CAVE_COMPONENT__AMPLITUDE] = {"amplitude", TM_THE_TRUTH_PROPERTY_TYPE_FLOAT},
    };

    const uint64_t cave_component_type = tm_the_truth_api->create_object_type(tt, TM_TT_TYPE__CAVE_COMPONENT, cave_component_properties, TM_ARRAY_COUNT(cave_component_properties));
    const tm_tt_id_t default_object = tm_the_truth_api->quick_create_object(tt, TM_TT_NO_UNDO_SCOPE, TM_TT_TYPE_HASH__CAVE_COMPONENT, TM_TT_PROP__CAVE_COMPONENT__FREQUENCY, 1.0f, TM_TT_PROP__CAVE_COMPONENT__AMPLITUDE, 1.0f, -1);
    tm_the_truth_api->set_default_object(tt, cave_component_type, default_object);

    tm_the_truth_api->set_aspect(tt, cave_component_type, TM_CI_EDITOR_UI, editor_aspect);
}

static bool component__load_asset(tm_component_manager_o *man, tm_entity_t e, void *c_vp, const tm_the_truth_o *tt, tm_tt_id_t asset)
{
    struct tm_cave_component_t *c = c_vp;
    const tm_the_truth_object_o *asset_r = tm_tt_read(tt, asset);
    c->y0 = 0;
    c->frequency = tm_the_truth_api->get_float(tt, asset_r, TM_TT_PROP__CAVE_COMPONENT__FREQUENCY);
    c->amplitude = tm_the_truth_api->get_float(tt, asset_r, TM_TT_PROP__CAVE_COMPONENT__AMPLITUDE);
    return true;
}

static void component__create(struct tm_entity_context_o *ctx)
{
    tm_component_i component = {
        .name = TM_TT_TYPE__CAVE_COMPONENT,
        .bytes = sizeof(struct tm_cave_component_t),
        .load_asset = component__load_asset,
    };

    tm_entity_api->register_component(ctx, &component);
}

// Runs on (cave_component, transform_component, link_component)
static void engine_update__cave_component(tm_engine_o *inst, tm_engine_update_set_t *data)
{
    //struct tm_entity_context_o *ctx = (struct tm_entity_context_o *)inst;

    double t = 0;
    for (const tm_entity_blackboard_value_t *bb = data->blackboard_start; bb != data->blackboard_end; ++bb)
    {
        if (bb->id == TM_ENTITY_BB__TIME)
            t = bb->double_value;
    }

    for (tm_engine_update_array_t *a = data->arrays; a < data->arrays + data->num_arrays; ++a)
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
    }

    //tm_entity_api->notify(ctx, data->engine->components[1], mod_transform, (uint32_t)tm_carray_size(mod_transform));
}

static bool engine_filter__cave_component(tm_engine_o *inst, const uint32_t *components, uint32_t num_components, const tm_component_mask_t *mask)
{
    return tm_entity_mask_has_component(mask, components[0]) &&
           tm_entity_mask_has_component(mask, components[1]) &&
           tm_entity_mask_has_component(mask, components[2]);
}

static void component__register_engine(struct tm_entity_context_o *ctx)
{
    const uint32_t cave_component = tm_entity_api->lookup_component(ctx, TM_TT_TYPE_HASH__CAVE_COMPONENT);
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
    tm_entity_api->register_engine(ctx, &cave_component_engine);
}

TM_DLL_EXPORT void tm_load_plugin(struct tm_api_registry_api *reg, bool load)
{
    tm_entity_api = reg->get(TM_ENTITY_API_NAME);
    tm_the_truth_api = reg->get(TM_THE_TRUTH_API_NAME);
    tm_graph_interpreter_api = reg->get(TM_GRAPH_INTERPRETER_API_NAME);
    tm_graph_component_api = reg->get(TM_GRAPH_COMPONENT_API_NAME);
    tm_logger_api = reg->get(TM_LOGGER_API_NAME);

    TM_LOG("LOAD");

    tm_add_or_remove_implementation(reg, load, TM_THE_TRUTH_CREATE_TYPES_INTERFACE_NAME, truth__create_types);
    tm_add_or_remove_implementation(reg, load, TM_ENTITY_CREATE_COMPONENT_INTERFACE_NAME, component__create);
    tm_add_or_remove_implementation(reg, load, TM_ENTITY_SIMULATION_INTERFACE_NAME, component__register_engine);
}
