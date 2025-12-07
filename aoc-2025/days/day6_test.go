package days

import "testing"

func Test_day6part1Parse(t *testing.T) {
	tests := []struct {
		name    string
		content string
		want    []Input
	}{
		{
			name: "parse input",
			content: `123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
`,
			want: []Input{
				{number: []int{123, 45, 6}, operator: mult},
				{number: []int{328, 64, 98}, operator: plus},
				{number: []int{51, 387, 215}, operator: mult},
				{number: []int{64, 23, 314}, operator: plus},
			},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := day6part1Parse(tt.content)

			// Compare lengths first
			if len(got) != len(tt.want) {
				t.Errorf("day6Parse() length = %v, want %v", len(got), len(tt.want))
				return
			}

			// Compare each Input struct
			for i := range got {
				// Compare operators
				if got[i].operator != tt.want[i].operator {
					t.Errorf("day6Parse()[%d].operator = %v, want %v", i, got[i].operator, tt.want[i].operator)
				}

				// Compare number slices
				if len(got[i].number) != len(tt.want[i].number) {
					t.Errorf("day6Parse()[%d].number length = %v, want %v", i, len(got[i].number), len(tt.want[i].number))
					continue
				}

				for j := range got[i].number {
					if got[i].number[j] != tt.want[i].number[j] {
						t.Errorf("day6Parse()[%d].number[%d] = %v, want %v", i, j, got[i].number[j], tt.want[i].number[j])
					}
				}
			}
		})
	}
}

func Test_day6part2Parse(t *testing.T) {
	tests := []struct {
		name    string
		content string
		want    []Input
	}{
		{
			name: "parse input",
			content: `123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
`,
			want: []Input{
				{number: []int{1, 24, 356}, operator: mult},
				{number: []int{369, 248, 8}, operator: plus},
				{number: []int{32, 581, 175}, operator: mult},
				{number: []int{623, 431, 4}, operator: plus},
			},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := day6part2Parse(tt.content)

			// Compare lengths first
			if len(got) != len(tt.want) {
				t.Errorf("day6Parse() length = %v, want %v", len(got), len(tt.want))
				return
			}

			// Compare each Input struct
			for i := range got {
				// Compare operators
				if got[i].operator != tt.want[i].operator {
					t.Errorf("day6Parse()[%d].operator = %v, want %v", i, got[i].operator, tt.want[i].operator)
				}

				// Compare number slices
				if len(got[i].number) != len(tt.want[i].number) {
					t.Errorf("day6Parse()[%d].number length = %v, want %v", i, len(got[i].number), len(tt.want[i].number))
					continue
				}

				for j := range got[i].number {
					if got[i].number[j] != tt.want[i].number[j] {
						t.Errorf("day6Parse()[%d].number[%d] = %v, want %v", i, j, got[i].number[j], tt.want[i].number[j])
					}
				}
			}
		})
	}
}
