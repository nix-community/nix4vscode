fn main() -> Result<(), xidl_build::IdlcError> {
    println!("cargo:rerun-if-changed=idl/gallery.idl");
    println!("cargo:rerun-if-changed=idl/gallery_support.idl");

    xidl_build::Builder::new()
        .with_lang("rust-axum")
        .with_server(false)
        .with_client(true)
        .compile(&["idl/gallery.idl"])?;

    xidl_build::Builder::new()
        .with_lang("rust")
        .compile(&["idl/gallery_support.idl"])?;

    Ok(())
}
