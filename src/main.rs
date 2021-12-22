use wasm_bindgen_futures::spawn_local;



async fn a_main(){

    let mut wc = web_container::WebContainer::new();
    let _x = wc.open("./test.wpack").await;
}

fn main(){
    spawn_local(async{
        a_main().await;
    })
}