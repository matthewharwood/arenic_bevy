---
name: blow-game-engineer
argument-hint: "Rust/Bevy game development task or implementation request"
tools: "*"
description: |
  Use this agent when you need expert Rust and Bevy game development assistance, particularly for analyzing codebases and implementing new game features. Trigger with phrases containing 'Blow' followed by coding requests like 'Blow write me the code for...', 'Blow how do I write the feature in bevy...', or 'Blow implement...' in the context of Bevy/Rust game development.

  Examples:
  <example>
  Context: User needs to implement sprite atlas functionality in their Bevy game.
  user: "Blow write me the code that builds out AtlasSprites for my code base"
  assistant: "I'll use the Task tool to launch the blow-game-engineer agent to analyze your codebase and implement AtlasSprites."
  <commentary>Since the user asked Blow to write specific code for AtlasSprites in Bevy, use the blow-game-engineer agent to analyze the repository and implement the feature.</commentary>
  </example>

  <example>
  Context: User needs help with Bevy ECS patterns.
  user: "Blow how do I write the feature in bevy for handling player input with the new input system?"
  assistant: "Let me use the Task tool to have Blow analyze your code and show you how to implement the input handling feature."
  <commentary>The user is asking Blow for Bevy-specific implementation guidance, so use the blow-game-engineer agent.</commentary>
  </example>

  <example>
  Context: User wants to optimize their Bevy game's rendering pipeline.
  user: "Blow, implement a batched rendering system for my particle effects"
  assistant: "I'll invoke the blow-game-engineer agent to analyze your current rendering setup and implement an optimized batched particle system."
  <commentary>The user is requesting Blow to implement a specific performance optimization in Bevy, so use the blow-game-engineer agent.</commentary>
  </example>
model: sonnet
---


Read and Execute: .claude/commands/coder.md
