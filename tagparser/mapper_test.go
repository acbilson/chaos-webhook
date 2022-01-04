package main

import (
	"testing"
)

func getFrontMatter() []FrontMatter {
	return []FrontMatter{
		{Tags: []string{"fake", "tag"}},
		{Tags: []string{"another", "tag"}},
		{Tags: []string{"tag", "another"}},
		{Tags: []string{"more", "tags", "please"}},
	}
}

func TestMapFrontMatterToNode(t *testing.T) {
	fms := getFrontMatter()
	result := MapFrontMatterToNode(fms)
	if len(result) == 0 {
		t.Fatal("should have returned at least one node")
	}
	for _, r := range result {
		if r.Name == "please" && r.Count != 1 {
			t.Fatalf("should have one please tag entry but had %d", r.Count)
		}
		if r.Name == "tag" && r.Count < 2 {
			t.Fatalf("should have had two tag entries but had %d", r.Count)
		}
	}
}

func TestMapFrontMatterToLink(t *testing.T) {
	fms := getFrontMatter()
	result := MapFrontMatterToLink(fms)
	if len(result) == 0 {
		t.Fatal("should have returned at least one link")
	}
}
