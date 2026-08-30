package fixture

import (
	"context"
	"crypto/hmac"
	"crypto/sha256"
	"crypto/tls"
	"errors"
	"fmt"
	"io"
	"mime"
	"net/http"
	"strings"
	"sync/atomic"
	"time"
)

type ServerConfig struct {
	ListenAddress       string
	TLSCertificate      string
	TLSPrivateKey       string
	BearerToken         string
	ResultDirectory     string
	MaximumRequestBytes int64
	ReadHeaderTimeout   time.Duration
	ReadTimeout         time.Duration
	WriteTimeout        time.Duration
	IdleTimeout         time.Duration
}

type Server struct {
	config    ServerConfig
	store     *ResultStore
	requests  atomic.Uint64
	computed  atomic.Uint64
	cacheHits atomic.Uint64
	accepted  atomic.Uint64
	rejected  atomic.Uint64
}

func NewServer(config ServerConfig) (*Server, error) {
	if strings.TrimSpace(config.ListenAddress) == "" {
		return nil, errors.New("listen address is required")
	}
	if config.TLSCertificate == "" || config.TLSPrivateKey == "" {
		return nil, errors.New("TLS certificate and private key paths are required")
	}
	if len(config.BearerToken) < 32 || len(config.BearerToken) > 4096 || strings.TrimSpace(config.BearerToken) != config.BearerToken {
		return nil, errors.New("bearer token must contain 32 through 4096 non-trimmed bytes")
	}
	if config.MaximumRequestBytes == 0 {
		config.MaximumRequestBytes = MaxRequestBytes
	}
	if config.MaximumRequestBytes < 1024 || config.MaximumRequestBytes > MaxRequestBytes {
		return nil, errors.New("maximum request bytes exceed the contract ceiling")
	}
	if config.ReadHeaderTimeout <= 0 {
		config.ReadHeaderTimeout = 5 * time.Second
	}
	if config.ReadTimeout <= 0 {
		config.ReadTimeout = 15 * time.Second
	}
	if config.WriteTimeout <= 0 {
		config.WriteTimeout = 15 * time.Second
	}
	if config.IdleTimeout <= 0 {
		config.IdleTimeout = 30 * time.Second
	}
	store, err := NewResultStore(config.ResultDirectory)
	if err != nil {
		return nil, err
	}
	return &Server{config: config, store: store}, nil
}

func (s *Server) Handler() http.Handler {
	mux := http.NewServeMux()
	mux.HandleFunc("/healthz", s.handleHealth)
	mux.HandleFunc("/v1/transition", s.handleTransition)
	mux.HandleFunc("/v1/result/", s.handleResult)
	mux.HandleFunc("/v1/stats", s.handleStats)
	return securityHeaders(mux)
}

func (s *Server) HTTPServer() *http.Server {
	return &http.Server{
		Addr:              s.config.ListenAddress,
		Handler:           s.Handler(),
		ReadHeaderTimeout: s.config.ReadHeaderTimeout,
		ReadTimeout:       s.config.ReadTimeout,
		WriteTimeout:      s.config.WriteTimeout,
		IdleTimeout:       s.config.IdleTimeout,
		MaxHeaderBytes:    16 * 1024,
		TLSConfig:         strictTLSConfig(),
	}
}

func (s *Server) Serve() error {
	server := s.HTTPServer()
	return server.ListenAndServeTLS(s.config.TLSCertificate, s.config.TLSPrivateKey)
}

func (s *Server) Shutdown(ctx context.Context, server *http.Server) error {
	if server == nil {
		return nil
	}
	return server.Shutdown(ctx)
}

func strictTLSConfig() *tls.Config {
	return &tls.Config{
		MinVersion: tls.VersionTLS13,
		MaxVersion: tls.VersionTLS13,
	}
}

func (s *Server) handleHealth(response http.ResponseWriter, request *http.Request) {
	if request.Method != http.MethodGet {
		writeHTTPError(response, http.StatusMethodNotAllowed, "method_not_allowed")
		return
	}
	writeCanonicalResponse(response, http.StatusOK, map[string]any{
		"contract_version": ContractVersion,
		"status":           "ok",
	})
}

func (s *Server) handleTransition(response http.ResponseWriter, request *http.Request) {
	if request.Method != http.MethodPost {
		writeHTTPError(response, http.StatusMethodNotAllowed, "method_not_allowed")
		return
	}
	if !s.authorized(request) {
		writeHTTPError(response, http.StatusUnauthorized, "authorization_rejected")
		return
	}
	if encoding := strings.TrimSpace(request.Header.Get("Content-Encoding")); encoding != "" && !strings.EqualFold(encoding, "identity") {
		writeHTTPError(response, http.StatusUnsupportedMediaType, "content_encoding_not_supported")
		return
	}
	mediaType, _, err := mime.ParseMediaType(request.Header.Get("Content-Type"))
	if err != nil || !strings.EqualFold(mediaType, "application/json") {
		writeHTTPError(response, http.StatusUnsupportedMediaType, "content_type_must_be_application_json")
		return
	}
	if request.ContentLength > s.config.MaximumRequestBytes {
		writeHTTPError(response, http.StatusRequestEntityTooLarge, "request_too_large")
		return
	}
	limited := io.LimitReader(request.Body, s.config.MaximumRequestBytes+1)
	body, err := io.ReadAll(limited)
	if err != nil {
		writeHTTPError(response, http.StatusBadRequest, "request_body_unreadable")
		return
	}
	if int64(len(body)) > s.config.MaximumRequestBytes {
		writeHTTPError(response, http.StatusRequestEntityTooLarge, "request_too_large")
		return
	}
	s.requests.Add(1)
	parsed, err := ParseRequest(body)
	if err != nil {
		writeHTTPError(response, http.StatusBadRequest, "invalid_canonical_transition_request")
		return
	}
	if cached, found, loadErr := s.store.Load(parsed.RequestHash); loadErr != nil {
		writeHTTPError(response, http.StatusInternalServerError, "result_store_unavailable")
		return
	} else if found {
		s.cacheHits.Add(1)
		writeRawCanonical(response, http.StatusOK, cached)
		return
	}
	result, err := Execute(parsed)
	if err != nil {
		writeHTTPError(response, http.StatusInternalServerError, "deterministic_execution_failed")
		return
	}
	stored, cacheHit, err := s.store.LoadOrStore(parsed.RequestHash, result.Canonical)
	if err != nil {
		writeHTTPError(response, http.StatusInternalServerError, "result_store_unavailable")
		return
	}
	if cacheHit {
		s.cacheHits.Add(1)
	} else {
		s.computed.Add(1)
		if result.Accepted {
			s.accepted.Add(1)
		} else {
			s.rejected.Add(1)
		}
	}
	writeRawCanonical(response, http.StatusOK, stored)
}

func (s *Server) handleResult(response http.ResponseWriter, request *http.Request) {
	if request.Method != http.MethodGet {
		writeHTTPError(response, http.StatusMethodNotAllowed, "method_not_allowed")
		return
	}
	if !s.authorized(request) {
		writeHTTPError(response, http.StatusUnauthorized, "authorization_rejected")
		return
	}
	requestHash := strings.TrimPrefix(request.URL.Path, "/v1/result/")
	if !hex64Pattern.MatchString(requestHash) {
		writeHTTPError(response, http.StatusBadRequest, "invalid_request_hash")
		return
	}
	result, found, err := s.store.Load(requestHash)
	if err != nil {
		writeHTTPError(response, http.StatusInternalServerError, "result_store_unavailable")
		return
	}
	if !found {
		writeHTTPError(response, http.StatusNotFound, "result_not_found")
		return
	}
	writeRawCanonical(response, http.StatusOK, result)
}

func (s *Server) handleStats(response http.ResponseWriter, request *http.Request) {
	if request.Method != http.MethodGet {
		writeHTTPError(response, http.StatusMethodNotAllowed, "method_not_allowed")
		return
	}
	if !s.authorized(request) {
		writeHTTPError(response, http.StatusUnauthorized, "authorization_rejected")
		return
	}
	writeCanonicalResponse(response, http.StatusOK, map[string]any{
		"accepted":                     int64(s.accepted.Load()),
		"cache_hits":                   int64(s.cacheHits.Load()),
		"computed":                     int64(s.computed.Load()),
		"contract_version":             ContractVersion,
		"cutover_authorized":           false,
		"public_online_enabled":        false,
		"public_player_market_enabled": false,
		"rejected":                     int64(s.rejected.Load()),
		"requests":                     int64(s.requests.Load()),
	})
}

func (s *Server) authorized(request *http.Request) bool {
	const prefix = "Bearer "
	header := request.Header.Get("Authorization")
	if !strings.HasPrefix(header, prefix) {
		return false
	}
	presented := sha256.Sum256([]byte(strings.TrimPrefix(header, prefix)))
	expected := sha256.Sum256([]byte(s.config.BearerToken))
	return hmac.Equal(presented[:], expected[:])
}

func securityHeaders(next http.Handler) http.Handler {
	return http.HandlerFunc(func(response http.ResponseWriter, request *http.Request) {
		response.Header().Set("Cache-Control", "no-store")
		response.Header().Set("Content-Security-Policy", "default-src 'none'")
		response.Header().Set("X-Content-Type-Options", "nosniff")
		response.Header().Set("X-Frame-Options", "DENY")
		next.ServeHTTP(response, request)
	})
}

func writeHTTPError(response http.ResponseWriter, status int, code string) {
	writeCanonicalResponse(response, status, map[string]any{
		"code":             code,
		"contract_version": "trnm_world_transition_https_fixture_http_v1",
		"detail":           http.StatusText(status),
	})
}

func writeCanonicalResponse(response http.ResponseWriter, status int, value any) {
	canonical, err := CanonicalJSON(value)
	if err != nil {
		http.Error(response, "internal canonicalization failure", http.StatusInternalServerError)
		return
	}
	writeRawCanonical(response, status, canonical)
}

func writeRawCanonical(response http.ResponseWriter, status int, canonical []byte) {
	response.Header().Set("Content-Type", "application/json")
	response.Header().Set("Content-Length", fmt.Sprintf("%d", len(canonical)))
	response.WriteHeader(status)
	_, _ = response.Write(canonical)
}
