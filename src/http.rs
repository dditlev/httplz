use tiny_http::{Response, Server};

pub fn create_server(host_addr: String, host_dir: std::path::PathBuf) {
    match Server::http(&host_addr) {
        Ok(server) => {
            println!("\nServing: {:?} @ http://{}", host_dir, host_addr);

            for request in server.incoming_requests() {
                match &request.url() {
                    &"/" => {
                        let mut index_path = host_dir.clone();
                        index_path.push(std::path::PathBuf::from("index.html"));
                        match std::fs::File::open(&index_path) {
                            Ok(file) => {
                                let response = tiny_http::Response::from_file(file);
                                match request.respond(response) {
                                    Ok(_) => {
                                        println!("Served [{:?}]", &index_path);
                                    }
                                    Err(e) => {
                                        println!(
                                            "[Error]: serving [{:?}]:{}",
                                            &index_path,
                                            e.to_string()
                                        );
                                    }
                                }
                            }
                            Err(err) => {
                                println!("\n[{:?}]: Request err: {}", index_path, err.to_string());
                                let response = Response::from_string(err.to_string());
                                let _responded = request.respond(response);
                            }
                        }
                    }
                    &_ => {
                        let mut file_path = host_dir.clone();
                        fn pure_filename(req: &str) -> String {
                            // ditch common subpaths in url (querystrings ? and hashs #)
                            if req.contains("?") {
                                if let Some(part) = req.split("?").nth(0) {
                                    part.to_owned()
                                } else {
                                    req.to_owned()
                                }
                            } else if req.contains("#") {
                                if let Some(part) = req.split("#").nth(0) {
                                    part.to_owned()
                                } else {
                                    req.to_owned()
                                }
                            } else {
                                req.to_owned()
                            }
                        }
                        file_path.push(std::path::PathBuf::from(&pure_filename(
                            &request.url()[1..], // ditch / prefix
                        )));
                        match std::fs::File::open(&file_path) {
                            Ok(file) => {
                                let response = tiny_http::Response::from_file(file);
                                match request.respond(response) {
                                    Ok(_) => {
                                        println!("Served [{:?}]", &file_path);
                                    }
                                    Err(e) => {
                                        println!(
                                            "[Error]: serving [{:?}]:{}",
                                            &file_path,
                                            e.to_string()
                                        );
                                    }
                                }
                            }
                            Err(err) => {
                                println!("\n[{:?}]: Request err: {}", file_path, err.to_string());
                                let response = Response::from_string(err.to_string());
                                let _responded = request.respond(response);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("[Error]: Could not start webserver: {}",e)
        }
    }
}
