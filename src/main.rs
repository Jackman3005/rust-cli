extern crate yaml_rust;

use yaml_rust::{YamlLoader, YamlEmitter};
use std::process::Command;
//use std::env;
use std::process::Stdio;
use std::io;
use yaml_rust::yaml;


fn main() {
    let s =
        "
servers:
  - name: test 1
    cmd: /usr/local/bin/yarn
  - name: Something Else!
    cmd: /bin/ls
    args:
      - /Users/jcoy/workspace
config:
  logging:
    directory: /tmp/test-cli
";
    let docs = YamlLoader::load_from_str(s).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];

    // Debug support
//    println!("{:?}", doc);

    // Index access for map & array
//    assert_eq!(doc["servers"][0]["name"].as_str().unwrap(), "test 1");
//    assert_eq!(doc["config"]["logging"]["directory"].as_str().unwrap(), "/tmp/test-cli");

    let servers = &doc["servers"];
    let num_servers = servers.as_vec().unwrap().len();

//    for (i, server) in servers.iter_mut().enumerate() {
//        *elem += i as u8
//    }
//
//

    use std::io::{Read, Write};

    for i in 0..num_servers {
//        let x = i.to_u8().unwrap();
        let server = &servers[i];
        let server_name = server["name"].as_str().unwrap();
        let server_command = server["cmd"].as_str().unwrap();

//        println!("{:?}", server);
        println!("Starting {}", server_name);

//        let path_env = env::var("PATH").unwrap_or_default();

        let command = &mut Command::new(server_command);
        command.env("LS_COLORS", "rs=0:di=38;5;27:mh=44;38;5;15");
//            .env("PATH", path_env)
//            .output()
//            .expect("failed to execute process");


        let args = &server["args"];
        if !args.is_badvalue() {
            let num_args = args.as_vec().unwrap().len();
            for j in 0..num_args {
                let arg = args[j].as_str().unwrap();
                println!("adding arg {}", arg);
                command.arg(arg);
            }
        }


        command.stdin(Stdio::inherit());
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());

        let mut child = command.spawn().unwrap();

//        let stderr = io::stderr();
//        let mut stderr = stderr.lock();
//        let mut child_stderr = child.stderr.take().unwrap();
//
//        let mut buf = [0u8; 1024];
//        loop {
//            let len = match child_stderr.read(&mut buf) {
//                Ok(0) => break,
//                Ok(len) => len,
//                Err(e) => panic!(e),
//            };
//            stderr.write_all(&buf[..len]).unwrap();
//        }
//
//        std::mem::drop(child_stderr);

        let status = child.wait().unwrap();

        assert!(status.success());
    }


    // Chained key/array access is checked and won't panic,
    // return BadValue if they are not exist.
//    assert!(doc["INVALID_KEY"][100].is_badvalue());

//    print_yaml(doc);
}

fn print_yaml(doc: &yaml::Yaml) {
    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);
}
