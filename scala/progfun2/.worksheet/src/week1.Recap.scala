package week1

import scala.util.parsing.json


object Recap {;import org.scalaide.worksheet.runtime.library.WorksheetSupport._; def main(args: Array[String])=$execute{;$skip(115); 
  println("Recap of Functions and Pattern Matching");$skip(844); 
  
  /**
  
  	Case Classes are Scala's preferred way to define comples data.
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
  **/
  
  def show (json: JSON) : String = json match {
  	case JSeq(elems) =>
  		"[" + (elems map show mkString + ", ") + "]"
  	case JObj (bindings) =>
  		val assocs = bindings map {
  			case (key, value) => "\"" + key + "\": " + show(value)
  		}
  		"{" + (assocs mkString ", ") + "}"
  		
  	case
  
  
  };System.out.println("""show: (json: <error>)String""")}
  
}
