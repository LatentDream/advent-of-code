

function getInputs()
    path_dicts = [Dict() for i in 1:7]
    seeds = []
    open("input.txt") do f
		seeds_str = readline(f)
		seeds = [parse(Int, m.match) for m in eachmatch(r"\b\d+\b", seeds_str)]
		println("seeds_str: $seeds")
		## Read problem infos

		path_dict_id = 1
		while true
			if eof(f) || path_dict_id > 7
				break
			end
			current_dict = path_dicts[path_dict_id]
			white_space_line = readline(f)
			map_name_line = readline(f)
			s = readline(f)
			while s  != ""
				nums = [parse(Int, m.match) for m in eachmatch(r"\b\d+\b", s)]
				destination = nums[1]
				source = nums[2]
				length = nums[3]
				current_dict[source] = [destination, length]
				s = readline(f)
			end
			path_dict_id += 1
		end
    end
    println("$path_dicts")
    return seeds, path_dicts
end

function part1()
    seeds, path_dicts = getInputs()
    ## Find seed location
	seed_locs = []
	for i in eachindex(seeds)
		loc = seeds[i]
		println("seed: $loc")
		for j in eachindex(path_dicts)
			path_dict = path_dicts[j]
			# find all path possibilities
			possibilities = []
			for (source, (destination, length)) in path_dict
				if loc >= source && loc < source + length
					push!(possibilities, destination + loc - source)
				end
			end
			# if there is only one possibility, go there
			if length(possibilities) == 1
				going_to = possibilities[1]
				println(" | $loc -> $going_to")
				loc = possibilities[1]
			end
			if length(possibilities) > 1
				println("More than one possibility: $possibilities")
			end
		end
		seed_locs = [seed_locs; loc]
	end
	println("seed_locs: $seed_locs")
	min_value = minimum(seed_locs)
	println("Minimum value: $min_value")
end

function getInputsAsMatrix()
    seeds = []
    data = Matrix{Int}[]
    open("input.txt") do f
		seeds_str = readline(f)
		seeds = [parse(Int, m.match) for m in eachmatch(r"\b\d+\b", seeds_str)]
		## Read problem infos
        _ = readline(f)
		while true
			if eof(f)
				break
			end
			_ = readline(f)
			s = readline(f)
            temp = []
			while s  != ""
                # Parse as array of integers 
                push!(temp, parse.(Int, split(s, " "))')
				s = readline(f)
			end
            push!(data, vcat(temp...))
		end
    end
    return seeds, data
end


function perform_mapping(data::Array{Matrix{Int}}, mapping::Int, numbers::Set{UnitRange{Int}})
    M = data[mapping]
    new_set = Set{UnitRange{Int}}()
    while !isempty(numbers)
        mapped = false
        range = pop!(numbers)
        for row in axes(M, 1)
            # Take the range, ∩ with the entry of the mapping
            inter = intersect(range, M[row, 2]:M[row, 2]+M[row, 3]-1)
            if !isempty(inter)
                mapped = true
                # Find the output range
                push!(new_set, inter[1] - M[row,2] + M[row,1] : inter[end] - M[row,2] + M[row,1])
                # Add the left and right ranges
                left = range[1]:inter[1]-1
                if !isempty(left)
                    push!(numbers, left)
                end
                right = inter[end]+1:range[end]
                if !isempty(right)
                    push!(numbers, right)
                end
                break
            end
        end
        if !mapped 
            push!(new_set, range)
        end
    end
    return new_set
end


function part2()
    locations = Set{UnitRange{Int}}()
    seeds, data = getInputsAsMatrix()
    seeds_start = seeds[1:2:end]
    seeds_end = seeds[2:2:end]

    for (seed_start, seed_length) in zip(seeds_start, seeds_end)
        println("Progressing: seed_start: $seed_start seed_length: $seed_length")
        numbers = Set([seed_start:seed_start+seed_length-1]) # Use UnitRange to avoid memory allocation
        for mapping in 1:7
            numbers = perform_mapping(data, mapping, numbers)
        end
        push!(locations, numbers...)
    end
    println("Minimum found: $(minimum(x -> x[1], locations))")
    
end


# Main
if abspath(PROGRAM_FILE) == @__FILE__
	println("Number of arguments: ", length(ARGS))
    println("Arguments: ", ARGS)
	if length(ARGS) != 1
		cmdError()
	end

    # Example usage
	if ARGS[1] == "1"
		part1()
	elseif ARGS[1] == "2"
		part2()
	else
		cmdError()
	end
end
