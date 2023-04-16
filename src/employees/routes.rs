use actix_rt::signal;
use crate::employees::{Employee, Employees};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use skywalking::{
    logging::{
        logger::Logger,
        record::{LogRecord, RecordType}
    },
//    metrics::{meter::Counter, metricer::Metricer},
//    reporter::grpc::GrpcReporter,
    trace::tracer::Tracer,
};
use std::sync::Arc;

pub struct skywalkingTool {
    pub tracer: Tracer,
    pub logger: Logger,
}

#[get("/emplyservice/employees")]
async fn find_all(
    //tracer: web::Data<Arc<skywalking::trace::tracer::Tracer>>,
    swtool: web::Data<Arc<skywalkingTool>>,
) -> Result<HttpResponse, CustomError> {
    info!("emplyservice got /employees request to list all employees");
    skProcessing(&swtool.tracer, &swtool.logger, "find_all");
    let employees = Employees::find_all()?;
    Ok(HttpResponse::Ok().json(employees))
}

#[get("/emplyservice/employees/{id}")]
async fn find(
    id: web::Path<i32>,
    swtool: web::Data<Arc<skywalkingTool>>,
) -> Result<HttpResponse, CustomError> {
/*    let mut ctx = TRACER.create_trace_context();
    let _span = ctx.create_entry_span("get_employee_by_id");*/
    skProcessing(&swtool.tracer, &swtool.logger, "find_employee_by_id");
    let employee = Employees::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[post("/emplyservice/employees")]
async fn create(
    employee: web::Json<Employee>,
    swtool: web::Data<Arc<skywalkingTool>>,
) -> Result<HttpResponse, CustomError> {
/*    let mut ctx = TRACER.create_trace_context();
    let _span = ctx.create_entry_span("post_employee");*/
    skProcessing(&swtool.tracer, &swtool.logger, "post employee");
    let employee = Employees::create(employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[put("/emplyservice/employees/{id}")]
async fn update(
    id: web::Path<i32>,
    employee: web::Json<Employee>,
    swtool: web::Data<Arc<skywalkingTool>>,
) -> Result<HttpResponse, CustomError> {
/*    let mut ctx = TRACER.create_trace_context();
    let _span = ctx.create_entry_span("put_employee");*/
    skProcessing(&swtool.tracer, &swtool.logger, "put employee");
    let employee = Employees::update(id.into_inner(), employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[delete("/emplyservice/employees/{id}")]
async fn delete(
    id: web::Path<i32>,
    swtool: web::Data<Arc<skywalkingTool>>,
) -> Result<HttpResponse, CustomError> {
/*    let mut ctx = TRACER.create_trace_context();
    let _span = ctx.create_entry_span("delete_employee");*/
    skProcessing(&swtool.tracer, &swtool.logger, "delete employee");
    let deleted_employee = Employees::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_employee })))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}

fn skProcessing(tracer: &Tracer, logger: &Logger, opName: &str){
    let mut ctx = tracer.create_trace_context();
    let _span = ctx.create_entry_span(opName);
    logger.log(
        LogRecord::new()
            .add_tag("level", "INFO")
            .with_tracing_context(&ctx)
            .with_span(&_span)
            .record_type(RecordType::Text)
            .content(opName)
    )
}