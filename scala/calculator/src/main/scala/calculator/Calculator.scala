package calculator

sealed abstract class Expr
final case class Literal(v: Double) extends Expr
final case class Ref(name: String) extends Expr
final case class Plus(a: Expr, b: Expr) extends Expr
final case class Minus(a: Expr, b: Expr) extends Expr
final case class Times(a: Expr, b: Expr) extends Expr
final case class Divide(a: Expr, b: Expr) extends Expr

object Calculator {
  def computeValues(
      namedExpressions: Map[String, Signal[Expr]]): Map[String, Signal[Double]] = {
    /*for { //maybe for is no good? 
      (st, ex) <- namedExpressions          
    } yield (st,Signal(eval(ex(), namedExpressions)))*/
    
    namedExpressions.mapValues { x => Signal(eval(x(), namedExpressions)) }
  }

  def eval(expr: Expr, references: Map[String, Signal[Expr]]): Double = {
    expr match {
      case Literal(x) => x
      case Ref(x) => {// ugh.. gotta improve this... how? fold? ... contains?
        if (!references.tail.contains(x)) eval(getReferenceExpr(x, references), references) 
        else Double.NaN  
        /*try {
          references(x) // will throw exception if don't exist
           } catch {
        case t: Throwable => Double.NaN
        }
          
        val expr_x = getReferenceExpr(x, references) 
        expr_x match {
          case Ref(y) => { 
            val exp_y = getReferenceExpr(y, references)
            assert(exp_y != Ref(x), "cyclic signal definition") 
            eval(expr_x, references)
          }
          case _ => eval(expr_x, references)
        }*/
      }
      case Plus(a,b)   => eval(a, references) + eval(b, references)
      case Minus(a,b)  => eval(a, references) - eval(b, references)
      case Times(a,b)  => eval(a, references) * eval(b, references)
      case Divide(a,b) => eval(a, references) / eval(b, references)
    }
  }

  /** Get the Expr for a referenced variables.
   *  If the variable is not known, returns a literal NaN.
   */
  private def getReferenceExpr(name: String, references: Map[String, Signal[Expr]]) = {
    references.get(name).fold[Expr] {
      Literal(Double.NaN)
    } { exprSignal =>
      exprSignal()
    }
  }
}
