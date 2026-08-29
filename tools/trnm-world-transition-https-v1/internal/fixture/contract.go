package fixture

import (
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"fmt"
	"math"
	"regexp"
	"strings"
	"unicode/utf8"
)

const (
	ContractVersion      = "trnm_world_transition_v1"
	RequestHashDomain    = "trnm.world.transition.request.v1"
	TransitionHashDomain = "trnm.world.transition.accepted.v1"
	OutcomeHashDomain    = "trnm.world.outcome.v1"
	SupportedRuleset     = "blackbox-ruleset-v1"
	SupportedContent     = "blackbox-content-v1"
	StateSchemaID        = "trnm.blackbox.state.v1"
	CommandSchemaID      = "trnm.blackbox.move.v1"
	ReplaySchemaID       = "trnm.blackbox.replay.v1"
	OutcomeSchemaID      = "trnm.blackbox.outcome.v1"
	MaxStateBytes        = 2 * 1024 * 1024
	MaxCommandBytes      = 128 * 1024
	MaxReplayBytes       = 2 * 1024 * 1024
	MaxOutcomeBytes      = 512 * 1024
	MaxRequestBytes      = MaxStateBytes + MaxCommandBytes + 16*1024
	MaxResponseBytes     = MaxStateBytes + MaxReplayBytes + MaxOutcomeBytes + 32*1024
)

var (
	identifierPattern      = regexp.MustCompile(`^[A-Za-z0-9._:/+@-]{1,160}$`)
	hex64Pattern           = regexp.MustCompile(`^[0-9a-f]{64}$`)
	forbiddenAuthorityKeys = map[string]struct{}{
		"nakama_session_token":          {},
		"nakama_private_key":            {},
		"match_authority_private_key":   {},
		"canonical_archive_root":        {},
		"chain_finality":                {},
		"chain_app_hash":                {},
		"match_completed_v1":            {},
		"participant_admission_receipt": {},
		"participant_roster":            {},
		"participant_role":              {},
		"global_event_cursor":           {},
		"global_event_sequence":         {},
		"match_version":                 {},
		"participant_sequence":          {},
		"command_idempotency_key":       {},
		"completion_signature":          {},
		"authority_private_key":         {},
		"wallet":                        {},
		"settlement":                    {},
	}
)

type Payload struct {
	SchemaID      string
	CanonicalJSON any
	Canonical     []byte
	SHA256        string
}

func (p Payload) Wire() map[string]any {
	return map[string]any{
		"canonical_json": p.CanonicalJSON,
		"schema_id":      p.SchemaID,
		"sha256":         p.SHA256,
	}
}

type Command struct {
	CommandID string
	Payload   Payload
}

type Request struct {
	TransitionID    string
	RulesetRevision string
	ContentRevision string
	ExpectedTick    int64
	PreviousState   Payload
	Command         Command
	Canonical       []byte
	RequestHash     string
}

type ContractResult struct {
	Canonical   []byte
	Accepted    bool
	RequestHash string
}

func ParseRequest(raw []byte) (Request, error) {
	value, err := ParseCanonical(raw, MaxRequestBytes)
	if err != nil {
		return Request{}, err
	}
	root, err := exactObject(value, []string{
		"command",
		"content_revision",
		"contract_version",
		"expected_tick",
		"previous_state",
		"ruleset_revision",
		"transition_id",
	}, "request")
	if err != nil {
		return Request{}, err
	}
	contractVersion, err := stringField(root, "contract_version")
	if err != nil || contractVersion != ContractVersion {
		return Request{}, errors.New("unsupported contract_version")
	}
	transitionID, err := identifierField(root, "transition_id")
	if err != nil {
		return Request{}, err
	}
	rulesetRevision, err := identifierField(root, "ruleset_revision")
	if err != nil {
		return Request{}, err
	}
	contentRevision, err := identifierField(root, "content_revision")
	if err != nil {
		return Request{}, err
	}
	expectedTick, ok := root["expected_tick"].(int64)
	if !ok || expectedTick < 0 {
		return Request{}, errors.New("expected_tick must be a non-negative signed-i64")
	}
	previousState, err := parsePayload(root["previous_state"], MaxStateBytes, "previous_state")
	if err != nil {
		return Request{}, err
	}
	commandObject, err := exactObject(root["command"], []string{"command_id", "payload"}, "command")
	if err != nil {
		return Request{}, err
	}
	commandID, err := identifierField(commandObject, "command_id")
	if err != nil {
		return Request{}, err
	}
	commandPayload, err := parsePayload(commandObject["payload"], MaxCommandBytes, "command.payload")
	if err != nil {
		return Request{}, err
	}
	return Request{
		TransitionID:    transitionID,
		RulesetRevision: rulesetRevision,
		ContentRevision: contentRevision,
		ExpectedTick:    expectedTick,
		PreviousState:   previousState,
		Command:         Command{CommandID: commandID, Payload: commandPayload},
		Canonical:       append([]byte(nil), raw...),
		RequestHash:     domainHash(RequestHashDomain, raw),
	}, nil
}

func Execute(request Request) (ContractResult, error) {
	if request.RulesetRevision != SupportedRuleset {
		return reject(request, "unknown_ruleset_revision", "ruleset revision is not supported", false)
	}
	if request.ContentRevision != SupportedContent {
		return reject(request, "unknown_content_revision", "content revision is not supported", false)
	}
	if request.PreviousState.SchemaID != StateSchemaID {
		return reject(request, "invalid_request", "previous state schema is not supported", false)
	}
	if request.Command.Payload.SchemaID != CommandSchemaID {
		return reject(request, "invalid_command", "command schema is not supported", false)
	}
	state, err := exactObject(request.PreviousState.CanonicalJSON, []string{"counter"}, "previous_state.canonical_json")
	if err != nil {
		return reject(request, "invalid_request", err.Error(), false)
	}
	counter, ok := state["counter"].(int64)
	if !ok {
		return reject(request, "invalid_request", "state counter must be a signed-i64", false)
	}
	command, err := exactObject(request.Command.Payload.CanonicalJSON, []string{"delta", "kind"}, "command.payload.canonical_json")
	if err != nil {
		return reject(request, "invalid_command", err.Error(), false)
	}
	kind, ok := command["kind"].(string)
	if !ok {
		return reject(request, "invalid_command", "command kind must be a string", false)
	}
	if kind == "reject" {
		return reject(request, "domain_rejected", "fixture command requested deterministic rejection", false)
	}
	if kind != "advance" {
		return reject(request, "invalid_command", "command kind is not supported", false)
	}
	delta, ok := command["delta"].(int64)
	if !ok || delta < -1_000_000 || delta > 1_000_000 {
		return reject(request, "invalid_command", "command delta is outside the fixture range", false)
	}
	if (delta > 0 && counter > math.MaxInt64-delta) || (delta < 0 && counter < math.MinInt64-delta) {
		return reject(request, "domain_rejected", "counter arithmetic would overflow signed-i64", false)
	}
	if request.ExpectedTick == math.MaxInt64 {
		return reject(request, "resource_budget_exceeded", "expected_tick cannot advance within signed-i64", false)
	}
	nextCounter := counter + delta
	nextTick := request.ExpectedTick + 1
	nextState, err := newPayload(StateSchemaID, map[string]any{"counter": nextCounter}, MaxStateBytes)
	if err != nil {
		return ContractResult{}, err
	}
	replay, err := newPayload(ReplaySchemaID, map[string]any{
		"applied_command_ids": []any{request.Command.CommandID},
		"next_tick":           nextTick,
		"request_hash":        request.RequestHash,
	}, MaxReplayBytes)
	if err != nil {
		return ContractResult{}, err
	}
	outcome, err := newPayload(OutcomeSchemaID, map[string]any{
		"counter": nextCounter,
		"result":  "advanced",
	}, MaxOutcomeBytes)
	if err != nil {
		return ContractResult{}, err
	}
	outcomeMaterial, err := CanonicalJSON(map[string]any{
		"content_revision":     request.ContentRevision,
		"outcome_payload_hash": outcome.SHA256,
		"outcome_schema_id":    outcome.SchemaID,
		"ruleset_revision":     request.RulesetRevision,
	})
	if err != nil {
		return ContractResult{}, err
	}
	worldOutcomeHash := domainHash(OutcomeHashDomain, outcomeMaterial)
	facts := map[string]any{
		"content_revision":    request.ContentRevision,
		"contract_version":    ContractVersion,
		"next_state":          nextState.Wire(),
		"next_tick":           nextTick,
		"outcome_material":    outcome.Wire(),
		"previous_state_hash": request.PreviousState.SHA256,
		"replay_material":     replay.Wire(),
		"request_hash":        request.RequestHash,
		"ruleset_revision":    request.RulesetRevision,
		"transition_id":       request.TransitionID,
		"world_outcome_hash":  worldOutcomeHash,
	}
	canonicalFacts, err := CanonicalJSON(facts)
	if err != nil {
		return ContractResult{}, err
	}
	facts["world_transition_hash"] = domainHash(TransitionHashDomain, canonicalFacts)
	canonical, err := CanonicalJSON(facts)
	if err != nil {
		return ContractResult{}, err
	}
	if len(canonical) > MaxResponseBytes {
		return reject(request, "resource_budget_exceeded", "fixture result exceeds response limit", false)
	}
	return ContractResult{Canonical: canonical, Accepted: true, RequestHash: request.RequestHash}, nil
}

func reject(request Request, code, detail string, retryable bool) (ContractResult, error) {
	if retryable != (code == "internal_unavailable") {
		return ContractResult{}, errors.New("retryable flag disagrees with the stable error catalogue")
	}
	result := map[string]any{
		"code":             code,
		"contract_version": ContractVersion,
		"detail":           boundedDetail(detail),
		"request_hash":     request.RequestHash,
		"retryable":        retryable,
		"transition_id":    request.TransitionID,
	}
	canonical, err := CanonicalJSON(result)
	if err != nil {
		return ContractResult{}, err
	}
	return ContractResult{Canonical: canonical, Accepted: false, RequestHash: request.RequestHash}, nil
}

func parsePayload(value any, maximumBytes int, label string) (Payload, error) {
	object, err := exactObject(value, []string{"canonical_json", "schema_id", "sha256"}, label)
	if err != nil {
		return Payload{}, err
	}
	schemaID, err := identifierField(object, "schema_id")
	if err != nil {
		return Payload{}, fmt.Errorf("%s: %w", label, err)
	}
	digest, err := stringField(object, "sha256")
	if err != nil || !hex64Pattern.MatchString(digest) {
		return Payload{}, fmt.Errorf("%s.sha256 must be lowercase 64-hex", label)
	}
	canonical, err := CanonicalJSON(object["canonical_json"])
	if err != nil {
		return Payload{}, fmt.Errorf("%s.canonical_json: %w", label, err)
	}
	if len(canonical) == 0 || len(canonical) > maximumBytes {
		return Payload{}, fmt.Errorf("%s.canonical_json exceeds its byte limit", label)
	}
	if _, ok := object["canonical_json"].(map[string]any); !ok {
		if _, ok := object["canonical_json"].([]any); !ok {
			return Payload{}, fmt.Errorf("%s.canonical_json must be an object or array", label)
		}
	}
	if sha256Hex(canonical) != digest {
		return Payload{}, fmt.Errorf("%s payload hash mismatch", label)
	}
	if err := rejectAuthoritySurface(object["canonical_json"], label+".canonical_json"); err != nil {
		return Payload{}, err
	}
	return Payload{SchemaID: schemaID, CanonicalJSON: object["canonical_json"], Canonical: canonical, SHA256: digest}, nil
}

func newPayload(schemaID string, value any, maximumBytes int) (Payload, error) {
	if !identifierPattern.MatchString(schemaID) {
		return Payload{}, errors.New("payload schema ID is invalid")
	}
	if err := rejectAuthoritySurface(value, "generated payload"); err != nil {
		return Payload{}, err
	}
	canonical, err := CanonicalJSON(value)
	if err != nil {
		return Payload{}, err
	}
	if len(canonical) > maximumBytes {
		return Payload{}, errors.New("generated payload exceeds its byte limit")
	}
	return Payload{SchemaID: schemaID, CanonicalJSON: value, Canonical: canonical, SHA256: sha256Hex(canonical)}, nil
}

func exactObject(value any, fields []string, label string) (map[string]any, error) {
	object, ok := value.(map[string]any)
	if !ok || len(object) != len(fields) {
		return nil, fmt.Errorf("%s must contain the exact field set", label)
	}
	for _, field := range fields {
		if _, exists := object[field]; !exists {
			return nil, fmt.Errorf("%s is missing field %s", label, field)
		}
	}
	return object, nil
}

func stringField(object map[string]any, field string) (string, error) {
	value, ok := object[field].(string)
	if !ok {
		return "", fmt.Errorf("%s must be a string", field)
	}
	return value, nil
}

func identifierField(object map[string]any, field string) (string, error) {
	value, err := stringField(object, field)
	if err != nil {
		return "", err
	}
	if !identifierPattern.MatchString(value) {
		return "", fmt.Errorf("%s is not a canonical identifier", field)
	}
	return value, nil
}

func rejectAuthoritySurface(value any, location string) error {
	switch typed := value.(type) {
	case map[string]any:
		for key, child := range typed {
			if _, forbidden := forbiddenAuthorityKeys[strings.ToLower(key)]; forbidden {
				return fmt.Errorf("%s contains forbidden authority field %q", location, key)
			}
			if err := rejectAuthoritySurface(child, location+"."+key); err != nil {
				return err
			}
		}
	case []any:
		for index, child := range typed {
			if err := rejectAuthoritySurface(child, fmt.Sprintf("%s[%d]", location, index)); err != nil {
				return err
			}
		}
	}
	return nil
}

func boundedDetail(value string) string {
	if !utf8.ValidString(value) {
		return "request rejected"
	}
	var builder strings.Builder
	for _, character := range value {
		if character < 0x20 || character == 0x7f {
			if builder.Len()+1 > 256 {
				break
			}
			builder.WriteByte(' ')
			continue
		}
		encodedBytes := utf8.RuneLen(character)
		if encodedBytes < 0 || builder.Len()+encodedBytes > 256 {
			break
		}
		builder.WriteRune(character)
	}
	result := strings.TrimSpace(builder.String())
	if result == "" {
		return "request rejected"
	}
	return result
}

func domainHash(domain string, material []byte) string {
	preimage := make([]byte, 0, len(domain)+1+len(material))
	preimage = append(preimage, domain...)
	preimage = append(preimage, '\n')
	preimage = append(preimage, material...)
	return sha256Hex(preimage)
}

func sha256Hex(value []byte) string {
	digest := sha256.Sum256(value)
	return hex.EncodeToString(digest[:])
}
