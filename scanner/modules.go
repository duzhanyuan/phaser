package scanner

import (
	"github.com/bloom42/phaser/scanner/module"
	"github.com/bloom42/phaser/scanner/module/ports"
)

// AllHostModules contains all phaser's modules which will be run for each host.
// You must register you module here in order to be able to use it.
var AllHostModules = []module.HostModule{
	ports.Ports{},
}

// AllPortModules contains all phaser's modules which will be run for each port for each host.
// You must register you module here in order to be able to use it.
var AllPortModules = []module.PortModule{}