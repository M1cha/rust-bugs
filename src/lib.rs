mod generated {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}
pub use generated::*;

fn load_data() -> std::io::Result<Box<()>> {
    Ok(Box::new(()))
}
