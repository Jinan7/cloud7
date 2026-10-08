use std::fmt::{Debug, Display};

use actix_web::error::ErrorInternalServerError;


pub fn e500<T> (e: T) -> actix_web::Error 
    where 
        T: Display + Debug + 'static
{
    ErrorInternalServerError(e)
}