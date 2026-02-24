use glib_build_tools::compile_resources;

fn main() {
    println!("cargo:rerun-if-changed=resources.gresource.xml");
    println!("cargo:rerun-if-changed=resources/");

    compile_resources(
        &["resources"],
        "resources/resources.gresource.xml",
        "compiled.gresource",
    );
}
