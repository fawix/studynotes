package streams

import org.scalatest.FunSuite

import org.junit.runner.RunWith
import org.scalatest.junit.JUnitRunner

import Bloxorz._

@RunWith(classOf[JUnitRunner])
class BloxorzSuite extends FunSuite {

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

  trait Level1 extends SolutionChecker {
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
  }
  
  
  
	test("terrain function level 1") {
    new Level1 {
      assert(terrain(Pos(0,0)), "0,0")
      assert(terrain(Pos(1,1)), "1,1") // start
      assert(terrain(Pos(4,7)), "4,7") // goal
      assert(terrain(Pos(5,8)), "5,8")
      assert(!terrain(Pos(5,9)), "5,9")
      assert(terrain(Pos(4,9)), "4,9")
      assert(!terrain(Pos(6,8)), "6,8")
      assert(!terrain(Pos(4,11)), "4,11")
      assert(!terrain(Pos(-1,0)), "-1,0")
      assert(!terrain(Pos(0,-1)), "0,-1")
    }
  }

	test("findChar level 1") {
    new Level1 {
      assert(startPos == Pos(1,1))
    }
  }
	
	test("is standing block") {
	  new Level1 {
	    assert(block.isStanding, "b1: 1,1 b2: 1,1")
		  assert(!block.right.isStanding, "b1: 1,2 b2: 1,3")
	  }
	  
	}
	
	test("neighbors level 1") {
	  new Level1 {
	    val neigh = block.neighbors
	    assert(neigh.size == 4)
		  assert(neigh contains (block.right, Right), "Right")
		  assert(neigh contains (block.left, Left), "Left")
		  assert(neigh contains (block.up, Up), "Up")
		  assert(neigh contains (block.down, Down), "Down")
	  }
	}

	test("legal neighbors level 1") {
	 new Level1 {
	    val neigh = block.legalNeighbors
	    assert(neigh.size == 2)
		  assert(neigh contains (block.right, Right), "legal Right")
		  assert(!(neigh contains (block.left, Left)), "illegal Left")
		  assert(!(neigh contains (block.up, Up)), "illegal Up")
		  assert(neigh contains (block.down, Down), "legal Down")
	  } 
	}
	
	test("done level 1") {
	 new Level1 {
	    assert(!done(block), "not done")
	    assert(done(block.right.right.down.right.right.right.down), "done")
	  }
	}

	test("neighborsWithHistory level 1") {
    new Level1 {
      val nwh = neighborsWithHistory(block, List(Left,Up))
      val r = (Block(Pos(1,2), Pos(1,3)), List(Right,Left,Up))
      val d = (Block(Pos(2,1), Pos(3,1)), List(Down,Left,Up))

      assert(nwh.size == 2)
      assert(nwh contains r)
      assert(nwh contains d)
    }
  }
	
	test("newNeighborsOnly level 1") {
    new Level1 {
      val r = (Block(Pos(1,2), Pos(1,3)), List(Right,Left,Up))
      val d = (Block(Pos(2,1), Pos(3,1)), List(Down,Left,Up))
      val nno = newNeighborsOnly(Set(r,d).toStream, Set(Block(Pos(1,2), Pos(1,3)),Block(Pos(1,1), Pos(1,1))))

      assert(nno.size == 1)
      assert(nno contains d)
    }
  }
	
	test("optimal solution for level 1") {
    new Level1 {
      println(solution)
      assert(solve(solution) == Block(goal, goal))
    }
  }
	
	
	


	test("optimal solution length for level 1") {
    new Level1 {
      assert(solution.length == optsolution.length)
    }
  }

}
