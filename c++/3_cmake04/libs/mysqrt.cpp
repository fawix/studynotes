//https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method

double mysqrt (double number) 
{
    double error = 0.00001; //define the precision of your result
    double s = number;

    while ((s - number / s) > error) //loop until precision satisfied 
    {
        s = (s + number / s) / 2;
    }

    return s;
}
