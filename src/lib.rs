use warp::Filter;

pub async fn start_server(port: u16) {
    let handler = warp::any().map(|| warp::reply());

    warp::serve(handler).run(([0, 0, 0, 0], port)).await;
}
