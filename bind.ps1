bindgen --allowlist-function "pt_.*" --allowlist-type "pt_.*" --allowlist-var "pt_.*" intel-pt.h -o src/bindings.rs
echo "if it cant find the bindgen command, run 'cargo install bindgen'"
