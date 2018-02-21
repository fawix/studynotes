package week2

object sheet21 {;import org.scalaide.worksheet.runtime.library.WorksheetSupport._; def main(args: Array[String])=$execute{;$skip(75); 
  println("Welcome to the Scala worksheet");$skip(126); 
  
  /**
  
  Stream tail is just evaluated on demand.
  
  **/
  
  
  val xs = Stream.cons(1, Stream.cons(2, Stream.empty));System.out.println("""xs  : Stream.Cons[Int] = """ + $show(xs ));$skip(14); 
  println(xs);$skip(140); 
  
  
  def streamRange (lo: Int, hi: Int): Stream[Int] =
    if (lo >= hi ) Stream.empty
    else Stream.cons(lo, streamRange(lo + 1, hi));System.out.println("""streamRange: (lo: Int, hi: Int)Stream[Int]""");$skip(278); 
    
  // stream range is only evaluated upon demand, defferently than listRange
  // the above is isomorphic to the below.
  // the differece is when it's evaluated.
  
  def listRange (lo: Int, hi: Int): List[Int] =
    if (lo >= hi ) Nil
    else lo :: listRange(lo + 1, hi);System.out.println("""listRange: (lo: Int, hi: Int)List[Int]""");$skip(150); 
  
  
  def expr = {
    val x = {print ("x"); 1}
    lazy val y   = {print ("y"); 2}
    val z = {print ("z"); 3}
    
    z + y + x + z + y + z
  };System.out.println("""expr: => Int""");$skip(10); val res$0 = 
  
  expr;System.out.println("""res0: Int = """ + $show(res$0))}
  
  
  
}
