package main

import (
	"testing"
)

func TestReadFrontMatter(t *testing.T) {
	path := "./data/why-do-i-write.md"
	result, err := readFrontMatter(path)
	if err != nil {
		t.Fatalf(`failed with error: %s`, err)
	}
	if len(result) < 1 {
		t.Fatalf(`no contents returned`)
	}
}

func TestReadFilesInDirectory(t *testing.T) {
	dir := "./data"
	result, err := readFilesInDirectory(dir)
	if err != nil {
		t.Fatalf(`failed with error: %s`, err)
	}
	if len(result) < 1 {
		t.Fatal("no results returned")
	}
	for _, v := range result {
		if v[0] == '_' {
			t.Fatalf(`returned result starting with underscore: %s`, v)
		}
	}
}
