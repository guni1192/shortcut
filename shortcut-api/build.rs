fn main() {
    tonic_build::configure().compile(&["../proto/shortcut.proto"], &["../proto"]).unwrap()
}
