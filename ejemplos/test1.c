#include <iostream>
#if defined(__GNUC__) && !defined(__clang__)
#warning "Compilando con GCC"
#endif

int main() { 
    int a; 
    int b; 
    a = b + 1;
    if (a && b) {
        b = a-1;
    } 
    int c = a+b+1; // Comentario
    return 0;  
} 