# Druid to Xilem: State & Theme Analysis

## State Architecture

The current application uses `druid::Data` to manage state changes. The root state is `AppState` in `src/data/mod.rs`.

### Key Structures

- **`AppState`**: The root state. Contains `session`, `nav`, `playback`, `config`, etc.
- **`Promise<T, D, E>`**: A custom enum for async data handling (`Empty`, `Deferred`, `Resolved`, `Rejected`).
- **`Vector` / `HashSet`**: Uses `druid::im` for immutable data structures.

### Migration Strategy

1. **Remove `druid::Data`**: Xilem does not strictly require `Data`. We can derive `PartialEq` for change detection if needed, or simply use `Arc` for cheap cloning.
    - `im` crates can be kept (swapping `druid::im` for `im` or `im-rc`).
2. **`Promise` Enum**: This can remain largely the same, just removing the `Data` trait bound.
3. **App Logic**: Methods on `AppState` (e.g., `navigate`, `start_playback`) are effectively the "Reducer" logic. These can be preserved.

## Theming (`src/widget/theme.rs` & `src/ui/theme.rs`)

### Current Approach

- **`src/ui/theme.rs`**: Defines constants (`Key<Color>`, `Key<FontDescriptor>`) and a `setup` function that populates a `druid::Env`.
- **`src/widget/theme.rs`**: A `ThemeScope` widget that listens for `AppState::config::theme` changes and calls `setup` to update the `Env`.

### Xilem Approach

Xilem does not use a central `Env` for styling in the same way. Styles are typically passed down or accessed via a context if implemented.

- **Option A (Custom Env)**: Implement a custom environment struct passed through the view tree.
- **Option B (Context)**: Use Xilem's context facilities if available.
- **Option C (Hardcoded/Constants)**: Just use Rust constants if runtime theme switching isn't needed (but it IS needed here).

### Recommendation

Refactor `src/ui/theme.rs` into a struct `AppTheme` that holds the colors/fonts.
Add `AppTheme` to `AppState` or wrap the root Xilem view in a provider that allows accessing the current theme.
Flatten `Key` usages to direct field access on the `AppTheme` struct.
