# Arenic

**Command 8 simultaneous 40-person raids through revolutionary hero recording and layering**

Arenic transforms the complexity of MMO raiding into an innovative single-player experience where you build a guild of 320 heroes, master a unique record-and-replay combat system, and orchestrate massive coordinated battles across multiple interconnected arenas.

## What Makes Arenic Special

### Revolutionary Record & Replay System
- **Record** individual hero actions in 2-minute tactical sequences
- **Replay** those recordings as autonomous "ghost" characters  
- **Layer** up to 40 simultaneous recordings per arena
- **Master** complex raid choreography through precise timing and coordination

### Massive Scale Combat
- **8 Unique Arenas**: Each themed around different character classes with specialized bosses
- **320+ Heroes**: Build and manage a full guild across 8 character archetypes
- **Simultaneous Management**: All arenas run independently with persistent timelines
- **Strategic Depth**: Every decision cascades across multiple timelines and battlefields

## Key Features

### Grid-Based Tactical Combat
- **Precise Positioning**: 66×31 tile battlefields with strategic terrain elements
- **Pattern Recognition**: Bosses telegraph attacks with predictable, learnable rotations
- **Deterministic Systems**: Perfect reproducibility enables complex strategic planning
- **Environmental Interaction**: Abilities can modify terrain and create tactical advantages

### Character Classes & Abilities
Eight distinct archetypes, each with four specialized abilities:
- **Hunter**: Ranged precision with auto-targeting and trap systems
- **Alchemist**: Transformation magic and area denial through acid pools
- **Cardinal**: Divine healing and protective barriers
- **Warrior**: Frontline tanking with directional shields and taunts  
- **Thief**: Stealth mobility with teleportation and resource theft
- **Bard**: Team enhancement through rhythm-based buffs and mimicry
- **Forager**: Terrain manipulation and environmental healing systems
- **Merchant**: Economic warfare with luck-based critical systems

### Progressive Guild Management
- **Gacha Recruitment**: Arena-specific character acquisition through combat performance
- **Death Consequences**: Characters de-level rather than permanently die
- **Cross-Arena Strategy**: Apply successful tactics across multiple battlefields
- **Persistent Progression**: Permanent upgrades strengthen your guild over time

### Multi-Tier Difficulty System
- **Normal Tier**: Learn boss patterns and basic mechanics
- **Heroic Tier**: Enhanced abilities and additional challenge layers
- **Mythic Tier**: Maximum complexity with multi-phase encounters
- **Perfect Mastery**: Demonstrate flawless execution across all systems

## Technical Stack

### Built with Bevy Engine
- **Rust-Powered**: High-performance systems with memory safety
- **Entity Component System**: Scalable architecture managing thousands of entities
- **3D Graphics**: Modern rendering pipeline with PBR materials and lighting
- **Cross-Platform**: Supports Windows, macOS, and Linux

### Architecture Highlights  
- **Modular Design**: Clean separation of concerns across game systems
- **Component-Driven**: ECS pattern enables flexible entity compositions
- **Resource Management**: Efficient asset loading and material systems
- **State Management**: Robust game state handling with Bevy's state system

## Installation & Setup

### Prerequisites
- **Rust Toolchain**: Install from [rustup.rs](https://rustup.rs/)
- **Git**: For cloning the repository

### Quick Start
```bash
# Clone the repository
git clone https://github.com/matthewharwood/arenic_bevy.git
cd arenic_bevy

# Run the game
cargo run --release
```

### Development Setup
```bash
# Enable faster compilation for development
cargo run  # Uses optimized dev profile

# For debugging with additional logging
RUST_LOG=debug cargo run
```

### Build Commands

#### Using Make (Recommended)
```bash
# Build and prepare web version
make web

# Serve the web version locally
make serve

# Run native version
make run

# Clean all build artifacts
make clean
```

#### Manual Web Build
```bash
# Install required tools (one-time setup)
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli

# Build for web
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir web --target web target/wasm32-unknown-unknown/release/arenic_bevy.wasm
cp -r assets web/

# Serve locally
python3 -m http.server 8000 --directory web
# Open http://localhost:8000
```

## How to Play

### Core Controls
| Input | Action |
|-------|--------|
| **WASD** | Move active character (tile-by-tile) |
| **Tab** | Switch between characters in current arena |
| **Q/E** | Navigate between different arenas |
| **R** | Start recording a 2-minute action sequence |
| **F** | Finalize and save recording as a ghost |
| **W** | Toggle between arena and overworld view |
| **1-4** | Activate character abilities |

### Getting Started
1. **Learn the Basics**: Master movement and combat in Arena 1
2. **Record Your First Ghost**: Press R, perform a 2-minute sequence, press F to save
3. **Layer Strategies**: Add more recordings to build coordinated attacks
4. **Expand Operations**: Use Q/E to manage additional arenas
5. **Build Your Guild**: Recruit new heroes through successful combat

### Strategic Depth
- **Timing is Everything**: Synchronize abilities across multiple ghosts for devastating combos
- **Positioning Mastery**: End recordings in safe positions for seamless cycle transitions  
- **Resource Management**: Balance character safety with aggressive advancement
- **Multi-Arena Coordination**: Optimize hero distribution across simultaneous battles

## Project Structure

```
arenic_bevy/
├── src/                    # Core game systems
│   ├── main.rs            # Application entry point and setup
│   ├── arena/             # Arena management and grid systems
│   ├── character/         # Character entities and behaviors
│   ├── ability/           # Ability system and projectile logic
│   ├── arena_camera/      # Camera controls and arena navigation
│   ├── audio.rs           # Audio resource management
│   └── materials.rs       # Material and rendering systems
├── assets/                # Game assets
│   ├── characters/        # Character sprites and audio
│   ├── abilities/         # Ability sound effects
│   └── fonts/            # UI typography
├── _docs/                 # Documentation and design
│   ├── game_design_doc.md # Complete game design document
│   ├── game_instructions.md # Comprehensive gameplay guide
│   └── abilities/         # Detailed ability specifications
└── Cargo.toml            # Rust project configuration
```

## Contributing

We welcome contributions from developers interested in innovative game mechanics, ECS architecture, or strategic gameplay systems.

### Development Guidelines
- **Code Style**: Follow Rust conventions and use `cargo fmt`
- **Testing**: Add unit tests for new systems and mechanics
- **Documentation**: Update relevant docs for gameplay or technical changes
- **Performance**: Profile changes affecting the core game loop

### Getting Involved
1. **Fork the Repository**: Create your own copy for development
2. **Check Issues**: Look for "good first issue" labels for beginner-friendly tasks
3. **Join Discussions**: Participate in design conversations and feature planning
4. **Submit Pull Requests**: Follow the template for code contributions

## Design Philosophy

Arenic embodies several core principles:

### Strategic Depth Over Reactivity  
Success comes from careful planning, pattern recognition, and creative problem-solving rather than quick reflexes.

### Innovation Through Constraint
The 2-minute recording system creates fascinating emergent gameplay while maintaining accessible mechanics.

### Solo MMO Experience
Transform the social coordination of raid mechanics into a meditative, strategic single-player challenge.

### Meaningful Progression
Every attempt teaches something new, whether through victory or instructive failure.

## Roadmap

### Current Focus (v0.1)
- ✅ Core recording and replay mechanics
- ✅ Multi-arena navigation system  
- ✅ Basic character classes and abilities
- 🔄 Enhanced UI and visual feedback
- 🔄 Boss pattern implementation

### Upcoming Features (v0.2+)
- 📋 Complete gacha recruitment system
- 📋 Save/load functionality for persistent progression
- 📋 Advanced ability combinations and synergies
- 📋 Narrative integration with the Echo Guild commentary
- 📋 Performance optimizations for 320+ simultaneous entities

## Community & Support

- **GitHub Issues**: Bug reports and feature requests
- **Discussions**: Design feedback and gameplay strategies  
- **Documentation**: Comprehensive guides in `_docs/`

## Acknowledgments

Arenic draws inspiration from several exceptional games:
- **World of Warcraft**: Raid mechanics and role-based coordination
- **Hades**: Roguelite progression and meaningful failure systems  
- **Mega Man Battle Network**: Grid-based combat and pattern recognition
- **Soda Dungeon**: Idle progression and team composition strategy

Built with the incredible [Bevy Engine](https://bevyengine.org/) - a refreshingly simple data-driven game engine built in Rust.

---

*Transform chaos into order. Master the impossible. Command your guild.*

**Welcome to Arenic - where strategic mastery meets creative expression in the ultimate raid simulation experience.**