package fixture

import (
	"bytes"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestPublishResultNoReplacePreservesCommittedBytes(t *testing.T) {
	directory := t.TempDir()
	temporary := filepath.Join(directory, ".result-new.tmp")
	final := filepath.Join(directory, "result.json")
	if err := os.WriteFile(temporary, []byte("new result"), 0o600); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(final, []byte("committed result"), 0o600); err != nil {
		t.Fatal(err)
	}
	published, err := publishResultNoReplace(temporary, final)
	if err != nil {
		t.Fatal(err)
	}
	if published {
		t.Fatal("no-replace publication reported success over an existing result")
	}
	committed, err := os.ReadFile(final)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(committed, []byte("committed result")) {
		t.Fatalf("existing result was replaced: %q", committed)
	}
}

func TestResultStoreRejectsSymlinkedResult(t *testing.T) {
	root := t.TempDir()
	store, err := NewResultStore(filepath.Join(root, "results"))
	if err != nil {
		t.Fatal(err)
	}
	requestHash := strings.Repeat("a", 64)
	target := filepath.Join(root, "outside.json")
	if err := os.WriteFile(target, []byte(`{"request_hash":"`+requestHash+`"}`), 0o600); err != nil {
		t.Fatal(err)
	}
	if err := os.Symlink(target, store.resultPath(requestHash)); err != nil {
		t.Skipf("symlinks unavailable: %v", err)
	}
	if _, _, err := store.Load(requestHash); err == nil {
		t.Fatal("symlinked result was accepted")
	}
}

func TestResultStoreRejectsSymlinkDirectory(t *testing.T) {
	root := t.TempDir()
	realDirectory := filepath.Join(root, "real-results")
	if err := os.Mkdir(realDirectory, 0o700); err != nil {
		t.Fatal(err)
	}
	linkDirectory := filepath.Join(root, "linked-results")
	if err := os.Symlink(realDirectory, linkDirectory); err != nil {
		t.Skipf("symlinks unavailable: %v", err)
	}
	if _, err := NewResultStore(linkDirectory); err == nil {
		t.Fatal("symlinked result directory was accepted")
	}
}
