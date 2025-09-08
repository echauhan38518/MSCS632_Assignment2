#include <iostream>
using namespace std;

int main() {
    {
        int* numbers = new int[5]; // allocated on heap memory
        for (int i = 0; i < 5; i++) {
            numbers[i] = i + 1;
        }
        cout << "C++: numbers[0] = " << numbers[0] << endl;
        cout << "C++: numbers[0] = " << numbers[1] << endl;
        cout << "C++: numbers[0] = " << numbers[2] << endl;
        cout << "C++: numbers[0] = " << numbers[3] << endl;
        cout << "C++: numbers[0] = " << numbers[4] << endl;

        delete[] numbers; // must free manually using keyword delete
    }
    cout << "C++: memory freed manually" << endl;
    return 0;
}