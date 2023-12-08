# Extract all the number on each line
extract_numbers <- function(line) {
  numbers <- as.numeric(regmatches(line, gregexpr("\\b\\d+\\b", line))[[1]])
  return(numbers)
}

find_nb_possible_win <- function(time, record) {
  a = 1
  b = -time
  c = -record 
  delta = b^2 + 4*a*c
  x_max = (-b + sqrt(delta)) / (2)
  x_min = (-b - sqrt(delta)) / (2)
  x_max = ifelse(x_max != trunc(x_max), floor(x_max), x_max - 1)
  x_min = ifelse(x_min != trunc(x_min), ceiling(x_min), x_min + 1)
  nb_win <- x_max - x_min + 1
  return(nb_win) 
}

# Get command-line arguments
args <- commandArgs(trailingOnly = TRUE)
file_path <- "input.txt"
lines <- readLines(file_path)

# Print the arguments
if  (length(args) > 0 && args[1] == "1") {
  print("Part 1")
  # Apply it to the file
  parsed_data <- lapply(lines, extract_numbers)
  # Find the data for each race and process it
  total = 1
  for (i in seq_along(parsed_data[[1]])) {
    time <- parsed_data[[1]][i]
    record <- parsed_data[[2]][i]
    nb_win = find_nb_possible_win(time, record)
    total = total * nb_win
  }
  print(total)
} else if (length(args) > 0 && args[1] == "2") {
  print("Part 2")
  result <- sapply(lines, function(lines) {
    numbers <- gsub("\\D", "", lines)  # Remove non-digit characters
    return(as.numeric(numbers))
  }, USE.NAMES = FALSE)
  nb_win = find_nb_possible_win(result[1], result[2])
  print(nb_win)

} else {
  print("No arguments, please specify `1` or `2`")
}
