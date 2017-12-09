snapshot-collector
------
**This directory for capture Grafana's URLs to genertate images**
### Attentions
- **make sure grafana server have  shared object file `libfontconfig.so.1`**
	- debian/ubuntu:  `apt-get install -y libfontconfig freetype-devel fontconfig-devel fontconfig`
	- centos: `yum install -y fontconfig freetype freetype-devel fontconfig-devel libstdc++`

### Build it
- install Golang
- `make`

**compiling binary is bin/snapshot-collector**

### Parameters
```
Usage of ./bin/snapshot-collector:
  -address string
    	input grafana address (default "http://192.168.2.188:3000")
  -dashboard string
    	input dashboard name
  -end string
    	input end time,default is now (default "2017-12-04 10:20:34")
  -name string
    	input panel name
  -password string
    	input granfana password (default "admin")
  -renderurl string
    	input render url
  -start string
    	input start time, default is 3 days ago (default "2017-12-01 10:20:34")
  -timeout int
    	input execute query timeout (default 30)
  -user string
    	input granfana user (default "admin")
```


### Examples:
- collector all of panels
	- `./snapshot-collector -address "http://192.168.2.188:3000" -user "admin" -password="admin" -start "2017-12-01 10:20:34" -end "2017-12-04 10:20:34"`
- collector all of panels on `Test-Cluster-TiKV` dashboard
	- `./snapshot-collector -address "http://192.168.2.188:3000"  -user "admin" -password="admin" -dashboard "Test-Cluster-TiKV"`
- collector one panel with URL
	- `./snapshot-collector  -user "admin" -password="admin" -renderurl "http://192.168.2.188:3000/dashboard/db/test-cluster-disk-performance?panelId=11&fullscreen&orgId=1"`
- collector one panel with name
	- `./snapshot-collector -address "http://192.168.2.188:3000"  -user "admin" -password="admin" -name "Disk Latency"`
	
