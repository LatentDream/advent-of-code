-- Parse a line of the map
function parse_input(inputString)
    local input, LR = inputString:match("(%w+)%s*=%s*%((.-)%)")

    -- If LR is not nil, split it into L and R
    if LR then
        L, R = LR:match("(%w+),%s*(%w+)")
    end

    return input, {L, R}
end



-- load the input.txt file
local input = {}
for line in io.lines("input.txt") do
  input[#input + 1] = line
end

-- Process line
local instruction = ""
local myMap = {}

print("Printing using pairs:")
for key, value in pairs(input) do
  if key == 1 then
    -- load the instruction
    instruction = value
  elseif key == 2 then
    -- Empty line
    goto continue
  else
    -- Load the map
    if value then
      local loc , LR = parse_input(value)
      myMap[loc] = LR
    end
  end
    ::continue::
end


local step = 0
local found = false
local loc = "AAA"

while found == false do

  for i = 1, #instruction do
    local char = instruction:sub(i, i)
    
    if char == 'L' then
      loc = myMap[loc][1]
      step = step + 1
    elseif char == "R" then
      loc = myMap[loc][2]
      step = step + 1
    end
    if loc == "ZZZ" then
      found = true
      break
    end
  end
end

print("Part 1: ", step)

-- Find all note finishing with 'A'
local locs = {}
for k, _ in pairs(myMap) do
  if k:sub(3,3) == "A" then
    table.insert(locs, k)
  end
end

print("\nPart 2 | Number of starting node: ", #locs)



-- Find all cycle
local cycles = {}
local min_it = -1
local instruction_lenght = string.len(instruction) - 2
print("Instruction len: ", instruction_lenght)
for j, loc in ipairs(locs) do
 local cycle_found = false
 local potentiel_stop = {}
 local nb_it = 0
 while not cycle_found do
  for i = 1, string.len(instruction) - 2 do
    local char = instruction:sub(i, i)
    if char == "R" or char == "L" then
      -- Take a step
      loc = (char == "L") and myMap[loc][1] or myMap[loc][2]
      -- Check if it's a potential stop
      if loc:sub(3,3) == "Z" then
        if potentiel_stop[loc .. i] ~= nil then
          -- found a cycle
          local min_nb_it, min_i, min_loc = table.unpack(potentiel_stop[loc .. i])
          cycle_found = true
          if nb_it > min_it then
            min_it = min_nb_it
          end
          -- Extract needed info of LCM
          for k, v in pairs(potentiel_stop) do
            local f_nb_it, f_i, f_loc = table.unpack(v)
            if f_nb_it < min_nb_it then
              goto continue
            elseif f_nb_it == min_nb_it and f_i < min_i then
              goto continue
            else
                local n = (nb_it - min_nb_it) * instruction_lenght
                local m = (f_nb_it - min_nb_it) * f_i
                local b = nb_it * instruction_lenght + i - (nb_it - min_nb_it) * instruction_lenght

                table.insert(cycles, {j, n, m, b})
            end
              ::continue::
          end

          break
        else
          --print("adding", loc .. i, nb_it)
          potentiel_stop[loc .. i] = {nb_it, i, loc}
        end
      end
    end
  end
  nb_it = nb_it + 1
 end
end


function gcd(a, b)
    while b ~= 0 do
        a, b = b, a % b
    end
    return math.abs(a)
end

function lcm(a, b)
    return math.abs(a * b) / gcd(a, b)
end

function array_lcm(arr)
    local result = arr[1] or 1
    for i = 2, #arr do
        result = lcm(result, arr[i])
    end
    return result
end


print("Cycle found: ")

local arr = {}
for i, cycle in ipairs(cycles) do
  local i, n, m, b = table.unpack(cycle)
  print(" | i:", i ,"n:", n, "m:", m, "b:", b)
  table.insert(arr, n) -- Assuming there is only one node finishing with Z in the cycle
end

local smallest_found = array_lcm(arr)
print("Part 2: ", smallest_found)
