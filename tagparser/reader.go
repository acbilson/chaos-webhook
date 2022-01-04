package main

import (
	"bufio"
	"errors"
	"os"
	"path/filepath"
)

func readFilesInDirectory(dir string) ([]string, error) {
	files, err := os.ReadDir(dir)
	if err != nil {
		return nil, err
	}
	if len(files) == 0 {
		return nil, errors.New("no matching files")
	}
	final := make([]string, 0)
	for _, f := range files {
		// filters out _index.html files
		name := f.Name()
		if name[0] != '_' {
			final = append(final, filepath.Join(dir, name))
		}
	}
	return final, nil
}

func readFrontMatter(path string) ([]string, error) {
	file, fileErr := os.Open(path)

	if fileErr != nil {
		return nil, fileErr
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)

	delimiter := "+++"
	delimiterCount := 0
	content := make([]string, 0)

	for scanner.Scan() {
		text := scanner.Text()
		if text == delimiter {
			delimiterCount++
		} else {
			content = append(content, text)
		}
		if delimiterCount > 1 {
			break
		}
	}

	return content, nil
}
