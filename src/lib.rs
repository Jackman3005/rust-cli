extern crate yaml_rust;

use yaml_rust::{YamlLoader};

pub struct Server {
    name: String,
    command: String
}

impl PartialEq for Server {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.command == other.command
    }
}

pub fn load_servers(yaml_contents: &str) -> Vec<Server> {
    let mut result = Vec::new();
    let docs = YamlLoader::load_from_str(&yaml_contents).unwrap();
    let doc = &docs[0];
    let servers = &doc["servers"];
    let num_servers = servers.as_vec().unwrap().len();
    for i in 0..num_servers {
      let server = &servers[i];
      let server = Server {
        name: String::from(server["name"].as_str().unwrap()),
        command: String::from(server["cmd"].as_str().unwrap())
      };
      result.push(server);
    }
    return result;
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_load_servers() {
        let yaml_contents = String::from("     
        servers:
          - name: test 1
            cmd: /usr/local/bin/yarn
          - name: Something Else!
            cmd: /bin/ls
        ");
        let servers = load_servers(&yaml_contents);
        assert_eq!(2, servers.len());
        let server1 = Server {
          name: String::from("test 1"),
          command: String::from("/usr/local/bin/yarn")
        };
        let server2 = Server {
          name: String::from("Something Else!"),
          command: String::from("/bin/ls")
        };
        assert!(servers.contains(&server1));
        assert!(servers.contains(&server2));
    }   
}
