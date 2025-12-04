package main

import "testing"

func Test_part1(t *testing.T) {
	tests := []struct {
		name  string
		banks []string
		want  int
	}{
		{
			name: "Part 1 - Example",
			banks: []string{
				"987654321111111",
				"811111111111119",
				"234234234234278",
				"818181911112111",
			},
			want: 357,
		},
		{
			name: "Part 1 - Edge case",
			banks: []string{
				"29224235122362251128",
			},
			want: 98,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := part1(tt.banks)
			if got != tt.want {
				t.Errorf("part1() = %v, want %v", got, tt.want)
			}
		})
	}
}
