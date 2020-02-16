# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
import grpc

from proto import spq_pb2 as proto_dot_spq__pb2


class SortingPriorityQueueServiceStub(object):
  # missing associated documentation comment in .proto file
  pass

  def __init__(self, channel):
    """Constructor.

    Args:
      channel: A grpc.Channel.
    """
    self.GetNextItem = channel.unary_unary(
        '/spq_generated.SortingPriorityQueueService/GetNextItem',
        request_serializer=proto_dot_spq__pb2.GetItemRequest.SerializeToString,
        response_deserializer=proto_dot_spq__pb2.ItemResponse.FromString,
        )
    self.PeekNextItem = channel.unary_unary(
        '/spq_generated.SortingPriorityQueueService/PeekNextItem',
        request_serializer=proto_dot_spq__pb2.PeekItemRequest.SerializeToString,
        response_deserializer=proto_dot_spq__pb2.ItemResponse.FromString,
        )
    self.GetSize = channel.unary_unary(
        '/spq_generated.SortingPriorityQueueService/GetSize',
        request_serializer=proto_dot_spq__pb2.GetSizeRequest.SerializeToString,
        response_deserializer=proto_dot_spq__pb2.GetSizeResponse.FromString,
        )
    self.AddItem = channel.unary_unary(
        '/spq_generated.SortingPriorityQueueService/AddItem',
        request_serializer=proto_dot_spq__pb2.AddItemRequest.SerializeToString,
        response_deserializer=proto_dot_spq__pb2.AddItemResponse.FromString,
        )


class SortingPriorityQueueServiceServicer(object):
  # missing associated documentation comment in .proto file
  pass

  def GetNextItem(self, request, context):
    # missing associated documentation comment in .proto file
    pass
    context.set_code(grpc.StatusCode.UNIMPLEMENTED)
    context.set_details('Method not implemented!')
    raise NotImplementedError('Method not implemented!')

  def PeekNextItem(self, request, context):
    # missing associated documentation comment in .proto file
    pass
    context.set_code(grpc.StatusCode.UNIMPLEMENTED)
    context.set_details('Method not implemented!')
    raise NotImplementedError('Method not implemented!')

  def GetSize(self, request, context):
    # missing associated documentation comment in .proto file
    pass
    context.set_code(grpc.StatusCode.UNIMPLEMENTED)
    context.set_details('Method not implemented!')
    raise NotImplementedError('Method not implemented!')

  def AddItem(self, request, context):
    # missing associated documentation comment in .proto file
    pass
    context.set_code(grpc.StatusCode.UNIMPLEMENTED)
    context.set_details('Method not implemented!')
    raise NotImplementedError('Method not implemented!')


def add_SortingPriorityQueueServiceServicer_to_server(servicer, server):
  rpc_method_handlers = {
      'GetNextItem': grpc.unary_unary_rpc_method_handler(
          servicer.GetNextItem,
          request_deserializer=proto_dot_spq__pb2.GetItemRequest.FromString,
          response_serializer=proto_dot_spq__pb2.ItemResponse.SerializeToString,
      ),
      'PeekNextItem': grpc.unary_unary_rpc_method_handler(
          servicer.PeekNextItem,
          request_deserializer=proto_dot_spq__pb2.PeekItemRequest.FromString,
          response_serializer=proto_dot_spq__pb2.ItemResponse.SerializeToString,
      ),
      'GetSize': grpc.unary_unary_rpc_method_handler(
          servicer.GetSize,
          request_deserializer=proto_dot_spq__pb2.GetSizeRequest.FromString,
          response_serializer=proto_dot_spq__pb2.GetSizeResponse.SerializeToString,
      ),
      'AddItem': grpc.unary_unary_rpc_method_handler(
          servicer.AddItem,
          request_deserializer=proto_dot_spq__pb2.AddItemRequest.FromString,
          response_serializer=proto_dot_spq__pb2.AddItemResponse.SerializeToString,
      ),
  }
  generic_handler = grpc.method_handlers_generic_handler(
      'spq_generated.SortingPriorityQueueService', rpc_method_handlers)
  server.add_generic_rpc_handlers((generic_handler,))


class HealthServiceStub(object):
  # missing associated documentation comment in .proto file
  pass

  def __init__(self, channel):
    """Constructor.

    Args:
      channel: A grpc.Channel.
    """
    self.Check = channel.unary_unary(
        '/spq_generated.HealthService/Check',
        request_serializer=proto_dot_spq__pb2.HealthCheckRequest.SerializeToString,
        response_deserializer=proto_dot_spq__pb2.HealthCheckResponse.FromString,
        )
    self.Watch = channel.unary_stream(
        '/spq_generated.HealthService/Watch',
        request_serializer=proto_dot_spq__pb2.HealthCheckRequest.SerializeToString,
        response_deserializer=proto_dot_spq__pb2.HealthCheckResponse.FromString,
        )


class HealthServiceServicer(object):
  # missing associated documentation comment in .proto file
  pass

  def Check(self, request, context):
    # missing associated documentation comment in .proto file
    pass
    context.set_code(grpc.StatusCode.UNIMPLEMENTED)
    context.set_details('Method not implemented!')
    raise NotImplementedError('Method not implemented!')

  def Watch(self, request, context):
    # missing associated documentation comment in .proto file
    pass
    context.set_code(grpc.StatusCode.UNIMPLEMENTED)
    context.set_details('Method not implemented!')
    raise NotImplementedError('Method not implemented!')


def add_HealthServiceServicer_to_server(servicer, server):
  rpc_method_handlers = {
      'Check': grpc.unary_unary_rpc_method_handler(
          servicer.Check,
          request_deserializer=proto_dot_spq__pb2.HealthCheckRequest.FromString,
          response_serializer=proto_dot_spq__pb2.HealthCheckResponse.SerializeToString,
      ),
      'Watch': grpc.unary_stream_rpc_method_handler(
          servicer.Watch,
          request_deserializer=proto_dot_spq__pb2.HealthCheckRequest.FromString,
          response_serializer=proto_dot_spq__pb2.HealthCheckResponse.SerializeToString,
      ),
  }
  generic_handler = grpc.method_handlers_generic_handler(
      'spq_generated.HealthService', rpc_method_handlers)
  server.add_generic_rpc_handlers((generic_handler,))
