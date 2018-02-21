package calculator

import org.scalatest.FunSuite

import org.junit.runner.RunWith
import org.scalatest.junit.JUnitRunner

import org.scalatest._

import TweetLength.MaxTweetLength

@RunWith(classOf[JUnitRunner])
class CalculatorSuite extends FunSuite with ShouldMatchers {

  /******************
   ** TWEET LENGTH **
   ******************/

  def tweetLength(text: String): Int =
    text.codePointCount(0, text.length)

  test("tweetRemainingCharsCount with a constant signal") {
    val result = TweetLength.tweetRemainingCharsCount(Var("hello world"))
    assert(result() == MaxTweetLength - tweetLength("hello world"))

    val tooLong = "foo" * 200
    val result2 = TweetLength.tweetRemainingCharsCount(Var(tooLong))
    assert(result2() == MaxTweetLength - tweetLength(tooLong))
  }

  test("tweetRemainingCharsCount with a supplementary char") {
    val result = TweetLength.tweetRemainingCharsCount(Var("foo blabla \uD83D\uDCA9 bar"))
    assert(result() == MaxTweetLength - tweetLength("foo blabla \uD83D\uDCA9 bar"))
  }


  test("colorForRemainingCharsCount with a constant signal") {
    val resultGreen1 = TweetLength.colorForRemainingCharsCount(Var(52))
    assert(resultGreen1() == "green")
    val resultGreen2 = TweetLength.colorForRemainingCharsCount(Var(15))
    assert(resultGreen2() == "green")

    val resultOrange1 = TweetLength.colorForRemainingCharsCount(Var(12))
    assert(resultOrange1() == "orange")
    val resultOrange2 = TweetLength.colorForRemainingCharsCount(Var(0))
    assert(resultOrange2() == "orange")

    val resultRed1 = TweetLength.colorForRemainingCharsCount(Var(-1))
    assert(resultRed1() == "red")
    val resultRed2 = TweetLength.colorForRemainingCharsCount(Var(-5))
    assert(resultRed2() == "red")
  }
  
  /******************
   **  POLYNOMIAL  **
   ******************/
  
  test("polinomial delta") {
    
    //x^2 + 2x + 1 = 0
    // root: -1
    // delta: 0
    val delta1 = Polynomial.computeDelta(Signal(1), Signal(2), Signal(1))
    assert(delta1() == 0)
    val roots1 = Polynomial.computeSolutions(Signal(1), Signal(2), Signal(1), delta1)
    assert(roots1() == Set(-1.0, -1.0)) 
    
    //2x^2 + 2x - 5
    //roots: -1/2 - sqrt(11)/2  AND sqrt(11)/2 -1/2
    //delta: 44
    val delta2 = Polynomial.computeDelta(Signal(2), Signal(2), Signal(-5))
    assert(delta2() == 44)
    
    val rs = Set(1.1583123951777, -2.1583123951777) // from wolfram
    
    val roots2 = Polynomial.computeSolutions(Signal(2), Signal(2), Signal(-5), delta2)
    assert(roots2() == rs)
  }
  
  
  /******************
   **  CALCULATOR  **
   ******************/
  
  test("calculator 1") {
    
    def getSig(exp: Expr): Signal[Expr] = {
     Signal(exp) 
    }
    
    val a = ("a", getSig(Literal(10.0)))
    val b = ("b", getSig(Plus(Literal(15.0), Literal(10.0))))
    val c = ("c", getSig(Minus(Literal(15.0), Literal(10.0))))
    val d = ("d", getSig(Times(Literal(10.0), Literal(10.0))))
    val e = ("e", getSig(Divide(Literal(8.0), Literal(10.0))))
    
    val f = ("f", getSig(Plus(Ref("a"), Ref("c"))))
    val g = ("g", getSig(Plus(Ref("b"), Ref("b"))))
    val h = ("h", getSig(Minus(Ref("b"), Ref("z"))))
    val i = ("i", getSig(Plus(Ref("j"), Ref("b"))))
    val j = ("j", getSig(Times(Literal(2), Ref("i"))))
    val k = ("k", getSig(Ref("l")))
    val l = ("l", getSig(Ref("k")))
    val m = ("m", getSig(Literal(1.0)))
    val n = ("n", getSig(Literal(1.0)))
    
    val blah = Map(("x",24),("y",25),("z",26))
    
    val input = Map(a,b,c,d,e,f,g,h)
//    val input2 = Map(a,b,c,d,e,f,g,h,i,j,k,l,m,n)
    
    val response = Calculator.computeValues(input)
    
    assert(response("a")() == 10.0)
    assert(response("b")() == 25.0)
    assert(response("c")() == 5.0)
    assert(response("d")() == 100.0)
    assert(response("e")() == 0.80)
    assert(response("f")() == 15.0)
    assert(response("g")() == 50.0)
    assert(response("h")().isNaN())
    
   /* val input2 = Map(k,l)
    val response2 = Calculator.computeValues(input2)
    
    val thrown = intercept[AssertionError] {
      response2("l")()
    }*/
    
    //assert(thrown.getMessage === "cyclic signal definition")
    
    //check for cyclic dependencies
    //check for nonexistent references
    
    val input3 = Map(i,j)
    val response3 = Calculator.computeValues(input3)
    
    val thrown2 = intercept[AssertionError] {
      response3("k")()
    }
  }
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  

}
