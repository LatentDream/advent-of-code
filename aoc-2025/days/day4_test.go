package days

import "testing"

func Test_day4part1(t *testing.T) {
	tests := []struct {
		name  string
		input []string
		want  int
	}{
		{
			name: "Demo",
			input: []string{
				//   0123456789
				"..@@.@@@@.", // 0
				"@@@.@.@.@@", // 1
				"@@@@@.@.@@", // 2
				"@.@@@@..@.", // 3
				"@@.@@@@.@@", // 4
				".@@@@@@@.@", // 5
				".@.@.@.@@@", // 6
				"@.@@@.@@@@", // 7
				".@@@@@@@@.", // 8
				"@.@.@@@.@.", // 9
			},
			want: 13,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := day4part1(tt.input)
			if got != tt.want {
				t.Errorf("day4part1() = %v, want %v", got, tt.want)
			}
		})
	}
}

func Test_day4part2(t *testing.T) {
	tests := []struct {
		name  string
		input []string
		want  int
	}{
		{
			name: "Demo",
			input: []string{
				//   0123456789
				"..@@.@@@@.", // 0
				"@@@.@.@.@@", // 1
				"@@@@@.@.@@", // 2
				"@.@@@@..@.", // 3
				"@@.@@@@.@@", // 4
				".@@@@@@@.@", // 5
				".@.@.@.@@@", // 6
				"@.@@@.@@@@", // 7
				".@@@@@@@@.", // 8
				"@.@.@@@.@.", // 9
			},
			want: 43,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := day4part2(tt.input)
			if got != tt.want {
				t.Errorf("day4part2() = %v, want %v", got, tt.want)
			}
		})
	}
}
