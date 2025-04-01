#include <iostream>
#if defined(__GNUC__) && !defined(__clang__)
#endif

int main() { 
    int a; 
    int b; 
    a = b + 1;
    if (a && b) {
        b = a / b;
    } 
    int c = a+b+1; // Comentario
    return 0;  
} 