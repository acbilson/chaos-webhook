package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"path/filepath"
	"strings"
)

func main() {

	rootFlag := flag.String("root", "../../chaos-content", "starting point for the command")
	dirsFlag := flag.String("dirs", "plants/business,plants/faith,plants/identity,plants/meta,plants/parenting,plants/science,plants/technology,plants/writing", "comma-separated list of folders to pull tags")

	outFlag := flag.String("output", "diagram.json", "file path to write json")
	flag.Parse()

	filePaths := getAbsolutePathsInDirectories(*rootFlag, *dirsFlag)

	fmt.Printf("total files: %d", len(filePaths))
	fmt.Printf("\nexamples: %s", strings.Join(filePaths[0:5], "\n"))

	frontMatters := getFrontMatterForFiles(filePaths)

	fmt.Printf("\ntotal front matters: %d", len(frontMatters))
	fmt.Print("\nexamples:")
	for _, example := range frontMatters[0:5] {
		fmt.Printf("\n%v", example)
	}

	nodes := MapFrontMatterToNode(frontMatters)

	fmt.Printf("\nexample nodes:")
	for _, example := range nodes[20:30] {
		fmt.Printf("\n%v", example)
	}

	links := MapFrontMatterToLink(frontMatters)

	fmt.Printf("\nexample link:")
	for _, example := range links {
		fmt.Printf("\n%v", example)
	}

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

func getAbsolutePathsInDirectories(root string, dirs string) []string {
	filePaths := make([]string, 0)
	directories := combinePaths(root, strings.Split(dirs, ","))
	for _, dir := range directories {
		if files, err := readFilesInDirectory(dir); err == nil {
			filePaths = append(filePaths, files...)
		} else {
			fmt.Print(err)
		}
	}
	return filePaths
}

func getFrontMatterForFiles(filePaths []string) []FrontMatter {
	frontMatters := make([]FrontMatter, 0)

	for _, path := range filePaths {
		content, readErr := readFrontMatter(path)
		if readErr != nil {
			fmt.Printf("\nCould not read file at: %s", path)
			break
		}

		fm, parseErr := parseFrontMatter(content)
		if parseErr != nil {
			fmt.Printf("\nat file: %s\nCould not parse content: %s\n%s", path, parseErr, strings.Join(content, "\n"))
			break
		}

		frontMatters = append(frontMatters, fm)
	}
	return frontMatters
}

func combinePaths(root string, directories []string) []string {
	paths := make([]string, 0)
	for _, dir := range directories {
		paths = append(paths, filepath.Join(root, dir))
	}
	return paths
}
