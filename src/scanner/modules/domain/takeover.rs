use crate::{
    scanner::{
        module,
        findings,
        Scan,
        Target,
        TargetKind,
    },
    error::PhaserError,
};
use serde::{Deserialize, Serialize};
use std::fs;


pub struct Takeover{}

impl module::BaseModule for Takeover {
    fn name(&self) -> String {
        return "domain/takeover".to_string();
    }

    fn description(&self) -> String {
        return "Check subdomain for takeover".to_string();
    }

    fn author(&self) -> String {
        return "Sylvain Kerkour <sylvain@kerkour.com>".to_string();
    }

    fn version(&self) -> String {
        return "0.1.0".to_string();
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Provider {
    pub service: String,
    #[serde(rename = "cname")]
    pub cnames: Vec<String>,
    #[serde(rename = "fingerprint")]
    pub fingerprints: Vec<String>,
    pub nxdomain: bool,
}


impl module::HostModule for Takeover {
    fn run(&self, scan: &Scan, target: &Target) -> Result<findings::Data, PhaserError> {
        let errs = vec!();
        let mut ret = findings::Data::None;

        if let TargetKind::Ip = target.kind {
            return Ok(findings::Data::None);
        };

        // parse fingerprints
        let fingerprints_path = format!("{}/takeover_fingerprints.json", &scan.config.assets_folder);
        let fingerprints_data = fs::read_to_string(fingerprints_path)
            .expect("Something went wrong reading the fingerprints file");

        let providers: Vec<Provider> = serde_json::from_str(&fingerprints_data)
            .expect("error parsing providers fingerprints");


        let body = reqwest::get(&format!("http://{}", &target.host))
        .expect("error fetching url for takeover")
        .text()
        .expect("error getting body to txt");

        'outer: for provider in &providers {
            for fingerprint in &provider.fingerprints {
                if body.contains(fingerprint) {
                    ret = findings::Data::Takeover(findings::domain::Takeover{
                        service: provider.service.to_string(),
                    });
                    break 'outer;
                }
            }
        }
        return Ok(ret);
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::module::BaseModule;

    #[test]
    fn module_name() {
        let module = super::Takeover{};
        assert_eq!("domain/takeover", module.name());
    }
}
