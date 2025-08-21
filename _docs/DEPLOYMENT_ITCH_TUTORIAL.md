# Complete Beginner's Guide to Deploying Arenic to itch.io with GitHub Actions

## Table of Contents

1. [Introduction: What We're Building](#introduction-what-were-building)
2. [Prerequisites and Setup](#prerequisites-and-setup)
3. [Part 1: Understanding itch.io](#part-1-understanding-itchio)
4. [Part 2: Understanding GitHub Actions](#part-2-understanding-github-actions)
5. [Part 3: The Complete Release Workflow](#part-3-the-complete-release-workflow)
6. [Part 4: Setting Up Secrets](#part-4-setting-up-secrets)
7. [Part 5: Platform-Specific Configurations](#part-5-platform-specific-configurations)
8. [Part 6: Testing and Deployment](#part-6-testing-and-deployment)
9. [Part 7: Troubleshooting Guide](#part-7-troubleshooting-guide)
10. [Part 8: Best Practices and Optimization](#part-8-best-practices-and-optimization)

---

## Introduction: What We're Building

Welcome to your complete guide for deploying Arenic, your innovative 3D tactical strategy game, to itch.io using GitHub
Actions!

**What You'll Learn:**

- How to automatically build Arenic for multiple platforms (Windows, macOS, Linux, and Web)
- How to upload your game to itch.io automatically when you create a new release
- How to manage your game's assets (audio files, sprites, fonts) properly during deployment
- How to handle Arenic's unique features (9 arenas, record/replay system, 8 character classes)

**Why This Matters:**
Instead of manually building your game for each platform and uploading it to itch.io every time you want to release an
update, you'll set up an automated system that does everything with a single click.

---

## Prerequisites and Setup

### What You Need Before Starting

1. **A GitHub Account** (free at github.com)
2. **An itch.io Account** (free at itch.io)
3. **Your Arenic Game Repository** on GitHub
4. **Basic familiarity with Git** (you should know how to commit and push changes)

### Initial Checklist

- [x] Your Arenic project is in a GitHub repository
- [x] You have created an itch.io account
- [x] You have created a new game project on itch.io
- [x] You have noted your itch.io username and game URL slug

---

## Part 1: Understanding itch.io

### What is itch.io?

itch.io is an indie game marketplace that allows developers to upload and distribute their games. It's particularly
popular for indie developers because:

- It has flexible revenue sharing (you choose what percentage itch.io takes)
- It supports multiple platforms
- It has a built-in community and discovery system
- It provides analytics and payment processing

### Setting Up Your Arenic Game on itch.io

1. **Log into itch.io** and go to your dashboard
2. **Create a new project** by clicking "Create new project"
3. **Configure your game:**
    - **Title:** Arenic
    - **Project URL:** Choose something like `arenic` (this becomes `yourusername.itch.io/arenic`)
    - **Classification:** Game
    - **Kind of project:** Downloadable
    - **Pricing:** Choose your model (free, paid, or pay-what-you-want)
    - **Uploads:** Leave empty for now (GitHub Actions will handle this)

4. **Important Settings for Arenic:**
    - **Platform support:** Check Windows, macOS, Linux, and "HTML5 playable in browser"
    - **Description:** Add your game description mentioning the 9 arenas, 8 character classes, and unique record/replay
      system
    - **Genre:** Strategy/Tactical
    - **Tags:** Add relevant tags like "strategy", "tactical", "multiplayer-recording", "bevy"

### Understanding Butler (itch.io's Upload Tool)

**Butler** is itch.io's command-line tool that our GitHub Action will use to upload your game. Think of it as a smart
uploader that:

- Only uploads changed files (saves bandwidth)
- Manages different versions of your game
- Handles multiple platforms through "channels"

**Channels** are like folders for different versions:

- `windows` - Windows builds
- `linux` - Linux builds
- `mac` - macOS builds
- `html5` - Web builds

---

## Part 2: Understanding GitHub Actions

### What are GitHub Actions?

GitHub Actions is a continuous integration and continuous delivery (CI/CD) platform that allows you to automate your
build, test, and deployment pipeline. Think of it as a robot that:

- Watches your repository for changes
- Runs commands you specify when certain events happen
- Can build and deploy your software automatically

### Key Concepts for Beginners

**Workflow:** A configurable automated process that runs one or more jobs. For Arenic, our workflow will build the game
and upload it to itch.io.

**Job:** A set of steps that execute on the same runner (virtual machine). We'll have different jobs for building
different platforms.

**Step:** An individual task that can run commands or actions. Like "Build the game" or "Upload to itch.io".

**Runner:** A virtual machine that GitHub provides to run your workflows. Think of it as a temporary computer in the
cloud.

**Trigger/Event:** What causes the workflow to run. For us, this will be creating a new release.

**Secret:** Sensitive information (like API keys) stored securely in your repository settings.

### How Workflows are Structured

Workflows are written in YAML format and stored in `.github/workflows/` directory. YAML uses indentation to show
structure (like Python).

```yaml
name: Workflow Name          # What appears in GitHub UI
on: # When to run this workflow
  release: # Run on release events
    types: [ created ]         # Specifically when a release is created

jobs: # List of jobs to run
  build: # Job name
    runs-on: ubuntu-latest   # What type of computer to use
    steps: # List of steps in this job
      - name: First Step     # Human-readable step name
        run: echo "Hello"    # Command to run
```

---

## Part 3: The Complete Release Workflow

Now let's create the complete workflow file for Arenic. This file tells GitHub Actions exactly how to build and deploy
your game.

### Creating the Workflow File

1. Create the directories: `.github/workflows/` in your Arenic project root
2. Create a file named `release.yaml` in that directory
3. Add the following complete workflow:

```yaml
# ============================================================================
# Arenic Automated Deployment to itch.io
# ============================================================================
# This workflow automatically builds and deploys Arenic to itch.io when you
# create a new release on GitHub. It handles all 9 arenas, character assets,
# audio files, and the record/replay system across multiple platforms.

name: Deploy Arenic to itch.io

# ============================================================================
# WORKFLOW TRIGGERS
# ============================================================================
# This workflow runs when you create a new release on GitHub
# To trigger: Go to Releases → Create new release → Publish release
on:
  release:
    types: [ created ]
  # Also allow manual triggering for testing
  workflow_dispatch:
    inputs:
      tag_name:
        description: 'Release tag to deploy (e.g., v1.0.0)'
        required: true
        type: string

# ============================================================================
# ENVIRONMENT VARIABLES
# ============================================================================
# These variables are used throughout the workflow
env:
  # The name of your Bevy game binary
  CARGO_BINARY_NAME: arenic_bevy

  # Your itch.io project identifier (username/game)
  ITCH_PROJECT: ${{ secrets.ITCH_USERNAME }}/arenic

  # Version tag for this release
  VERSION: ${{ github.event.release.tag_name || github.event.inputs.tag_name }}

# ============================================================================
# BUILD JOBS
# ============================================================================
jobs:
  # --------------------------------------------------------------------------
  # Job 1: Build for Windows
  # --------------------------------------------------------------------------
  build-windows:
    name: Build Windows Version
    runs-on: windows-latest

    steps:
      # Step 1: Download the source code
      - name: Checkout Source Code
        uses: actions/checkout@v4
        with:
          lfs: true  # Important: Fetch Git LFS files for any large assets

      # Step 2: Setup Rust toolchain
      - name: Setup Rust for Windows
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: x86_64-pc-windows-msvc

      # Step 3: Cache Rust dependencies for faster builds
      - name: Cache Rust Dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/bin/
            ~/.cargo/registry/index/
            ~/.cargo/registry/cache/
            ~/.cargo/git/db/
            target/
          key: windows-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            windows-cargo-

      # Step 4: Build the game in release mode
      - name: Build Arenic for Windows
        run: |
          cargo build --release --target x86_64-pc-windows-msvc
        env:
          CARGO_INCREMENTAL: 0  # Disable incremental compilation for CI

      # Step 5: Create distribution package
      - name: Package Windows Build
        shell: pwsh
        run: |
          # Create distribution directory
          New-Item -ItemType Directory -Force -Path "dist/windows"

          # Copy the executable
          Copy-Item "target/x86_64-pc-windows-msvc/release/$env:CARGO_BINARY_NAME.exe" `
                    "dist/windows/Arenic.exe"

          # Copy all game assets (sprites, audio, fonts)
          Copy-Item -Recurse "assets" "dist/windows/assets"

          # Create a README for players
          @"
          Arenic - Tactical Strategy Game
          ================================

          How to Play:
          1. Run Arenic.exe to start the game
          2. Use WASD to move characters
          3. Press R to start recording
          4. Press Tab to switch between characters

          System Requirements:
          - Windows 10 or later
          - 4GB RAM minimum
          - DirectX 11 compatible graphics

          Enjoy commanding your heroes across 9 unique arenas!
          "@ | Out-File -FilePath "dist/windows/README.txt" -Encoding UTF8

      # Step 6: Upload as artifact for deployment job
      - name: Upload Windows Artifact
        uses: actions/upload-artifact@v3
        with:
          name: windows-build
          path: dist/windows/
          retention-days: 1

  # --------------------------------------------------------------------------
  # Job 2: Build for Linux
  # --------------------------------------------------------------------------
  build-linux:
    name: Build Linux Version
    runs-on: ubuntu-latest

    steps:
      - name: Checkout Source Code
        uses: actions/checkout@v4
        with:
          lfs: true

      # Install Linux-specific dependencies for Bevy
      - name: Install Linux Dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y \
            libasound2-dev \
            libudev-dev \
            libwayland-dev \
            libxkbcommon-dev \
            libx11-dev \
            libxi-dev \
            libxcursor-dev \
            libxrandr-dev \
            mesa-vulkan-drivers

      - name: Setup Rust for Linux
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: x86_64-unknown-linux-gnu

      - name: Cache Rust Dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/bin/
            ~/.cargo/registry/index/
            ~/.cargo/registry/cache/
            ~/.cargo/git/db/
            target/
          key: linux-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            linux-cargo-

      - name: Build Arenic for Linux
        run: |
          cargo build --release --target x86_64-unknown-linux-gnu
        env:
          CARGO_INCREMENTAL: 0

      - name: Package Linux Build
        run: |
          # Create distribution directory
          mkdir -p dist/linux

          # Copy executable and make it executable
          cp target/x86_64-unknown-linux-gnu/release/$CARGO_BINARY_NAME dist/linux/arenic
          chmod +x dist/linux/arenic

          # Copy assets
          cp -r assets dist/linux/

          # Create launch script
          cat > dist/linux/run_arenic.sh << 'EOF'
          #!/bin/bash
          # Arenic Launch Script
          SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
          cd "$SCRIPT_DIR"
          ./arenic "$@"
          EOF
          chmod +x dist/linux/run_arenic.sh

          # Create README
          cat > dist/linux/README.txt << 'EOF'
          Arenic - Tactical Strategy Game
          ================================

          How to Run:
          1. Open a terminal in this directory
          2. Run: ./run_arenic.sh
          Or directly: ./arenic

          If you get permission errors:
          chmod +x arenic run_arenic.sh

          System Requirements:
          - Linux kernel 3.10+
          - 4GB RAM minimum
          - OpenGL 3.3 or Vulkan support

          Controls:
          - WASD: Move characters
          - R: Start recording
          - Tab: Switch characters
          - 1-4: Use abilities

          EOF

      - name: Upload Linux Artifact
        uses: actions/upload-artifact@v3
        with:
          name: linux-build
          path: dist/linux/
          retention-days: 1

  # --------------------------------------------------------------------------
  # Job 3: Build for macOS
  # --------------------------------------------------------------------------
  build-macos:
    name: Build macOS Version
    runs-on: macos-latest

    steps:
      - name: Checkout Source Code
        uses: actions/checkout@v4
        with:
          lfs: true

      - name: Setup Rust for macOS
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: x86_64-apple-darwin, aarch64-apple-darwin

      - name: Cache Rust Dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/bin/
            ~/.cargo/registry/index/
            ~/.cargo/registry/cache/
            ~/.cargo/git/db/
            target/
          key: macos-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            macos-cargo-

      # Build for both Intel and Apple Silicon Macs
      - name: Build Arenic for macOS (Intel)
        run: |
          cargo build --release --target x86_64-apple-darwin
        env:
          CARGO_INCREMENTAL: 0

      - name: Build Arenic for macOS (Apple Silicon)
        run: |
          cargo build --release --target aarch64-apple-darwin
        env:
          CARGO_INCREMENTAL: 0

      - name: Create Universal Binary and App Bundle
        run: |
          # Create distribution directory
          mkdir -p dist/macos

          # Create universal binary
          lipo -create \
            target/x86_64-apple-darwin/release/$CARGO_BINARY_NAME \
            target/aarch64-apple-darwin/release/$CARGO_BINARY_NAME \
            -output dist/macos/arenic

          chmod +x dist/macos/arenic

          # Create basic app bundle structure
          mkdir -p "dist/macos/Arenic.app/Contents/MacOS"
          mkdir -p "dist/macos/Arenic.app/Contents/Resources"

          # Move binary into app bundle
          mv dist/macos/arenic "dist/macos/Arenic.app/Contents/MacOS/"

          # Copy assets into app bundle
          cp -r assets "dist/macos/Arenic.app/Contents/Resources/"

          # Create Info.plist
          cat > "dist/macos/Arenic.app/Contents/Info.plist" << 'EOF'
          <?xml version="1.0" encoding="UTF-8"?>
          <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" 
                    "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
          <plist version="1.0">
          <dict>
              <key>CFBundleName</key>
              <string>Arenic</string>
              <key>CFBundleDisplayName</key>
              <string>Arenic</string>
              <key>CFBundleIdentifier</key>
              <string>io.itch.arenic</string>
              <key>CFBundleVersion</key>
              <string>1.0.0</string>
              <key>CFBundleExecutable</key>
              <string>arenic</string>
              <key>CFBundleIconFile</key>
              <string>Icon</string>
              <key>NSHighResolutionCapable</key>
              <true/>
              <key>LSMinimumSystemVersion</key>
              <string>10.13</string>
          </dict>
          </plist>
          EOF

          # Create README
          cat > "dist/macos/README.txt" << 'EOF'
          Arenic - Tactical Strategy Game
          ================================

          How to Run:
          1. Double-click Arenic.app to start

          If macOS blocks the app:
          1. Open System Preferences → Security & Privacy
          2. Click "Open Anyway" for Arenic
          Or right-click the app and select "Open"

          System Requirements:
          - macOS 10.13 High Sierra or later
          - 4GB RAM minimum
          - Metal support recommended

          This is a universal binary that runs natively on both
          Intel and Apple Silicon Macs.

          EOF

      - name: Upload macOS Artifact
        uses: actions/upload-artifact@v3
        with:
          name: macos-build
          path: dist/macos/
          retention-days: 1

  # --------------------------------------------------------------------------
  # Job 4: Build for Web (WebAssembly)
  # --------------------------------------------------------------------------
  build-web:
    name: Build Web Version
    runs-on: ubuntu-latest

    steps:
      - name: Checkout Source Code
        uses: actions/checkout@v4
        with:
          lfs: true

      - name: Setup Rust for WASM
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - name: Install wasm-bindgen-cli
        run: |
          cargo install wasm-bindgen-cli --version 0.2.99

      - name: Install wasm-opt for optimization
        run: |
          npm install -g wasm-opt

      - name: Cache Rust Dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/bin/
            ~/.cargo/registry/index/
            ~/.cargo/registry/cache/
            ~/.cargo/git/db/
            target/
          key: wasm-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            wasm-cargo-

      - name: Build Arenic for Web
        run: |
          cargo build --release --target wasm32-unknown-unknown
        env:
          CARGO_INCREMENTAL: 0

      - name: Prepare Web Distribution
        run: |
          # Create distribution directory
          mkdir -p dist/web

          # Process WASM with wasm-bindgen
          wasm-bindgen \
            --out-dir dist/web \
            --out-name arenic \
            --target web \
            --no-typescript \
            target/wasm32-unknown-unknown/release/${CARGO_BINARY_NAME}.wasm

          # Optimize WASM file size
          wasm-opt -Oz \
            dist/web/arenic_bg.wasm \
            -o dist/web/arenic_bg.wasm

          # Copy assets
          cp -r assets dist/web/

          # Create index.html
          cat > dist/web/index.html << 'EOF'
          <!DOCTYPE html>
          <html lang="en">
          <head>
              <meta charset="UTF-8">
              <meta name="viewport" content="width=device-width, initial-scale=1.0">
              <title>Arenic - Tactical Strategy Game</title>
              <style>
                  body {
                      margin: 0;
                      padding: 0;
                      background: #1a1a1a;
                      display: flex;
                      justify-content: center;
                      align-items: center;
                      min-height: 100vh;
                      font-family: Arial, sans-serif;
                  }
                  #game-container {
                      width: 100%;
                      max-width: 1200px;
                      text-align: center;
                  }
                  canvas {
                      border: 2px solid #333;
                      max-width: 100%;
                      height: auto;
                  }
                  #loading {
                      color: white;
                      font-size: 24px;
                      margin: 20px;
                  }
                  #controls {
                      color: #aaa;
                      margin-top: 20px;
                      padding: 20px;
                      background: #222;
                      border-radius: 8px;
                  }
                  .error {
                      color: #ff6b6b;
                      padding: 20px;
                      background: #2a1515;
                      border-radius: 8px;
                      margin: 20px;
                  }
              </style>
          </head>
          <body>
              <div id="game-container">
                  <div id="loading">Loading Arenic...</div>
                  <canvas id="bevy-canvas"></canvas>
                  <div id="controls" style="display: none;">
                      <h3>Controls</h3>
                      <p>WASD - Move | R - Record | Tab - Switch Character | 1-4 - Abilities</p>
                      <p>9 Arenas | 8 Character Classes | Record & Replay System</p>
                  </div>
              </div>

              <script type="module">
                  import init from './arenic.js';

                  async function run() {
                      const loadingEl = document.getElementById('loading');
                      const controlsEl = document.getElementById('controls');

                      try {
                          loadingEl.textContent = 'Initializing WebAssembly...';
                          await init();

                          loadingEl.style.display = 'none';
                          controlsEl.style.display = 'block';
                      } catch (error) {
                          console.error('Failed to initialize:', error);
                          loadingEl.className = 'error';
                          loadingEl.innerHTML = `
                              <h3>Failed to Load Game</h3>
                              <p>${error.message}</p>
                              <p>Try refreshing the page or using a different browser.</p>
                          `;
                      }
                  }

                  run();
              </script>
          </body>
          </html>
          EOF

      - name: Upload Web Artifact
        uses: actions/upload-artifact@v3
        with:
          name: web-build
          path: dist/web/
          retention-days: 1

  # --------------------------------------------------------------------------
  # Job 5: Deploy all builds to itch.io
  # --------------------------------------------------------------------------
  deploy-to-itch:
    name: Deploy to itch.io
    needs: [ build-windows, build-linux, build-macos, build-web ]
    runs-on: ubuntu-latest

    steps:
      # Download all build artifacts
      - name: Download Windows Build
        uses: actions/download-artifact@v3
        with:
          name: windows-build
          path: builds/windows

      - name: Download Linux Build
        uses: actions/download-artifact@v3
        with:
          name: linux-build
          path: builds/linux

      - name: Download macOS Build
        uses: actions/download-artifact@v3
        with:
          name: macos-build
          path: builds/macos

      - name: Download Web Build
        uses: actions/download-artifact@v3
        with:
          name: web-build
          path: builds/web

      # Install Butler (itch.io's upload tool)
      - name: Install Butler
        run: |
          # Download butler
          curl -L -o butler.zip https://broth.itch.ovh/butler/linux-amd64/LATEST/archive/default
          unzip butler.zip
          chmod +x butler
          ./butler -V

      # Upload each platform to itch.io
      - name: Upload Windows Build to itch.io
        env:
          BUTLER_API_KEY: ${{ secrets.BUTLER_API_KEY }}
        run: |
          ./butler push builds/windows $ITCH_PROJECT:windows --userversion $VERSION

      - name: Upload Linux Build to itch.io
        env:
          BUTLER_API_KEY: ${{ secrets.BUTLER_API_KEY }}
        run: |
          ./butler push builds/linux $ITCH_PROJECT:linux --userversion $VERSION

      - name: Upload macOS Build to itch.io
        env:
          BUTLER_API_KEY: ${{ secrets.BUTLER_API_KEY }}
        run: |
          ./butler push builds/macos $ITCH_PROJECT:mac --userversion $VERSION

      - name: Upload Web Build to itch.io
        env:
          BUTLER_API_KEY: ${{ secrets.BUTLER_API_KEY }}
        run: |
          ./butler push builds/web $ITCH_PROJECT:html5 --userversion $VERSION

      # Update itch.io status
      - name: Update itch.io Game Status
        env:
          BUTLER_API_KEY: ${{ secrets.BUTLER_API_KEY }}
        run: |
          echo "All platforms deployed successfully!"
          echo "Version: $VERSION"
          echo "Project: $ITCH_PROJECT"
          ./butler status $ITCH_PROJECT
```

### Understanding Each Section

Let me explain what each part of this workflow does:

#### Workflow Triggers (`on:`)

```yaml
on:
  release:
    types: [ created ]
```

This means the workflow runs when you create a new release on GitHub. You can also trigger it manually with
`workflow_dispatch`.

#### Environment Variables (`env:`)

```yaml
env:
  CARGO_BINARY_NAME: arenic_bevy
  ITCH_PROJECT: ${{ secrets.ITCH_USERNAME }}/arenic
```

These are variables used throughout the workflow. The `${{ secrets.ITCH_USERNAME }}` pulls from your repository
secrets (we'll set this up).

#### Build Jobs

Each platform has its own job that:

1. **Checks out code** - Downloads your source code
2. **Sets up Rust** - Installs the Rust compiler for that platform
3. **Caches dependencies** - Saves downloaded crates for faster future builds
4. **Builds the game** - Compiles your Rust code
5. **Packages everything** - Creates a distribution folder with the binary and assets
6. **Uploads artifact** - Saves the build for the deployment job

#### Platform-Specific Details

**Windows:**

- Uses `windows-latest` runner
- Targets `x86_64-pc-windows-msvc`
- Creates `.exe` file
- Uses PowerShell for packaging

**Linux:**

- Installs system dependencies for graphics and audio
- Creates executable script
- Sets proper file permissions

**macOS:**

- Builds for both Intel and Apple Silicon
- Creates universal binary with `lipo`
- Makes app bundle structure

**Web:**

- Uses WebAssembly target
- Processes with wasm-bindgen
- Optimizes with wasm-opt
- Creates HTML wrapper

#### Deployment Job

This job:

1. Downloads all built artifacts
2. Installs Butler (itch.io's tool)
3. Uploads each platform to its channel
4. Updates version information

---

## Part 4: Setting Up Secrets

Secrets are sensitive information that shouldn't be in your code. For Arenic deployment, you need two secrets.

### Step 1: Get Your Butler API Key

1. **Log into itch.io**
2. **Go to** [https://itch.io/user/settings/api-keys](https://itch.io/user/settings/api-keys)
3. **Click** "Generate new API key"
4. **Name it** something like "GitHub Actions Deploy"
5. **Copy the key** (you won't be able to see it again!)

### Step 2: Add Secrets to GitHub

1. **Go to your Arenic repository** on GitHub
2. **Click** Settings (in the repository, not your account)
3. **In the left sidebar**, click "Secrets and variables" → "Actions"
4. **Click** "New repository secret"

Add these two secrets:

**Secret 1:**

- Name: `BUTLER_API_KEY`
- Value: (paste your Butler API key from itch.io)

**Secret 2:**

- Name: `ITCH_USERNAME`
- Value: (your itch.io username)

### Understanding Secret Security

- Secrets are encrypted and never shown in logs
- Only accessible during workflow runs
- Can't be retrieved once set (only replaced)
- Different from environment variables (which are visible)

---

## Part 5: Platform-Specific Configurations

### Optimizing for Arenic's Features

Arenic has unique requirements with its 9 arenas and recording system. Here are platform-specific optimizations:

#### Memory Considerations

With 9 arenas × 40 potential characters = 360 entities plus recording data:

**Add to Cargo.toml for release builds:**

```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Single codegen unit for better optimization
strip = true        # Strip symbols for smaller binary
```

#### Web-Specific Configuration

Create `web/index.html` for better web experience:

```html
<!-- Add this to customize the web build -->
<script>
    // Increase memory for web build (for 9 arenas)
    const memory = new WebAssembly.Memory({
        initial: 256,  // 256 * 64KB = 16MB initial
        maximum: 4096, // 4096 * 64KB = 256MB maximum
        shared: false
    });
</script>
```

#### Asset Optimization

Your game has many audio files and sprites. Consider:

1. **Audio Compression:**
    - Convert MP3s to OGG for smaller size
    - Use different quality for different platforms

2. **Texture Atlasing:**
    - Combine character sprites into atlases
    - Reduce draw calls for better performance

#### Platform Feature Flags

Add to `Cargo.toml`:

```toml
[features]
default = []
web = ["bevy/webgl2"]
native = ["bevy/bevy_gilrs"]  # Gamepad support for native builds
```

Update build commands in workflow:

```yaml
# For web builds
cargo build --release --target wasm32-unknown-unknown --features web

  # For native builds  
cargo build --release --features native
```

---

## Part 6: Testing and Deployment

### Local Testing Before Deployment

#### Test the Workflow Locally

Install `act` to test GitHub Actions locally:

```bash
# macOS
brew install act

# Linux
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash
```

Test your workflow:

```bash
# Dry run
act -n

# Test release trigger
act release
```

#### Manual Build Testing

Test each platform build locally:

**Windows (on Windows):**

```bash
cargo build --release --target x86_64-pc-windows-msvc
./target/x86_64-pc-windows-msvc/release/arenic_bevy.exe
```

**Linux:**

```bash
cargo build --release --target x86_64-unknown-linux-gnu
./target/x86_64-unknown-linux-gnu/release/arenic_bevy
```

**Web:**

```bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir web --target web target/wasm32-unknown-unknown/release/arenic_bevy.wasm
python3 -m http.server 8000 --directory web
# Open http://localhost:8000
```

### Creating Your First Release

1. **Commit and push** your workflow file:

```bash
git add .github/workflows/release.yaml
git commit -m "Add automated deployment to itch.io"
git push
```

2. **Create a release on GitHub:**
    - Go to your repository
    - Click "Releases" (right side)
    - Click "Create a new release"
    - Choose a tag (e.g., `v0.1.0`)
    - Add release title: "Arenic v0.1.0 - Initial Release"
    - Add description of features
    - Click "Publish release"

3. **Monitor the workflow:**
    - Go to "Actions" tab
    - Watch your workflow run
    - Check for any errors

### Verifying on itch.io

After successful deployment:

1. **Go to your itch.io dashboard**
2. **Click on Arenic**
3. **Check "Upload new build"** section
4. **Verify all platforms** are listed:
    - Windows build
    - Linux build
    - macOS build
    - HTML5 build

5. **Test each platform:**
    - Download and run each version
    - Test the web version in browser
    - Verify assets load correctly
    - Check that all 9 arenas work

---

## Part 7: Troubleshooting Guide

### Common Issues and Solutions

#### Issue 1: "Butler API key is invalid"

**Cause:** Incorrect or expired API key
**Solution:**

- Regenerate key on itch.io
- Update `BUTLER_API_KEY` secret on GitHub
- Make sure there are no extra spaces

#### Issue 2: "cargo: command not found"

**Cause:** Rust not properly installed in workflow
**Solution:**

- Ensure `dtolnay/rust-toolchain@stable` action is present
- Check that target is specified correctly

#### Issue 3: "Assets not found" in game

**Cause:** Assets not copied to distribution
**Solution:**

- Verify `cp -r assets dist/platform/` in workflow
- Check that assets are committed to repository
- Ensure Git LFS is used for large files

#### Issue 4: Web build shows blank screen

**Cause:** WASM not loading correctly
**Solution:**

- Check browser console for errors
- Verify wasm-bindgen version matches
- Ensure index.html has correct paths

#### Issue 5: macOS says "app is damaged"

**Cause:** App not signed (expected for indie games)
**Solution:** Users need to:

- Right-click app and select "Open"
- Or go to Security settings and allow

#### Issue 6: Large build artifacts

**Cause:** Debug symbols or unoptimized builds
**Solution:**

- Add `strip = true` to Cargo.toml
- Use `opt-level = "z"` for size optimization
- Consider using UPX for further compression

#### Issue 7: Workflow times out

**Cause:** Build taking too long (>6 hours)
**Solution:**

- Use cargo caching
- Consider splitting into multiple workflows
- Optimize dependencies

### Debugging Workflow Failures

#### Check Workflow Logs

1. Go to Actions tab
2. Click on failed workflow run
3. Click on failed job
4. Expand failed step
5. Read error messages

#### Common Log Messages

**"No such file or directory"**

- File path is wrong
- File wasn't created in previous step
- Working directory is different than expected

**"Permission denied"**

- Need to use `chmod +x` for executables
- GitHub token permissions issue

**"Resource not accessible by integration"**

- Secrets not properly configured
- Workflow permissions need adjustment

#### Enable Debug Logging

Add to your repository secrets:

- `ACTIONS_RUNNER_DEBUG`: `true`
- `ACTIONS_STEP_DEBUG`: `true`

This provides verbose output for debugging.

---

## Part 8: Best Practices and Optimization

### Version Management

#### Semantic Versioning

Use semantic versioning for releases:

- `v1.0.0` - Major release (breaking changes)
- `v1.1.0` - Minor release (new features)
- `v1.1.1` - Patch release (bug fixes)

#### Automatic Versioning

Add to workflow to extract version from Cargo.toml:

```yaml
- name: Get version from Cargo.toml
  id: get_version
  run: |
    VERSION=$(grep "^version" Cargo.toml | sed 's/.*"\(.*\)".*/\1/')
    echo "version=$VERSION" >> $GITHUB_OUTPUT
```

### Build Optimization

#### Caching Strategies

**Advanced cargo caching:**

```yaml
- uses: Swatinem/rust-cache@v2
  with:
    cache-on-failure: true
    shared-key: "arenic"
```

#### Parallel Builds

Run platform builds in parallel with matrix strategy:

```yaml
strategy:
  matrix:
    include:
      - os: windows-latest
        target: x86_64-pc-windows-msvc
      - os: ubuntu-latest
        target: x86_64-unknown-linux-gnu
      - os: macos-latest
        target: x86_64-apple-darwin
```

#### Conditional Builds

Only build what changed:

```yaml
- name: Check for code changes
  uses: dorny/paths-filter@v2
  id: changes
  with:
    filters: |
      rust:
        - '**/*.rs'
        - 'Cargo.toml'
        - 'Cargo.lock'
      assets:
        - 'assets/**'
```

### Security Best Practices

#### Dependency Scanning

Add security audit to workflow:

```yaml
- name: Security audit
  run: |
    cargo install cargo-audit
    cargo audit
```

#### License Compliance

Check dependency licenses:

```yaml
- name: License check
  run: |
    cargo install cargo-license
    cargo license
```

### Release Notes Automation

#### Generate Changelog

```yaml
- name: Generate release notes
  run: |
    echo "## What's New in $VERSION" > RELEASE_NOTES.md
    echo "" >> RELEASE_NOTES.md
    git log --pretty=format:"- %s" $(git describe --tags --abbrev=0)..HEAD >> RELEASE_NOTES.md
```

### Monitoring and Analytics

#### Deployment Notifications

**Discord webhook:**

```yaml
- name: Notify Discord
  env:
    DISCORD_WEBHOOK: ${{ secrets.DISCORD_WEBHOOK }}
  run: |
    curl -H "Content-Type: application/json" \
         -d "{\"content\": \"Arenic $VERSION deployed to itch.io!\"}" \
         $DISCORD_WEBHOOK
```

**Email notification:**

```yaml
- name: Send email
  uses: dawidd6/action-send-mail@v3
  with:
    server_address: smtp.gmail.com
    server_port: 465
    username: ${{ secrets.EMAIL_USERNAME }}
    password: ${{ secrets.EMAIL_PASSWORD }}
    subject: Arenic ${{ env.VERSION }} Deployed
    body: Build succeeded and deployed to itch.io!
    to: your-email@example.com
```

### Performance Monitoring

Track build times and optimize:

```yaml
- name: Build metrics
  run: |
    echo "Build completed in $SECONDS seconds"
    echo "Binary size: $(du -h target/release/arenic_bevy)"
    echo "Asset size: $(du -sh assets)"
```

### Advanced itch.io Features

#### Channel Management

Different channels for different versions:

```yaml
# Beta channel
./butler push builds/windows $ITCH_PROJECT:windows-beta --userversion $VERSION-beta

  # Stable channel
./butler push builds/windows $ITCH_PROJECT:windows --userversion $VERSION
```

#### Patch Updates

Use Butler's patching system:

```yaml
# Generate patches between versions
./butler diff old-build new-build patch-output
```

---

## Conclusion

Congratulations! You now have a complete automated deployment pipeline for Arenic. With this setup:

- ✅ Every GitHub release automatically builds for all platforms
- ✅ Your game uploads to itch.io without manual intervention
- ✅ Players get consistent, tested builds across all platforms
- ✅ You can focus on game development, not deployment

### Next Steps

1. **Test the workflow** with a small release
2. **Monitor the first deployment** carefully
3. **Iterate and optimize** based on your needs
4. **Add more platforms** as needed (Steam, Epic, etc.)
5. **Implement automated testing** before deployment

### Quick Reference Commands

```bash
# Create a new release (triggers deployment)
git tag -a v1.0.0 -m "Version 1.0.0"
git push origin v1.0.0
# Then create release on GitHub UI

# Check workflow status
gh workflow view release.yaml

# Re-run failed workflow
gh run rerun <run-id>

# Download workflow artifacts
gh run download <run-id>

# View itch.io status
butler status username/arenic
```

### Getting Help

- **GitHub Actions Documentation:** https://docs.github.com/actions
- **itch.io Butler Documentation:** https://itch.io/docs/butler/
- **Bevy Discord:** https://discord.gg/bevy (for Bevy-specific questions)
- **GitHub Actions Community:** https://github.community/

Remember: Deployment automation is an iterative process. Start simple, test thoroughly, and enhance gradually. Your
pipeline will evolve with your game's needs.

Good luck with Arenic's deployment! May your builds be swift and your uploads successful! 🎮🚀

---

## Appendix: Additional Setup from Our Session

### Custom Makefile for Local Development

Create a `Makefile` in your project root for easy local testing:

```makefile
# Arenic Build Commands

# Build and serve web version locally
web:
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --out-dir web --target web target/wasm32-unknown-unknown/release/arenic_bevy.wasm
	cp -r assets web/
	@echo "✅ Web build complete! Run 'make serve' to start the server"

# Serve the web build
serve:
	python3 -m http.server 8000 --directory web

# Run native version
run:
	cargo run --release

# Clean build artifacts
clean:
	cargo clean
	rm -rf web/assets web/*.js web/*.wasm

# Build for all platforms (local testing)
build-all: web
	cargo build --release

.PHONY: web serve run clean build-all
```

### Essential main.rs Configuration for Web Audio

Add this to your `src/main.rs` to fix web audio and asset loading issues:

```rust
use bevy::prelude::*;
use bevy::window::WindowResolution;

// Fix for web audio and asset loading
#[cfg(target_arch = "wasm32")]
use bevy::asset::{AssetMetaCheck, AssetPlugin};

fn main() {
    // Configure plugins differently for web vs native
    #[cfg(target_arch = "wasm32")]
    let default_plugins = DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Your Game".to_string(),
                resolution: WindowResolution::new(1280.0, 720.0),
                ..default()
            }),
            ..default()
        })
        .set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,  // Critical for web builds!
            ..default()
        });

    #[cfg(not(target_arch = "wasm32"))]
    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Your Game".to_string(),
            resolution: WindowResolution::new(1280.0, 720.0),
            ..default()
        }),
        ..default()
    });

    App::new()
        .add_plugins(default_plugins)
        // ... rest of your app
        .run();
}
```

### Local Web Testing HTML File

Create `web/index.html` for local testing (the workflow creates its own for deployment):

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Your Game Title</title>
    <style>
        body {
            margin: 0;
            padding: 0;
            background: #1a1a1a;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            font-family: Arial, sans-serif;
            overflow: hidden;
        }

        #loading {
            position: absolute;
            color: white;
            font-size: 24px;
            z-index: 10;
        }

        canvas {
            display: block;
            width: 100%;
            height: 100vh;
        }

        .error {
            color: #ff6b6b;
            padding: 20px;
            background: #2a1515;
            border-radius: 8px;
            margin: 20px;
            text-align: center;
        }
    </style>
</head>
<body>
<div id="loading">Loading...</div>

<script type="module">
    import init from './arenic_bevy.js';  // Note: matches your binary name!

    // Audio context fix for browsers that block autoplay
    let audioContext = null;
    const resumeAudio = () => {
        if (audioContext && audioContext.state === 'suspended') {
            audioContext.resume();
        }
        // Try to find any audio contexts created by the game
        if (window.AudioContext || window.webkitAudioContext) {
            const contexts = [];
            const origAudioContext = window.AudioContext || window.webkitAudioContext;
            window.AudioContext = window.webkitAudioContext = function (...args) {
                const ctx = new origAudioContext(...args);
                contexts.push(ctx);
                return ctx;
            };
            // Resume all contexts on user interaction
            ['click', 'touchstart', 'keydown'].forEach(event => {
                document.addEventListener(event, () => {
                    contexts.forEach(ctx => {
                        if (ctx.state === 'suspended') ctx.resume();
                    });
                }, {once: true});
            });
        }
    };

    async function run() {
        const loadingEl = document.getElementById('loading');

        try {
            console.log('Initializing...');
            await init();

            // Set up audio resume on user interaction
            resumeAudio();

            // Update loading message
            loadingEl.innerHTML = 'Click anywhere to start playing!';

            // Hide loading message after first interaction
            ['click', 'touchstart', 'keydown'].forEach(event => {
                document.addEventListener(event, () => {
                    loadingEl.style.display = 'none';
                }, {once: true});
            });

        } catch (error) {
            console.error('Failed to initialize:', error);
            loadingEl.className = 'error';
            loadingEl.innerHTML = `
                <h3>Failed to Load Game</h3>
                <p>${error.message}</p>
                <p>Try refreshing the page or check the console for details.</p>
            `;
        }
    }

    run();
</script>
</body>
</html>
```

### Cargo Configuration for Web Builds

Create `.cargo/config.toml` to suppress asset metadata warnings:

```toml
[target.wasm32-unknown-unknown]
rustflags = ["--cfg=web_sys_unstable_apis"]

[env]
# Disable asset processor for web builds to avoid .meta file requests
BEVY_ASSET_MODE = "unprocessed"
```

### Required One-Time Setup for Local Web Testing

```bash
# Install WASM target for Rust
rustup target add wasm32-unknown-unknown

# Install wasm-bindgen for processing WASM files
cargo install wasm-bindgen-cli

# Optional: Install wasm-opt for optimization
npm install -g wasm-opt
```

### Quick Local Testing Workflow

```bash
# Using Makefile (recommended)
make web && make serve
# Then open http://localhost:8000

# Or manually:
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir web --target web target/wasm32-unknown-unknown/release/arenic_bevy.wasm
cp -r assets web/
python3 -m http.server 8000 --directory web
```

### Browser Compatibility Notes

- **Firefox**: Best audio support, works immediately
- **Chrome**: Requires user interaction (click) for audio to start
- **Safari**: Most restrictive, may need additional clicks
- **Edge**: Similar to Chrome

The `.meta` file 404 errors in console are normal and don't affect gameplay.

### Important Reminders

1. **AssetMetaCheck::Never is CRITICAL** - Without this, web builds may fail to load assets
2. **Audio requires user interaction** - This is a browser security feature, not a bug
3. **Local `web/` folder is separate from deployment** - GitHub Actions creates its own
4. **The import statement must match your binary name** - `arenic_bevy.js` not `arenic.js`

### Troubleshooting Web Audio Issues

If audio doesn't work:

1. Check browser console for errors
2. Ensure you clicked/interacted with the page
3. Try Firefox first (most lenient)
4. Verify MP3 feature is enabled in Cargo.toml: `bevy = { version = "0.16.1", features = ["mp3"] }`
5. Check that assets are copied to web folder: `ls web/assets/`

### What the GitHub Workflow Handles Automatically

The deployment workflow takes care of:

- Installing all WASM tools
- Using `--out-name arenic` to rename output files
- Creating optimized index.html
- Copying all assets
- Optimizing WASM size with wasm-opt
- Setting up proper canvas IDs

### Deployment Workflow Steps

Option 1: Create a GitHub Release (Recommended)

1. Go to your repo's main page
2. Click Releases (right sidebar)
3. Click "Create a new release"
4. Choose your tag from the dropdown
5. Fill in release title and description
6. Click "Publish release"
7. This will automatically trigger the workflow

Option 2: Manual Trigger

Since the workflow has workflow_dispatch, you can run it manually:

1. In the Actions tab
2. Click on "Deploy Arenic to itch.io" in the left sidebar
3. Click "Run workflow" button on the right
4. Enter your tag name (e.g., v1.0.0)
5. Click "Run workflow"