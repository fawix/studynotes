//https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method

double mysqrt (double number) 
{

#if defined (HAME_LOG) && (HAVE_EXP)
    return exp(log(number)*0.5);
#else 
    double error = 0.00001; //define the precision of your result
    double s = number;

    while ((s - number / s) > error) //loop until precision satisfied 
    {
        s = (s + number / s) / 2;
    }

    return s;
#endif
}
