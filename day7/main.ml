(* That's a mess, First time doing Ocaml *)

(* Read each line of the input.txt file and return it as an array *)
let process_file filename =
  let lines = ref [] in
  try
    let in_channel = open_in filename in
    try
      let rec process_lines () =
        try
          let line = input_line in_channel in
          lines := line :: !lines;
          process_lines ()
        with
        | End_of_file -> ()
      in
      process_lines ();
      close_in in_channel;
      Array.of_list (List.rev !lines)
    with
    | Sys_error msg ->
      Printf.printf "Error: %s\n" msg;
      [||] 
  with
  | Sys_error msg ->
    Printf.printf "Error: %s\n" msg;
    [||]
;;

(* Extract the components of each line *)
let extract_components input_string =
  match String.split_on_char ' ' input_string with
  | [str; num_str] -> (str, int_of_string num_str)
  | _ -> failwith "Invalid input format"

let inputs = process_file "input.txt";;
let parsed_inputs = Array.map extract_components inputs;;

(* Part 1 *)
type card_label = A | K | Q | J | T | L9 | L8 | L7 | L6 | L5 | L4 | L3 | L2

let string_of_label = function
  | A -> "A" | K -> "K" | Q -> "Q" | J -> "J" | T -> "T"
  | L9 -> "9" | L8 -> "8" | L7 -> "7" | L6 -> "6" | L5 -> "5"
  | L4 -> "4" | L3 -> "3" | L2 -> "2"

let compare_labels label1 label2 =
  match label1, label2 with
  | A, A -> 0
  | A, _ -> 1
  | _, A -> -1
  | K, K -> 0
  | K, _ -> 1
  | _, K -> -1
  | Q, Q -> 0
  | Q, _ -> 1
  | _, Q -> -1
  | J, J -> 0
  | J, _ -> 1
  | _, J -> -1
  | T, T -> 0
  | T, _ -> 1
  | _, T -> -1
  | L9, L9 -> 0
  | L9, _ -> 1
  | _, L9 -> -1
  | L8, L8 -> 0
  | L8, _ -> 1
  | _, L8 -> -1
  | L7, L7 -> 0
  | L7, _ -> 1
  | _, L7 -> -1
  | L6, L6 -> 0
  | L6, _ -> 1
  | _, L6 -> -1
  | L5, L5 -> 0
  | L5, _ -> 1
  | _, L5 -> -1
  | L4, L4 -> 0
  | L4, _ -> 1
  | _, L4 -> -1
  | L3, L3 -> 0
  | L3, _ -> 1
  | _, L3 -> -1
  | L2, L2 -> 0

type hand_type =
  | Five_of_a_kind of card_label
  | Four_of_a_kind of card_label * card_label
  | Full_house of card_label * card_label
  | Three_of_a_kind of card_label * card_label * card_label
  | Two_pair of card_label * card_label * card_label
  | One_pair of card_label * card_label * card_label * card_label
  | High_card of card_label * card_label * card_label * card_label * card_label 

let hand_of_string_sorted s =
  let label_of_char c =
    match c with
    | 'A' -> A | 'K' -> K | 'Q' -> Q | 'J' -> J | 'T' -> T
    | '9' -> L9 | '8' -> L8 | '7' -> L7 | '6' -> L6 | '5' -> L5
    | '4' -> L4 | '3' -> L3 | '2' -> L2
    | _ -> failwith "Invalid card label"
  in

  let rec parse_cards acc = function
    | [] -> List.rev acc
    | hd :: tl -> parse_cards ((label_of_char hd) :: acc) tl
  in

  let compare_cards_in_list lst card1 card2 =
    let count_card lst card =
      List.fold_left (fun acc x -> if x = card then acc + 1 else acc) 0 lst
    in
    let count_compare = compare (count_card lst card2) (count_card lst card1) in
    if count_compare = 0 then compare card1 card2 else count_compare
  in
  
  let sort_by_frequency cards =
    List.sort (fun card1 card2 -> compare_cards_in_list cards card1 card2) cards
  in

  match String.to_seq s |> List.of_seq with
  | [c1; c2; c3; c4; c5] ->
    let cards = parse_cards [] [c1; c2; c3; c4; c5] in
    let sorted_cards = sort_by_frequency cards in
    
    (* Printf.printf "Sorted Hand: %s | " (String.concat " " (List.map string_of_label sorted_cards)); *)
    (match sorted_cards with
    | [l1; _l2; _l3; _l4; _l5] when List.for_all (( = ) l1) cards -> Five_of_a_kind l1
    | [l1; l2; l3; l4; _l5] when List.for_all (( = ) l1) [l2; l3; l4;] -> Four_of_a_kind (l1, l2)
    | [l1; l2; l3; l4; l5] when List.for_all (( = ) l1) [l2; l3;] && List.for_all (( = ) l4) [l5;] -> Full_house (l1, l4)
    | [l1; l2; l3; _l4; _l5] when List.for_all (( = ) l1) [l2; l3;] -> Three_of_a_kind (l1, l2, l3)
    | [l1; l2; l3; l4; _l5] when List.for_all (( = ) l1) [l2;] && List.for_all (( = ) l3) [l4;] -> Two_pair (l1, l3, l4)
    | [l1; l2; l3; l4; l5] when List.for_all (( = ) l1) [l2;] -> One_pair (l1, l3, l4, l5)
    | [l1; l2; l3; l4; l5] -> High_card (l1, l2, l3, l4, l5)
    | _ -> failwith "Invalid hand string"
    )
  | _ -> failwith "Invalid hand string"

let print_hand_type hand_type s =
  match hand_type with
  | Five_of_a_kind _ -> Printf.printf "%s is a Five of a Kind.\n" s
  | Four_of_a_kind _ -> Printf.printf "%s is a Four of a Kind.\n" s
  | Full_house _ -> Printf.printf "%s is a Full House.\n" s
  | Three_of_a_kind _ -> Printf.printf "%s is a Three of a Kind.\n" s
  | Two_pair _ -> Printf.printf "%s is a Two Pair.\n" s
  | One_pair _ -> Printf.printf "%s is a One Pair.\n" s
  | High_card _ -> Printf.printf "%s is a High Card.\n" s

(* Test *)
(* let hand_string = "J8JJ8";;
print_hand_type hand_string;; *)

(* sort by hand_type *)
let rec compare_hand_types (hand1str, _) (hand2str, _) =
  let hand1 = hand_of_string_sorted hand1str in
  (* print_hand_type hand1 hand1str; *)
  let hand2 = hand_of_string_sorted hand2str in
  (* print_hand_type hand2 hand2str; *)

  match hand1, hand2 with
  | Five_of_a_kind _, Five_of_a_kind _ -> compare_high_cards hand1str hand2str
  | Five_of_a_kind _, _ -> 1
  | _, Five_of_a_kind _ -> -1

  | Four_of_a_kind _, Four_of_a_kind _ -> compare_high_cards hand1str hand2str
  | Four_of_a_kind _, _ -> 1
  | _, Four_of_a_kind _ -> -1

  | Full_house _, Full_house _ -> compare_high_cards hand1str hand2str
  | Full_house _, _ -> 1
  | _, Full_house _ -> -1

  | Three_of_a_kind _, Three_of_a_kind _ -> compare_high_cards hand1str hand2str
  | Three_of_a_kind _, _ -> 1
  | _, Three_of_a_kind _ -> -1

  | Two_pair _, Two_pair _ -> compare_high_cards hand1str hand2str
  | Two_pair _, _ -> 1
  | _, Two_pair _ -> -1

  | One_pair _, One_pair _ -> compare_high_cards hand1str hand2str
  | One_pair _, _ -> 1
  | _, One_pair _ -> -1

  | High_card _, High_card _ -> compare_high_cards hand1str hand2str

and compare_high_cards hand1str hand2str =
  let rec compare_cards cards1 cards2 =
    match cards1, cards2 with
    | [], [] -> 0
    | [], _ -> 1
    | _, [] -> -1
    | card1 :: rest1, card2 :: rest2 ->
      let cmp = compare_labels card1 card2 in
      (* Printf.printf "Card1: %s, Card2: %s, Cmp: %d\n" (string_of_label card1) (string_of_label card2) cmp; *)
      if cmp = 0 then compare_cards rest1 rest2
      else cmp
  in
  let label_of_char c =
    match c with
    | 'A' -> A | 'K' -> K | 'Q' -> Q | 'J' -> J | 'T' -> T
    | '9' -> L9 | '8' -> L8 | '7' -> L7 | '6' -> L6 | '5' -> L5
    | '4' -> L4 | '3' -> L3 | '2' -> L2
    | _ -> failwith "Invalid card label"
  in
  let rec parse_cards acc = function
    | [] -> List.rev acc
    | hd :: tl -> parse_cards ((label_of_char hd) :: acc) tl
  in
  let hand1 = parse_cards [] [hand1str.[0]; hand1str.[1]; hand1str.[2]; hand1str.[3]; hand1str.[4]] in
  let hand2 = parse_cards [] [hand2str.[0]; hand2str.[1]; hand2str.[2]; hand2str.[3]; hand2str.[4]] in
  match hand1, hand2 with
  | cards1, cards2 -> compare_cards cards1 cards2
;;
  
let array_to_list arr =
  Array.to_list arr
let parsed_inputs = array_to_list parsed_inputs
let sorted_inputs = List.sort compare_hand_types parsed_inputs;;

(* let () =
  List.iter (fun (x, y) -> Printf.printf "(%s, %d) \n" x y) sorted_inputs *)

let calculate_score hands =
  let rec calculate_score_recursive acc m =
    if m > List.length hands then
      acc
    else
      let (_hand, score) = List.nth hands (m - 1) in
      calculate_score_recursive (acc + score * m) (m + 1)
  in
  calculate_score_recursive 0 1
  
let score = calculate_score sorted_inputs;;
Printf.printf "Part 1 | Score: %d\n" score;;

(* Part 2 *)
(* J are nore Joker -> but for comparable, they have 0 value *)

let compare_labels_joker label1 label2 =
  match label1, label2 with
  | A, A -> 0
  | A, _ -> 1
  | _, A -> -1
  | K, K -> 0
  | K, _ -> 1
  | _, K -> -1
  | Q, Q -> 0
  | Q, _ -> 1
  | _, Q -> -1
  | T, T -> 0
  | T, _ -> 1
  | _, T -> -1
  | L9, L9 -> 0
  | L9, _ -> 1
  | _, L9 -> -1
  | L8, L8 -> 0
  | L8, _ -> 1
  | _, L8 -> -1
  | L7, L7 -> 0
  | L7, _ -> 1
  | _, L7 -> -1
  | L6, L6 -> 0
  | L6, _ -> 1
  | _, L6 -> -1
  | L5, L5 -> 0
  | L5, _ -> 1
  | _, L5 -> -1
  | L4, L4 -> 0
  | L4, _ -> 1
  | _, L4 -> -1
  | L3, L3 -> 0
  | L3, _ -> 1
  | _, L3 -> -1
  | L2, L2 -> 0
  | L2, _ -> 1
  | _, L2 -> -1
  | J, J -> 0


let hand_of_string_sorted s =
  let label_of_char c =
    match c with
    | 'A' -> A | 'K' -> K | 'Q' -> Q | 'J' -> J | 'T' -> T
    | '9' -> L9 | '8' -> L8 | '7' -> L7 | '6' -> L6 | '5' -> L5
    | '4' -> L4 | '3' -> L3 | '2' -> L2
    | _ -> failwith "Invalid card label"
  in

  let rec parse_cards acc = function
    | [] -> List.rev acc
    | hd :: tl -> parse_cards ((label_of_char hd) :: acc) tl
  in

  let compare_cards_in_list lst card1 card2 =
    let count_card lst card =
      List.fold_left (fun acc x -> if x = card then acc + 1 else acc) 0 lst
    in
    let count_compare = compare (count_card lst card2) (count_card lst card1) in
    if count_compare = 0 then compare card1 card2 else count_compare
  in
  
  let sort_by_frequency cards =
    List.sort (fun card1 card2 -> compare_cards_in_list cards card1 card2) cards
  in

  match String.to_seq s |> List.of_seq with
  | [c1; c2; c3; c4; c5] ->
    let cards = parse_cards [] [c1; c2; c3; c4; c5] in
    let cards = List.filter (fun x -> x <> J) cards in
    let sorted_cards = sort_by_frequency cards in
    (* Insert the missing number of card (5-lenght) as the most frequent one in the begening*)
    let sorted_cards = match List.length sorted_cards with
      | 5 -> sorted_cards
      | 4 -> [List.hd sorted_cards] @ sorted_cards
      | 3 -> [List.hd sorted_cards] @ [List.hd sorted_cards] @ sorted_cards
      | 2 -> [List.hd sorted_cards] @ [List.hd sorted_cards] @ [List.hd sorted_cards] @ sorted_cards
      | 1 -> [List.hd sorted_cards] @ [List.hd sorted_cards] @ [List.hd sorted_cards] @ [List.hd sorted_cards] @ sorted_cards
      | 0 -> [A; A; A; A; A]
      | _ -> failwith "Invalid hand string"
    in
    (match sorted_cards with
    | [l1; _l2; _l3; _l4; _l5] when List.for_all (( = ) l1) cards -> Five_of_a_kind l1
    | [l1; l2; l3; l4; _l5] when List.for_all (( = ) l1) [l2; l3; l4;] -> Four_of_a_kind (l1, l2)
    | [l1; l2; l3; l4; l5] when List.for_all (( = ) l1) [l2; l3;] && List.for_all (( = ) l4) [l5;] -> Full_house (l1, l4)
    | [l1; l2; l3; _l4; _l5] when List.for_all (( = ) l1) [l2; l3;] -> Three_of_a_kind (l1, l2, l3)
    | [l1; l2; l3; l4; _l5] when List.for_all (( = ) l1) [l2;] && List.for_all (( = ) l3) [l4;] -> Two_pair (l1, l3, l4)
    | [l1; l2; l3; l4; l5] when List.for_all (( = ) l1) [l2;] -> One_pair (l1, l3, l4, l5)
    | [l1; l2; l3; l4; l5] -> High_card (l1, l2, l3, l4, l5)
    | _ -> failwith "Invalid hand string"
    )
  | _ -> failwith "Invalid hand string"

let rec compare_hand_types (hand1str, _) (hand2str, _) =
  let hand1 = hand_of_string_sorted hand1str in
  let hand2 = hand_of_string_sorted hand2str in

  match hand1, hand2 with
  | Five_of_a_kind _, Five_of_a_kind _ -> compare_high_cards hand1str hand2str
  | Five_of_a_kind _, _ -> 1
  | _, Five_of_a_kind _ -> -1

  | Four_of_a_kind _, Four_of_a_kind _ -> compare_high_cards hand1str hand2str
  | Four_of_a_kind _, _ -> 1
  | _, Four_of_a_kind _ -> -1

  | Full_house _, Full_house _ -> compare_high_cards hand1str hand2str
  | Full_house _, _ -> 1
  | _, Full_house _ -> -1

  | Three_of_a_kind _, Three_of_a_kind _ -> compare_high_cards hand1str hand2str
  | Three_of_a_kind _, _ -> 1
  | _, Three_of_a_kind _ -> -1

  | Two_pair _, Two_pair _ -> compare_high_cards hand1str hand2str
  | Two_pair _, _ -> 1
  | _, Two_pair _ -> -1

  | One_pair _, One_pair _ -> compare_high_cards hand1str hand2str
  | One_pair _, _ -> 1
  | _, One_pair _ -> -1

  | High_card _, High_card _ -> compare_high_cards hand1str hand2str

and compare_high_cards hand1str hand2str =
  let rec compare_cards cards1 cards2 =
    match cards1, cards2 with
    | [], [] -> 0
    | [], _ -> 1
    | _, [] -> -1
    | card1 :: rest1, card2 :: rest2 ->
      let cmp = compare_labels_joker card1 card2 in
      if cmp = 0 then compare_cards rest1 rest2
      else cmp
  in
  let label_of_char c =
    match c with
    | 'A' -> A | 'K' -> K | 'Q' -> Q | 'J' -> J | 'T' -> T
    | '9' -> L9 | '8' -> L8 | '7' -> L7 | '6' -> L6 | '5' -> L5
    | '4' -> L4 | '3' -> L3 | '2' -> L2
    | _ -> failwith "Invalid card label"
  in
  let rec parse_cards acc = function
    | [] -> List.rev acc
    | hd :: tl -> parse_cards ((label_of_char hd) :: acc) tl
  in
  let hand1 = parse_cards [] [hand1str.[0]; hand1str.[1]; hand1str.[2]; hand1str.[3]; hand1str.[4]] in
  let hand2 = parse_cards [] [hand2str.[0]; hand2str.[1]; hand2str.[2]; hand2str.[3]; hand2str.[4]] in
  match hand1, hand2 with
  | cards1, cards2 -> compare_cards cards1 cards2
;;


let sorted_inputs = List.sort compare_hand_types parsed_inputs;;

let calculate_score hands =
  let rec calculate_score_recursive acc m =
    if m > List.length hands then
      acc
    else
      let (_hand, score) = List.nth hands (m - 1) in
      calculate_score_recursive (acc + score * m) (m + 1)
  in
  calculate_score_recursive 0 1
  
let score = calculate_score sorted_inputs;;
Printf.printf "Part 2 | Score: %d\n" score;;

