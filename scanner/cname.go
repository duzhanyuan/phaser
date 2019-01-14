package scanner

import (
	"net"

	"gitlab.com/bloom42/shared/phaser"
)

func CNAME(scan *phaser.Scan, target *phaser.Target) []error {
	errs := []error{}

	cname, err := net.LookupCNAME(target.Host)
	if err != nil {
		errs = append(errs, err)
		// e := formatError(err.Error(), target.Target.Host, target.Target.Type, nil)
		return errs
	}
	target.Findings.CNAME = &cname
	return errs
}