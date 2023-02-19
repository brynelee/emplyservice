use crate::employees::{Employee, Employees};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;


#[get("/emplyservice/employees")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    info!("emplyservice got /employees request to list all employees");
    let employees = Employees::find_all()?;
    Ok(HttpResponse::Ok().json(employees))
}

#[get("/emplyservice/employees/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let employee = Employees::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[post("/emplyservice/employees")]
async fn create(employee: web::Json<Employee>) -> Result<HttpResponse, CustomError> {
    let employee = Employees::create(employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[put("/emplyservice/employees/{id}")]
async fn update(
    id: web::Path<i32>,
    employee: web::Json<Employee>,
) -> Result<HttpResponse, CustomError> {
    let employee = Employees::update(id.into_inner(), employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[delete("/emplyservice/employees/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
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
