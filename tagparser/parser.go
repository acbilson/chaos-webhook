package main

import (
	"bufio"
	"encoding/json"
	"flag"
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"
	"strings"
)

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

func main() {

	rootFlag := flag.String("root", "../../chaos-content", "starting point for the command")
	dirsFlag := flag.String("dirs", "notes,posts/achievements,posts/insights,posts/letters,posts/recommendations,posts/resources,posts/stories", "comma-separated list of folders to pull tags")
	outFlag := flag.String("output", "diagram.json", "file path to write json")
	flag.Parse()

	var allTagsMap []map[string]Tag

	for _, folder := range strings.Split(*dirsFlag, ",") {
		path := filepath.Join(*rootFlag, folder)
		tagMap := readTagsInPath(path, folder)
		allTagsMap = append(allTagsMap, tagMap)
	}

	everything := merge(allTagsMap)
	data := convertToNodes(everything)
	content, err := json.Marshal(data)

	if err != nil {
		fmt.Println("Error parsing to json", err)
	}

	writeToFile(content, *outFlag)
}

func merge(maps []map[string]Tag) map[string]Tag {
	merged := make(map[string]Tag)
	for _, m := range maps {
		for k, v := range m {
			_, ok := merged[k]
			// if this tag has been set in another folder
			if ok {
				merged[k] = Tag{
					Folder: "both",
					Count:  merged[k].Count + v.Count,
					Near:   appendNewTags(merged[k].Near, v.Near),
				}
			} else {
				merged[k] = v
			}
		}
	}
	return merged
}

func readTagsInPath(path, folder string) map[string]Tag {
	files, err := getFiles(path)

	if err != nil {
		fmt.Println("Directory read error", err)
		return nil
	}

	tagMap := make(map[string]Tag)

	for x := 0; x < len(files); x++ {
		if !files[x].IsDir() {
			fileName := files[x].Name()

			path := filepath.Join(path, fileName)
			lines, err := scanTags(path)

			if err != nil {
				fmt.Println("File reading error", err)
				return nil
			}

			if lines == nil {
				fmt.Println("no tags found in file", fileName)
			}

			// prints a summary of the file
			/*
				fmt.Println("")
				fmt.Println("file:", fileName)
				fmt.Println("tags:", lines)
			*/

			for i := 0; i < len(lines); i++ {
				tag := strings.Trim(lines[i], " \"")

				nearTags := getNearTags(lines, tag)

				_, ok := tagMap[tag]

				// when this tag has a mapping
				if ok == true {
					var allNearTags []string

					// when no previous near tags exist
					if tagMap[tag].Near != nil {
						allNearTags = appendNewTags(tagMap[tag].Near, nearTags)
					} else {
						allNearTags = nearTags
					}

					tagMap[tag] = Tag{
						Folder: tagMap[tag].Folder,
						Count:  tagMap[tag].Count + 1,
						Near:   allNearTags}

					// when this is a new tag
				} else {
					tagMap[tag] = Tag{
						Folder: folder,
						Count:  1,
						Near:   nearTags}
				}
			}
		}
	}
	return tagMap
}

func convertToNodes(tags map[string]Tag) Nodes {
	var nodes []Node
	var links []Link

	for key, val := range tags {
		node := Node{
			Name:  key,
			Type:  val.Folder,
			Count: val.Count,
			Path:  fmt.Sprintf("/tags/%s/", key)}

		for _, tag := range val.Near {
			link := Link{Source: key, Target: tag}
			links = append(links, link)
		}
		nodes = append(nodes, node)
	}

	return Nodes{Nodes: nodes, Links: links}
}

func appendNewTags(existingTags []string, newTags []string) []string {
	for _, newTag := range newTags {
		if !contains(existingTags, newTag) {
			existingTags = append(existingTags, newTag)
		}
	}
	return existingTags
}

func contains(list []string, value string) bool {
	for _, item := range list {
		if item == value {
			return true
		}
	}
	return false
}

func writeToFile(content []byte, output string) {
	err := ioutil.WriteFile(output, content, 0644)
	if err != nil {
		fmt.Println("failed to write to file", err)
		return
	}
}

func getNearTags(tags []string, skippedTag string) []string {
	// no need to check for near tags if there's only one
	if len(tags) < 2 {
		return nil
	}

	nearTags := make([]string, len(tags)-1)

	for _, v := range tags {
		tag := strings.Trim(v, " \"")
		if tag != skippedTag {
			nearTags = append(nearTags[1:], tag)
		}
	}

	return nearTags
}

func getFiles(dir string) ([]os.FileInfo, error) {
	d, err := os.Open(dir)

	if err != nil {
		return nil, err
	}

	defer d.Close()

	info, err := d.Readdir(-1)

	if err != nil {
		return nil, err
	}

	return info, nil
}

func isTagLine(line string) bool {
	if len(line) > 5 {
		return line[:6] == "tags ="
	} else {
		return false
	}
}

func parseTags(line string) []string {

	// strips the leading characters
	tagLine := line[8 : len(line)-1]

	// splits on comma
	return strings.Split(tagLine, ",")
}

func scanTags(path string) ([]string, error) {

	file, fileErr := os.Open(path)

	if fileErr != nil {
		return nil, fileErr
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	for scanner.Scan() {
		text := scanner.Text()

		if isTagLine(text) {
			return parseTags(text), nil
		}
	}

	return nil, nil
}
