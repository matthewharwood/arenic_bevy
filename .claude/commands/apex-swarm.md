---
description: "Execute sophisticated multi-agent workflows with quality gates and recursive refinement"
allowed-tools: [ "Task", "Read", "Write", "Edit", "MultiEdit", "Grep", "Glob", "TodoWrite" ]
---

# /apex-swarm - Intelligent Multi-Agent Orchestration

Execute complex development workflows using APEX's intelligent agent swarm with quality-gated refinement loops.

## Command Syntax

```bash
/apex-swarm [workflow-type] [target] [options]
```

## Workflow Types

### 1. `develop` - Full Development Pipeline

Complete development from idea to production-ready code with tests and documentation.

```bash
/apex-swarm develop "ghost recording system for racing game" --quality 90 --iterations 3
```

**Chain Pattern**:

```
First use the spec-analyst sub agent to analyze requirements for $FEATURE,
then use the calvin-game-designer sub agent to design game mechanics,
then use the alice-bevy-expert sub agent to design ECS architecture,
then use the jon-game-engineer sub agent to implement in Bevy,
then use the spec-validator sub agent to score implementation quality,
then if score ≥$QUALITY% use the kent-test-engineer sub agent to create comprehensive tests,
otherwise first use the spec-analyst sub agent to refine based on validation feedback and repeat chain (max $ITERATIONS times).
```

### 2. `optimize` - Performance Optimization Pipeline

Analyze and optimize existing code for maximum performance.

```bash
/apex-swarm optimize "./src/systems/render.rs" --target-improvement 50 --profile gpu
```

**Chain Pattern**:

```
First use the jon-game-engineer sub agent to profile $TARGET for $PROFILE bottlenecks,
then use the gjengset-rust-expert sub agent to eliminate allocations in hot paths,
then use the alice-bevy-expert sub agent to optimize ECS query patterns,
then use the carmack-rendering-expert sub agent to optimize GPU utilization,
then use the kent-test-engineer sub agent to validate performance improvements,
then if improvement ≥$TARGET_IMPROVEMENT% use the spec-orchestrator sub agent to document optimizations,
otherwise first use the gjengset-rust-expert sub agent to apply more aggressive optimizations and repeat.
```

### 3. `review` - Comprehensive Code Review

Multi-perspective review with architectural, performance, and accessibility analysis.

```bash
/apex-swarm review "./src" --depth full --accessibility AAA
```

**Chain Pattern**:

```
First use the alice-bevy-expert sub agent to review ECS patterns in $TARGET,
then use the gjengset-rust-expert sub agent to audit memory allocations,
then use the casey-gameplay-engineer sub agent to verify determinism,
then use the ian-accessibility-expert sub agent to check $ACCESSIBILITY compliance,
then use the kent-test-engineer sub agent to assess test coverage,
then use the spec-orchestrator sub agent to synthesize all findings into comprehensive report.
```

### 4. `document` - Documentation Generation

Create learning-optimized documentation with examples and exercises.

```bash
/apex-swarm document "Bevy ECS tutorial series" --audience beginner --validate true
```

**Chain Pattern**:

```
First use the natasha-learning-scientist sub agent to design learning progression for $TOPIC,
then use the marcus-technical-writer sub agent to write educational content,
then use the jon-game-engineer sub agent to create working code examples,
then use the steve-technical-editor sub agent to edit for clarity and consistency,
then use the tom-docs-qa-engineer sub agent to validate all code snippets compile,
then use the amy-ux-researcher sub agent to evaluate learning effectiveness,
then if effectiveness ≥85% use the tim-editor-in-chief sub agent to ensure series cohesion,
otherwise first use the marcus-technical-writer sub agent to revise based on UX feedback and repeat.
```

### 5. `design` - Game Feature Design

Design complete game features with mechanics, narrative, and atmosphere.

```bash
/apex-swarm design "roguelike dungeon system" --style "dark fantasy" --complexity medium
```

**Chain Pattern**:

```
First use the calvin-game-designer sub agent to design core mechanics for $FEATURE with $COMPLEXITY complexity,
then use the adam-narrative-designer sub agent to create $STYLE narrative framework,
then use the damien-lighting-designer sub agent to design atmospheric lighting,
then use the swink-game-feel-designer sub agent to define responsive feedback systems,
then use the spec-architect sub agent to create technical architecture,
then use the spec-validator sub agent to evaluate design coherence,
then if coherence ≥90% use the spec-orchestrator sub agent to compile design document,
otherwise first use the calvin-game-designer sub agent to iterate based on feedback.
```

### 6. `test` - Test Suite Generation

Create comprehensive test suites with deterministic validation.

```bash
/apex-swarm test "./src/systems" --coverage 95 --deterministic true
```

**Chain Pattern**:

```
First use the kent-test-engineer sub agent to analyze $TARGET for test requirements,
then use the casey-gameplay-engineer sub agent to design deterministic test scenarios,
then use the kent-test-engineer sub agent to generate unit tests with $COVERAGE% coverage,
then use the spec-validator sub agent to score test quality,
then if score ≥90% use the tom-docs-qa-engineer sub agent to document test cases,
otherwise first use the kent-test-engineer sub agent to add missing test cases and repeat.
```

### 7. `accessibility` - Accessibility Audit & Enhancement

Comprehensive accessibility review and implementation.

```bash
/apex-swarm accessibility "./src/ui" --standard WCAG3 --fix true
```

**Chain Pattern**:

```
First use the ian-accessibility-expert sub agent to audit $TARGET against $STANDARD,
then use the torrey-ux-writer sub agent to improve UI text and error messages,
then use the carmack-rendering-expert sub agent to implement visual accessibility features,
then use the jon-game-engineer sub agent to implement accessibility fixes if $FIX is true,
then use the ian-accessibility-expert sub agent to validate improvements,
then use the spec-orchestrator sub agent to generate accessibility report.
```

### 8. `recursive` - Recursive Deep Analysis

Use APEX recursively for extremely complex tasks.

```bash
/apex-swarm recursive "complete game engine refactor" --max-depth 3
```

**Chain Pattern**:

```
First use the apex-orchestrator-v2 sub agent to decompose $TASK into subtasks,
then use the apex-orchestrator-v2 sub agent to handle critical path subtask,
then use the apex-orchestrator-v2 sub agent to handle secondary subtasks,
then use the spec-orchestrator sub agent to synthesize all outputs,
then use the spec-validator sub agent to score overall completeness,
then if completeness <85% and depth <$MAX_DEPTH recurse with remaining tasks.
```

## Options

### Global Options

- `--quality [0-100]` - Set quality gate threshold (default: 85)
- `--iterations [1-5]` - Maximum refinement loops (default: 3)
- `--parallel [true/false]` - Enable parallel execution where possible
- `--verbose [true/false]` - Detailed progress reporting
- `--checkpoint [true/false]` - Save progress at each phase

### Quality Gates

- `--gate-planning [0-100]` - Planning phase threshold (default: 90)
- `--gate-development [0-100]` - Development phase threshold (default: 85)
- `--gate-validation [0-100]` - Validation phase threshold (default: 95)

### Performance Options

- `--profile [cpu/gpu/memory/all]` - Profiling focus
- `--target-fps [30/60/120/144]` - Target frame rate
- `--optimization [safe/moderate/aggressive]` - Optimization level

### Documentation Options

- `--audience [beginner/intermediate/advanced/expert]`
- `--format [markdown/html/pdf/jupyter]`
- `--examples [minimal/moderate/comprehensive]`
- `--exercises [true/false]`

## Examples

### Example 1: Implement Complex Feature

```bash
/apex-swarm develop "multiplayer lobby system with matchmaking" \
  --quality 95 \
  --iterations 3 \
  --parallel true
```

### Example 2: Optimize Critical Path

```bash
/apex-swarm optimize "./src/systems/physics.rs" \
  --target-improvement 75 \
  --profile cpu \
  --optimization aggressive
```

### Example 3: Create Tutorial Series

```bash
/apex-swarm document "Complete Bevy Game Development Course" \
  --audience beginner \
  --examples comprehensive \
  --exercises true \
  --validate true
```

### Example 4: Design Game System

```bash
/apex-swarm design "procedural magic system with elemental combinations" \
  --style "whimsical fantasy" \
  --complexity high
```

### Example 5: Comprehensive Review

```bash
/apex-swarm review "./game/src" \
  --depth full \
  --accessibility AAA \
  --performance true
```

### Example 6: Recursive Refactor

```bash
/apex-swarm recursive "migrate entire codebase to Bevy 0.16" \
  --max-depth 2 \
  --checkpoint true \
  --verbose true
```

## Workflow Execution Process

### Phase 1: Initialization

```
1. Parse command and options
2. Validate target exists (if applicable)
3. Initialize quality gates
4. Prepare checkpoint system
```

### Phase 2: Chain Construction

```
1. Select appropriate workflow template
2. Inject parameters into chain
3. Add conditional branches
4. Set up iteration counters
```

### Phase 3: Execution

```
1. Execute sub-agent chain
2. Monitor quality gates
3. Handle conditional branches
4. Manage iteration loops
5. Save checkpoints
```

### Phase 4: Synthesis

```
1. Collect all agent outputs
2. Validate quality metrics
3. Generate final report
4. Save artifacts
```

## Output Structure

```
/apex-swarm-output/
├── reports/
│   ├── orchestration-report.md
│   ├── quality-metrics.json
│   └── agent-contributions.md
├── artifacts/
│   ├── code/
│   ├── tests/
│   ├── docs/
│   └── designs/
├── checkpoints/
│   ├── phase-1-complete.json
│   ├── phase-2-complete.json
│   └── phase-3-complete.json
└── logs/
    ├── execution.log
    ├── errors.log
    └── performance.log
```

## Quality Metrics

Each workflow tracks:

- **Completeness**: Task requirements fulfilled (0-100%)
- **Quality**: Code/content quality score (0-100%)
- **Performance**: Optimization targets met (0-100%)
- **Coverage**: Test/documentation coverage (0-100%)
- **Accessibility**: Standards compliance (0-100%)

## Best Practices

1. **Start Simple**: Begin with lower quality thresholds and increase gradually
2. **Use Checkpoints**: Enable for long-running workflows
3. **Monitor Iterations**: If hitting max iterations, consider adjusting approach
4. **Combine Workflows**: Chain multiple commands for complex projects
5. **Review Reports**: Always examine the orchestration report for insights

## Troubleshooting

### Common Issues

**Quality Gate Failures**:

- Lower threshold temporarily
- Increase max iterations
- Review validation criteria

**Performance Bottlenecks**:

- Enable parallel execution
- Use checkpoints to resume
- Break into smaller workflows

**Agent Conflicts**:

- Check agent compatibility
- Adjust execution order
- Use explicit dependencies

## Advanced Usage

### Custom Chains

Create custom sub-agent chains by directly specifying the pattern:

```bash
/apex-swarm custom "First use alice-bevy-expert sub agent to review, \
  then use gjengset-rust-expert sub agent to optimize, \
  then use kent-test-engineer sub agent to validate"
```

### Pipeline Composition

Combine multiple workflows:

```bash
/apex-swarm develop "feature" && \
/apex-swarm optimize "./src" && \
/apex-swarm test "./src" && \
/apex-swarm document "feature"
```

### Conditional Execution

Use shell conditionals with quality metrics:

```bash
if /apex-swarm review "./src" --quality 90; then
  /apex-swarm optimize "./src"
else
  /apex-swarm refactor "./src"
fi
```

---

Remember: The power of APEX lies in intelligent orchestration. Use quality gates to ensure excellence, iterations to
refine solutions, and the full swarm's expertise to achieve outcomes beyond individual capabilities.