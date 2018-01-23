package main

import (
	"database/sql"
	"flag"
	"fmt"
	_ "github.com/go-sql-driver/mysql"
	"github.com/prometheus/client_golang/prometheus"
	log "github.com/sirupsen/logrus"
	"os"
	"os/signal"
	"sync"
	"syscall"
	"time"
)

const (
	//TestQuery check tidb normal
	TestQuery = "SELECT count(*) FROM mysql.tidb"
	//KillCMD kill tidb process
	KillCMD = "kill -9"
)

var (
	host          = flag.String("host", "127.0.0.1", "tidb host")
	port          = flag.Int("port", 4000, "tidb port")
	user          = flag.String("user", "root", "tidb user")
	password      = flag.String("password", "", "tidb password")
	metrics       = flag.String("metrics", "", "metrics address")
	querytimeout  = flag.Int("query-timeout", 30, "execute query timeout")
	suffixCommand = flag.String("suffix-command", "", "when check tidb failed and run shell command")
	killTrigger   = flag.Bool("kill-trigger", false, "kill -9 tidb's process that listen port")
	interval      = flag.Int64("interval", 180, "check alive interval")
	logFile       = flag.String("log-file", "", "log filename")
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

func doTest() bool {
	var err error
	for i := 0; i < 3; i++ {
		err = mysqlTest()
		if err == nil {
			return true
		}
		log.Errorf("check %d mysql failed, error : %v", i, err)
		time.Sleep(3 * time.Second)
	}
	return false

}

func scheduler() {
	tk := time.NewTicker(time.Duration(*interval) * time.Second)
	instance := getHostName()

	for {
		select {
		case <-tk.C:
			tidbFunctioning := doTest()
			if *metrics != "" {
				if tidbFunctioning {
					checkAlive.WithLabelValues("success").Inc()
				} else {
					checkAlive.WithLabelValues("fail").Inc()
				}

				if err := reportProm(instance); err != nil {
					log.Errorf("report prometheus error : %v", err)
				}
			}
			if !tidbFunctioning {
				var CMD string
				if pid := getPidFromPort(int64(*port)); pid != 0 && *killTrigger {
					CMD = fmt.Sprintf("%s %d", KillCMD, pid)
				}
				if *suffixCommand != "" && CMD != "" {
					CMD = fmt.Sprintf("%s && %s ", CMD, *suffixCommand)
				} else if *suffixCommand != "" {
					CMD = *suffixCommand
				}
				if CMD != "" {
					exitCode, cmdOut, errCMD := runCommand(CMD)
					log.Infof("execute command result,exitCode %d information %v error %v", exitCode, cmdOut, errCMD)
				}
			}
			if !tidbFunctioning {
				log.Errorf("tidb_need_restart_now")
			}

		}
	}

}

func main() {
	flag.Parse()
	if err := checkParams(); err != nil {
		log.Fatalf("parms error : %v", err)
		return
	}
	if *logFile != "" {
		lf, err := os.OpenFile(*logFile, os.O_RDWR|os.O_CREATE|os.O_APPEND, 0660)
		if err != nil {
			fmt.Printf("can not open log file %s error %v", *logFile, err)
			return
		}
		log.SetOutput(lf)
		defer lf.Close()
	}
	log.SetLevel(log.DebugLevel)

	if *metrics != "" {
		prometheus.MustRegister(checkAlive)
	}

	go scheduler()

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
		log.Errorf("get signal [%d] and exit", sig)
		wg.Done()
	}()
	wg.Wait()
}
