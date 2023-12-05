
import scala.io.Source
import scala.io.BufferedSource


def readFile(): BufferedSource = {
  val filePath = "input.txt"
  Source.fromFile(filePath)
}

def ProcessGame(input: String): (Int, Set[Int], Array[Int]) = {
      val result = input.split(":")  
      val gameID = result(0).replace("Card", "").trim().toInt
      val nums = result(1)
      val cardsSplit = nums.split("\\|")  
      val winningCards = cardsSplit(0).trim.split("\\s+").map(_.toInt)
      val myCards = cardsSplit(1).trim.split("\\s+").map(_.toInt)
      val winningCardsSet: Set[Int] = winningCards.toSet
      (gameID, winningCardsSet, myCards)
}

object Part1 extends App {
  // O(n) in time
  val filePath = "input.txt"
  val source = readFile()

  try {
    var totalSum = 0
    for (line <- source.getLines()) {
      val (gameID, winningCardsSet, myCards) = ProcessGame(line)
      var gameSum = 0
      for (i <- 0 to myCards.length - 1) {
        if (winningCardsSet.contains(myCards(i))) {
          if (gameSum == 0) {
            gameSum += 1
          } else {
            gameSum *= 2
          }
        }
      }
      totalSum += gameSum
    }
    println(s"Part 1 -- Sum: $totalSum\n")
  } finally {
    source.close()
  }
}

object Part2 extends App {
  // O(2n) in time - Memory space not optimal
  val filePath = "input.txt"
  val source = readFile()

  try {
    var gamesWinMap = scala.collection.mutable.Map[Int, Array[Int]]()
    var myWinningCards = scala.collection.mutable.Map[Int, Int]()
    var lastCard = 0
    // Build a map of gameID to winning cards
    for (line <- source.getLines()) {
      val (gameID, winningCardsSet, myCards) = ProcessGame(line)
      myWinningCards(gameID) = 1
      lastCard = gameID
      var gameSum = 0
      gamesWinMap(gameID) = Array()
      for (i <- 0 to myCards.length - 1) {
        if (winningCardsSet.contains(myCards(i))) {
          gamesWinMap(gameID) = gamesWinMap(gameID) :+ myCards(i)
        }
      }
    }
    // Now that we have a map of gameID to winning cards, we can process the winning cards
    var totalSum = 0
    for (i <- 1 to lastCard) {
      val nbCardInstance = myWinningCards(i)
      totalSum += nbCardInstance
      var nbWinningCards = 1
      for (gameID <- gamesWinMap(i)) {
        myWinningCards(i + nbWinningCards) += nbCardInstance
        nbWinningCards += 1
      }
    }

    println(s"Part 2 -- Sum: $totalSum\n")
  } finally {
    // Make sure to close the source to free up resources
    source.close()
  }
}
