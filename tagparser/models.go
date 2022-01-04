package main

type EpistemicType string

const (
	Undefined EpistemicType = ""
	Evergreen = "evergreen"
	Plant = "plant"
	Sprout = "sprout"
	Seedling = "seedling"
)

type FrontMatter struct {
	Title string    `json:"title"`
	Author string    `json:"author"`
	Date  string     `json:"date"`
	LastMod string   `json:"lastmod"`
	InReplyTo string   `json:"in-reply-to"`
	TOC bool `json:"toc"`
	Epistemic EpistemicType `json:"epistemic"`
	Tags []string      `json:"tags"`
}

type Tag struct {
	Folder string
	Count  int
	Near   []string
}

type Node struct {
	Name  string `json:"name"`
	Type  string `json:"type"`
	Count int    `json:"count"`
	Path  string `json:"path"`
}

type Link struct {
	Source string `json:"source"`
	Target string `json:"target"`
}

type Nodes struct {
	Nodes []Node `json:"nodes"`
	Links []Link `json:"links"`
}

// implements a Set Node type using Map
var exists = struct{}{}

type Set struct {
	m map[Link]struct{}
}

func NewSet() Set {
	s := Set{}
	s.m = make(map[Link]struct{})
	return s
}

func (s *Set) Add(v Link) {
	s.m[v] = exists
}

func (s *Set) Remove(v Link) {
	delete(s.m, v)
}

func (s *Set) Contains(v Link) bool {
	_, c := s.m[v]
	return c
}

func (s *Set) Size() int {
	return len(s.m)
}

func (s *Set) Get() map[Link]struct{} {
	return s.m
}
