#include <iostream>

using std::cout;
using std::endl;
using std::cin;

double calculateFahrenheit(double);

int main(void)
{
    double dc;
    cout << "Please enter a Celsius value: ";
    cin >> dc;
    cout << dc << " degree Celsius is ";
    cout << calculateFahrenheit(dc);
    cout << " degrees Fahrenheit." << endl;
}

double calculateFahrenheit(double dc)
{
    double F = 0;

    F = dc * 1.8 + 32.0;

    return F;
}