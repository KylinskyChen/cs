#include <iostream>

using std::cout;
using std::endl;
using std::cin;

double calculateAstonomicalUnits(double);

int main(void)
{
    double ly;
    cout << "Enter the number of light years: ";
    cin >> ly;
    cout << ly << " light years = ";
    cout << calculateAstonomicalUnits(ly);
    cout << " astonomical units." << endl;
}

double calculateAstonomicalUnits(double ly)
{
    double au = 0;

    au = ly * 63240;

    return au;
}