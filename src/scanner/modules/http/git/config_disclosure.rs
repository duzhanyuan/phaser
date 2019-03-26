use crate::{
    scanner::{
        module,
        findings,
        Scan,
        Target,
    },
    error::PhaserError,
};
use regex::Regex;



pub struct ConfigDisclosure{}

impl module::BaseModule for ConfigDisclosure {
    fn name(&self) -> String {
        return "http/git/config-disclosure".to_string();
    }

    fn description(&self) -> String {
        return "Check for .git/config file disclosure".to_string();
    }

    fn author(&self) -> String {
        return "Sylvain Kerkour <sylvain@kerkour.com>".to_string();
    }

    fn version(&self) -> String {
        return "0.1.0".to_string();
    }
}

// TODO: error handling not found
impl module::PortModule for ConfigDisclosure {
    fn run(&self, _: &Scan, target: &Target, port: &findings::Port) -> Result<findings::Data, PhaserError> {
        let protocol = if port.http {
            "http"
        } else if port.https {
            "https"
        } else {
            ""
        };

        if protocol.is_empty() {
            return Ok(findings::Data::None);
        }

        let url = format!("{}://{}:{}/.git/config", &protocol, &target.host, &port.id);
        let body = reqwest::get(&url)?
            .text()?;

        if is_config_file(&(body.trim().to_string().to_lowercase())) {
            return Ok(findings::Data::Url(findings::Url{
                url,
            }));
        }

        return Ok(findings::Data::None);
    }
}

fn is_config_file(file_content: &str) -> bool {
    let re = Regex::new(r#"\[branch "[^"]*"\]"#).expect("compiling http/git/config-disclosure regexp");
    return re.is_match(file_content);
}


#[cfg(test)]
mod tests {
    use crate::scanner::module::BaseModule;

    #[test]
    fn module_name() {
        let module = super::ConfigDisclosure{};
        assert_eq!("http/git/config-disclosure", module.name());
    }

    #[test]
    fn is_config_file() {
        let body = r#"[core]
        repositoryformatversion = 0
        filemode = true
        bare = false
        logallrefupdates = true
        ignorecase = true
        precomposeunicode = true
[remote "origin"]
        url = git@github.com:bloom42/phaser.git
        fetch = +refs/heads/*:refs/remotes/origin/*
[branch "master"]
        remote = origin
        merge = refs/heads/master"#;

        let body2 = "lol lol lol ol ol< LO> OL  <tle>Index of kerkour.com</title> sdsds";

        assert_eq!(true, super::is_config_file(body));
        assert_eq!(false, super::is_config_file(body2));
    }
}
