package twentytwentytwo

import (
	"strings"
)

func day05Part01(input string) string {
	x := strings.Split(strings.Trim(input, "\n"), "\n\n")
	k := parseX(x[0])
	m := parseMoves(x[1])
	return foobar(k, m)
}

func foobar(k [][]string, moves []CrateMove) string {
	for _, move := range moves {
		var crates []string
		if len(k[move.from]) <= move.numberOfCrates {
			crates = k[move.from]
			k[move.from] = k[move.from][move.numberOfCrates:]
		} else {
			crates = k[move.from][len(k[move.from])-move.numberOfCrates:]
			k[move.from] = k[move.from][:len(k[move.from])-move.numberOfCrates]
		}
		for i := len(crates); i > 0; i-- {
			k[move.to] = append(k[move.to], crates[i-1])
		}
	}

	var fd []string
	for _, fdf := range k {
		fd = append(fd, fdf[len(fdf)-1])
	}
	return strings.Join(fd, "")
}

func parseX(s string) [][]string {
	lines := readLines(s)

	x := lines[len(lines)-1]
	b := strings.Replace(x, " ", "", -1)
	var h []int
	for _, y := range b {
		i := strings.Index(x, string(y))
		h = append(h, i)
	}
	var d [][]string
	for _, k := range h {
		var crates []string
		for i := len(lines) - 2; i >= 0; i-- {
			line := lines[i]
			if len(line) > k {
				elems := strings.Trim(string(line[k]), " ")
				if len(elems) == 1 {
					crates = append(crates, elems)
				}
			}
		}
		d = append(d, crates)
	}
	return d
}

type CrateMove struct {
	numberOfCrates int
	from           int
	to             int
}

func parseMoves(moves string) []CrateMove {
	var result []CrateMove

	for _, s := range readLines(moves) {
		x := strings.Split(s, " ")
		result = append(result, CrateMove{
			numberOfCrates: parseInt(x[1]),
			from:           parseInt(x[3]) - 1,
			to:             parseInt(x[5]) - 1,
		})
	}

	return result
}

func day05Part02(input string) string {
	x := strings.Split(strings.Trim(input, "\n"), "\n\n")
	k := parseX(x[0])
	m := parseMoves(x[1])
	return foobar2(k, m)
}

func foobar2(k [][]string, moves []CrateMove) string {
	for _, move := range moves {
		var crates []string
		if len(k[move.from]) <= move.numberOfCrates {
			crates = k[move.from]
			k[move.from] = k[move.from][move.numberOfCrates:]
		} else {
			crates = k[move.from][len(k[move.from])-move.numberOfCrates:]
			k[move.from] = k[move.from][:len(k[move.from])-move.numberOfCrates]
		}
		for _, crate := range crates {
			k[move.to] = append(k[move.to], crate)
		}
	}

	var fd []string
	for _, fdf := range k {
		fd = append(fd, fdf[len(fdf)-1])
	}
	return strings.Join(fd, "")
}
