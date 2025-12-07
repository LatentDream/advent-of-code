package days

import (
	"fmt"
	"testing"
)



func Test_split(t *testing.T) {
	tests := []struct {
		name string // description of this test case
		// Named input parameters for target function.
		input []string
		lenDB  int
		lenID int
	}{
		{
			name: "parse input",
			input: []string{
				"3-5",
				"10-14",
				"16-20",
				"12-18",
				"",
				"1",
				"5",
				"8",
				"11",
				"17",
				"32",
			},
			lenDB: 4,
			lenID: 6,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			DB, idx := split(tt.input)
			fmt.Println("DB: ", DB)
			fmt.Println("Idx: ", DB)
			if len(DB) != tt.lenDB {
				t.Errorf("split() = %v, want %v", len(DB), tt.lenDB)
			}
			if len(idx) != tt.lenID {
				t.Errorf("split() = %v, want %v", len(idx), tt.lenID)
			}
		})
	}
}

func Test_day5part2(t *testing.T) {
	tests := []struct {
		name string // description of this test case
		// Named input parameters for target function.
		ranges []Range
		want   int
	}{
		{
			name: "Input",
			ranges: []Range {
				{ start:3, end: 5},
				{ start:10, end: 14},
				{ start:16, end: 20},
				{ start:12, end: 18},
				// 3 4 5 
				//       10 11 12 13 14
				//                         16 17 18 19 20
				//             12 13 14 15 16 17 18

				// Sorted and merged
				// { start:3, end: 5},
				// { start:10, end: 20},
				// 3 4 5
				//       10 11 12 13 14 15 16 17 18 19 20
			},
			want: 14 ,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := day5part2(tt.ranges)
			if got != tt.want {
				t.Errorf("day5part2() = %v, want %v", got, tt.want)
			}
		})
	}
}

