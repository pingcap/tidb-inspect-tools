package main

import (
	gonetstat "github.com/drael/GOnetstat"
	log "github.com/sirupsen/logrus"
	"os"
	"os/exec"
	"strconv"
	"strings"
	"syscall"
)

func getHostName() string {
	instance, err := os.Hostname()
	if err != nil {
		instance = "Unknownhost"
	}
	return instance
}

func runCommand(command string) (int, string, error) {
	cmd := exec.Command("/bin/bash", "-c", command)
	out, err := cmd.CombinedOutput()
	if err != nil {
		log.Errorf("exec command %s, output %v, and error %v", command, string(out), err)
		return -1, string(out), err
	}
	waitStatus := cmd.ProcessState.Sys().(syscall.WaitStatus)
	return int(waitStatus), string(out), err
}

func getPidFromPort(port int64) int64 {
	var pid int64
	if port == 0 {
		return pid
	}

	netstat := gonetstat.Tcp()
	netstat = append(netstat, gonetstat.Tcp6()...)
	for _, p := range netstat {
		if p.Port == port && !strings.Contains(p.Pid, "-") {
			if tPid, err := strconv.ParseInt(p.Pid, 10, 64); err != nil {
				log.Errorf("can not tranform pid %s with error %v", p.Pid, err)
			} else {
				pid = tPid
			}
			break
		}
	}
	return pid
}
