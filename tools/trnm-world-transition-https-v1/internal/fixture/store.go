package fixture

import (
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"sync"
)

type ResultStore struct {
	directory string
	mu        sync.Mutex
}

func NewResultStore(directory string) (*ResultStore, error) {
	if directory == "" || !filepath.IsAbs(directory) {
		return nil, errors.New("result store directory must be an absolute path")
	}
	if err := os.MkdirAll(directory, 0o700); err != nil {
		return nil, fmt.Errorf("create result store: %w", err)
	}
	if err := os.Chmod(directory, 0o700); err != nil {
		return nil, fmt.Errorf("restrict result store: %w", err)
	}
	return &ResultStore{directory: directory}, nil
}

func (s *ResultStore) Load(requestHash string) ([]byte, bool, error) {
	if s == nil || !hex64Pattern.MatchString(requestHash) {
		return nil, false, errors.New("request hash is invalid")
	}
	s.mu.Lock()
	defer s.mu.Unlock()
	return s.loadLocked(requestHash)
}

func (s *ResultStore) LoadOrStore(requestHash string, canonicalResult []byte) ([]byte, bool, error) {
	if s == nil || !hex64Pattern.MatchString(requestHash) {
		return nil, false, errors.New("request hash is invalid")
	}
	if len(canonicalResult) == 0 || len(canonicalResult) > MaxResponseBytes {
		return nil, false, errors.New("canonical result size is invalid")
	}
	value, err := ParseCanonical(canonicalResult, MaxResponseBytes)
	if err != nil {
		return nil, false, fmt.Errorf("canonical result is invalid: %w", err)
	}
	object, ok := value.(map[string]any)
	if !ok || object["request_hash"] != requestHash {
		return nil, false, errors.New("canonical result request identity is inconsistent")
	}
	s.mu.Lock()
	defer s.mu.Unlock()
	if existing, found, err := s.loadLocked(requestHash); err != nil {
		return nil, false, err
	} else if found {
		return existing, true, nil
	}

	temporary, err := os.CreateTemp(s.directory, ".result-*.tmp")
	if err != nil {
		return nil, false, fmt.Errorf("create temporary result: %w", err)
	}
	temporaryName := temporary.Name()
	cleanup := func() {
		_ = temporary.Close()
		_ = os.Remove(temporaryName)
	}
	if err := temporary.Chmod(0o600); err != nil {
		cleanup()
		return nil, false, fmt.Errorf("restrict temporary result: %w", err)
	}
	if _, err := temporary.Write(canonicalResult); err != nil {
		cleanup()
		return nil, false, fmt.Errorf("write temporary result: %w", err)
	}
	if err := temporary.Sync(); err != nil {
		cleanup()
		return nil, false, fmt.Errorf("fsync temporary result: %w", err)
	}
	if err := temporary.Close(); err != nil {
		_ = os.Remove(temporaryName)
		return nil, false, fmt.Errorf("close temporary result: %w", err)
	}
	finalName := s.resultPath(requestHash)
	if err := os.Rename(temporaryName, finalName); err != nil {
		_ = os.Remove(temporaryName)
		return nil, false, fmt.Errorf("atomically publish result: %w", err)
	}
	if err := os.Chmod(finalName, 0o600); err != nil {
		return nil, false, fmt.Errorf("restrict published result: %w", err)
	}
	directory, err := os.Open(s.directory)
	if err != nil {
		return nil, false, fmt.Errorf("open result directory for fsync: %w", err)
	}
	if err := directory.Sync(); err != nil {
		_ = directory.Close()
		return nil, false, fmt.Errorf("fsync result directory: %w", err)
	}
	if err := directory.Close(); err != nil {
		return nil, false, fmt.Errorf("close result directory: %w", err)
	}
	return append([]byte(nil), canonicalResult...), false, nil
}

func (s *ResultStore) loadLocked(requestHash string) ([]byte, bool, error) {
	payload, err := os.ReadFile(s.resultPath(requestHash))
	if errors.Is(err, fs.ErrNotExist) {
		return nil, false, nil
	}
	if err != nil {
		return nil, false, fmt.Errorf("read cached result: %w", err)
	}
	if len(payload) == 0 || len(payload) > MaxResponseBytes {
		return nil, false, errors.New("cached result size is invalid")
	}
	value, err := ParseCanonical(payload, MaxResponseBytes)
	if err != nil {
		return nil, false, fmt.Errorf("cached result is corrupt: %w", err)
	}
	object, ok := value.(map[string]any)
	if !ok || object["request_hash"] != requestHash {
		return nil, false, errors.New("cached result request identity is inconsistent")
	}
	return payload, true, nil
}

func (s *ResultStore) resultPath(requestHash string) string {
	return filepath.Join(s.directory, requestHash+".json")
}
