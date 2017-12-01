package main

import (
	"github.com/ngaut/log"
	sc "github.com/pingcap/tidb-inspector-tools/metrics/snapshot-collector"
	"sync"
)

func main() {
	r, err := sc.InitRun()
	if err != nil {
		log.Errorf("can not init,error %v", err)
		return
	}
	if err := r.PreData(); err != nil {
		log.Errorf("get error %v", err)
		return
	}
	var wg sync.WaitGroup
	close(r.imageURLs)
	for i := 0; i < sc.GetCPUNum(); i++ {
		wg.Add(1)
		go func() {
			r.GetRenderImages()
			wg.Done()
		}()
	}
	wg.Wait()
}
