use crate::{
    scanner::{
        module,
        findings,
        Scan,
        Target,
    },
    error::PhaserError,
};
use std::process::{Command};
use crate::scanner::modules::ssltls::sslyze;


pub struct Cve2014_0160{}

impl module::BaseModule for Cve2014_0160 {
    fn name(&self) -> String {
        return String::from("ssltls/cve-2014-0160");
    }

    fn description(&self) -> String {
        return String::from("Check for CVE-2014-0160 (a.k.a. heartbleed). See http://heartbleed.com for more information");
    }

    fn author(&self) -> String {
        return String::from("Sylvain Kerkour <sylvain@kerkour.com>")
    }

    fn version(&self) -> String {
        return String::from("0.1.0");
    }
}

impl module::PortModule for Cve2014_0160 {
    fn run(&self, _: &Scan, target: &Target, port: &findings::Port) -> Result<findings::Data, PhaserError> {
        let mut errs = vec!();
        let mut ret = findings::Data::None;

        if !port.https {
            return Ok(findings::Data::None);
        }

        let url = format!("{}:{}", &target.host, port.id);
        let sslyze_output = Command::new("sslyze")
            .arg("--heartbleed")
            .arg("--json_out=-")
            .arg(&url)
            .output()?;
        let output = String::from_utf8_lossy(&sslyze_output.stdout).to_string();

        if !output.trim().is_empty() {
            let sslyze_scan = serde_json::from_str::<sslyze::Scan>(&output)?;
            if sslyze_scan.accepted_targets.len() != 1 {
                return Err(PhaserError::Sslyze(format!("wrong number of sslyze accepted_targets: expected 1, got: {}", sslyze_scan.accepted_targets.len())));
            }
            if sslyze_scan.accepted_targets[0].commands_results.heartbleed.is_vulnerable_to_heartbleed {
                ret = findings::Data::Url(findings::Url{
                    url: format!("https://{}", url),
                });
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
        let module = super::Cve2014_0160{};
        assert_eq!("ssltls/cve-2014-0160", module.name());
    }
}
