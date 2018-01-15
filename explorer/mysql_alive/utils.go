package main

import (
	log "github.com/sirupsen/logrus"
	"os/exec"
	"syscall"
)

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
