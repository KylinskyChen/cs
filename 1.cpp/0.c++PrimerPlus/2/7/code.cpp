#include <iostream>
#include <string>

using namespace std;

int getNumber(string);
void showTime(int, int);

int main(void)
{
    int h, m;

    h = getNumber("hours");
    m = getNumber("minutes");

    showTime(h, m);
}

int getNumber(string info)
{
    cout << "Enter the number of " << info << ":";
    int num = 0;
    cin >> num;
    return num;
}

void showTime(int h, int m)
{
    cout << "Time: " << h << ":" << m << endl;
}