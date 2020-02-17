# spq
Sorting Priority Queue

https://github.com/ptravers/spq/workflows/CI/badge.sv

SPQ provides a Priority Queue that ensures an even distribution of elements over a herirarchical feature space.

e.g.
If the features are Class and Child. Where each child belongs to a class at School.
The queue could represent a line of children waiting for sweets.

SPQ will guarantee that the sweets are given to the children in a round robin whilst ensuring that the classes the children
belong too are participanting in a round robin.

Class 1 = [Child_A, Child_B]
Class 2 = [Child_C, Child_D]

The queue will return Child_A, Child_C, Child_B, Child_D
