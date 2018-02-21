package calculator

object Polynomial {
  def computeDelta(a: Signal[Double], b: Signal[Double],
      c: Signal[Double]): Signal[Double] = {
    Signal (math.pow(b(), 2) - (4*a()*c()))
  }

  //it's failing on irrational but what should i do??
  def computeSolutions(a: Signal[Double], b: Signal[Double],
      c: Signal[Double], delta: Signal[Double]): Signal[Set[Double]] = {
    Signal(
        if (a() != 0 )
          Set(((-b() + math.sqrt(delta())) / (2*a())),((-b() - math.sqrt(delta())) / (2*a())))
         else Set() 
        )
  }
}
