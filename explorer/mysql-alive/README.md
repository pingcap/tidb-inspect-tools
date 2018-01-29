mysql-alive
------

**This tool is used to check tidb alive**
### Build
- install Golang(1.8.3+)
- `make`

**The target executable binary file is bin/mysql-alive**

### Usages
```
Usage of ./bin/mysql-alive:
  -host string
    	tidb host (default "127.0.0.1")
  -interval int
    	check alive interval (default 180)
  -kill-trigger
    	kill -9 tidb's process that listen port
  -log-file string
    	log filename
  -metrics string
    	metrics address
  -password string
    	tidb password
  -port int
    	tidb port (default 4000)
  -query-timeout int
    	execute query timeout (default 30)
  -suffix-command string
    	when check tidb failed and run shell command
  -user string
    	tidb user (default "root")
```


### Attentions
- `kill-trigger` is **very careful**, It's use command **`kill -9 $PID`**
- `kill-trigger` and `suffix-command` all setting
	- **`kill-trigger` execute first**


### Examples:
- check tidb 
	- `./bin/mysql-alive -interval 30 -kill-trigger  -log-file mysql_alive.log -metrics "10.0.3.6:9091" -port 4000 -suffix-command "/mnt/resource/tidb_cluster/scripts/start_tidb.sh"`

	
