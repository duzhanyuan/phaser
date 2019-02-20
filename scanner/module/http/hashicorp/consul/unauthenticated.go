package consul

import (
	"fmt"
	"io/ioutil"
	"net/http"
	"strings"

	"github.com/bloom42/phaser/common/phaser"
	"github.com/bloom42/phaser/scanner/module"
)

type UnauthenticatedAccess struct{}

func (UnauthenticatedAccess) Name() string {
	return "http/hashicorp/consul/unauthenticated_access"
}

func (UnauthenticatedAccess) Description() string {
	return "Check for consul Unauthenticated Access"
}

func (UnauthenticatedAccess) Author() string {
	return "Sylvain Kerkour <sylvain@kerkour.com>"
}

func (UnauthenticatedAccess) Version() string {
	return "0.1.0"
}

type VulnerableURL struct {
	URL string `json:"url"`
}

func (UnauthenticatedAccess) Run(scan *phaser.Scan, target *phaser.Target, port phaser.Port) (module.Result, []error) {
	errs := []error{}
	var ret module.Result
	protocol := "http"

	if !port.HTTP && !port.HTTPS {
		return ret, errs
	}
	if port.HTTPS {
		protocol = "https"
	}

	URL := fmt.Sprintf("%s://%s:%d", protocol, target.Host, port.ID)
	req, err := http.NewRequest("GET", URL, nil)
	if err != nil {
		errs = append(errs, err)
		return ret, errs
	}
	req.Header.Set("User-Agent", scan.Profile.HTTP.UserAgent)
	res, err := scan.HTTPClient.Do(req)
	if err != nil {
		errs = append(errs, err)
		return ret, errs
	}
	// response, _ := httputil.DumpResponse(res, true)
	body, err := ioutil.ReadAll(res.Body)
	if err != nil {
		errs = append(errs, err)
		return ret, errs
	}
	res.Body.Close()
	bodyStr := string(body)

	if strings.Contains(bodyStr, `Please enable JavaScript in your web browser to use Consul UI.`) &&
		strings.Contains(bodyStr, `the UI on the same host as a consul instance.`) {
		ret := VulnerableURL{URL}
		return ret, errs
	}

	return ret, errs
}
