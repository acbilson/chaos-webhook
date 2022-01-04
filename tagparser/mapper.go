package main

import (
	"fmt"
	"sort"
)

func MapFrontMatterToNode(fms []FrontMatter) []Node {
	nodes := make([]Node, 0)
	matterMap := make(map[string]int)
	for _, fm := range fms {
		for _, tag := range fm.Tags {
			matterMap[tag]++
		}
	}

	for tag, cnt := range matterMap {
		node := Node{
			Name:  tag,
			Count: cnt,
			Path:  fmt.Sprintf("/tags/%s/", tag),
		}
		nodes = append(nodes, node)
	}
	return nodes
}

func MapFrontMatterToLink(fms []FrontMatter) []Link {
	linkMap := make(map[Link]struct{})
	for _, fm := range fms {
		// skips single tags
		if len(fm.Tags) > 1 {
			// by sorting, prevents duplicates
			sort.Strings(fm.Tags)
			for i, tag := range fm.Tags {
				// skips first entry
				if i != 0 {
					l := Link{
						Source: fm.Tags[i-1],
						Target: tag,
					}
					linkMap[l] = exists
				}
			}
		}
	}
	// converts from map to list
	links := make([]Link, 0)
	for key, _ := range linkMap {
		links = append(links, key)
	}
	return links
}
