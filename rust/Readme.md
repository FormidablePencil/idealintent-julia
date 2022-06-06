# Architecture

src/api.rs contain the functions that call the actual code. Later on
api will be split up to encapsulate functionalities such as one for
account related and another for compositions related.

### Run codegen flutter rust bridge

flutter_rust_bridge_codegen --rust-input frb_example/with_flutter/rust/src/compositions_api.rs --dart-output frb_example/with_flutter/lib/bridge_generated.dart --c-output frb_example/with_flutter/ios/Runner/bridge_generated.h

