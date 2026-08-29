package fixture

import (
	"bytes"
	"errors"
	"fmt"
	"io"
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
	directory = filepath.Clean(directory)
	if err := os.MkdirAll(directory, 0o700); err != nil {
		return nil, fmt.Errorf("create result store: %w", err)
	}
	info, err := os.Lstat(directory)
	if err != nil {
		return nil, fmt.Errorf("inspect result store: %w", err)
	}
	if info.Mode()&os.ModeSymlink != 0 || !info.IsDir() {
		return nil, errors.New("result store must be a real directory, not a symlink")
	}
	if err := os.Chmod(directory, 0o700); err != nil {
		return nil, fmt.Errorf("restrict result store: %w", err)
	}
	info, err = os.Lstat(directory)
	if err != nil {
		return nil, fmt.Errorf("reinspect result store: %w", err)
	}
	if info.Mode()&os.ModeSymlink != 0 || !info.IsDir() || info.Mode().Perm() != 0o700 {
		return nil, errors.New("result store permissions or type are unsafe")
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
	published, err := publishResultNoReplace(temporaryName, finalName)
	if err != nil {
		_ = os.Remove(temporaryName)
		return nil, false, fmt.Errorf("publish result without replacement: %w", err)
	}
	if !published {
		if err := os.Remove(temporaryName); err != nil {
			return nil, false, fmt.Errorf("remove losing temporary result: %w", err)
		}
		existing, found, loadErr := s.loadLocked(requestHash)
		if loadErr != nil {
			return nil, false, loadErr
		}
		if !found {
			return nil, false, errors.New("result publication collision disappeared")
		}
		return existing, true, nil
	}

	if err := s.syncDirectory(); err != nil {
		return nil, false, err
	}
	if err := os.Remove(temporaryName); err != nil {
		return nil, false, fmt.Errorf("remove published temporary result: %w", err)
	}
	if err := s.syncDirectory(); err != nil {
		return nil, false, err
	}
	stored, found, err := s.loadLocked(requestHash)
	if err != nil {
		return nil, false, err
	}
	if !found || !bytes.Equal(stored, canonicalResult) {
		return nil, false, errors.New("published result bytes changed unexpectedly")
	}
	return stored, false, nil
}

func publishResultNoReplace(temporaryName, finalName string) (bool, error) {
	if err := os.Link(temporaryName, finalName); err != nil {
		if errors.Is(err, fs.ErrExist) {
			return false, nil
		}
		return false, err
	}
	return true, nil
}

func (s *ResultStore) syncDirectory() error {
	directory, err := os.Open(s.directory)
	if err != nil {
		return fmt.Errorf("open result directory for fsync: %w", err)
	}
	if err := directory.Sync(); err != nil {
		_ = directory.Close()
		return fmt.Errorf("fsync result directory: %w", err)
	}
	if err := directory.Close(); err != nil {
		return fmt.Errorf("close result directory: %w", err)
	}
	return nil
}

func (s *ResultStore) loadLocked(requestHash string) ([]byte, bool, error) {
	path := s.resultPath(requestHash)
	linkInfo, err := os.Lstat(path)
	if errors.Is(err, fs.ErrNotExist) {
		return nil, false, nil
	}
	if err != nil {
		return nil, false, fmt.Errorf("inspect cached result: %w", err)
	}
	if linkInfo.Mode()&os.ModeSymlink != 0 || !linkInfo.Mode().IsRegular() {
		return nil, false, errors.New("cached result is not a regular non-symlink file")
	}
	if linkInfo.Mode().Perm() != 0o600 {
		return nil, false, errors.New("cached result permissions are unsafe")
	}
	if linkInfo.Size() <= 0 || linkInfo.Size() > int64(MaxResponseBytes) {
		return nil, false, errors.New("cached result size is invalid")
	}
	file, err := os.Open(path)
	if err != nil {
		return nil, false, fmt.Errorf("open cached result: %w", err)
	}
	openedInfo, err := file.Stat()
	if err != nil {
		_ = file.Close()
		return nil, false, fmt.Errorf("inspect opened cached result: %w", err)
	}
	if !os.SameFile(linkInfo, openedInfo) {
		_ = file.Close()
		return nil, false, errors.New("cached result changed during open")
	}
	payload, readErr := io.ReadAll(io.LimitReader(file, int64(MaxResponseBytes)+1))
	closeErr := file.Close()
	if readErr != nil {
		return nil, false, fmt.Errorf("read cached result: %w", readErr)
	}
	if closeErr != nil {
		return nil, false, fmt.Errorf("close cached result: %w", closeErr)
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
