package main

import (
	"flag"
	"github.com/araddon/dateparse"
	"github.com/juju/errors"
	"github.com/ngaut/log"
	"net/http"
	"strings"
	"time"
)

//Run base struct
type Run struct {
	client           *http.Client
	dashboards       []*Dashboard
	imageURLs        chan URL
	User             string `json:"user"`
	Password         string `json:"password"`
	url              string
	from             int64
	to               int64
	tz               string
	width            int64
	height           int64
	timeout          int64
	name             string
	pngDir           string
	requestDashboard string
	requestRenderURL string
}

var (
	addr      = flag.String("address", "http://192.168.2.188:3000", "input grafana address")
	from      = flag.String("start", time.Now().AddDate(0, 0, -3).Format(TimeFormat), "input start time, default is 3 days ago")
	to        = flag.String("end", time.Now().Format(TimeFormat), "input end time,default is now")
	name      = flag.String("name", "", "input panel name")
	timeout   = flag.Int64("timeout", 60, "input execute query timeout")
	user      = flag.String("user", "admin", "input granfana user")
	password  = flag.String("password", "admin", "input granfana password")
	dashboard = flag.String("dashboard", "", "input dashboard name")
	renderURL = flag.String("renderurl", "", "input render url")

	stringReplacer = strings.NewReplacer(" ", "_", "/", "_", "\\", "_")
)

//InitRun init struct
func InitRun() (*Run, error) {
	flag.Parse()
	log.Infof("init..")
	c, err := NewSession()
	if err != nil {
		return nil, errors.Errorf("create http client with error %v", err)
	}
	ft, err := dateparse.ParseAny(*from)
	if err != nil {
		return nil, errors.Errorf("start time is error %v", err)
	}
	et, err := dateparse.ParseAny(*to)
	if err != nil {
		return nil, errors.Errorf("end time is error %v", err)
	}
	tz, _ := time.Now().Zone()

	return &Run{
		client:           c,
		imageURLs:        make(chan URL, 10000),
		width:            1980,
		height:           1080,
		timeout:          *timeout,
		from:             ft.UnixNano() / 1000000,
		to:               et.UnixNano() / 1000000,
		tz:               tz,
		name:             stringReplacer.Replace(*name),
		User:             *user,
		Password:         *password,
		url:              *addr,
		requestDashboard: stringReplacer.Replace(*dashboard),
		requestRenderURL: *renderURL,
	}, nil
}

//PreData prepare data
func (r *Run) PreData() error {
	log.Infof("prepare data start...")
	if err := r.PrefixWork(); err != nil {
		return err
	}

	if r.requestRenderURL != "" {
		log.Infof("handler render url...")
		if err := r.HandlerRequestURL(); err != nil {
			return err
		}
	}

	log.Infof("login grafana...")
	if err := r.LoginGrafana(); err != nil || len(r.imageURLs) > 0 {
		close(r.imageURLs)
		return err
	}
	log.Infof("get dashboards...")
	if err := r.GetDashboards(); err != nil {
		return err
	}
	log.Infof("get panels...")
	if err := r.GetDashboardPanels(); err != nil {
		return err
	}

	log.Infof("generate image url...")
	if err := r.GenerateURL(); err != nil {
		return err
	}

	close(r.imageURLs)
	log.Infof("prepare data finished...")

	return nil

}
