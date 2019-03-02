use crate::scanner::{
    module,
    findings,
    Scan,
    Target,
    TargetKind,
};
use postgres::{Connection, TlsMode};

pub struct Subdomains{}

impl module::BaseModule for Subdomains {
    fn name(&self) -> String {
        return String::from("domain/subdomains");
    }

    fn description(&self) -> String {
        return String::from("Find subdomains for a given domain");
    }

    fn author(&self) -> String {
        return String::from("Sylvain Kerkour <sylvain@kerkour.com>")
    }

    fn version(&self) -> String {
        return String::from("0.1.0");
    }
}

impl module::HostModule for Subdomains {
    fn run(&self, _: &Scan, target: &Target) -> (Option<findings::Data>, Vec<String>) {
        let mut errs = vec!();
        let mut ret = None;
        let mut domains = vec!();
        let mut index = 0;
        let mut index_reverse = 0;

        match target.kind {
            TargetKind::Ip => { return (ret, errs); },
            _ => {}, // if domain, continue
        }

        let conn = Connection::connect("postgres://guest@crt.sh:5432/certwatch", TlsMode::None).unwrap();

        let subdomains_pattern = format!("%.{}", &target.host);

        if let Some(i) = subdomains_pattern.find('%') {
            index = i;
        }
        if let Some(i) = subdomains_pattern.chars().rev().collect::<String>().find('%') {
            index_reverse = i;
        }

        let query = if index < index_reverse {
            "SELECT DISTINCT ci.NAME_VALUE as domain
			FROM certificate_identity ci
			WHERE reverse(lower(ci.NAME_VALUE)) LIKE reverse(lower($1))"
        } else {
             "SELECT DISTINCT ci.NAME_VALUE as domain
            FROM certificate_identity ci
            WHERE lower(ci.NAME_VALUE) LIKE lower($1)"
        };

        let rows = conn.query(query, &[&subdomains_pattern]).unwrap();
        for row in &rows {
            domains.push(row.get(0));
        }
        ret = Some(findings::Data::Domains(domains));

        return (ret, errs);
    }
}
