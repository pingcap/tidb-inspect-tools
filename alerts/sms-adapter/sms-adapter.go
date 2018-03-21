package main

import (
	"flag"
	"fmt"
	"github.com/juju/errors"
	"github.com/ngaut/log"
	"github.com/unrolled/render"
	"net"
	"net/http"
	"os"
	"os/signal"
	"strings"
	"sync"
	"syscall"
	"time"
)

var (
	port         = flag.Int("port", 28082, "http port")
	smsHost      = flag.String("sms-host", "127.0.0.1", "sms host")
	smsPort      = flag.Int("sms-port", 6661, "sms port")
	smsPackfmt   = flag.String("sms-packfmt", "V", "sms packfmt")
	smsTransCode = flag.String("sms-transcode", "", "sms transcode")
	smsRetval    = flag.String("sms-retval", "1", "sms retval")
	smsChanno    = flag.String("sms-channo", "", "sms channo")
	smsUsername  = flag.String("sms-username", "sa", "sms username")
	smsPasswd    = flag.String("sms-passwd", "sa", "sms passwd")
	smsMobile    = flag.String("sms-mobile", "", "sms mobile,delimiter by ';', max 10 numbers")
	smsMsgcont   = flag.String("sms-message", "test", "sms test message")
	smsImmedflag = flag.String("sms-immedflag", "1", "sms immedflag")
	smsSendTime  = flag.String("sms-sendtime", "0", "sms send time")
	smsBranchno  = flag.String("sms-branchno", "", "sms branchno")
	smsOperator  = flag.String("sms-operator", "system", "sms operator")
)

//Run base running
type Run struct {
	Rdr       *render.Render
	AlertMsgs chan *AlertData
}

func checkParams() error {
	if *smsPort == 0 || *smsHost == "" || *smsChanno == "" || *smsMobile == "" {
		return errors.Errorf("parmas  error")
	}
	return nil
}

func getValue(kv KV, key string) string {
	if val, ok := kv[key]; ok {
		return val
	}
	return ""
}

func (r *Run) sendData(data string) error {
	n, err := net.DialTimeout("tcp", fmt.Sprintf("%s:%d", *smsHost, *smsPort), 10*time.Second)
	if err != nil {
		return errors.Trace(err)
	}
	defer n.Close()
	log.Infof("send message:%s", data)
	if _, err := n.Write([]byte(data)); err != nil {
		return errors.Trace(err)
	}
	return nil
}

func (r *Run) gendata(msg string) (string, error) {
	/* data format
	   0189VZF01|1|77|sa|sa|15810270712|您data|1|0||system|
	data format */
	data := []string{
		fmt.Sprintf("%s%s", *smsPackfmt, *smsTransCode),
		fmt.Sprintf("%s", *smsRetval),
		fmt.Sprintf("%s", *smsChanno),
		fmt.Sprintf("%s", *smsUsername),
		fmt.Sprintf("%s", *smsPasswd),
		fmt.Sprintf("%s", *smsMobile),
		fmt.Sprintf("%s", msg),
		fmt.Sprintf("%s", *smsImmedflag),
		fmt.Sprintf("%s", *smsSendTime),
		fmt.Sprintf("%s", *smsBranchno),
		fmt.Sprintf("%s", *smsOperator),
	}

	dataString := strings.Join(data, "|")
	return fmt.Sprintf("%04d%s", len(dataString), dataString), nil
}

func (r *Run) handlerMsg(originMsg string) error {
	dataString, err := r.gendata(originMsg)
	if err != nil {
		return errors.Trace(err)
	}
	return r.sendData(dataString)

}

func (r *Run) handlerAlert(ad *AlertData) {
	for _, at := range ad.Alerts {
		omsg := fmt.Sprintf("name:%s url:%s", getValue(at.Labels, "alertname"), at.GeneratorURL)
		if len(omsg) > 1024 {
			omsg = omsg[:1024]
		}
		if err := r.handlerMsg(omsg); err != nil {
			log.Errorf("can not send alert omsg %s with error %v", omsg, err)
		}

	}
}

func (r *Run) scheduler() {
	for {
		lenAlertMsgs := len(r.AlertMsgs)
		if lenAlertMsgs > 0 {
			for i := 0; i < lenAlertMsgs; i++ {
				r.handlerAlert(<-r.AlertMsgs)
			}
		}
		time.Sleep(3 * time.Second)
	}
}

func main() {
	flag.Parse()
	r := &Run{
		AlertMsgs: make(chan *AlertData, 1000),
	}
	if err := checkParams(); err != nil {
		log.Errorf("params error %v", err)
		return
	}
	if *smsMsgcont != "" {
		if err := r.handlerMsg(*smsMsgcont); err != nil {
			log.Errorf("can not send alert omsg %s with error %v", *smsMsgcont, err)
			return
		}
		log.Infof("send message successful")
		return
	}
	go r.scheduler()
	go func() {
		log.Infof("create http server")
		r.CreateRender()
		http.ListenAndServe(fmt.Sprintf(":%d", *port), r.CreateRouter())
	}()

	sc := make(chan os.Signal, 1)
	signal.Notify(sc,
		syscall.SIGHUP,
		syscall.SIGINT,
		syscall.SIGTERM,
		syscall.SIGQUIT)

	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		sig := <-sc
		log.Errorf("Got signal [%d] to exit.", sig)
		wg.Done()
	}()

	wg.Wait()
}
