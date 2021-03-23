#include <iostream>

using std::cout;
using std::endl;
using std::cin;

int cntYears(int age);

int main(void)
{
    int age;
    cin >> age;
    cout << cntYears(age) << endl;
}

int cntYears(int age)
{
    int years = 0;

    years = age * 12;

    return years;
}