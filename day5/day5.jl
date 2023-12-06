using Pkg
Pkg.add("ProgressMeter")
using ProgressMeter

function part1()
	open("input.txt") do f
		line = 0

		seeds_str = readline(f)
		seeds = [parse(Int, m.match) for m in eachmatch(r"\b\d+\b", seeds_str)]
		println("seeds_str: $seeds")
		
		seed2soil = Dict()
		soil2fert = Dict()
		fert2water = Dict()
		water2light = Dict()
		light2temp = Dict()
		temp2hum = Dict()
		hum2loc = Dict()
		
		## Read problem infos
		path_dicts = [seed2soil, soil2fert, fert2water, water2light, light2temp, temp2hum, hum2loc]
		path_dict_id = 1
		while true
			if eof(f) || path_dict_id > 7
				break
			end
			println("path_dict_id: $path_dict_id")
			current_dict = path_dicts[path_dict_id]
			white_space_line = readline(f)
			map_name_line = readline(f)
			s = readline(f)
			while s  != ""
				nums = [parse(Int, m.match) for m in eachmatch(r"\b\d+\b", s)]
				destination = nums[1]
				source = nums[2]
				length = nums[3]
				println("$s | Source: $source destination: $destination length: $length")
				current_dict[source] = [destination, length]
				s = readline(f)
			end
			path_dict_id += 1
		end

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
end


function part2()
	open("input.txt") do f
		line = 0

		seeds_str = readline(f)
		raw_seeds = [parse(Int, m.match) for m in eachmatch(r"\b\d+\b", seeds_str)]
		seeds = []
		total_nb_seeds = 0
		for i in 1:length(raw_seeds)/2
			seeds = push!(seeds,[raw_seeds[Int(i*2-1)], raw_seeds[Int(i*2)]])
			total_nb_seeds += raw_seeds[Int(i*2)]
		end
		println("seeds $(length(seeds)): $seeds")
		
		seed2soil = Dict()
		soil2fert = Dict()
		fert2water = Dict()
		water2light = Dict()
		light2temp = Dict()
		temp2hum = Dict()
		hum2loc = Dict()
		
		## Read problem infos
		path_dicts = [seed2soil, soil2fert, fert2water, water2light, light2temp, temp2hum, hum2loc]
		path_dict_id = 1
		white_space_line = readline(f)
		while true
			if eof(f) || path_dict_id > 7
				break
			end
			current_dict = path_dicts[path_dict_id]
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

		## Find seed location
		println("Number of seeds: $(total_nb_seeds)")
		seed_locs = []
		debug = false
		progress = Progress(total_nb_seeds)
		for (seed, range) in seeds
			for loc in seed:seed+range
				if loc == 82
					debug = true
				else 
					debug = false
				end
				for j in eachindex(path_dicts)
					path_dict = path_dicts[j]
					# find all path possibilities
					possibilities = []
					for (source, (destination, length)) in path_dict
						if debug
							println(" $j source $source <= loc: $loc <= source + length $(source + length) | destination: $destination length: $length")
							println(" ? $(loc >= source && loc < source + length)")
						end
						if loc >= source && loc < source + length
							push!(possibilities, destination + loc - source)
						end
					end
					# if there is only one possibility, go there
					if length(possibilities) == 1
						going_to = possibilities[1]
						if debug
							println(" | $loc -> $going_to & $j")
						end
						loc = possibilities[1]
					end
					if length(possibilities) > 1
						println("More than one possibility: $possibilities")
					end
				end
				seed_locs = [seed_locs; loc]
				next!(progress)  
			end
		end
		finish!(progress)
		println("seed_locs: $seed_locs")
		min_value = minimum(seed_locs)
		println("Minimum value: $min_value")
	end
end

function cmdError()
	("Usage: julia day5.jl part_number\n part_number: 1 or 2")	
	exit(1)
end

# Main section
if abspath(PROGRAM_FILE) == @__FILE__
    # Code in this block will only run if the script is executed directly, not included as a module
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
