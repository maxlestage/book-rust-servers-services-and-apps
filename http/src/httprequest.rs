use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

// https://doc.rust-lang.org/std/convert/trait.From.html
/*
Convertit une chaine de caractère en méthode souhaité = >  "GET" => Method::Get,
L'implémentation de la méthode from dans le trait From nous permet de lire la chaîne de méthode à partir de la ligne de requête HTTP et de la convertir en variante Method :: Get ou Method :: Post .
*/
impl From<&str> for Method {
    fn from(method_http: &str) -> Method {
        match method_http {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

/*
Version
La définition de Version enum est illustrée ci-dessous.
Nous prendrons en charge deux versions HTTP uniquement à titre d'illustration, mais nous ne travaillerons qu'avec HTTP/1.1 pour nos exemples.
 Il existe également un troisième type - Uninitialized , à utiliser comme valeur initiale par défaut.
*/
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(version_http: &str) -> Version {
        match version_http {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

//  Test
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }
}
