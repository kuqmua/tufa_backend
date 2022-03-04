use actix_web::{post, HttpResponse, Responder};

#[post("/post_echo")]
async fn post_echo(req_body: String) -> impl Responder {
    println!("post_echo");
    HttpResponse::Ok().body(req_body)
}
//example from browser console (works only on  http://127.0.0.1:8080/ coz cors)
// let user = {
//     name: 'John',
//     surname: 'Smith'
// };

// let response = await fetch('http://127.0.0.1:8080/post_echo', {
//     method: 'POST',
//     headers: {
//       'Content-Type': 'application/json;charset=utf-8'
//     },
//     body: JSON.stringify(user)
// });
