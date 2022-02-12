
use clipboard::{ClipboardProvider, ClipboardContext};
use warp::Filter;


pub async fn start_clip_server() {
    pretty_env_logger::init();
    let path = warp::path!("clipboard").and(warp::post()).and(warp::body::bytes())
        .map(|b: bytes::Bytes| {
            let c = String::from_utf8(b.to_vec()).unwrap();
            println!("receive clipboard:\n{}",&c);
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(c)?;
            "ok"
        });
    println!("begin to start server");
    warp::serve(path).run(([0,0,0,0], 3001)).await
    // warp::path!("clipboard")
}

pub async fn clipboard_sync(url:&str) -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let data = ctx.get_contents().unwrap();
    reqwest::Client::new().post(url).body(data).send().await.unwrap().text().await.unwrap()
}

