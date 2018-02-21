package week2

object sheet21 {
  println("Welcome to the Scala worksheet")
  
  /**
  
  Stream tail is just evaluated on demand.
  
  **/
  
  
  val xs = Stream.cons(1, Stream.cons(2, Stream.empty))
  println(xs)
  
  
  def streamRange (lo: Int, hi: Int): Stream[Int] =
    if (lo >= hi ) Stream.empty
    else Stream.cons(lo, streamRange(lo + 1, hi))
    
  // stream range is only evaluated upon demand, defferently than listRange
  // the above is isomorphic to the below.
  // the differece is when it's evaluated.
  
  def listRange (lo: Int, hi: Int): List[Int] =
    if (lo >= hi ) Nil
    else lo :: listRange(lo + 1, hi)
  
  
  def expr = {
    val x = {print ("x"); 1}
    lazy val y   = {print ("y"); 2}
    val z = {print ("z"); 3}
    
    z + y + x + z + y + z
  }
  
  expr
  
  
  
}