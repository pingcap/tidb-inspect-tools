package main

import (
	"database/sql"
	"flag"
	"fmt"
	_ "github.com/go-sql-driver/mysql"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/push"
	log "github.com/sirupsen/logrus"
	"os"
	"os/exec"
	"syscall"
	"time"
)

const (
	//TestQuery check tidb normal
	TestQuery = "SELECT count(*) FROM mysql.tidb"
)

var (
	host          = flag.String("host", "127.0.0.1", "tidb host")
	port          = flag.Int("port", 4000, "tidb port")
	user          = flag.String("user", "root", "tidb user")
	password      = flag.String("password", "", "tidb password")
	metrics       = flag.String("metrics", "", "pushgateway address")
	querytimeout  = flag.Int("query-timeout", 30, "execute query timeout")
	suffixCommand = flag.String("suffix-command", "", "when check tidb failed and run shell command")

	tidbFunctioning bool
	checkAlive      = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Namespace: "tools",
			Subsystem: "tidb",
			Name:      "check_alive",
			Help:      "check tidb is alive.",
		}, []string{"status"})
)

func checkParams() error {
	return nil
}

func mysqlTest() error {
	var dsn string
	if *password == "" {
		dsn = fmt.Sprintf("%s@tcp(%s:%d)/mysql?charset=utf8&timeout=%ds&writeTimeout=%ds&readTimeout=%ds",
			*user, *host, *port, *querytimeout, *querytimeout, *querytimeout)
	} else {
		dsn = fmt.Sprintf("%s:%s@tcp(%s:%d)/mysql?charset=utf8&timeout=%ds&writeTimeout=%ds&readTimeout=%ds",
			*user, *password, *host, *port, *querytimeout, *querytimeout, *querytimeout)
	}

	db, err := sql.Open("mysql", dsn)
	if err != nil {
		return err
	}
	defer db.Close()
	rs, err := db.Query(TestQuery)
	if err != nil {
		return err
	}
	defer rs.Close()
	var tc int64
	for rs.Next() {
		err := rs.Scan(&tc)
		if err != nil {
			return err
		}
		break
	}
	return nil
}

func reportProm(tf bool) error {
	instance, err := os.Hostname()
	if err != nil {
		instance = "Unknow_host"
	}
	prometheus.MustRegister(checkAlive)
	if !tf {
		checkAlive.WithLabelValues("fail").Inc()
	} else {
		checkAlive.WithLabelValues("success").Inc()
	}
	return push.AddFromGatherer(
		"tools",
		map[string]string{"instance": instance},
		*metrics,
		prometheus.DefaultGatherer,
	)
}

func runSuffixCommand(command string) (int, string, error) {
	cmd := exec.Command("/bin/bash", "-c", command)
	out, err := cmd.CombinedOutput()
	if err != nil {
		log.Errorf("exec command %s, output %v, and error %v", command, string(out), err)
		return -1, string(out), err
	}
	log.Infof("output %v", string(out))
	waitStatus := cmd.ProcessState.Sys().(syscall.WaitStatus)
	return int(waitStatus), string(out), err
}

func main() {
	flag.Parse()
	if err := checkParams(); err != nil {
		log.Fatalf("parms error : %v", err)
		return
	}
	var err error
	for i := 0; i < 3; i++ {
		err = mysqlTest()
		if err == nil {
			tidbFunctioning = true
			break
		}
		log.Errorf("check %d mysql failed, error : %v", i, err)
		time.Sleep(time.Second)
	}
	if *metrics != "" {
		if err := reportProm(tidbFunctioning); err != nil {
			log.Errorf("report prometheus error : %v", err)
		}
	}
	if *suffixCommand != "" {
		if exitCode, cmdOut, errCMD := runSuffixCommand(*suffixCommand); errCMD != nil || exitCode != 0 {
			log.Errorf("execute command error,exitCode %d error information %v", exitCode, cmdOut)
		}
	}

	if !tidbFunctioning {
		log.Errorf("tidb_need_restart_now")
	}
}
