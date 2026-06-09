fn main() -> Result<(), xidl_build::IdlcError> {
    println!("cargo:rerun-if-changed=idl/gallery.idl");

    xidl_build::Builder::new()
        .with_lang("rust")
        .compile(&["idl/gallery.idl"])?;

    Ok(())
}
