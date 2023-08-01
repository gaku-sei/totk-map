from subprocess import run
from shutil import which

if which("wasm-bindgen") is None:
    run(
        "cargo install wasm-bindgen-cli",
        shell=True,
    )

run("cargo build --release --target wasm32-unknown-unknown", shell=True)
run(
    "wasm-bindgen target/wasm32-unknown-unknown/release/totk_map.wasm --out-dir pkg --target web",
    shell=True,
)
