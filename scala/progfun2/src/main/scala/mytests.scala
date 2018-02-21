package streams

import scala.Vector

object mytests extends App {
  
  def blah = {
  
  val level =
    """ooo-------
      |oSoooo----
      |ooooooooo-
      |-ooooooooo
      |-----ooToo
      |------ooo-""".stripMargin
  
  lazy val vector: Vector[Vector[Char]] =
    Vector(level.split("\n").map(str => Vector(str: _*)): _*)
    
  println(vector)
  println(vector(1))
  println(vector(1)(1))
  
  
  }
  
  trait SolutionChecker extends GameDef with Solver with StringParserTerrain {
    /**
     * This method applies a list of moves `ls` to the block at position
     * `startPos`. This can be used to verify if a certain list of moves
     * is a valid solution, i.e. leads to the goal.
     */
    def solve(ls: List[Move]): Block =
      ls.foldLeft(startBlock) { case (block, move) =>
        require(block.isLegal) // The solution must always lead to legal blocks
        move match {
          case Left => block.left
          case Right => block.right
          case Up => block.up
          case Down => block.down
        }
    }
  }
  
  class Level1 extends SolutionChecker {
      /* terrain for level 1*/

    val level =
    """ooo-------
      |oSoooo----
      |ooooooooo-
      |-ooooooooo
      |-----ooToo
      |------ooo-""".stripMargin

    val optsolution = List(Right, Right, Down, Right, Right, Right, Down)
  
    val block = Block(Pos(1,1), Pos(1,1))
    
    def callme (x: String) = {
      println(x)
      solution
    }
    
    def callme2 (x: String) = {
      
      val lv = Vector(Vector("o", "o", "o", "-", "-", "-", "-", "-", "-", "-"), Vector("o", "S", "o", "o", "o", "o", "-", "-", "-", "-"), Vector("o", "o", "o", "o", "o", "o", "o", "o", "o", "-"), Vector("-", "o", "o", "o", "o", "o", "o", "o", "o", "o"), Vector("-", "-", "-", "-", "-", "o", "o", "T", "o", "o"), Vector("-", "-", "-", "-", "-", "-", "o", "o", "o", "-"))
      
      println(lv.map (_ indexOf "S").zipWithIndex.find(_._1 > -1).get)
      println(lv.map (_ indexOf "T").zipWithIndex.find(_._1 > -1).get)
      
      println(x)
      goal
    }
    
  }
  
  
  val x = new Level1
  x.callme("test")
  
  
  
  
  
  
}