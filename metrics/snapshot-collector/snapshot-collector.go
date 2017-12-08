package main

import (
	"github.com/ngaut/log"
	"sync"
)

func main() {
	r, err := InitRun()
	if err != nil {
		log.Errorf("can not init,error %v", err)
		return
	}
	if err := r.PreData(); err != nil {
		log.Errorf("get error %v", err)
		return
	}
	var wg sync.WaitGroup
	for i := 0; i < GetCPUNum(); i++ {
		wg.Add(1)
		go func() {
			r.GetRenderImages()
			wg.Done()
		}()
	}
	wg.Wait()
}
