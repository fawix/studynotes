package week1

object example01 extends App {

   println("Recap of Functions and Pattern Matching")
  
  /**
  
  	Case Classes are Scala's preferred way to define complex data.
  	Example of complex data (JSON):
  	
  	{
  		"firstName"    : "John",
  		"lastName"     : "Smith",
  		"address"      : {
  			"streetAddress" : "21 2nd Street",
  			"state"         : "NY"
  			"postalCode"    : 10021
  		},
  		"phoneNumbers" : [
  			{"type" : "home", "number" : "212 555-1234" },
  			{"type" : "fax", "number" : "646 555-4567" }
  		]
  		
  	}
  	
  	The below is a function that returns the string representing that JSON.
  
  
  def show (json: JSON) : String = json match {
  	case JSeq(elems) =>
  		"[" + (elems map show mkString + ", ") + "]"
  	case JObj (bindings) =>
  		val assocs = bindings map {
  			case (key, value) => "\"" + key + "\": " + show(value)
  		}
  		"{" + (assocs mkString ", ") + "}"
  		
  	case JNum(num)  => num.toString
  	case JStr (str) => '\"' + str + '\"'
  	case JBool (b)  => b.toString
  	case JNull      => "null"
  }

  **/
   
   
}