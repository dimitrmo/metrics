package main

import (
	"fmt"
	"log"
	"net/http"

	"github.com/gorilla/websocket"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
	"github.com/prometheus/client_golang/prometheus/promhttp"
)

var upgrader = websocket.Upgrader{
	//CheckOrigin: func(r *http.Request) bool {
	//return false
	//},
}

var (
	liveUsers = promauto.NewGauge(prometheus.GaugeOpts{
		Name:        "total_live_users",
		Help:        "The total number of live users",
		ConstLabels: prometheus.Labels{"name": "ws-server-go"},
	})
)

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "Golang server: Welcome user")
	})

	http.HandleFunc("/ws", func(w http.ResponseWriter, r *http.Request) {
		c, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			log.Print("Upgrade error:", err)
			return
		}

		log.Println("User connected")
		liveUsers.Inc()

		defer c.Close()

		for {
			mt, message, err := c.ReadMessage()
			if err != nil {
				log.Println("Read error:", err)
				break
			}

			err = c.WriteMessage(mt, message)
			if err != nil {
				log.Println("Write error:", err)
				break
			}

			log.Println("Message echoed back to client")
		}

		log.Println("User disconnected")
		liveUsers.Dec()
	})

	http.Handle("/metrics", promhttp.Handler())

	log.Println("Starting golang http server")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
