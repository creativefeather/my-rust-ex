# Issues
If you have imports for a wasm file, you have to include them in WebAssembly.instantiate(), for example.  Otherwise, an error is thrown.

Parcel doesn't include imports when importing with it's wasm loader.