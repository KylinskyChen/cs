#include <iostream>

using std::cout;
using std::endl;
using std::cin;

int transform(int);

int main(void)
{
    int longg;
    cout << "Enter the length in long:";
    cin >> longg;
    int size = transform(longg);
    cout << longg << " long = ";
    cout << size << " size." << endl;
}

int transform(int longs)
{
    int size = 220 * longs;
    return size;
}