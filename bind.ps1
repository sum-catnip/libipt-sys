bindgen --whitelist-function "pt_.*" --whitelist-type "pt_.*" --whitelist-var "pt_.*" intel-pt.h -o src/bindings.rs
echo "if it cant find the bindgen command, run 'cargo install bindgen'"