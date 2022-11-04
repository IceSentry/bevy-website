+++
title = "0.8 to 0.9"
weight = 5
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
[extra]
long_title = "Migration Guide: 0.8 to 0.9"
+++

Before migrating make sure to run rustup update

Bevy relies heavily on improvements in the Rust language and compiler. As a result, the Minimum Supported Rust Version (MSRV) is "the latest stable release" of Rust.

### [Use plugin setup for resource only used at setup time](https://github.com/bevyengine/bevy/pull/6360)

The `LogSettings` settings have been moved from a resource to `LogPlugin` configuration:

```rust
// Old (Bevy 0.8)
app
  .insert_resource(LogSettings {
    level: Level::DEBUG,
    filter: "wgpu=error,bevy_render=info,bevy_ecs=trace".to_string(),
  })
  .add_plugins(DefaultPlugins)

// New (Bevy 0.9)
app.add_plugins(DefaultPlugins.set(LogPlugin {
    level: Level::DEBUG,
    filter: "wgpu=error,bevy_render=info,bevy_ecs=trace".to_string(),
}))
```

The `ImageSettings` settings have been moved from a resource to `ImagePlugin` configuration:

```rust
// Old (Bevy 0.8)
app
  .insert_resource(ImageSettings::default_nearest())
  .add_plugins(DefaultPlugins)

// New (Bevy 0.9)
app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
```

The `DefaultTaskPoolOptions` settings have been moved from a resource to `CorePlugin::task_pool_options`:

```rust
// Old (Bevy 0.8)
app
  .insert_resource(DefaultTaskPoolOptions::with_num_threads(4))
  .add_plugins(DefaultPlugins)

// New (Bevy 0.9)
app.add_plugins(DefaultPlugins.set(CorePlugin {
  task_pool_options: TaskPoolOptions::with_num_threads(4),
}))
```

### [bevy_scene: Replace root list with struct](https://github.com/bevyengine/bevy/pull/6354)

The scene file format now uses a struct as the root object rather than a list of entities. The list of entities is now found in the `entities` field of this struct.

```rust
// Old (Bevy 0.8)
[
  (
    entity: 0,
    components: [
      // Components...
    ]
  ),
]

// New (Bevy 0.9)
(
  entities: [
    (
      entity: 0,
      components: [
        // Components...
      ]
    ),
  ]
)
```

### [Rename `play` to `start` and add new `play` method that won't overwrite the existing animation if it's already playing](https://github.com/bevyengine/bevy/pull/6350)

If you were using `play` to restart an animation that was already playing, that functionality has been moved to `start`. Now, `play` won’t have any effect if the requested animation is already playing.

### [bevy_scene: Use map for scene `components`](https://github.com/bevyengine/bevy/pull/6345)

The scene format now uses a map to represent the collection of components. Scene files will need to update from the old list format.

```rust
// Old (Bevy 0.8)
[
  (
    entity: 0,
    components: [
      {
        "bevy_transform::components::transform::Transform": (
          translation: (
            x: 0.0,
            y: 0.0,
            z: 0.0
          ),
          rotation: (0.0, 0.0, 0.0, 1.0),
          scale: (
            x: 1.0,
            y: 1.0,
            z: 1.0
          ),
        ),
      },
      {
        "my_crate::Foo": (
          text: "Hello World",
        ),
      },
      {
        "my_crate::Bar": (
          baz: 123,
        ),
      },
    ],
  ),
]

// New (Bevy 0.9)
[
  (
    entity: 0,
    components: {
      "bevy_transform::components::transform::Transform": (
        translation: (
          x: 0.0,
          y: 0.0,
          z: 0.0
        ),
        rotation: (0.0, 0.0, 0.0, 1.0),
        scale: (
          x: 1.0,
          y: 1.0,
          z: 1.0
        ),
      ),
      "my_crate::Foo": (
        text: "Hello World",
      ),
      "my_crate::Bar": (
        baz: 123
      ),
    },
  ),
]
```

### [Add GamepadInfo, expose gamepad names](https://github.com/bevyengine/bevy/pull/6342)

* Pattern matches on `GamepadEventType::Connected` will need to be updated, as the form of the variant has changed.
* Code that requires `GamepadEvent`, `GamepadEventRaw` or `GamepadEventType` to be `Copy` will need to be updated.

### [Plugins own their settings. Rework PluginGroup trait.](https://github.com/bevyengine/bevy/pull/6336)

The `WindowDescriptor` settings have been moved from a resource to `WindowPlugin::window`:

```rust
// Old (Bevy 0.8)
app
  .insert_resource(WindowDescriptor {
    width: 400.0,
    ..default()
  })
  .add_plugins(DefaultPlugins)

// New (Bevy 0.9)
app.add_plugins(DefaultPlugins.set(WindowPlugin {
  window: WindowDescriptor {
    width: 400.0,
    ..default()
  },
  ..default()
}))
```

The `AssetServerSettings` resource has been removed in favor of direct `AssetPlugin` configuration:

```rust
// Old (Bevy 0.8)
app
  .insert_resource(AssetServerSettings {
    watch_for_changes: true,
    ..default()
  })
  .add_plugins(DefaultPlugins)

// New (Bevy 0.9)
app.add_plugins(DefaultPlugins.set(AssetPlugin {
  watch_for_changes: true,
  ..default()
}))
```

`add_plugins_with` has been replaced by `add_plugins` in combination with the builder pattern:

```rust
// Old (Bevy 0.8)
app.add_plugins_with(DefaultPlugins, |group| group.disable::<AssetPlugin>());

// New (Bevy 0.9)
app.add_plugins(DefaultPlugins.build().disable::<AssetPlugin>());
```

### [Replace `WorldQueryGats` trait with actual gats](https://github.com/bevyengine/bevy/pull/6319)

Replace usage of `WorldQueryGats` assoc types with the actual gats on `WorldQuery` trait

### [Add a method for accessing the width of a `Table`](https://github.com/bevyengine/bevy/pull/6249)

Any use of `Table::len` should now be `Table::entity_count`. Any use of `Table::capacity` should now be `Table::entity_capacity`.

### [Replace the `bool` argument of `Timer` with `TimerMode`](https://github.com/bevyengine/bevy/pull/6247)

* Replace `Timer::new(duration, false)` with `Timer::new(duration, TimerMode::Once)`.
* Replace `Timer::new(duration, true)` with `Timer::new(duration, TimerMode::Repeating)`.
* Replace `Timer::from_seconds(seconds, false)` with `Timer::from_seconds(seconds, TimerMode::Once)`.
* Replace `Timer::from_seconds(seconds, true)` with `Timer::from_seconds(seconds, TimerMode::Repeating)`.
* Change `timer.repeating()` to `timer.mode() == TimerMode::Repeating`.

### [Derive `Reflect` + `FromReflect` for input types](https://github.com/bevyengine/bevy/pull/6232)

* `Input<T>` now implements `Reflect` via `#[reflect]` instead of `#[reflect_value]`. This means it now exposes its private fields via the `Reflect` trait rather than being treated as a value type. For code that relies on the `Input<T>` struct being treated as a value type by reflection, it is still possible to wrap the `Input<T>` type with a wrapper struct and apply `#[reflect_value]` to it.
* As a reminder, private fields exposed via reflection are not subject to any stability guarantees.

### [Rename system chaining to system piping](https://github.com/bevyengine/bevy/pull/6230)

The `.chain(handler_system)` method on systems is now `.pipe(handler_system)`.
The `IntoChainSystem` trait is now `IntoPipeSystem`, and the `ChainSystem` struct is now `PipeSystem`.

### [Update `wgpu` to 0.14.0, `naga` to `0.10.0`, `winit` to 0.27.4, `raw-window-handle` to 0.5.0, `ndk` to 0.7](https://github.com/bevyengine/bevy/pull/6218)

Adjust usage of `bevy_window::WindowDescriptor`’s `cursor_locked` to `cursor_grab_mode`, and adjust its type from `bool` to `bevy_window::CursorGrabMode`.

### [Make the default background color of `NodeBundle` transparent](https://github.com/bevyengine/bevy/pull/6211)

If you want a `NodeBundle` with a white background color, you must explicitly specify it:

```rust
// Old (Bevy 0.8)
let node = NodeBundle {
    ..default()
}

// New (Bevy 0.9)
let node = NodeBundle {
    background_color: Color::WHITE.into(),
    ..default()
}
```

### [Make `Handle::<T>` field id private, and replace with a getter](https://github.com/bevyengine/bevy/pull/6176)

If you were accessing the value `handle.id`, you can now do so with `handle.id()`

### [Add `TimeUpdateStrategy` resource for manual `Time` updating](https://github.com/bevyengine/bevy/pull/6159)

Changes the value reported by `time.delta()` on startup.

Before it would be `[0, 0, correct]` and this PR changes it to be `[0, "approximately the time between the time_system and present_frame", correct]`.

### [Add methods for silencing system-order ambiguity warnings](https://github.com/bevyengine/bevy/pull/6158)

Ambiguity sets have been replaced with a simpler API.

```rust
// These systems technically conflict, but we don't care which order they run in.
fn jump_on_click(mouse: Res<Input<MouseButton>>, mut transforms: Query<&mut Transform>) { ... }
fn jump_on_spacebar(keys: Res<Input<KeyCode>>, mut transforms: Query<&mut Transform>) { ... }

// Old (Bevy 0.8)
#[derive(AmbiguitySetLabel)]
struct JumpSystems;

app
  .add_system(jump_on_click.in_ambiguity_set(JumpSystems))
  .add_system(jump_on_spacebar.in_ambiguity_set(JumpSystems));

// New (Bevy 0.9)
app
  .add_system(jump_on_click.ambiguous_with(jump_on_spacebar))
  .add_system(jump_on_spacebar);

```

### [Utility methods for Val](https://github.com/bevyengine/bevy/pull/6134)

Instead of using the + and - operators, perform calculations on `Val`s using the new `try_add` and `try_sub` methods. Multiplication and division remained unchanged. Also, when adding or subtracting from `Size`, ~~use a `Val` tuple instead of `Vec2`~~ perform the addition on `width` and `height` separately.

### [Remove `Transform::apply_non_uniform_scale`](https://github.com/bevyengine/bevy/pull/6133)

`Transform::apply_non_uniform_scale` has been removed.
It can be replaced with the following snippet:

```rust
transform.scale *= scale_factor;
```

### [Rename `Transform::mul_vec3` to `transform_point` and improve docs](https://github.com/bevyengine/bevy/pull/6132)

`Transform::mul_vec3` has been renamed to `transform_point`.

### [Make `raw_window_handle` field in `Window` and `ExtractedWindow` an `Option`.](https://github.com/bevyengine/bevy/pull/6114)

`Window::raw_window_handle()` now returns `Option<RawWindowHandleWrapper>`.

### [``Entity``'s “ID” should be named “index” instead](https://github.com/bevyengine/bevy/pull/6107)

The `Entity::id()` method was renamed to `Entity::index()`.

### [Add getters and setters for `InputAxis` and `ButtonSettings`](https://github.com/bevyengine/bevy/pull/6088)

`AxisSettings` now has a `new()`, which may return an `AxisSettingsError`.
`AxisSettings` fields made private; now must be accessed through getters and setters.  There’s a dead zone, from `.deadzone_upperbound()` to `.deadzone_lowerbound()`, and a live zone, from `.deadzone_upperbound()` to `.livezone_upperbound()` and from `.deadzone_lowerbound()` to `.livezone_lowerbound()`.
`AxisSettings` setters no longer panic.
`ButtonSettings` fields made private; now must be accessed through getters and setters.
`ButtonSettings` now has a `new()`, which may return a `ButtonSettingsError`.

### [Rename `UiColor`  to `BackgroundColor`](https://github.com/bevyengine/bevy/pull/6087)

`UiColor` has been renamed to `BackgroundColor`. This change affects `NodeBundle`, `ButtonBundle` and `ImageBundle`. In addition, the corresponding field on `ExtractedUiNode` has been renamed to `background_color` for consistency.

### [Exclusive Systems Now Implement `System`. Flexible Exclusive System Params](https://github.com/bevyengine/bevy/pull/6083)

Calling `.exclusive_system()` is no longer required (or supported) for converting exclusive system functions to exclusive systems:

```rust
// Old (0.8)
app.add_system(some_exclusive_system.exclusive_system());
// New (0.9)
app.add_system(some_exclusive_system);
```

Converting “normal” parallel systems to exclusive systems is done by calling the exclusive ordering apis:

```rust
// Old (0.8)
app.add_system(some_system.exclusive_system().at_end());
// New (0.9)
app.add_system(some_system.at_end());
```

Query state in exclusive systems can now be cached via ExclusiveSystemParams, which should be preferred for clarity and performance reasons:

```rust
// Old (0.8)
fn some_system(world: &mut World) {
  let mut transforms = world.query::<&Transform>();
  for transform in transforms.iter(world) {
  }
}
// New (0.9)
fn some_system(world: &mut World, transforms: &mut QueryState<&Transform>) {
  for transform in transforms.iter(world) {
  }
}
```

### [Merge TextureAtlas::from_grid_with_padding into TextureAtlas::from_grid through option arguments](https://github.com/bevyengine/bevy/pull/6057)

`TextureAtlas::from_grid_with_padding` was merged into `from_grid` which takes two additional parameters for padding and an offset.

```rust
// 0.8
TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
// 0.9
TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None)

// 0.8
TextureAtlas::from_grid_with_padding(texture_handle, Vec2::new(24.0, 24.0), 7, 1, Vec2::new(4.0, 4.0));
// 0.9
TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, Some(Vec2::new(4.0, 4.0)), None)
```

### [Spawn now takes a Bundle](https://github.com/bevyengine/bevy/pull/6054)

```rust
// Old (0.8):
commands
  .spawn()
  .insert_bundle((A, B, C));
// New (0.9)
commands.spawn((A, B, C));

// Old (0.8):
commands.spawn_bundle((A, B, C));
// New (0.9)
commands.spawn((A, B, C));

// Old (0.8):
let entity = commands.spawn().id();
// New (0.9)
let entity = commands.spawn_empty().id();

// Old (0.8)
let entity = world.spawn().id();
// New (0.9)
let entity = world.spawn_empty();
```

### [Accept Bundles for insert and remove. Deprecate `insert`/`remove_bundle`](https://github.com/bevyengine/bevy/pull/6039)

Replace `insert_bundle` with `insert`:

```rust
// Old (0.8)
commands.spawn().insert_bundle(SomeBundle::default());
// New (0.9)
commands.spawn().insert(SomeBundle::default());
```

Replace `remove_bundle` with `remove`:

```rust
// Old (0.8)
commands.entity(some_entity).remove_bundle::<SomeBundle>();
// New (0.9)
commands.entity(some_entity).remove::<SomeBundle>();
```

Replace `remove_bundle_intersection` with `remove_intersection`:

```rust
// Old (0.8)
world.entity_mut(some_entity).remove_bundle_intersection::<SomeBundle>();
// New (0.9)
world.entity_mut(some_entity).remove_intersection::<SomeBundle>();
```

Consider consolidating as many operations as possible to improve ergonomics and cut down on archetype moves:

```rust
// Old (0.8)
commands.spawn()
  .insert_bundle(SomeBundle::default())
  .insert(SomeComponent);

// New (0.9) - Option 1
commands.spawn_empty().insert((
  SomeBundle::default(),
  SomeComponent,
))

// New (0.9) - Option 2
commands.spawn((
  SomeBundle::default(),
  SomeComponent,
))
```

### [Fix inconsistent children removal behavior](https://github.com/bevyengine/bevy/pull/6017)

* Queries with `Changed<Children>` will no longer match entities that had all of their children removed using `remove_children`.
* `RemovedComponents<Children>` will now contain entities that had all of their children removed using `remove_children`.

### [`Query` filter types must be `ReadOnlyWorldQuery`](https://github.com/bevyengine/bevy/pull/6008)

Query filter (`F`) generics are now bound by `ReadOnlyWorldQuery`, rather than `WorldQuery`. If for some reason you were requesting `Query<&A, &mut B>`, please use `Query<&A, With<B>>` instead.

### [Change UI coordinate system to have origin at top left corner](https://github.com/bevyengine/bevy/pull/6000)

All flex layout should be inverted (ColumnReverse => Column, FlexStart => FlexEnd, WrapReverse => Wrap)
System where dealing with cursor position should be changed to account for cursor position being based on the top left instead of bottom left

### [Clarify `bevy::ui::Node` field and documentation](https://github.com/bevyengine/bevy/pull/5995)

All references to the old `size` name has been changed, to access `bevy::ui::Node` `size` field use `calculated_size`

### [Remove `AssetServer::watch_for_changes()`](https://github.com/bevyengine/bevy/pull/5968)

`AssetServer::watch_for_changes()` was removed.
Instead, set it directly on the `AssetPlugin`.

```rust
app
  .add_plugin(DefaultPlugins.set(AssetPlugin {
    watch_for_changes: true,
    ..default()
  }))
```

### [Remove ambiguity sets](https://github.com/bevyengine/bevy/pull/5916)

Ambiguity sets have been removed.

### [Remove `ExactSizeIterator` from `QueryCombinationIter`](https://github.com/bevyengine/bevy/pull/5895)

Switch to using other methods of getting the length.

### [Support monitor selection for all window modes.](https://github.com/bevyengine/bevy/pull/5878)

`MonitorSelection` was moved out of `WindowPosition::Centered`, into `WindowDescriptor`.
`MonitorSelection::Number` was renamed to `MonitorSelection::Index`.

```rust
// Before
.insert_resource(WindowDescriptor {
    position: WindowPosition::Centered(MonitorSelection::Number(1)),
    ..default()
})
// After
.add_plugins(DefaultPlugins.set(WindowPlugin {
    window: WindowDescriptor {
        monitor: MonitorSelection::Index(1),
        position: WindowPosition::Centered,
        ..default()
    },
    ..default()
}))
```

`Window::set_position` now takes a `MonitorSelection` as argument.

```rust
window.set_position(MonitorSelection::Current, position);
```

### [Add `pop` method for `List` trait.](https://github.com/bevyengine/bevy/pull/5797)

Any custom type that implements the `List` trait will now need to implement the `pop` method.

### [bevy_pbr: Fix incorrect and unnecessary normal-mapping code](https://github.com/bevyengine/bevy/pull/5766)

`prepare_normal` from the `bevy_pbr::pbr_functions` shader import has been reworked.

Before:

```wgsl
    pbr_input.world_normal = in.world_normal;

    pbr_input.N = prepare_normal(
        pbr_input.material.flags,
        in.world_normal,
#ifdef VERTEX_TANGENTS
#ifdef STANDARDMATERIAL_NORMAL_MAP
        in.world_tangent,
#endif
#endif
        in.uv,
        in.is_front,
    );
```

After:

```wgsl
    pbr_input.world_normal = prepare_world_normal(
        in.world_normal,
        (material.flags & STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT) != 0u,
        in.is_front,
    );

    pbr_input.N = apply_normal_mapping(
        pbr_input.material.flags,
        pbr_input.world_normal,
#ifdef VERTEX_TANGENTS
#ifdef STANDARDMATERIAL_NORMAL_MAP
        in.world_tangent,
#endif
#endif
        in.uv,
    );
```

### [Add global time scaling](https://github.com/bevyengine/bevy/pull/5752)

* `time.time_since_startup()` -> `time.elapsed()`
* `time.seconds_since_startup()` -> `time.elapsed_seconds_f64()`
* `time.seconds_since_startup_wrapped_f32()` -> `time.elapsed_seconds_wrapped()`

If you aren’t sure which to use, most systems should continue to use “scaled” time (e.g. `time.delta_seconds()`). The realtime “unscaled” time measurements (e.g. `time.raw_delta_seconds()`) are mostly for debugging and profiling.

### [bevy_reflect: Improve serialization format even more](https://github.com/bevyengine/bevy/pull/5723)

This PR reduces the verbosity of the scene format. Scenes will need to be updated accordingly:

```js
// Old format
{
  "type": "my_game::item::Item",
  "struct": {
    "id": {
      "type": "alloc::string::String",
      "value": "bevycraft:stone",
    },
    "tags": {
      "type": "alloc::vec::Vec<alloc::string::String>",
      "list": [
        {
          "type": "alloc::string::String",
          "value": "material"
        },
      ],
    },
  }
}

// New format
{
  "my_game::item::Item": (
    id: "bevycraft:stone",
    tags: ["material"]
  )
}
```

### [Move `sprite::Rect` into `bevy_math`](https://github.com/bevyengine/bevy/pull/5686)

The `bevy::sprite::Rect` type moved to the math utility crate as
`bevy::math::Rect`. You should change your imports from `use bevy::sprite::Rect` to `use bevy::math::Rect`.

### [Remove unused DepthCalculation enum](https://github.com/bevyengine/bevy/pull/5684)

Remove references to `bevy_render::camera::DepthCalculation`, such as `use bevy_render::camera::DepthCalculation`. Remove `depth_calculation` fields from Projections.

### [Remove an outdated workaround for `impl Trait`](https://github.com/bevyengine/bevy/pull/5659)

The methods `Schedule::get_stage` and `get_stage_mut` now accept `impl StageLabel` instead of `&dyn StageLabel`.

### [bevy_reflect: Relax bounds on `Option<T>`](https://github.com/bevyengine/bevy/pull/5658)

If using `Option<T>` with Bevy’s reflection API, `T` now needs to implement `FromReflect` rather than just `Clone`. This can be achieved easily by simply deriving `FromReflect`:

```rust

// OLD
#[derive(Reflect, Clone)]
struct Foo;

let reflected: Box<dyn Reflect> = Box::new(Some(Foo));

// NEW
#[derive(Reflect, FromReflect)]
struct Foo;

let reflected: Box<dyn Reflect> = Box::new(Some(Foo));
```

Note: You can still derive `Clone`, but it’s not required in order to compile.

### [Add a change detection bypass and manual control over change ticks](https://github.com/bevyengine/bevy/pull/5635)

Add the `Inner` associated type and new methods to any type that you’ve implemented `DetectChanges` for.

### [Remove `ReflectMut` in favor of `Mut<dyn Reflect>`](https://github.com/bevyengine/bevy/pull/5630)

* relax `T: ?Sized` bound in `Mut<T>`
* replace all instances of `ReflectMut` with `Mut<dyn Reflect>`

### [Make internal struct `ShaderData` non-`pub`](https://github.com/bevyengine/bevy/pull/5609)

<!-- TODO -->

### [Make `Resource` trait opt-in, requiring `#[derive(Resource)]` V2](https://github.com/bevyengine/bevy/pull/5577)

Add `#[derive(Resource)]` to all types you are using as a resource.

If you are using a third party type as a resource, wrap it in a tuple struct to bypass orphan rules. Consider deriving `Deref` and `DerefMut` to improve ergonomics.

`ClearColor` no longer implements `Component`. Using `ClearColor` as a component in 0.8 did nothing.
Use the `ClearColorConfig` in the `Camera3d` and `Camera2d` components instead.

### [Changed diagnostics from seconds to milliseconds](https://github.com/bevyengine/bevy/pull/5554)

Diagnostics values are now in milliseconds. If you need seconds, simply divide it by 1000.0;

### [Make `Children` constructor `pub(crate)`.](https://github.com/bevyengine/bevy/pull/5532)

`Children::with()` is now renamed `Children::from_entities()` and is now `pub(crate)`

### [Expose `Image` conversion functions (fixes #5452)](https://github.com/bevyengine/bevy/pull/5527)

* Rename `image_to_texture` to `Image::from_dynamic`
* Rename `texture_to_image` to `Image::try_into_dynamic`
* `Image::try_into_dynamic` now returns a `Result` (this is to make it easier for users who didn't read that only a few conversions are supported to figure it out.)

### [Remove `Sync` bound from `Local`](https://github.com/bevyengine/bevy/pull/5483)

Any code relying on `Local<T>` having `T: Resource` may have to be changed, but this is unlikely.

### [Add `FromWorld` bound to `T` in `Local<T>`](https://github.com/bevyengine/bevy/pull/5481)

It might be possible for references to `Local`s without `T: FromWorld` to exist, but these should be exceedingly rare and probably dead code. In the event that one of these is encountered, the easiest solutions are to delete the code or wrap the inner `T` in an `Option` to allow it to be default constructed to `None`.

### [bevy_reflect: Update enum derives](https://github.com/bevyengine/bevy/pull/5473)

Bevy-defined enums have been updated to implement `Enum` and are not considered value types (`ReflectRef::Value`) anymore. This means that their serialized representations will need to be updated. For example, given the Bevy enum:

```rust
pub enum ScalingMode {
  None,
  WindowSize,
  Auto { min_width: f32, min_height: f32 },
  FixedVertical(f32),
  FixedHorizontal(f32),
}
```

You will need to update the serialized versions accordingly.

```js
// OLD FORMAT
{
  "type": "bevy_render::camera::projection::ScalingMode",
  "value": FixedHorizontal(720),
},

// NEW FORMAT
{
  "type": "bevy_render::camera::projection::ScalingMode",
  "enum": {
    "variant": "FixedHorizontal",
    "tuple": [
      {
        "type": "f32",
        "value": 720,
      },
    ],
  },
},
```

This may also have other smaller implications (such as `Debug` representation), but serialization is probably the most prominent.

### [Remove `Size` and `UiRect` generics](https://github.com/bevyengine/bevy/pull/5404)

The generic `T` of `Size` and `UiRect` got removed and instead they both now always use `Val`. If you used a `Size<f32>` consider replacing it with a `Vec2` which is way more powerful.

### [Add associated constant `IDENTITY` to `Transform` and friends.](https://github.com/bevyengine/bevy/pull/5340)

The method `identity()` on `Transform`, `GlobalTransform` and `TransformBundle` has been removed.
Use the associated constant `IDENTITY` instead.

### [Rename Handle::as_weak() to cast_weak()](https://github.com/bevyengine/bevy/pull/5321)

* Rename `Handle::as_weak` uses to `Handle::cast_weak`

The method now properly sets the associated type uuid if the handle is a direct reference (e.g. not a reference to an `AssetPath`), so adjust you code accordingly if you relied on the previous behavior.

### [`Gamepad` type is `Copy`; do not require / return references to it in `Gamepads` API](https://github.com/bevyengine/bevy/pull/5296)

* `Gamepads::iter` now returns an iterator of `Gamepad`. rather than an iterator of `&Gamepad`.
* `Gamepads::contains` now accepts a `Gamepad`, rather than a `&Gamepad`.

### [remove blanket `Serialize + Deserialize` requirement for `Reflect` on generic types](https://github.com/bevyengine/bevy/pull/5197)

`.register_type` for generic types like `Option<T>`, `Vec<T>`, `HashMap<K, V>` will no longer insert `ReflectSerialize` and `ReflectDeserialize` type data. Instead you need to register it separately for concrete generic types like so:

```rust
        .register_type::<Option<String>>()
        .register_type_data::<Option<String>, ReflectSerialize>()
        .register_type_data::<Option<String>, ReflectDeserialize>()
```

### [Add Exponential Moving Average into diagnostics](https://github.com/bevyengine/bevy/pull/4992)

`LogDiagnosticsPlugin` now records the smoothed value rather than the raw value.

* For diagnostics recorded less often than every 0.1 seconds, this change to defaults will have no visible effect.
* For discrete diagnostics where this smoothing is not desirable, set a smoothing factor of 0 to disable smoothing.
* The average of the recent history is still shown when available.

### [Swap out `num_cpus` for `std::thread::available_parallelism`](https://github.com/bevyengine/bevy/pull/4970)

`bevy_tasks::logical_core_count` and `bevy_tasks::physical_core_count` have been removed. `logical_core_count` has been replaced with `bevy_tasks::available_parallelism`, which works identically. If `bevy_tasks::physical_core_count` is required, the `num_cpus` crate can be used directly, as these two were just aliases for `num_cpus` APIs.

### [Extract Resources into their own dedicated storage](https://github.com/bevyengine/bevy/pull/4809)

Resources have been moved to `Resources` under `Storages` in `World`. All code dependent on `Archetype::unique_components(_mut)` should access it via `world.storages().resources()` instead.

All APIs accessing the raw data of individual resources (mutable _and_ read-only) have been removed as these APIs allowed for unsound unsafe code. All usages of these APIs should be changed to use `World::{get, insert, remove}_resource`.

### [Clean up Fetch code](https://github.com/bevyengine/bevy/pull/4800)

Changed: `Fetch::table_fetch` and `Fetch::archetype_fetch` have been merged into a single `Fetch::fetch` function.

### [Change `gamepad.rs` tuples to normal structs](https://github.com/bevyengine/bevy/pull/4519)

The `Gamepad`, `GamepadButton`, `GamepadAxis`, `GamepadEvent` and `GamepadEventRaw` types are now normal structs instead of tuple structs and have a `new()` function. To migrate change every instantiation to use the `new()` function instead and use the appropriate field names instead of `.0` and `.1`.

### [Nested spawns on scope](https://github.com/bevyengine/bevy/pull/4466)

If you were using explicit lifetimes and Passing Scope you’ll need to specify two lifetimes now.

```rust
// 0.8
fn scoped_function<'scope>(scope: &mut Scope<'scope, ()>) {}

// 0.9
fn scoped_function<'scope>(scope: &Scope<'_, 'scope, ()>) {}
```

`scope.spawn_local` changed to `scope.spawn_on_scope` this should cover cases where you needed to run tasks on the local thread, but does not cover spawning Nonsend Futures. Spawning of NonSend futures on scope is no longer supported.

### [Rename `ElementState` to `ButtonState`](https://github.com/bevyengine/bevy/pull/4314)

The `ElementState` type received a rename and is now called `ButtonState`. To migrate you just have to change every occurrence of `ElementState` to `ButtonState`.

### [Move `Size` to `bevy_ui`](https://github.com/bevyengine/bevy/pull/4285)

The `Size` type got moved from `bevy::math` to `bevy::ui`. To migrate you just have to import `bevy::ui::Size` instead of `bevy::math::Math` or use the `bevy::prelude` instead.

### [Remove `margins.rs`](https://github.com/bevyengine/bevy/pull/4284)

The `Margins` type got removed. To migrate you just have to change every occurrence of `Margins` to `UiRect`.

### [Remove `face_toward.rs`](https://github.com/bevyengine/bevy/pull/4277)

The `FaceToward` trait got removed. To migrate you just have to change every occurrence of `Mat4::face_toward` to `Mat4::look_at_rh`.

### [Move `Rect` to `bevy_ui` and rename it to `UiRect`](https://github.com/bevyengine/bevy/pull/4276)

The `Rect` type got renamed to `UiRect`. To migrate you just have to change every occurrence of `Rect` to `UiRect`.

### [Implement `Bundle` for `Component`. Use `Bundle` tuples for insertion](https://github.com/bevyengine/bevy/pull/2975)

In `derive(Bundle)`, the `bundle` attribute has been removed. Nested bundles are now collapsed automatically. You should remove `#[bundle]` attributes.