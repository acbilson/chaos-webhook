package main

type EpistemicType string

const (
	Undefined EpistemicType = ""
	Evergreen               = "evergreen"
	Plant                   = "plant"
	Sprout                  = "sprout"
	Seedling                = "seedling"
)

type FrontMatter struct {
	Title     string        `json:"title"`
	Author    string        `json:"author"`
	Date      string        `json:"date"`
	LastMod   string        `json:"lastmod"`
	InReplyTo string        `json:"in-reply-to"`
	TOC       bool          `json:"toc"`
	Epistemic EpistemicType `json:"epistemic"`
	Tags      []string      `json:"tags"`
}

type Diagram struct {
	Nodes []Node `json:"nodes"`
	Links []Link `json:"links"`
}

type Node struct {
	Name  string `json:"name"`
	Count int    `json:"count"`
	Path  string `json:"path"`
}

type Link struct {
	Source string `json:"source"`
	Target string `json:"target"`
}
