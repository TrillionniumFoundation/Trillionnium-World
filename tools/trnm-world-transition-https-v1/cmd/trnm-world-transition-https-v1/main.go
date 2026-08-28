package main

import (
	"context"
	"errors"
	"log"
	"net/http"
	"os"
	"os/signal"
	"strconv"
	"strings"
	"syscall"
	"time"

	"github.com/TrillionniumFoundation/Trillionnium-World/tools/trnm-world-transition-https-v1/internal/fixture"
)

const (
	envListen      = "TRNM_WORLD_FIXTURE_LISTEN"
	envTLSCert     = "TRNM_WORLD_FIXTURE_TLS_CERT"
	envTLSKey      = "TRNM_WORLD_FIXTURE_TLS_KEY"
	envBearer      = "TRNM_WORLD_FIXTURE_BEARER_TOKEN"
	envResultStore = "TRNM_WORLD_FIXTURE_RESULT_DIR"
	envMaxRequest  = "TRNM_WORLD_FIXTURE_MAX_REQUEST_BYTES"
)

func main() {
	config, err := loadConfig()
	if err != nil {
		log.Fatalf("World transition HTTPS fixture configuration rejected: %v", err)
	}
	service, err := fixture.NewServer(config)
	if err != nil {
		log.Fatalf("World transition HTTPS fixture initialization rejected: %v", err)
	}
	server := service.HTTPServer()
	failures := make(chan error, 1)
	go func() {
		log.Printf("World transition HTTPS fixture listening on %s", config.ListenAddress)
		failures <- server.ListenAndServeTLS(config.TLSCertificate, config.TLSPrivateKey)
	}()

	signals := make(chan os.Signal, 1)
	signal.Notify(signals, syscall.SIGINT, syscall.SIGTERM)
	select {
	case received := <-signals:
		log.Printf("World transition HTTPS fixture received %s", received)
		shutdownContext, cancel := context.WithTimeout(context.Background(), 10*time.Second)
		defer cancel()
		if err := server.Shutdown(shutdownContext); err != nil {
			log.Fatalf("World transition HTTPS fixture shutdown failed: %v", err)
		}
	case err := <-failures:
		if err != nil && !errors.Is(err, http.ErrServerClosed) {
			log.Fatalf("World transition HTTPS fixture stopped unexpectedly: %v", err)
		}
	}
}

func loadConfig() (fixture.ServerConfig, error) {
	listen := strings.TrimSpace(os.Getenv(envListen))
	if listen == "" {
		listen = ":7443"
	}
	maximum := int64(fixture.MaxRequestBytes)
	if raw := strings.TrimSpace(os.Getenv(envMaxRequest)); raw != "" {
		parsed, err := strconv.ParseInt(raw, 10, 64)
		if err != nil {
			return fixture.ServerConfig{}, errors.New(envMaxRequest + " must be a base-10 integer")
		}
		maximum = parsed
	}
	return fixture.ServerConfig{
		ListenAddress:       listen,
		TLSCertificate:      strings.TrimSpace(os.Getenv(envTLSCert)),
		TLSPrivateKey:       strings.TrimSpace(os.Getenv(envTLSKey)),
		BearerToken:         os.Getenv(envBearer),
		ResultDirectory:     strings.TrimSpace(os.Getenv(envResultStore)),
		MaximumRequestBytes: maximum,
		ReadHeaderTimeout:   5 * time.Second,
		ReadTimeout:         15 * time.Second,
		WriteTimeout:        15 * time.Second,
		IdleTimeout:         30 * time.Second,
	}, nil
}
