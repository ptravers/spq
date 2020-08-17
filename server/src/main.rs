use std::pin::Pin;
use std::time::Duration;
use tokio::stream::Stream;
use tokio::stream::StreamExt;
use tokio::time;
use tonic::{transport::Server, Code, Request, Response, Status};
mod spq_generated {
    tonic::include_proto!("spq_generated");
}
use sp_queue::error::Error;
use sp_queue::feature_space::FeatureValue;
use sp_queue::SortingPriorityQueue;
use spq_generated::enqueue_request::Feature;
use spq_generated::health_check_response::ServingStatus;
use spq_generated::health_service_server::{HealthService, HealthServiceServer};
use spq_generated::sorting_priority_queue_service_server::{
    SortingPriorityQueueService, SortingPriorityQueueServiceServer,
};
use spq_generated::{
    DequeueRequest, EnqueueRequest, EnqueueResponse, GetEpochRequest, GetEpochResponse,
    GetSizeRequest, GetSizeResponse, HealthCheckRequest, HealthCheckResponse, ItemResponse,
    PeekItemRequest,
};
use std::sync::RwLock;

pub struct DefaultSortingPriorityQueueService {
    queue: RwLock<SortingPriorityQueue>,
}

fn to_feature_value(feature: Feature) -> FeatureValue {
    FeatureValue::new(feature.name, feature.value as usize)
}

fn to_status<V>(result: Result<V, Error>) -> Result<V, Status> {
    result.map_err(|err| match err {
        Error::Standard { message } => Status::new(Code::Internal, message),
        Error::Empty { message } => Status::new(Code::Internal, message),
    })
}

#[tonic::async_trait]
impl SortingPriorityQueueService for DefaultSortingPriorityQueueService {
    async fn enqueue(
        &self,
        _request: Request<EnqueueRequest>,
    ) -> Result<Response<EnqueueResponse>, Status> {
        self.queue
            .try_write()
            .map_err(|_| Status::new(Code::Unavailable, "Update in progress please retry"))
            .and_then(|mut queue| {
                let add_item_request = _request.get_ref();
                let size = to_status(queue.size())?;
                queue
                    .enqueue(
                        add_item_request.item.clone(),
                        add_item_request
                            .features
                            .clone()
                            .into_iter()
                            .map(to_feature_value)
                            .collect(),
                    )
                    .map(|_| size + 1)
                    .map_err(|err| Status::new(Code::Internal, err))
            })
            .map(|size| Response::new(EnqueueResponse { size: size as i64 }))
    }

    async fn dequeue(
        &self,
        _request: Request<DequeueRequest>,
    ) -> Result<Response<ItemResponse>, Status> {
        self.queue
            .try_write()
            .map_err(|_| Status::new(Code::Unavailable, "Update in progress please retry"))
            .and_then(|mut queue| {
                let (maybe_next, _) = to_status(queue.dequeue())?;
                let size = to_status(queue.size())?;

                Ok(Response::new(ItemResponse {
                    has_item: maybe_next.is_some(),
                    item: maybe_next.unwrap_or_default(),
                    size: size as i64,
                }))
            })
    }

    async fn peek_next_item(
        &self,
        _request: Request<PeekItemRequest>,
    ) -> Result<Response<ItemResponse>, Status> {
        self.queue
            .try_read()
            .map_err(|_| Status::new(Code::Unavailable, "Update in progress please retry"))
            .and_then(|queue| {
                let maybe_next = to_status(queue.peek())?;
                let size = to_status(queue.size())?;

                Ok(Response::new(ItemResponse {
                    has_item: maybe_next.is_some(),
                    item: maybe_next.unwrap_or_default(),
                    size: size as i64,
                }))
            })
    }

    async fn get_size(
        &self,
        _request: Request<GetSizeRequest>,
    ) -> Result<Response<GetSizeResponse>, Status> {
        self.queue
            .try_read()
            .map_err(|_| Status::new(Code::Unavailable, "Update in progress please retry"))
            .and_then(|queue| {
                let size = to_status(queue.size())?;

                Ok(Response::new(GetSizeResponse { size: size as i64 }))
            })
    }

    async fn get_epoch(
        &self,
        _request: Request<GetEpochRequest>,
    ) -> Result<Response<GetEpochResponse>, Status> {
        self.queue
            .try_read()
            .map_err(|_| Status::new(Code::Unavailable, "Update in progress please retry"))
            .and_then(|queue| {
                let epoch = to_status(queue.get_epoch())?;

                Ok(Response::new(GetEpochResponse {
                    epoch: epoch as i64,
                }))
            })
    }
}

#[derive(Default)]
pub struct DefaultHealthService {}

#[tonic::async_trait]
impl HealthService for DefaultHealthService {
    async fn check(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        Ok(Response::new(HealthCheckResponse {
            status: ServingStatus::Serving as i32,
        }))
    }

    type WatchStream =
        Pin<Box<dyn Stream<Item = Result<HealthCheckResponse, Status>> + Send + Sync + 'static>>;
    async fn watch(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<Self::WatchStream>, Status> {
        let health_stream: Self::WatchStream = Box::pin({
            time::interval(Duration::from_secs(1)).map(move |_| {
                Ok(HealthCheckResponse {
                    status: ServingStatus::Serving as i32,
                })
            })
        });

        Ok(Response::new(health_stream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::0]:9090".parse()?;
    let queue = SortingPriorityQueue::new_durable(
        vec!["feature_name".to_string()],
        "/var/lib/spqr".to_string(),
    )
    .unwrap_or_else(|err| panic!(err));

    let spq_service = DefaultSortingPriorityQueueService {
        queue: RwLock::new(queue),
    };
    let health_service = DefaultHealthService::default();

    println!("Booting");

    Server::builder()
        .add_service(SortingPriorityQueueServiceServer::new(spq_service))
        .add_service(HealthServiceServer::new(health_service))
        .serve(addr)
        .await?;

    println!("Shutting down");

    Ok(())
}
