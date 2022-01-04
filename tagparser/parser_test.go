package main

import (
	"testing"
)

func TestIsTagLine(t *testing.T) {
	var tests = map[string]bool{
		"tags = [\"fake\", \"tags\"]": true,
		"author = [\"Alex Bilson\"]":  false,
	}

	for line, want := range tests {
		result := isTagLine(line)
		if result != want {
			t.Fatalf("%v is a tag line but did not match", line)
		}
	}
}

func TestParseTags(t *testing.T) {
	var tests = map[string][]string{
		"tags = [fake,tags]": {"fake", "tags"},
	}

	for tags, want := range tests {
		result := parseTags(tags)
		if want[0] != result[0] || want[1] != result[1] {
			t.Fatalf("result %v did not match wanted %v", result, want)
		}
	}
}

func TestFirstWord(t *testing.T) {
	var tests = map[string]string{
		"tags = [fake,tags]":       "tags",
		"author = \"Alex Bilson\"": "author",
	}
	for text, want := range tests {
		result, err := firstWord(text)
		if err != nil {
			t.Fatal(err)
		}
		if result != want {
			t.Fatalf("%s does not match %s", result, want)
		}
	}
}

func TestParseFrontMatter(t *testing.T) {
	fm := []string{
		"author = \"Alex Bilson\"",
		"date = \"2021-07-10\"",
		"lastmod = \"2021-07-30\"",
		"epistemic = \"plant\"",
		"tags = [fake,tag]",
	}
	result, err := parseFrontMatter(fm)

	if err != nil {
		t.Fatal(err)
	}
	switch {
	case result.Author != "Alex Bilson":
		t.Fatalf("Should be Alex Bilson, was %s", result.Author)

	case result.Date != "2021-07-10":
		t.Fatalf("Should be 2021-07-10, was %s", result.Date)

	case result.LastMod != "2021-07-30":
		t.Fatalf("Should be 2021-07-30, was %s", result.LastMod)

	case result.Epistemic != Plant:
		t.Fatalf("Should be plant, was %s", result.Epistemic)

	case result.Tags[0] != "fake" || result.Tags[1] != "tag":
		t.Fatalf("Should be [fake, tag], was %s", result.Tags)
	}
}
