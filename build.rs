extern crate embed_resource;

fn main() {
    #[cfg(windows)]
    embed_resource::compile("resources/windows/res.rc", embed_resource::NONE);
    // embed_resource::compile_for("assets/poke-a-mango.rc", &["poke-a-mango", "poke-a-mango-installer"], &["VERSION=\"0.5.0\""]);
    // embed_resource::compile_for("assets/uninstaller.rc", &["unins001"], embed_resource::NONE);
}
