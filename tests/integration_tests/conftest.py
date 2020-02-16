import pytest
import grpc
from proto import spq_pb2_grpc, spq_pb2


@pytest.fixture(scope="session")
def channel():
    return grpc.insecure_channel('spq:9090')


@pytest.fixture(scope="session")
def health_client(channel):
    return spq_pb2_grpc.HealthServiceStub(channel)


@pytest.fixture(scope="session")
def spq_client(channel):
    return spq_pb2_grpc.SortingPriorityQueueServiceStub(channel)
