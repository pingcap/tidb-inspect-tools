package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"github.com/juju/errors"
	"github.com/ngaut/log"
	"io"
	"io/ioutil"
	"net/http"
	"net/http/cookiejar"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"time"
)

const (
	LoginPath               = "/login"
	DashboardAPI            = "/api/search?query="
	DashboardAPIPrefix      = "/api/dashboards/"
	CreateHttpClientTimeout = 15

	PngDir     = "PngDir"
	TimeFormat = "2006-01-02 15:04:05"
)

type Panel struct {
	Title string
	ID    int64
	URL   string
}

//NewSession crate http client
func NewSession() (*http.Client, error) {
	jar, err := cookiejar.New(nil)
	if err != nil {
		return nil, err
	}

	return &http.Client{
		Jar:     jar,
		Timeout: time.Second * CreateHttpClientTimeout,
	}, nil
}

//LoginGrafana login granfana
func (r *Run) LoginGrafana() (err error) {
	jsonData, jerr := json.Marshal(r)
	if jerr != nil {
		return jerr
	}
	_, err = r.XHttp("POST", fmt.Sprintf("%s%s", r.url, LoginPath), bytes.NewBuffer(jsonData))
	return err
}

//XHttp http
func (r *Run) XHttp(method string, url string, body io.Reader) ([]byte, error) {
	req, err := http.NewRequest(method, url, body)
	if err != nil {
		return nil, err
	}
	if body != nil {
		req.Header.Set("Content-Type", "application/json")
	}
	for i := 0; i < 3; i++ {
		resp, err := r.client.Do(req)
		if err != nil && i == 2 {
			return nil, err
		}
		if err == nil && resp.StatusCode == 200 {
			return ioutil.ReadAll(resp.Body)
		}
	}
	return nil, errors.Errorf("can not post data to url %s", url)
}

//Xget http get
func (r *Run) Xget(urlParams string) (interface{}, error) {
	d, err := r.XHttp("GET", fmt.Sprintf("%s%s", r.url, urlParams), nil)
	if err != nil {
		return nil, err
	}
	var iface interface{}
	errJ := json.Unmarshal(d, &iface)
	return iface, errJ

}

//GetDashboards http get dashboards
func (r *Run) GetDashboards() error {
	iface, err := r.Xget(DashboardAPI)
	if err != nil {
		return err
	}
	r.JSONData(iface, "")
	close(r.dashboards)
	return nil
}

//JSONData handle http json data
func (r *Run) JSONData(iface interface{}, dashboard string) {
	switch s := iface.(type) {
	case map[string]interface{}:
		if _, ok := s["uri"]; ok {
			r.dashboards <- s["uri"].(string)
		}

		if _, ok := s["dashboard"]; ok {
			r.JSONData(s["dashboard"], dashboard)
		}

		if _, ok := s["rows"]; ok {
			r.JSONData(s["rows"], dashboard)
		}
		if _, ok := s["panels"]; ok {
			r.JSONData(s["panels"], dashboard)
		}

		title, okTitle := s["title"]
		id, okID := s["id"]
		if okTitle && okID && dashboard != "" && title.(string) != "" {
			replacer := strings.NewReplacer(" ", "_", "/", "_", "\\", "_")
			if r.name == "" || replacer.Replace(r.name) == replacer.Replace(title.(string)) {
				r.panels <- Panel{
					Title: replacer.Replace(title.(string)),
					ID:    int64(id.(float64)),
					URL: fmt.Sprintf("%s/render/dashboard/%s?panelId=%d&from=%d&to=%d&width=%d&height=%d&fullscreen&timeout=%d&tz=%s",
						r.url, dashboard, int64(s["id"].(float64)), r.from, r.to, r.width, r.height, r.timeout, r.tz),
				}
			}

		}
	case []interface{}:
		for _, sub := range s {
			r.JSONData(sub, dashboard)
		}
	}
}

//GetDashboardAPIs http panels
func (r *Run) GetDashboardAPIs() error {
	for d := range r.dashboards {
		iface, err := r.Xget(fmt.Sprintf("%s%s", DashboardAPIPrefix, d))
		if err != nil {
			return err
		}
		r.JSONData(iface, d)
	}
	close(r.panels)
	return nil
}

//PrefixWork prefixWork
func (r *Run) PrefixWork() error {
	workDir, err := filepath.Abs(filepath.Dir(os.Args[0]))
	if err != nil {
		return err
	}

	r.pngDir = filepath.Join(workDir, PngDir)
	if _, err := os.Stat(r.pngDir); os.IsNotExist(err) {
		return os.Mkdir(r.pngDir, 0774)
	}

	return nil
}

//GetRenderImages get redner images
func (r *Run) GetRenderImages() {
	for p := range r.panels {
		log.Infof("get %s render image...", p.Title)
		data, err := r.XHttp("GET", p.URL, nil)
		if err != nil {
			log.Errorf("http get url %s render image error %v", p.URL, err)
			continue
		}
		if err := r.SaveImage(p.Title, data); err != nil {
			log.Errorf("write image file error %v", err)
		}

	}
}

//SaveImage save image
func (r *Run) SaveImage(title string, data []byte) error {
	dstImage := filepath.Join(r.pngDir, fmt.Sprintf("%s.png", title))
	return ioutil.WriteFile(dstImage, data, 0666)
}

//getCPUNum get cpu number
func getCPUNum() int {
	return runtime.NumCPU()
}
