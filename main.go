package main

import (
	"context"
	"log"
	"net/http"
	"os"
	"os/signal"
	"time"

	"github.com/gorilla/mux"
	"github.com/mezni/gogenerator/handlers"
)

func main() {
	logger := log.New(os.Stdout, "generator ", log.LstdFlags)
	mainRouter := mux.NewRouter()
	mainRouter.HandleFunc("/api/health", handlers.HealthHandler).Methods("GET")
	mainRouter.HandleFunc("/api/event", handlers.GenerateEventsHandler).Methods("POST")

	srv := http.Server{
		Handler:      mainRouter,
		Addr:         ":9090",
		ErrorLog:     logger,
		WriteTimeout: 15 * time.Second,
		ReadTimeout:  15 * time.Second,
		IdleTimeout:  120 * time.Second,
	}

	go func() {
		logger.Println("Starting server on port 9090")

		err := srv.ListenAndServe()
		if err != nil {
			logger.Printf("Error starting server: %s\n", err)
			os.Exit(1)
		}
	}()

	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt)
	signal.Notify(c, os.Kill)

	sig := <-c
	log.Println("Got signal:", sig)

	ctx, _ := context.WithTimeout(context.Background(), 30*time.Second)
	srv.Shutdown(ctx)
}
