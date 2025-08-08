# Bevy Animation Features Guide

## Core Animation Features

### Animation Playback

**File:** `animated_mesh.rs`  
**GitHub:
** [examples/animation/animated_mesh.rs](https://github.com/bevyengine/bevy/blob/main/examples/animation/animated_mesh.rs)  
**Description:** Basic animation playback on skinned meshes. What: Plays animations from glTF files. Why: Foundation for
all animated content. When: Loading 3D models with animations. Where: Any animated entity. Who: Anyone needing basic
animation playback.

```rust
let (graph, index) = AnimationGraph::from_clip(
asset_server.load(GltfAssetLabel::Animation(2).from_asset("Fox.glb"))
);
player.play(index).repeat();
```

---

### Animation Control

**File:** `animated_mesh_control.rs`  
**GitHub:
** [examples/animation/animated_mesh_control.rs](https://github.com/bevyengine/bevy/blob/main/examples/animation/animated_mesh_control.rs)  
**Description:** Runtime animation control with transitions. What: Play/pause, speed control, seeking, and smooth
transitions. Why: Interactive animation systems need user control. When: Building games with controllable characters.
Where: Player controllers, cutscene systems. Who: Game developers needing animation state machines.

```rust
transitions.play( & mut player, animation_index, Duration::from_millis(250))
.repeat();
player.animation_mut(index).unwrap().set_speed(1.2);
```

---

### Animation Events

**File:** `animated_mesh_events.rs`  
**GitHub:
** [examples/animation/animated_mesh_events.rs](https://github.com/bevyengine/bevy/blob/main/examples/animation/animated_mesh_events.rs)  
**Description:** Trigger game events at specific animation times. What: Events fired at animation keyframes. Why:
Synchronize game logic with animations. When: Footsteps, attack hits, sound effects. Where: Combat systems, movement
feedback. Who: Developers needing animation-synchronized effects.

```rust
animation_clip.add_event_to_target(
target_id,
0.625, // time in seconds
OnStep
);
```

---

### Programmatic Animation

**File:** `animated_transform.rs`  
**GitHub:
** [examples/animation/animated_transform.rs](https://github.com/bevyengine/bevy/blob/main/examples/animation/animated_transform.rs)  
**Description:** Create animations in code without external files. What: Define animation curves programmatically. Why:
Dynamic or procedural animations. When: Runtime-generated animations. Where: Procedural content, dynamic effects. Who:
Developers creating animations without art assets.

```rust
animation.add_curve_to_target(
target_id,
AnimatableCurve::new(
animated_field!(Transform::translation),
UnevenSampleAutoCurve::new([(0.0, Vec3::X), (1.0, Vec3::Y)])
)
);
```

---

### UI Animation

**File:** `animated_ui.rs`  
**GitHub:
** [examples/animation/animated_ui.rs](https://github.com/bevyengine/bevy/blob/main/examples/animation/animated_ui.rs)  
**Description:** Animate UI properties like text size and color. What: Animations for UI elements. Why: Dynamic,
engaging user interfaces. When: Menu transitions, notifications. Where: HUD elements, menus. Who: UI/UX developers
creating polished interfaces.

```rust
animation_clip.add_curve_to_target(
target_id,
AnimatableCurve::new(
animated_field!(TextFont::font_size),
AnimatableKeyframeCurve::new([
(0.0, 24.0), (0.5, 80.0), (1.0, 24.0)
])
)
);
```

---

### Animation Blending

**File:** `animation_graph.rs`  
**GitHub:
** [examples/animation/animation_graph.rs](https://github.com/bevyengine/bevy/blob/main/examples/animation/animation_graph.rs)  
**Description:** Blend multiple animations with weighted mixing. What: Combine animations using blend nodes. Why: Smooth
transitions between animation states. When: Character locomotion blending. Where: Movement systems, animation state
machines. Who: Character animators and gameplay programmers.

```rust
let blend_node = animation_graph.add_blend(0.5, animation_graph.root);
animation_graph.add_clip(walk_clip, 1.0, blend_node);
animation_graph.add_clip(run_clip, 1.0, blend_node);
```

---

### Animation Masks

**File:** `animation_masks.rs`  
**GitHub:
** [examples/animation/animation_masks.rs](https://github.com/bevyengine/bevy/blob/main/examples/animation/animation_masks.rs)  
**Description:** Limit animations to specific bones or body parts. What: Selective animation on skeleton subsets. Why:
Layer animations (upper/lower body). When: Aiming while running, gestures. Where: Complex character controllers. Who:
Advanced animation system developers.

```rust
animation_graph.add_target_to_mask_group(
animation_target_id,
MASK_GROUP_HEAD
);
animation_graph.add_clip_with_mask(handle, 0x3f, 1.0, blend_node);
```

---

### Color Animation

**File:** `color_animation.rs`  
**GitHub:
** [examples/animation/color_animation.rs](https://github.com/bevyengine/bevy/blob/main/examples/animation/color_animation.rs)  
**Description:** Animate colors in different color spaces. What: Color transitions using curves or mixing. Why: Smooth,
perceptually correct color changes. When: Visual effects, UI feedback. Where: Materials, sprites, UI elements. Who:
Visual effects artists and shader programmers.

```rust
let curve = CubicBezier::new([
LinearRgba::WHITE,
LinearRgba::RED,
LinearRgba::BLACK
]).to_curve();
sprite.color = curve.position(t).into();
```

---

### Skinned Mesh Creation

**File:** `custom_skinned_mesh.rs`  
**GitHub:
** [examples/animation/custom_skinned_mesh.rs](https://github.com/bevyengine/bevy/blob/main/examples/animation/custom_skinned_mesh.rs)  
**Description:** Programmatically create skinned meshes with joints. What: Build skeletal meshes in code. Why:
Procedural character generation. When: Runtime mesh creation. Where: Procedural animation systems. Who: Technical
artists and tool developers.

```rust
mesh.with_inserted_attribute(
Mesh::ATTRIBUTE_JOINT_INDEX,
VertexAttributeValues::Uint16x4(vec![[0, 1, 0, 0]])
)
.with_inserted_attribute(
Mesh::ATTRIBUTE_JOINT_WEIGHT,
vec![[0.5, 0.5, 0.0, 0.0]]
);
```

---

### Easing Functions

**File:** `eased_motion.rs`  
**GitHub:
** [examples/animation/eased_motion.rs](https://github.com/bevyengine/bevy/blob/main/examples/animation/eased_motion.rs)  
**Description:** Apply easing curves for smooth motion transitions. What: Non-linear animation interpolation. Why:
Natural, polished movement. When: UI animations, camera movements. Where: Any smooth transition effect. Who: Developers
creating polished animations.

```rust
let curve = EasingCurve::new(
start_value,
end_value,
EaseFunction::CubicInOut
).reparametrize_linear(interval(0.0, 3.0));
```

---

### Easing Visualization

**File:** `easing_functions.rs`  
**GitHub:
** [examples/animation/easing_functions.rs](https://github.com/bevyengine/bevy/blob/main/examples/animation/easing_functions.rs)  
**Description:** Visual demonstration of all built-in easing functions. What: Interactive easing curve display. Why:
Understanding easing behavior. When: Selecting appropriate easing. Where: Development tools, debugging. Who: Animators
choosing easing functions.

```rust
EaseFunction::ElasticInOut
EaseFunction::BounceIn
EaseFunction::SmoothStep
EaseFunction::Steps(4, JumpAt::Start)
```

---

### glTF Skinned Mesh

**File:** `gltf_skinned_mesh.rs`  
**GitHub:
** [examples/animation/gltf_skinned_mesh.rs](https://github.com/bevyengine/bevy/blob/main/examples/animation/gltf_skinned_mesh.rs)  
**Description:** Load and manipulate skinned meshes from glTF files. What: Access joint hierarchy from loaded models.
Why: Procedural bone manipulation. When: IK systems, ragdolls. Where: Character animation systems. Who: Gameplay
programmers working with animated models.

```rust
// Access joint hierarchy
let first_joint = mesh_node_children[1];
let second_joint = parents.get(first_joint).unwrap()[0];
transform_query.get_mut(second_joint).unwrap().rotation =
Quat::from_rotation_z(angle);
```

---

### Morph Targets

**File:** `morph_targets.rs`  
**GitHub:
** [examples/animation/morph_targets.rs](https://github.com/bevyengine/bevy/blob/main/examples/animation/morph_targets.rs)  
**Description:** Control blend shapes and morph targets. What: Deform meshes using morph targets. Why: Facial animation,
shape morphing. When: Character expressions, shape transitions. Where: Face rigs, customization systems. Who: Character
artists and expression systems developers.

```rust
// Access morph target names
let names = mesh.morph_target_names().unwrap();
// Morph targets animate automatically via AnimationClip
```

---

### Simple Events

**File:** `animation_events.rs`  
**GitHub:
** [examples/animation/animation_events.rs](https://github.com/bevyengine/bevy/blob/main/examples/animation/animation_events.rs)  
**Description:** Basic event system for animation-triggered actions. What: Simple event firing during playback. Why:
Lightweight animation callbacks. When: Text changes, simple triggers. Where: UI animations, notifications. Who:
Developers needing basic animation callbacks.

```rust
animation.add_event(
0.0, // time
MessageEvent {
value: "HELLO".into(),
color: Color::WHITE
}
);
```