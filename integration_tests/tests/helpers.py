from proto import spq_pb2

def drain_queue(spq_client):
    size = spq_client.GetSize(spq_pb2.GetSizeRequest()).size

    if size == 0:
        return None

    for i in range(0, size):
        spq_client.Dequeue(spq_pb2.DequeueRequest())
