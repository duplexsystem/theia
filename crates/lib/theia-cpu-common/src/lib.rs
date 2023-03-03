use std::collections::HashMap;

struct SPIRVModule {
    shaders: HashMap<String, SPIRVShader>,
    spirv: Vec<u8>,
}

struct SPIRVShader {
    module: SPIRVModule,
    entrypoint: String,
}
