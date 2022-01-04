package main

import (
	"testing"
)

func TestCombinePaths(t *testing.T) {
	root := "/mnt/msata/content"
	dirs := []string{"plants/technology", "plants/identity"}
	result := combinePaths(root, dirs)
	if len(result) < 1 {
		t.Fatal("no paths combined")
	}
	if result[0] != "/mnt/msata/content/plants/technology" || result[1] != "/mnt/msata/content/plants/identity" {
		t.Fatal("paths not a match")
	}
}
