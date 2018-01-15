package main

import (
	"database/sql"
	"flag"
	"fmt"
	_ "github.com/go-sql-driver/mysql"
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
)

var (
	host          = flag.String("host", "127.0.0.1", "tidb host")
	port          = flag.Int("port", 4000, "tidb port")
	user          = flag.String("user", "root", "tidb user")
	password      = flag.String("password", "", "tidb password")
	metrics       = flag.String("metrics", "", "metrics address")
	querytimeout  = flag.Int("query-timeout", 30, "execute query timeout")
	suffixCommand = flag.String("suffix-command", "", "when check tidb failed and run shell command")
	interval      = flag.Int64("interval", 180, "check alive interval")
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
			checkAlive.WithLabelValues("success").Inc()
			return true
		}
		log.Errorf("check %d mysql failed, error : %v", i, err)
		time.Sleep(time.Second)
	}
	checkAlive.WithLabelValues("fail").Inc()
	return false

}

func scheduler() {
	tk := time.NewTicker(time.Duration(*interval) * time.Second)

	for {
		select {
		case <-tk.C:
			tidbFunctioning := doTest()
			if *metrics != "" {
				if err := reportProm(); err != nil {
					log.Errorf("report prometheus error : %v", err)
				}
			}
			if !tidbFunctioning && *suffixCommand != "" {
				if exitCode, cmdOut, errCMD := runSuffixCommand(*suffixCommand); errCMD != nil || exitCode != 0 {
					log.Errorf("execute command error,exitCode %d error information %v", exitCode, cmdOut)
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
