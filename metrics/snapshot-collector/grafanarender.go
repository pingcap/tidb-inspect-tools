package main

import (
	"flag"
	"github.com/ngaut/log"
	"net/http"
	"strings"
	"sync"
	"time"
)

type Run struct {
	client     *http.Client
	dashboards []*Dashboard
	imageURLs  chan URL
	User       string `json:"user"`
	Password   string `json:"password"`
	url        string
	from       int64
	to         int64
	tz         string
	width      int64
	height     int64
	timeout    int64
	name       string
	pngDir     string
}

var (
	addr     = flag.String("address", "http://192.168.2.188:3000", "input grafana address")
	from     = flag.String("start", time.Now().AddDate(0, 0, -3).Format(TimeFormat), "input start time, default is 3 days ago")
	to       = flag.String("end", time.Now().Format(TimeFormat), "input end time,default is now")
	name     = flag.String("name", "", "input panel name")
	timeout  = flag.Int64("timeout", 30, "input execute query timeout")
	user     = flag.String("user", "admin", "input granfana user")
	password = flag.String("password", "admin", "input granfana password")
)

func main() {
	flag.Parse()
	log.Infof("init..")
	c, err := NewSession()
	if err != nil {
		log.Errorf("create http client with error %v", err)
		return
	}
	ft, err := time.Parse(TimeFormat, *from)
	if err != nil {
		log.Errorf("start time is error %v", err)
		return
	}
	et, err := time.Parse(TimeFormat, *to)
	if err != nil {
		log.Errorf("end time is error %v", err)
		return
	}
	tz, _ := time.Now().Zone()

	r := &Run{
		client:    c,
		imageURLs: make(chan URL, 10000),
		width:     1980,
		height:    1080,
		timeout:   *timeout,
		from:      ft.UnixNano() / 1000000,
		to:        et.UnixNano() / 1000000,
		tz:        tz,
		name:      strings.Replace(*name, " ", "_", -1),
		User:      *user,
		Password:  *password,
		url:       *addr,
	}

	log.Infof("start...")
	if errP := r.PrefixWork(); err != nil {
		log.Errorf("prefix work error %v", errP)
		return
	}

	log.Infof("login grafana...")
	if errL := r.LoginGrafana(); errL != nil {
		log.Errorf("log in granfana error %v", errL)
		return
	}
	log.Infof("get dashboards...")
	if errD := r.GetDashboards(); errD != nil {
		log.Errorf("get dashboards error %v", errD)
		return
	}
	log.Infof("get panels...")
	if errP := r.GetDashboardPanels(); errP != nil {
		log.Errorf("get panel error %v", errP)
		return
	}

	log.Infof("generate image url...")
	if errG := r.GenerateURL(); errG != nil {
		log.Errorf("generate url error %v", errG)
		return
	}

	log.Infof("get render images...")
	var wg sync.WaitGroup
	for i := 0; i < getCPUNum(); i++ {
		wg.Add(1)
		go func() {
			r.GetRenderImages()
			wg.Done()
		}()
	}
	wg.Wait()

}
