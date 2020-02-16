from proto import spq_pb2

def test_check_health(health_client):
    response = health_client.Check(spq_pb2.HealthCheckRequest())
    assert response.status == 1
