# spq
Sorting Priority Queue

![](https://github.com/ptravers/spq/workflows/CI/badge.svg?branch=master)

SPQ provides a Priority Queue that ensures an even distribution of dequeued elements over a herirarchical feature space. The queue is a grpc service that can be run with docker. The project is a WIP and doesn't currently support full durability. But it shouldn't be too long before it does.

## But Why?
At my current place of work we had a problem of fair resource sharing for a pool of worker nodes. We wanted a queue that provided **exactly once delivery** and would be [**CP**](https://en.wikipedia.org/wiki/CAP_theorem) or [**PC+EC**](https://en.wikipedia.org/wiki/PACELC_theorem) in the event of a network partition or the choice of latency over consistency. But most importantly we wanted the queue to provide the ability to resort the queue based on which features had most recently been removed from the queue. The system we wanted didn't exist so we built a much less generic solution than this one that only supported two features.

## Toy Example
If the features are Class and Child. Where each child belongs to a class at School.
The queue could represent a line of children waiting for sweets.

SPQ will guarantee that the sweets are given to the children in a round robin whilst ensuring that the classes the children
belong too are participanting in a round robin.

Class 1 = [Child_A, Child_B]
Class 2 = [Child_C, Child_D]

The queue will return Child_A, Child_C, Child_B and Child_D
