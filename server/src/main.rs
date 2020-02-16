use std::pin::Pin;
use std::time::Duration;
use tokio::stream::Stream;
use tokio::stream::StreamExt;
use tokio::time;
use tonic::{transport::Server, Code, Request, Response, Status};
mod spq_generated {
    tonic::include_proto!("spq_generated");
}
use sp_queue::feature_space::FeatureValue;
use sp_queue::SortingPriorityQueue;
use spq_generated::add_item_request::Feature;
use spq_generated::health_check_response::ServingStatus;
use spq_generated::health_service_server::{HealthService, HealthServiceServer};
use spq_generated::sorting_priority_queue_service_server::{
    SortingPriorityQueueService, SortingPriorityQueueServiceServer,
};
use spq_generated::{
    AddItemRequest, AddItemResponse, GetItemRequest, GetSizeRequest, GetSizeResponse,
    HealthCheckRequest, HealthCheckResponse, ItemResponse, PeekItemRequest,
};
use std::sync::{Arc, RwLock};

pub struct DefaultSortingPriorityQueueService {
    queue: Arc<RwLock<SortingPriorityQueue<Vec<u8>>>>,
}

fn to_feature_value(feature: Feature) -> FeatureValue {
    FeatureValue::new(feature.name, feature.value as usize)
}

#[tonic::async_trait]
impl SortingPriorityQueueService for DefaultSortingPriorityQueueService {
    async fn add_item(
        &self,
        _request: Request<AddItemRequest>,
    ) -> Result<Response<AddItemResponse>, Status> {
        return self
            .queue
            .try_write()
            .map_err(|_| Status::new(Code::Unavailable, "Update in progress please retry"))
            .and_then(|mut queue| {
                let add_item_request = _request.get_ref();
                let size = queue.size();
                queue
                    .add(
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
            .map(|size| Response::new(AddItemResponse { size: size as i64 }));
    }

    async fn get_next_item(
        &self,
        _request: Request<GetItemRequest>,
    ) -> Result<Response<ItemResponse>, Status> {
        return self
            .queue
            .try_write()
            .map(|mut queue| {
                let (maybe_next, _) = queue.next();
                let size = queue.size();

                return Response::new(ItemResponse {
                    has_item: maybe_next.is_some(),
                    item: maybe_next.unwrap_or(vec![]),
                    size: size as i64,
                });
            })
            .map_err(|_| Status::new(Code::Unavailable, "Update in progress please retry"));
    }

    async fn peek_next_item(
        &self,
        _request: Request<PeekItemRequest>,
    ) -> Result<Response<ItemResponse>, Status> {
        return self
            .queue
            .try_read()
            .map(|queue| {
                let maybe_next = queue.peek();
                let size = queue.size();

                return Response::new(ItemResponse {
                    has_item: maybe_next.is_some(),
                    item: maybe_next.unwrap_or(&vec![]).clone(),
                    size: size as i64,
                });
            })
            .map_err(|_| Status::new(Code::Unavailable, "Update in progress please retry"));
    }

    async fn get_size(
        &self,
        _request: Request<GetSizeRequest>,
    ) -> Result<Response<GetSizeResponse>, Status> {
        return self
            .queue
            .try_read()
            .map(|queue| {
                let size = queue.size();

                return Response::new(GetSizeResponse { size: size as i64 });
            })
            .map_err(|_| Status::new(Code::Unavailable, "Update in progress please retry"));
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
        return Ok(Response::new(HealthCheckResponse {
            status: ServingStatus::Serving as i32,
        }));
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
    let queue = Arc::new(RwLock::new(SortingPriorityQueue::new(vec![
        "feature_name".to_string()
    ])));

    let spq_service = DefaultSortingPriorityQueueService { queue };
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
