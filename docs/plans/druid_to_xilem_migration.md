# Migration Plan: Druid to Xilem

This document outlines the strategy for migrating `psst-gui` from the [Druid](https://github.com/linebender/druid) UI toolkit to [Xilem](https://github.com/linebender/xilem). Xilem represents a shift to a reactive architecture, requiring significant changes to how state and UI implementation are handled.

## Phase 1: Preparation & Infrastructure

**Goal:** Establish the foundation for Xilem without breaking the existing Druid application immediately (if possible), or prepare for a clean cutover branch.

### Subtasks
- [ ] **Dependency Management**: Add `xilem` to `Cargo.toml`.
    - Note: Verify compatibility of `xilem` versions with other dependencies.
- [ ] **State Architecture Analysis**:
    - Review `src/data/` modules. Xilem uses a different state management approach (often strictly immutable or adaptors).
    - Map `druid::Data` traits to standard Rust data structures or Xilem `State`.
- [ ] **Theming & Resources**:
    - Port `src/widget/theme.rs` definitions to a Xilem-compatible theme system or CSS-like styling if using a web-backend, though Xilem native uses Masonry/Vello.
    - Milestone: `xilem` dependency added and compiling.

## Phase 2: Core Components & Entry Point

**Goal:** Get a "Hello World" window running with Xilem that mimics the main application shell.

### Subtasks
- [ ] **Entry Point**:
    - Create a parallel entry point (e.g., `examples/xilem_main.rs` or a feature flag) to run Xilem.
    - Replicate existing window configuration (title, size, min-size) from `src/main.rs`.
- [ ] **Root Widget**:
    - Recreate the main layout structure found in `src/ui/mod.rs` (likely a `Flex` or `Split` equivalent).
    - Milestone: Xilem application launches with the basic application shell (sidebar + main view + player bar placeholders).

## Phase 3: Base Widgets & Utilities

**Goal:** Migrate the reusable building blocks used throughout the app.

### Subtasks
- [ ] **Icons & SVG**:
    - Migrate icon loading from `src/widget/icons.rs`.
- [ ] **Common Widgets**:
    - Migrate `src/widget/dummy.rs`, `src/widget/maybe.rs`, `src/widget/utils.rs`.
    - Identify Xilem equivalents for `Label`, `Button`, `Spinner`, `Switch`.
- [ ] **Custom Widgets**:
    - Rewrite `src/widget/fill_between.rs` using Xilem's drawing primitives (Vello/Masonry).
    - Rewrite `src/widget/remote_image.rs` for async image loading in Xilem.
    - Milestone: All custom base widgets are ported and verifiable in isolation.

## Phase 4: Main Application Panels (Parallel Execution)

**Goal:** Port the primary content areas. This can be done incrementally.

### Subtasks
- [ ] **Playback Control Bar** (`src/ui/playback.rs`):
    - Reimplement the bottom player bar (Play/Pause, Seek, Volume).
    - Connect to existing audio backend (commands).
- [ ] **Sidebar Navigation** (`src/ui/menu.rs`, `src/ui/nav.rs`):
    - Reimplement the left-hand navigation menu.
    - Connect navigation events to switch the main view.
- [ ] **Home & Search Views** (`src/ui/home.rs`, `src/ui/search.rs`):
    - Port the lists and grids for Home and Search results.
- [ ] **Library & Playlists** (`src/ui/library.rs`, `src/ui/playlist.rs`):
    - Port the user library and playlist detail views.

## Phase 5: State & Event Wiring

**Goal:** Reconnect the application logic (Spotify Web API, Audio Player) to the new UI.

### Subtasks
- [ ] **Commands & Delegates**:
    - Refactor `src/delegate.rs` and `src/controller/` to work with Xilem's event handling.
    - Ensure `OnCommand` equivalents invoke the correct async tasks.
- [ ] **Async Data Loading**:
    - Adapt `src/webapi/` calls to trigger UI updates in Xilem's reactive system.
    - Milestone: Full application interactivity (Playback, Navigation, Search, Data Loading).

## Phase 6: Polish & Cleanup

**Goal:** Finalize the migration and remove legacy code.

### Subtasks
- [ ] **Visual Polish**:
    - Fine-tune padding, colors, and typography to match or improve upon the original design.
- [ ] **Cleanup**:
    - Remove `druid` dependency from `Cargo.toml`.
    - Delete all `druid`-specific implementations in `src/ui` and `src/widget`.
- [ ] **Testing**:
    - Verify all features (Login, Playback, Search, Library management).

## Key Risks & Considerations
- **Ecosystem Maturity**: Xilem is currently alpha. Expect breaking changes and missing documentation.
- **Widget Availability**: Some complex Druid widgets (e.g., complex lists or specific layouts) might need manual implementation in Xilem/Masonry.
- **Async Integration**: Re-verifying how async tasks (network requests) feed back into the reactive UI loop is critical.
