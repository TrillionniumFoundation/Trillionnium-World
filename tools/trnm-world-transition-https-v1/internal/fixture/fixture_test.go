package fixture

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestCanonicalProfileRejectsNonCanonicalInputs(t *testing.T) {
	valid := []byte(`{"a":[1,true,null,"é"],"b":{"c":"x"}}`)
	value, err := ParseCanonical(valid, len(valid))
	if err != nil {
		t.Fatalf("valid canonical JSON rejected: %v", err)
	}
	reencoded, err := CanonicalJSON(value)
	if err != nil || !bytes.Equal(valid, reencoded) {
		t.Fatalf("canonical round trip mismatch: %s %v", reencoded, err)
	}
	invalid := []string{
		` {"a":1}`,
		`{"b":1,"a":2}`,
		`{"a":1,"a":1}`,
		`{"a":1.0}`,
		`{"a":1e0}`,
		`{"a":-0}`,
		`{"a":"\u0061"}`,
		`{"a":9223372036854775808}`,
		`"scalar"`,
	}
	for _, raw := range invalid {
		if _, err := ParseCanonical([]byte(raw), -1); err == nil {
			t.Fatalf("noncanonical input accepted: %s", raw)
		}
	}
}

func TestAcceptedTransitionHashesAreSelfConsistent(t *testing.T) {
	requestBytes := fixtureRequest(t, SupportedRuleset, SupportedContent, map[string]any{"delta": int64(2), "kind": "advance"})
	request, err := ParseRequest(requestBytes)
	if err != nil {
		t.Fatal(err)
	}
	result, err := Execute(request)
	if err != nil {
		t.Fatal(err)
	}
	if !result.Accepted {
		t.Fatal("valid fixture transition was rejected")
	}
	value, err := ParseCanonical(result.Canonical, MaxResponseBytes)
	if err != nil {
		t.Fatal(err)
	}
	object := value.(map[string]any)
	if object["request_hash"] != request.RequestHash || object["previous_state_hash"] != request.PreviousState.SHA256 {
		t.Fatal("accepted result lost request or previous-state binding")
	}
	transitionHash := object["world_transition_hash"].(string)
	delete(object, "world_transition_hash")
	facts, err := CanonicalJSON(object)
	if err != nil {
		t.Fatal(err)
	}
	if transitionHash != domainHash(TransitionHashDomain, facts) {
		t.Fatal("accepted transition hash is inconsistent")
	}
	outcome := object["outcome_material"].(map[string]any)
	outcomeMaterial, err := CanonicalJSON(map[string]any{
		"content_revision":     request.ContentRevision,
		"outcome_payload_hash": outcome["sha256"],
		"outcome_schema_id":    outcome["schema_id"],
		"ruleset_revision":     request.RulesetRevision,
	})
	if err != nil {
		t.Fatal(err)
	}
	if object["world_outcome_hash"] != domainHash(OutcomeHashDomain, outcomeMaterial) {
		t.Fatal("accepted outcome hash is inconsistent")
	}
	nextState := object["next_state"].(map[string]any)
	nextStateJSON, err := CanonicalJSON(nextState["canonical_json"])
	if err != nil {
		t.Fatal(err)
	}
	if nextState["sha256"] != sha256Hex(nextStateJSON) {
		t.Fatal("next-state hash mismatch")
	}
}

func TestStableRejectedResult(t *testing.T) {
	requestBytes := fixtureRequest(t, SupportedRuleset, SupportedContent, map[string]any{"delta": int64(1), "kind": "reject"})
	request, err := ParseRequest(requestBytes)
	if err != nil {
		t.Fatal(err)
	}
	first, err := Execute(request)
	if err != nil {
		t.Fatal(err)
	}
	second, err := Execute(request)
	if err != nil {
		t.Fatal(err)
	}
	if first.Accepted || !bytes.Equal(first.Canonical, second.Canonical) {
		t.Fatal("deterministic rejection was not byte stable")
	}
	value, err := ParseCanonical(first.Canonical, MaxResponseBytes)
	if err != nil {
		t.Fatal(err)
	}
	object := value.(map[string]any)
	if object["code"] != "domain_rejected" || object["retryable"] != false || object["request_hash"] != request.RequestHash {
		t.Fatalf("unexpected rejection: %#v", object)
	}
}

func TestAuthoritySmugglingFailsBeforeExecution(t *testing.T) {
	state := map[string]any{"counter": int64(0), "global_event_sequence": int64(7)}
	stateCanonical, _ := CanonicalJSON(state)
	command := map[string]any{"delta": int64(1), "kind": "advance"}
	commandCanonical, _ := CanonicalJSON(command)
	request := map[string]any{
		"command": map[string]any{
			"command_id": "fixture-command-1",
			"payload": map[string]any{
				"canonical_json": command,
				"schema_id":      CommandSchemaID,
				"sha256":         sha256Hex(commandCanonical),
			},
		},
		"content_revision": SupportedContent,
		"contract_version": ContractVersion,
		"expected_tick":    int64(10),
		"previous_state": map[string]any{
			"canonical_json": state,
			"schema_id":      StateSchemaID,
			"sha256":         sha256Hex(stateCanonical),
		},
		"ruleset_revision": SupportedRuleset,
		"transition_id":    "fixture-transition-1",
	}
	raw, _ := CanonicalJSON(request)
	if _, err := ParseRequest(raw); err == nil {
		t.Fatal("authority-bearing state was accepted")
	}
}

func TestResultStoreReturnsExactCommittedBytes(t *testing.T) {
	store, err := NewResultStore(filepath.Join(t.TempDir(), "results"))
	if err != nil {
		t.Fatal(err)
	}
	requestBytes := fixtureRequest(t, SupportedRuleset, SupportedContent, map[string]any{"delta": int64(1), "kind": "advance"})
	request, _ := ParseRequest(requestBytes)
	result, _ := Execute(request)
	first, hit, err := store.LoadOrStore(request.RequestHash, result.Canonical)
	if err != nil || hit {
		t.Fatalf("first store failed: hit=%v err=%v", hit, err)
	}
	second, hit, err := store.LoadOrStore(request.RequestHash, []byte(`{"request_hash":"`+request.RequestHash+`"}`))
	if err != nil || !hit || !bytes.Equal(first, second) {
		t.Fatalf("cache did not preserve exact first result: hit=%v err=%v", hit, err)
	}
	info, err := os.Stat(filepath.Join(store.directory, request.RequestHash+".json"))
	if err != nil {
		t.Fatal(err)
	}
	if info.Mode().Perm() != 0o600 {
		t.Fatalf("result mode is %o", info.Mode().Perm())
	}
	entries, err := os.ReadDir(store.directory)
	if err != nil {
		t.Fatal(err)
	}
	for _, entry := range entries {
		if strings.HasSuffix(entry.Name(), ".tmp") {
			t.Fatalf("temporary result leaked: %s", entry.Name())
		}
	}
}

func TestHTTPHandlerAuthenticatesAndCaches(t *testing.T) {
	server, err := NewServer(ServerConfig{
		ListenAddress:   ":7443",
		TLSCertificate:  "/fixture/cert.pem",
		TLSPrivateKey:   "/fixture/key.pem",
		BearerToken:     strings.Repeat("a", 32),
		ResultDirectory: filepath.Join(t.TempDir(), "results"),
	})
	if err != nil {
		t.Fatal(err)
	}
	requestBytes := fixtureRequest(t, SupportedRuleset, SupportedContent, map[string]any{"delta": int64(1), "kind": "advance"})
	unauthorized := httptest.NewRequest(http.MethodPost, "/v1/transition", bytes.NewReader(requestBytes))
	unauthorized.Header.Set("Content-Type", "application/json")
	unauthorizedResponse := httptest.NewRecorder()
	server.Handler().ServeHTTP(unauthorizedResponse, unauthorized)
	if unauthorizedResponse.Code != http.StatusUnauthorized {
		t.Fatalf("unauthorized request returned %d", unauthorizedResponse.Code)
	}

	call := func() []byte {
		req := httptest.NewRequest(http.MethodPost, "/v1/transition", bytes.NewReader(requestBytes))
		req.Header.Set("Authorization", "Bearer "+strings.Repeat("a", 32))
		req.Header.Set("Content-Type", "application/json")
		response := httptest.NewRecorder()
		server.Handler().ServeHTTP(response, req)
		if response.Code != http.StatusOK {
			t.Fatalf("transition returned %d: %s", response.Code, response.Body.Bytes())
		}
		return append([]byte(nil), response.Body.Bytes()...)
	}
	first := call()
	second := call()
	if !bytes.Equal(first, second) {
		t.Fatal("retry returned different result bytes")
	}
	statsRequest := httptest.NewRequest(http.MethodGet, "/v1/stats", nil)
	statsRequest.Header.Set("Authorization", "Bearer "+strings.Repeat("a", 32))
	statsResponse := httptest.NewRecorder()
	server.Handler().ServeHTTP(statsResponse, statsRequest)
	var stats map[string]any
	if err := json.Unmarshal(statsResponse.Body.Bytes(), &stats); err != nil {
		t.Fatal(err)
	}
	if stats["cache_hits"].(float64) != 1 || stats["cutover_authorized"] != false || stats["public_online_enabled"] != false {
		t.Fatalf("unexpected fixture stats: %#v", stats)
	}
}

func TestBoundedDetailRespectsUTF8ByteLimit(t *testing.T) {
	value := strings.Repeat("a", 255) + "界"
	detail := boundedDetail(value)
	if len(detail) > 256 || !strings.HasSuffix(detail, "a") {
		t.Fatalf("bounded detail crossed the byte ceiling: bytes=%d", len(detail))
	}
	control := boundedDetail("failure\nreason")
	if strings.ContainsAny(control, "\n\r\t") {
		t.Fatalf("bounded detail retained a control character: %q", control)
	}
}

func TestTLSProfileIsExactlyTLS13(t *testing.T) {
	config := strictTLSConfig()
	if config.MinVersion != config.MaxVersion || config.MinVersion != 0x0304 {
		t.Fatalf("unexpected TLS profile: min=%x max=%x", config.MinVersion, config.MaxVersion)
	}
}

func fixtureRequest(t *testing.T, ruleset, content string, command map[string]any) []byte {
	t.Helper()
	state := map[string]any{"counter": int64(5)}
	stateCanonical, err := CanonicalJSON(state)
	if err != nil {
		t.Fatal(err)
	}
	commandCanonical, err := CanonicalJSON(command)
	if err != nil {
		t.Fatal(err)
	}
	request := map[string]any{
		"command": map[string]any{
			"command_id": "fixture-command-1",
			"payload": map[string]any{
				"canonical_json": command,
				"schema_id":      CommandSchemaID,
				"sha256":         sha256Hex(commandCanonical),
			},
		},
		"content_revision": content,
		"contract_version": ContractVersion,
		"expected_tick":    int64(10),
		"previous_state": map[string]any{
			"canonical_json": state,
			"schema_id":      StateSchemaID,
			"sha256":         sha256Hex(stateCanonical),
		},
		"ruleset_revision": ruleset,
		"transition_id":    "fixture-transition-1",
	}
	canonical, err := CanonicalJSON(request)
	if err != nil {
		t.Fatal(err)
	}
	return canonical
}
