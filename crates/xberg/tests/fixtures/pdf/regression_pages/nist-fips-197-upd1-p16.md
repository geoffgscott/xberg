# 4\. Mathematical Preliminaries

For some transformations of the AES algorithms specifed in Sec. 5, each byte in the state array is interpreted as one of the 256 elements of a fnite feld, also known as a Galois Field, denoted by GF(2

8). 1

In order to defne addition and multiplication in GF(28), each byte {b b b b b b b b } is

7 6 5 4 3 2 1 0

interpreted as a polynomial, denoted by b(x), as follows: b(x) = b x7 +b x6 +b x5 +b x4 +b x3 +b x2 +b x+b . (4.1)

7 6 5 4 3 2 1 0

For example, { 01100011 } is represented by the polynomial x6 +x5 +x+1.

**4.1 Addition in GF(28)**

In order to add two elements in the fnite feld GF(28), the coeffcients of the polynomials that represent the elements are added modulo 2 (i.e., with the exclusive-OR operation, denoted by ⊕), so that 1⊕1 = 0, 1⊕0 = 1, and 0 ⊕0 = 0.

Equivalently, two bytes can be added by applying the exclusive-OR operation to each pair of corresponding bits in the bytes. Thus, the sum of {a a a a a a a a } and {b b b b b b b b }

7 6 5 4 3 2 1 0 7 6 5 4 3 2 1 0

is {a ⊕b a ⊕b a ⊕b a ⊕b a ⊕b a ⊕b a ⊕b a ⊕b }. (In Section 5.1.4, this

7 7 6 6 5 5 4 4 3 3 2 2 1 1 0 0

defnition is extended to words.) For example, the following three representations of addition are equivalent:

(x6 +x4 +x2 +x+1) + (x7 +x+1) = x7 +x6 +x4 +x2 (polynomial) { 01010111 } ⊕ { 10000011 } = { 11010100 } (binary) (4.2) { 57 } ⊕ { 83 } = { d4 } (hexadecimal).

Because the coeffcients of the polynomials are reduced modulo 2, the coeffcient 1 is equivalent to the coeffcient –1, so addition is equivalent to subtraction. For example, x

4 +x2 represents the same fnite feld element as x4 −x2, −x4 +x2, and −x4 −x2. Similarly, the sum of any element with itself is the zero element.

**4.2 Multiplication in GF(28)**

The symbol · denotes multiplication in GF 28 . Conceptually, this multiplication is defned on two bytes in two steps: 1) the two polynomials that represent the bytes are multiplied as polynomials, and 2) the resulting polynomial is reduced modulo the following fxed polynomial: m(x) = x8 +x4 +x3 +x+1. (4.3) Within both steps, the individual coeffcients of the polynomials are reduced modulo 2.

1

Information about the properties of fnite felds can be found in textbooks, such as Michael Artin's Algebra [4].
