use std::error::Error;

use spirv_builder::{MetadataPrintout, SpirvBuilder};

fn main() -> Result<(), Box<dyn Error>> {
    let shaders = SpirvBuilder::new("../theia-gpu", "spirv-unknown-vulkan1.2")
        .print_metadata(MetadataPrintout::Full)
        .build()?;

    shadersOk(())
}
