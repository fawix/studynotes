package quickcheck

import java.util.NoSuchElementException

import org.scalacheck.Arbitrary
import org.scalacheck.Arbitrary.arbInt
import org.scalacheck.Arbitrary.arbitrary
import org.scalacheck.Gen
import org.scalacheck.Gen.const
import org.scalacheck.Gen.oneOf
import org.scalacheck.Prop.forAll
import org.scalacheck.Prop.propBoolean
import org.scalacheck.Prop.throws
import org.scalacheck.Properties

abstract class QuickCheckHeap extends Properties("Heap") with IntHeap {

  lazy val genHeap: Gen[H] = for {
    a <- arbitrary[A]
    h <- oneOf(const(empty), genHeap)
  } yield insert(a,h)
  
  implicit lazy val arbHeap: Arbitrary[H] = Arbitrary(genHeap)

  property("gen1") = forAll { (h: H) =>
    val m = if (isEmpty(h)) 0 else findMin(h)
    findMin(insert(m, h)) == m
  }
  
  property("find minimum  with 1 element") = forAll { a: A => 
    val h = insert(a, empty)
    findMin(h) == a
  }
  
  property("find mininmum value with 2 elements") = forAll { (a1: A, a2:A) => 
    val h = insert(a2, insert(a1, empty))
    val min = findMin(h)
    
    if (a1 < a2) min == a1
    else min == a2
  }
  
  property("insert immutability") = forAll { (a1: A, a2:A) => 
    val h1 = insert(a1, empty)
    val h2 = insert(a2, h1)
    
    deleteMin(h1) == empty
  }
  
  property("delete with 1 element") = forAll { a: A => 
    val h = insert(a, empty)
    val h1 = deleteMin(h)
    
    ((h1 == empty) && (findMin(h) == a)) 
  }
  
  property("meld lists") = forAll { (a1: A, a2:A) => 
    val h1 = insert(a1, empty)
    val h2 = insert(a2, empty)
    val h3 = meld(h1, h2)
    
    if ( a1 < a2 ) findMin(h3) == a1 && findMin(h1) == a1 && findMin(h2) == a2  && deleteMin(deleteMin(h3)) == empty
    else findMin(h3) == a2 && findMin(h1) == a1 && findMin(h2) == a2 && deleteMin(deleteMin(h3)) == empty
    
  }
  
  property("deleteMin from empty heap") = forAll { a: A => 
    throws(classOf[NoSuchElementException])(deleteMin(empty))
  }
  
  property("findMin from empty heap") = forAll { a: A => 
    throws(classOf[NoSuchElementException])(findMin(empty))
  }
  
  property("meld ordering") = forAll { (a1: A, a2:A, a3:A) => 
    val h1 = insert(a1, empty)
    val h2 = insert(a2, empty)
    val h3 = insert(a3, empty)
    val h4 = meld(h1, h2)
    val h5 = meld(h3, h4)
    
    val aux = if (a1 < a2) a1 else a2 
    val min = if (a3 < aux) a3 else aux
    
    findMin(h5) == min
  }
  
  property("meld ordering2") = forAll { (h1: H, h2:H) =>
    val h3 = meld(h1, h2)
    val min1 = findMin(h1)
    val min2 = findMin(h2)
    
    if (min1 < min2) findMin(h3) == min1
    else findMin(h3) == min2
  }
    
  property("meld add remove ordering") = forAll { (h1: H, h2: H) =>
    def remMin(ts: H, as: List[Int]): List[Int] = {
      if (isEmpty(ts)) as
      else findMin(ts) :: remMin(deleteMin(ts), as)
    }
    val meld1 = meld(h1, h2)
    val min1 = findMin(h1)
    val meld2 = meld(deleteMin(h1), insert(min1, h2))
    val xs1 = remMin(meld1, Nil)
    val xs2 = remMin(meld2, Nil)
    xs1 == xs2
  }
  


}
