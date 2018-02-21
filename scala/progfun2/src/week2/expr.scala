package week2

object expr extends App{
  
  
  /**
  
  Stream tail is just evaluated on demand.
  
  **/
  
  def expr = {
    val x = {print ("x"); 1}
    lazy val y   = {print ("y"); 2}
    def z = {print ("z"); 3}
    
    z + y + x + z + y + x
  }
  
  expr
  
}