use std::{net::TcpListener, io::Error};

use actix_web::{HttpServer, App,dev::Server};

use crate::routes::{health_check, transaction};


pub fn run(listener: TcpListener) -> Result<Server, Error>{
    let server = HttpServer::new(move || {
        App::new()
            .service(health_check)
            .service(transaction)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
