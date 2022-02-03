package main

import (
	"errors"
	"fmt"
	"strings"
)

func firstWord(l string) (string, error) {
	i := strings.IndexRune(l, ' ')
	if i != -1 {
		return l[0:i], nil
	} else {
		msg := fmt.Sprintf("No first word in: %s", l)
		return "", errors.New(msg)
	}
}

func parseFrontMatter(matter []string) (FrontMatter, error) {
	fm := FrontMatter{
		Author:    "",
		Alias:     "",
		Date:      "",
		LastMod:   "",
		Epistemic: "",
		InReplyTo: "",
		TOC:       false,
		Tags:      make([]string, 0),
	}

	trim := func(s int, l string) string {
		if s < len(l) {
			return strings.Trim(l[s:], "\"")
		} else {
			return ""
		}
	}

	for _, line := range matter {
		word, err := firstWord(line)
		if err != nil {
			return fm, err
		}

		switch word {
		case "author":
			fm.Author = trim(len("author = "), line)
		case "date":
			fm.Date = trim(len("date = "), line)
		case "lastmod":
			fm.LastMod = trim(len("lastmod = "), line)
		case "in-reply-to":
			fm.InReplyTo = trim(len("in-reply-to = "), line)
		case "epistemic":
			val := trim(len("epistemic = "), line)
			fm.Epistemic = EpistemicType(val)
		case "tags":
			fm.Tags = parseTags(line)
		case "toc":
			fm.TOC = true
		case "title":
			fm.Title = trim(len("title = "), line)
		case "aliases":
			fm.Alias = trim(len("aliases = "), line)
		default:
			msg := fmt.Sprintf("%s is not a handled key", word)
			return fm, errors.New(msg)
		}
	}

	return fm, nil
}

func isTagLine(line string) bool {
	if len(line) > 5 {
		return line[:6] == "tags ="
	} else {
		return false
	}
}

func parseTags(line string) []string {
	tagLine := line[8 : len(line)-1]
	tags := make([]string, 0)
	for _, tag := range strings.Split(tagLine, ",") {
		tags = append(tags, strings.Trim(tag, " \""))
	}
	return tags
}
