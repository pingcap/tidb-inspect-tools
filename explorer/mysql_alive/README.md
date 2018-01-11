mysql_alive
------

**This tool is used to check tidb alive**
### Build
- install Golang(1.8.3+)
- `make`

**The target executable binary file is bin/mysql_alive**

### Usages
```
Usage of ./mysql_alive:
  -host string
    	tidb host (default "127.0.0.1")
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


### Examples:
- check tidb 
	- `./mysql_alive -host 127.0.0.1  -port 4000  -metrics 10.0.3.6:9091`

	
