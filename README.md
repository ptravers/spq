# spq
Sorting Priority Queue

![](https://github.com/ptravers/spq/workflows/CI/badge.svg?branch=master)

SPQ provides a Priority Queue that ensures an even distribution of dequeued elements over a herirarchical feature space. The queue is a grpc service that can be run with docker. The project is a WIP and doesn't currently support full durability. But it shouldn't be too long before it does.

## But Why?
At my current place of work we had a problem of fair resource sharing for a pool of worker nodes. We wanted a queue that provided **exactly once delivery** and would be [**CP**](https://en.wikipedia.org/wiki/CAP_theorem) or [**PC+EC**](https://en.wikipedia.org/wiki/PACELC_theorem) in the event of a network partition or the choice of latency over consistency. But most importantly we wanted the queue to provide the ability to resort the queue based on which features had most recently been removed from the queue. The system we wanted didn't exist so we built a much less generic solution than this one that only supported two features.

## Toy Example
If you imagine the queue as a line of children waiting for lunch and the features are Class and Age. Where each child belongs to a class at School and is a number of years old.

SPQ will guarantee that the sweets are given to the children in a round robin based on age and class.
Class 1 = [Child_A, Child_B]
Class 2 = [Child_C, Child_D]

Age 8 = [Child_D, Child_A]
Age 10 = [Child_B, Child_C]

The children having arrived in order in alphabetic order e.g. A, B, C and D.

The queue will return Child_A, Child_C, Child_B and Child_D
