
function build_recorder_release() {
    cargo build --target=wasm32-unknown-unknown --release &&
        wasm-bindgen ./target/wasm32-unknown-unknown/release/web_container.wasm --out-dir ./resources/ --target web
}
function start_server() {
    simple-http-server -p 6969 -l 500000000 -u -- ./resources/
}

function build_and_run(){
    build_recorder_release && start_server
}

case $1 in
build_recorder)
    build_recorder
    ;;
run_recorder)
    run_recorder
    ;;
build_recorder_release)
    build_recorder_release
    ;;
build_and_run)
    build_and_run
    ;;
*)
    printf "error!\ncommands are:\nbuild_recorder\n"
    ;;
esac
