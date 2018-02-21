package main.scala

object generators {;import org.scalaide.worksheet.runtime.library.WorksheetSupport._; def main(args: Array[String])=$execute{;$skip(83); 
  println("Welcome to the Scala worksheet");$skip(88); 
  
	val integers = new Generator[Int] {
		def generate = scala.util.Random.nextInt()
	};System.out.println("""integers  : AnyRef{def generate: Int} = """ + $show(integers ));$skip(42); 
	
	val booleans = integers.map ( _ >= 0 );System.out.println("""booleans  : <error> = """ + $show(booleans ))}
	
	//def leafs: Generator[Leaf] =

  
}
