/*
    http please
    php has been remove from macos.
    with all the bad, it was so simple to create
    a webserver, serving a directory, to overcome
    any CORS related issues in the browser, when
    developing/prototyping

    // serving current working directory
    httplz 0.0.0.0:7484
    httplz 127.0.0.1:5005
    httplz localhost:2424

    // serving current working directory
    httplz localhost:2424 /Users/test/example/dir

*/
mod http;

fn main() {
    if let Some(host_addr) = std::env::args().nth(1) {
        if let Some(defined_path) = std::env::args().nth(2) {
            http::create_server(host_addr, std::path::PathBuf::from(defined_path));
        } else {
            match std::env::current_dir() {
                Ok(cwd_path) => {
                    http::create_server(host_addr, cwd_path);
                }
                Err(e) => {
                    println!(
                        "[Error]: failed optaining path to current working directory: {}",
                        e
                    )
                }
            }
        }
    } else {
        println!("[Error]: Missing ip and port, ex: httplz 127.0.0.1:5005")
    }
}
#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn can_serve() {
        let _server_thread = std::thread::spawn(|| {
            http::create_server(
                "127.0.0.1:5050".to_owned(),
                std::path::PathBuf::from("resources/test"),
            );
        });

        let mut curl_command_index = std::process::Command::new("curl");
        curl_command_index.arg("127.0.0.1:5050");
        let result = curl_command_index
            .output()
            .expect("failed to execute curl command")
            .stdout;
        match std::fs::read(std::path::PathBuf::from(
            "resources/test/index.html",
        )) {
            Ok(compare) => {
                assert_eq!(result, compare);
            }
            Err(e) => {
                panic!("Could not compare with file in curl test error {:?}", e)
            }
        };

        let mut curl_command_png = std::process::Command::new("curl");
        curl_command_png.arg("127.0.0.1:5050/test.png");
        let result = curl_command_png
            .output()
            .expect("failed to execute curl command")
            .stdout;
        match std::fs::read(std::path::PathBuf::from(
            "resources/test/test.png",
        )) {
            Ok(compare) => {
                assert_eq!(result, compare);
            }
            Err(e) => {
                panic!("Could not compare with file in curl test error {:?}", e)
            }
        };

        let mut curl_command_queryparams = std::process::Command::new("curl");
        curl_command_queryparams.arg("127.0.0.1:5050/js/scripts.js?v=1");
        let result = curl_command_queryparams
            .output()
            .expect("failed to execute curl command")
            .stdout;
        match std::fs::read(std::path::PathBuf::from(
            "resources/test/js/scripts.js",
        )) {
            Ok(compare) => {
                assert_eq!(result, compare);
            }
            Err(e) => {
                panic!("Could not compare with file in curl test error {:?}", e)
            }
        };

        let mut curl_command_hash = std::process::Command::new("curl");
        curl_command_hash.arg("127.0.0.1:5050/index.html#test");
        let result = curl_command_hash
            .output()
            .expect("failed to execute curl command")
            .stdout;
        match std::fs::read(std::path::PathBuf::from(
            "resources/test/index.html",
        )) {
            Ok(compare) => {
                assert_eq!(result, compare);
            }
            Err(e) => {
                panic!("Could not compare with file in curl test error {:?}", e)
            }
        };
    }

    
}
