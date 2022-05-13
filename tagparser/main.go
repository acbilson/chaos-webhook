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
	dirsFlag := flag.String("dirs", "plants/business,plants/culture,plants/entrepreneurship,plants/faith,plants/identity,plants/leadership,plants/meta,plants/parenting,plants/science,plants/technology,plants/writing", "comma-separated list of folders to pull tags")

	outFlag := flag.String("output", "diagram.json", "file path to write json")
	debugFlag := flag.Bool("debug", false, "run the program in debug mode")
	flag.Parse()

	fmt.Printf("\nwrites file to %s", *outFlag)
	fmt.Printf("\nfor folders: %s", *dirsFlag)

	filePaths := getAbsolutePathsInDirectories(*rootFlag, *dirsFlag)

	fmt.Printf("\ntotal files: %d", len(filePaths))
	if *debugFlag == true {
		fmt.Printf("\nexamples: %s", strings.Join(filePaths[0:5], "\n"))
	}

	frontMatters := getFrontMatterForFiles(filePaths)

	fmt.Printf("\ntotal front matters: %d", len(frontMatters))
	if *debugFlag == true {
		fmt.Print("\nexamples:")
		for _, example := range frontMatters[0:5] {
			fmt.Printf("\n%v", example)
		}
	}

	nodes := MapFrontMatterToNode(frontMatters)

	if *debugFlag == true {
		fmt.Printf("\nexample nodes:")
		for _, example := range nodes[20:30] {
			fmt.Printf("\n%v", example)
		}
	}

	links := MapFrontMatterToLink(frontMatters)

	if *debugFlag == true {
		fmt.Printf("\nexample link:")
		for _, example := range links {
			fmt.Printf("\n%v", example)
		}
	}

	content, err := json.Marshal(
		Diagram{
			Nodes: nodes,
			Links:links,
	})

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
