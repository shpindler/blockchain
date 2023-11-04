// mod cli;
// mod api;

// use std::thread;
// use actix_web::{App, HttpServer};
// use api::routes::configure;

// #[actix_web::main]
fn main() {
    // // Launch the CLI in a separate thread
    // let cli_handle = thread::spawn(move || {
    //     cli::run_cli();
    // });

    // // Run the Web API on the main thread
    // let web_api_server = HttpServer::new(move || {
    //     App::new().configure(configure)
    // })
    // .bind("127.0.0.1:8080")?
    // .run();

    // // Await the HTTP server to finish
    // web_api_server.await?;

    // // Join the CLI thread (assuming you want to wait for it to finish for some reason)
    // match cli_handle.join() {
    //     Ok(_) => Ok(()),
    //     Err(e) => {
    //         eprintln!("CLI thread ended with an error: {:?}", e);
    //         Err(std::io::Error::new(std::io::ErrorKind::Other, "CLI thread error"))
    //     }
    // }
}
