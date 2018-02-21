package calculator

object TweetLength {
  final val MaxTweetLength = 140

  def tweetRemainingCharsCount(tweetText: Signal[String]): Signal[Int] = {
    Signal(MaxTweetLength - tweetLength(tweetText()))
  }

  // note to future self
  // found!
  // mistake here was silly, 
  // should not to return different signals per threshold but a 
  // signal that behave differently on threshold change
  def colorForRemainingCharsCount(remainingCharsCount: Signal[Int]): Signal[String] = {
    
    Signal(remainingCharsCount() match {
      case x if x >= 15 => "green"
      case x if x >= 0 && x <= 14 => "orange"
      case _ => "red"
    })
    
    
    /*val count = remainingCharsCount()
    
    if (count >= 15) Signal ("green")
    else if (count >= 0 && count <= 14) Signal ("orange")
    else Signal ("red")*/
    
  }

  /** Computes the length of a tweet, given its text string.
   *  This is not equivalent to text.length, as tweet lengths count the number
   *  of Unicode *code points* in the string.
   *  Note that this is still a simplified view of the reality. Full details
   *  can be found at
   *  https://dev.twitter.com/overview/api/counting-characters
   */
  private def tweetLength(text: String): Int = {
    /* This should be simply text.codePointCount(0, text.length), but it
     * is not implemented in Scala.js 0.6.2.
     */
    if (text.isEmpty) 0
    else {
      text.length - text.init.zip(text.tail).count(
          (Character.isSurrogatePair _).tupled)
    }
  }
}
