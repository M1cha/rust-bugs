use std::io::Write;

fn main() {
    let out_dir: std::path::PathBuf = std::env::var("OUT_DIR").unwrap().into();
    let mut out = std::fs::File::create(out_dir.join("generated.rs")).unwrap();

    let num_fields: usize = 400;

    writeln!(out, "pub struct S {{").unwrap();

    for fieldid in 0..num_fields {
        writeln!(out, "pub field_{id}: Box<()>,", id = fieldid).unwrap();
    }

    writeln!(
        out,
        "}}
        impl S {{
            pub fn load() -> std::io::Result<Self> {{
                Ok(Self {{
    "
    )
    .unwrap();

    for fieldid in 0..num_fields {
        writeln!(out, "field_{id}: super::load_data()?,", id = fieldid).unwrap();
    }

    writeln!(
        out,
        "
                }})
            }}
        }}
    "
    )
    .unwrap();
}
