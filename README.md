# spq
Sorting Priority Queue

![](https://github.com/ptravers/spq/workflows/CI/badge.svg?branch=master)

SPQ provides a Priority Queue that ensures an even distribution of dequeued elements over a herirarchical feature space. The queue is a grpc service that can be run with docker. The project is a WIP and doesn't currently support full durability. But it shouldn't be too long before it does.

## But Why?
At previous place of work we had a problem of fair resource sharing for a pool of worker nodes. We wanted a queue that provided **exactly once delivery** and would be [**CP**](https://en.wikipedia.org/wiki/CAP_theorem) or [**PC+EC**](https://en.wikipedia.org/wiki/PACELC_theorem) in the event of a network partition or the choice of latency over consistency. But most importantly we wanted the queue to provide the ability to re-sort on which features had most recently been removed from the queue. The system we wanted didn't exist so we built a much less generic solution than this one that only supported two features and leveraged postgres as a data store.

## Toy Example
If you imagine the queue as a line of children waiting for lunch and the features are Class and Age. Where each child belongs to a class at School and is a number of years old.

SPQ will guarantee that the sweets are given to the children in a round robin based on age and class.
Class 1 = [Child_A, Child_B]
Class 2 = [Child_C, Child_D]

Age 9 = [Child_D, Child_A]
Age 10 = [Child_B, Child_C]

The children having arrived in order in alphabetic order e.g. A, B, C and D.

The queue will return Child_A, Child_C, Child_B and Child_D

## API
The queue leverages gRPC for all communication and thus requires support for HTTP 2.0

### Create queue
Creates a queue with a set of features that all items inserted must have

e.g.
Create queue named "school" with features Age and Class

### Enqueue
Adds an item to the queue. Request must contain:
- Name of the Queue
- The list of Features with a value for each feature
- The item to be stored. Which is an arbitrary set of bytes

e.g.
Enqueue item request:
- queue named "school"
- features [(Age, 9), (Class, 2)]

### Dequeue
Remove next item from the queue. Request must contain:
- Name of the Queue

e.g.
Dequeue item request:
- queue named "school"

### Peek
View the next item in the queue without removing it from the queue. Request must contain:
- Name of the Queue

e.g.
Peek item request:
- queue named "school"

### Get Size
Get the current size of the queue
- Name of the Queue

e.g.
Get Size request:
- queue named "school"

### Get Epoch
Get the current "epoch" of the queue. See documentation for details of semantics of epoch
- Name of the Queue

e.g.
Get Epoch request:
- queue named "school"

## Glossary
- Epoch = A Lamport Clock that increases for each mutation of the queue
- Feature = A category of values i.e. Age in Years
- Feature Value = A value in the feature category i.e. 8 years old

## Implementation
The system is structured into two packages a grpc server and the queue itself

The queue is built atop RocksDB as it's persistence layer.

**This README is underconstruction**
