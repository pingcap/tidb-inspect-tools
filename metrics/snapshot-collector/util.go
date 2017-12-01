package snapshot

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
	"net/url"
	"os"
	"path/filepath"
	"reflect"
	"regexp"
	"runtime"
	"strings"
	"time"
)

const (
	//LoginPath grafana login path
	LoginPath = "/login"
	//DashboardAPI grafana dashboards
	DashboardAPI = "/api/search?query="
	//DashboardAPIPrefix grafana dashboard detail
	DashboardAPIPrefix = "/api/dashboards/"
	//DashboardDatasource grafana datasource
	DashboardDatasource = "/api/datasources"
	//DatasourceProxy grafana request prometheus proxy
	DatasourceProxy = "/api/datasources/proxy/"
	//PrometheusMatchAPI prometheus match api
	PrometheusMatchAPI = "/api/v1/series?match[]="
	//CreateHTTPClientTimeout http timeout
	CreateHTTPClientTimeout = 15

	//TemplateVar panel templating variables
	TemplateVar = "Host"
	//PanelsAllInOnePicturePoint delimit one picture informations
	PanelsAllInOnePicturePoint = 30

	//PngDir save images directory
	PngDir = "PngDir"
	//TimeFormat time format
	TimeFormat = "2006-01-02 15:04:05"
)

//Dashboard informations
type Dashboard struct {
	TemplateVar    string
	Host           []string
	URI            string
	title          string
	ID             int64
	DataSourceID   int64
	DataSourceType string
	Panels         []Panel
}

//Panel informations
type Panel struct {
	Title string
	ID    int64
	URL   string
}

//URL redner images struct
type URL struct {
	Title string
	URL   string
}

//InitDashboard init struct
func InitDashboard() *Dashboard {
	return &Dashboard{
		DataSourceType: "prometheus",
		DataSourceID:   1,
	}
}

//NewSession crate http client
func NewSession() (*http.Client, error) {
	jar, err := cookiejar.New(nil)
	if err != nil {
		return nil, err
	}

	return &http.Client{
		Jar:     jar,
		Timeout: time.Second * CreateHTTPClientTimeout,
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
	return nil, errors.Errorf("can not request url %s", url)
}

//Xget http get
func (r *Run) Xget(url string) (interface{}, error) {
	d, err := r.XHttp("GET", url, nil)
	if err != nil {
		return nil, err
	}
	var iface interface{}
	errJ := json.Unmarshal(d, &iface)
	return iface, errJ

}

//HandlerRequestURL handler offer URL to render
func (r *Run) HandlerRequestURL() error {
	u, err := url.Parse(r.requestRenderURL)
	if err != nil {
		return err
	}
	uPath := strings.Split(u.Path, "/")
	u.Path = fmt.Sprintf("/render%s", u.Path)
	return r.AddImageURL(fmt.Sprintf("%s_%d", uPath[len(uPath)-1], time.Now().UnixNano()), u.String())
}

//GetDashboards http get dashboards
func (r *Run) GetDashboards() error {
	// http://192.168.2.188:3000/api/search?query=
	iface, err := r.Xget(fmt.Sprintf("%s%s", r.url, DashboardAPI))
	if err != nil {
		return err
	}
	r.JSONData(iface, InitDashboard())
	return nil
}

//JSONData handle http json data
func (r *Run) JSONData(iface interface{}, dash *Dashboard) {
	// dashboard, rows, panels   handle dashboard json
	// templating, list handle template variables
	// data options from [templating, list]
	loopKey := []string{"dashboard", "rows", "panels", "templating", "list", "data"}
	switch s := iface.(type) {
	case map[string]interface{}:
		// handler dashboards
		if _, ok := s["uri"]; ok && dash.URI == "" {
			d := InitDashboard()
			d.URI = s["uri"].(string)
			d.ID = int64(s["id"].(float64))
			d.title = stringReplacer.Replace(s["title"].(string))
			r.dashboards = append(r.dashboards, d)
		}

		for _, key := range loopKey {
			if _, ok := s[key]; ok {
				r.JSONData(s[key], dash)
			}
		}

		//handler templating map, get label query info
		label, okLabel := s["label"]
		query, okQuery := s["query"]
		if okLabel && okQuery && reflect.ValueOf(label) != reflect.ValueOf(nil) && label.(string) == TemplateVar {
			//such like label_values(node_disk_reads_completed, instance)
			dash.TemplateVar = regexp.MustCompile("[(,]").Split(query.(string), -1)[1]
		}

		//handler panels map, get panel info
		title, okTitle := s["title"]
		id, okID := s["id"]
		if (okTitle && okID && title.(string) != "") &&
			(r.name == "" || stringReplacer.Replace(r.name) == stringReplacer.Replace(title.(string))) {
			dash.Panels = append(dash.Panels, Panel{
				Title: stringReplacer.Replace(title.(string)),
				ID:    int64(id.(float64)),
			})
		}
		//handler option variables, get host list
		if _, okInstance := s["instance"]; okInstance {
			h := s["instance"].(string)
			if len(dash.Host) == 0 {
				dash.Host = append(dash.Host, h)
			}
			for i, host := range dash.Host {
				if host == h {
					break
				}
				if i == len(dash.Host)-1 {
					dash.Host = append(dash.Host, h)
				}
			}
		}
	case []interface{}:
		for _, sub := range s {
			r.JSONData(sub, dash)
		}
	}
}

//GetDashboardPanels http panels
func (r *Run) GetDashboardPanels() error {
	for _, dash := range r.dashboards {
		if r.requestDashboard != "" && r.requestDashboard != dash.title {
			continue
		}
		// http://192.168.2.188:3000/api/dashboards/db/test-cluster-disk-performance
		iface, err := r.Xget(fmt.Sprintf("%s%s%s", r.url, DashboardAPIPrefix, dash.URI))
		if err != nil {
			return err
		}
		r.JSONData(iface, dash)

		if dash.TemplateVar != "" {
			// http://192.168.2.188:3000/api/datasources/proxy/1/api/v1/series?match[]=node_disk_reads_completed&start=1511935611&end=1511939211
			ifaceHost, errHost := r.Xget(fmt.Sprintf("%s%s%d%s%s&start=%d&end=%d",
				r.url, DatasourceProxy, dash.DataSourceID, PrometheusMatchAPI, dash.TemplateVar, r.from, r.to))
			if errHost != nil {
				return errHost
			}
			r.JSONData(ifaceHost, dash)
		}
	}
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

//GenerateURL generate image url
func (r *Run) GenerateURL() error {
	for _, d := range r.dashboards {
		baseURL := fmt.Sprintf("%s/render/dashboard/%s?from=%d&to=%d&width=%d&height=%d&timeout=%d&tz=%s",
			r.url, d.URI, r.from, r.to, r.width, r.height, r.timeout, r.tz)
		// all of panels less than PanelsAllInOnePicturePoint, generate one picture
		if len(d.Panels) < PanelsAllInOnePicturePoint && len(d.Host) > 1 {
			for _, h := range d.Host {
				r.AddImageURL(fmt.Sprintf("%s_%s", d.title, h), fmt.Sprintf("%s&var-host=%s", baseURL, h))
			}
		} else if len(d.Panels) < PanelsAllInOnePicturePoint {
			r.AddImageURL(d.title, baseURL)
		} else {
			for _, p := range d.Panels {
				r.AddImageURL(fmt.Sprintf("%s_%s", d.title, p.Title), fmt.Sprintf("%s&panelId=%d&fullscreen", baseURL, p.ID))
			}
		}
	}
	return nil
}

//AddImageURL add url
func (r *Run) AddImageURL(title string, url string) error {
	r.imageURLs <- URL{
		Title: title,
		URL:   url,
	}
	return nil
}

//GetRenderImages get redner images
func (r *Run) GetRenderImages() {
	for p := range r.imageURLs {
		log.Infof("remain image %d, get %s render image...", len(r.imageURLs), p.Title)
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

//GetCPUNum get cpu number
func GetCPUNum() int {
	return runtime.NumCPU()
}
