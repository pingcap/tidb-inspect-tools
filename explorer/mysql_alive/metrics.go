package main

import (
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/push"
)

var (
	checkAlive = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Namespace: "tools",
			Subsystem: "tidb",
			Name:      "check_alive",
			Help:      "check tidb is alive.",
		}, []string{"status"})
)

func reportProm(instance string) error {
	return push.AddFromGatherer(
		"tools",
		map[string]string{"instance": instance},
		*metrics,
		prometheus.DefaultGatherer,
	)
}

func init() {
	prometheus.MustRegister(checkAlive)

}
