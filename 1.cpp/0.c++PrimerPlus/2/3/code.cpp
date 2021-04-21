#include <iostream>

using std::cout;
using std::endl;
using std::cin;

void printStart(void);
void printEnd(void);

int main(void)
{
    printStart();
    printStart();
    printEnd();
    printEnd();
}

void printStart(void)
{
    cout << "Three blind mice" << endl;
}

void printEnd(void)
{
    cout << "See how they run" << endl;
}