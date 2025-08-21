
1. **Type Domain Separation - Absolute Rule**: Every type belongs to EXACTLY one domain: Components (entity state) OR
   Values (data passing). **Components**: Attach to entities via `commands.spawn()` or `.insert()`, queried via
   `Query<&T>`, never appear in event payloads, function parameters (except queries), or standalone data structures.
   **Values**: Used in events (`EventWriter<T>`), function parameters, return types, Resources, and data structures.
   **Entity References**: Use `Entity` type when referring to specific entities in value contexts (e.g.,
   `struct DamageEvent { target: Entity, amount: f32 }`). **Dual Concepts**: When the same logical concept exists in
   both domains, create TWO distinct types with clear names (e.g., `Health` component for entity state vs `HealthUpdate`
   event payload, `Position` component vs `PositionData` for serialization). **System Boundaries**: Systems operate on
   Components through queries; helper functions take value types as parameters. **Never Mix**: A type that is a
   Component can NEVER be used as an event payload, function parameter, or in non-entity data structures—this is an
   architectural violation that breaks ECS principles.
2. **Entity References vs Value Types**: Use `Entity` references when pointing to specific entities; use value types for
   data that exists independently of the entity system.
3. **Static Data Lookup**: Game data in `const` arrays/structs; apply via systems.
4. **Events for Communication**: Changes flow through events; decouple with event boundaries. Event payloads must use
   value types, never Components.
5. **Typed Asset Components**: For mesh and material assets, use typed wrappers: `Mesh3d(Handle<Mesh>)`,
   `MeshMaterial3d(Handle<Material>)` instead of generic `Handle<T>`. This provides type safety and clearer intent.
6. **Marker Components**: Zero-sized markers for categorization/toggles, never used outside entity context.
7. **Required Components**: Use `#[require()]` to auto-include component dependencies. Simple components like
   `Camera3d`, `PointLight` automatically add their requirements. For custom components, define requirements to
   eliminate manual bundle management.
8. **Change Detection**: Use `Added<T>`/`Changed<T>`; process only what changed.
9. **Component Composition**: Many simple, single-purpose components compose into complex entity behavior. Components
   never serve as general-purpose data containers.
10. **Design for Idempotency**: Operations produce same result when applied multiple times; mathematical properties
    verified with property tests when appropriate.
11. **Use Display/FromStr**: Human-readable boundaries; no internal representation leakage.
12. **Prefer Borrowing**: Use `&str`/slices rather than allocate; avoid `.to_string()` churn.
13. **No Global Mutable State**: If absolutely unavoidable, a single owner with documented initialization.
14. **Docs Are Tests**: Rustdoc examples compile; executable documentation; with terse, proper grammar that includes
    articles and punctuation.
15. **Never Ignore Result**: Handle or propagate with context using `.with_context()` when error origin would be
    unclear.
16. **Direct Field Access**: Prefer public fields or direct mutation when there's no invariant to maintain.
17. **Naming Conventions**: Components use suffixes like `Component`, `Tag`, `State` when entity relationship isn't
    obvious. Value types use prefixes/suffixes like `Event`, `Data`, `Config`, `Payload`. Examples: `HealthComponent` vs
    `HealthData`.
18. **Pattern Matching**: When extracting values from enums, prefer `match` expressions over `if let` chains for
    exhaustiveness.
19. **Query Efficiency**: Use `With<T>`/`Without<T>` filters; minimize lookups; cache locally if reused.
20. **Single Responsibility Systems**: One job per system; explicit order with sets/labels; <50 LOC.
21. **Finite Sets Are Enums**: When a type has fewer than 20 valid values known at compile-time, use an enum.
22. **Three-Layer Error Pattern**: Domain errors with `thiserror`, systems return `Result` (imported from bevy prelude),
    propagate with `?`. Never `.unwrap()` in systems.
23. **Development vs Production**: Let errors panic in dev (default), log in production. Configure once in main.
24. **Import at Module Level**: Import enum variants, associated constants, and frequently used types at the top of a
    file. Avoid inline qualified paths in favor of clean imports.
25. **Test Systems in Isolation**: Create minimal worlds for system tests. Test queries with known entity
    configurations. Mock events and resources. Property test mathematical components.
26. **States as Components**: Implement FSMs as enum components with transition systems. One state component per state
    machine, not per state.
27. **Entity Relationships**: Use Bevy's relationship system with `ChildOf` for hierarchies. For custom relationships,
    use `#[relationship]` attributes. Transform propagation automatically only for parent-child.
28. **Minimize Archetype Moves**: Avoid frequent component additions/removals. Use `Option<T>` or enums for togglable
    state. Profile archetype fragmentation in performance-critical paths.
29. **One interface per concept**: When multiple methods expose variations of the same underlying data, provide only the
    variant that serves the API's purpose. Internal representations and intermediate forms should stay private unless
    they represent genuinely different concepts.
30. **Build for Now**: Don't build for tomorrow. Build exactly what is needed right now for today.
