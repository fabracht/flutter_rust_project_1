# flutter_rust_project_1

A new Flutter project.

## Getting Started

To run the project, first, make sure the Rust bindings are up to date.
On a terminal, within the project's folder, type:
```bash
flutter_rust_bridge_codegen -r ping_bridge/src/api.rs -d lib/bridge_generated.dart
```

After that, make sure you have an android emulator or device paired up.

Finally, run the flutter application:

```bash
flutter run
```