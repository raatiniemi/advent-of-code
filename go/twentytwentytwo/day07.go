package twentytwentytwo

import (
	"sort"
	"strings"
)

type FilesystemCommand struct {
	command string
	output  []string
}

func day07Part01(input string) int {
	commands := parseCommandsWithOutput(input)
	return calculateTotal(calculateSizeForPath(commands))
}

func parseCommandsWithOutput(input string) []FilesystemCommand {
	var commands []FilesystemCommand

	command := ""
	var output []string
	for _, line := range readLines(input) {
		if line[0] == '$' {
			if len(command) > 0 {
				commands = append(commands, FilesystemCommand{command: command, output: output})
			}

			command = strings.Replace(line, "$ ", "", 1)
			// Clear output for next command
			output = output[len(output):]
			continue
		}

		output = append(output, line)
	}
	commands = append(commands, FilesystemCommand{command: command, output: output})
	return commands
}

func calculateSizeForPath(v []FilesystemCommand) map[string]int {
	var nodes = make(map[string]int)

	path := ""
	for _, x := range v {
		if x.command == "cd .." {
			path = path[:strings.LastIndex(strings.TrimRight(path, "/"), "/")+1]
			continue
		}

		if strings.HasPrefix(x.command, "cd ") {
			path = strings.TrimRight(path+x.command[3:], "/") + "/"
			continue
		}

		if x.command == "ls" {
			size := 0
			for _, o := range x.output {
				if !strings.HasPrefix(o, "dir") {
					f := strings.Split(o, " ")
					size += parseInt(f[0])
				}
			}
			nodes[path] = size
			continue
		}
	}
	return nodes
}

func calculateTotal(values map[string]int) int {
	total := 0
	for path := range values {
		sizeForPathInclusive := calculatePathInclusive(path, values)
		if sizeForPathInclusive > 100_000 {
			continue
		}

		total += sizeForPathInclusive
	}
	return total
}

func calculatePathInclusive(path string, nodes map[string]int) int {
	total := 0
	for p, s := range nodes {
		if strings.HasPrefix(p, path) {
			total += s
		}
	}
	return total
}

func day07Part02(input string) int {
	commands := parseCommandsWithOutput(input)
	sizeByPaths := sizeByPathInclusive(calculateSizeForPath(commands))

	const totalFilesystemSize = 70000000
	const updateSize = 30000000
	spaceUsed := sizeByPaths["/"]

	var sizeForRemovePath []int
	for _, sizeForPath := range sizeByPaths {
		potentialAvailableSpace := totalFilesystemSize - spaceUsed + sizeForPath
		if potentialAvailableSpace >= updateSize {
			sizeForRemovePath = append(sizeForRemovePath, sizeForPath)
		}
	}
	sort.Ints(sizeForRemovePath)
	return sizeForRemovePath[0]
}

func sizeByPathInclusive(values map[string]int) map[string]int {
	sizeByPath := make(map[string]int)
	for path := range values {
		sizeByPath[path] = calculatePathInclusive(path, values)
	}
	return sizeByPath
}
