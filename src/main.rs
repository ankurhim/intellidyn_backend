use lambda_http::{
    Response, Body, IntoResponse
};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct PizzaList {
    pizzas: Vec<Pizza>
}

#[derive(Serialize)]
struct Pizza {
    name: String,
    price: i32,
}

impl PizzaList {
    fn new() -> PizzaList {
        PizzaList { pizzas: vec![
            Pizza { name: "veggie".to_string(), price: 10 },
            Pizza { name: "regina".to_string(), price: 12 },
            Pizza { name: "deluxe".to_string(), price: 09 },
        ] }
    }
}

fn get_pizza_from_name<'a>(pizza_name: &str, pizza_list: &'a PizzaList) -> Option<&'a Pizza> {
    let mut iter: core::slice::Iter<Pizza>  = pizza_list.pizzas.iter();
    iter.find(|pizza: &&Pizza| pizza.name == pizza_name)
}

async fn build_success_response(pizza: &Pizza) -> Response<Body> {
    json!(pizza).into_response().await
}

async fn build_failure_response(error_message: &str) -> Response<Body> {
    Response::builder()
    .status(400)
    .header("content-type", "application/json")
    .body(Body::from(json!({"error": error_message}).to_string()))
    .expect("Could not build the error response")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_pizza_list_test() {
        let all_pizzas = PizzaList::new();
        assert_eq!(3, all_pizzas.pizzas.len());

        let veggie: Option<&Pizza> = get_pizza_from_name("veggie", &all_pizzas);
        assert_eq!(10, veggie.unwrap().price);

        let regina: Option<&Pizza> = get_pizza_from_name("regina", &all_pizzas);
        assert_eq!(12, regina.unwrap().price);

        let deluxe: Option<&Pizza> = get_pizza_from_name("deluxe", &all_pizzas);
        assert_eq!(9, deluxe.unwrap().price);
    }

    #[tokio::test]
    async fn build_success_response_test() {
        let test_pizza = Pizza { name: String::from("test_pizza"), price: 100};
        let result: Response<Body> = build_success_response(&test_pizza).await;
        let (parts, body) = result.into_parts();
        assert_eq!(200, parts.status);
        assert_eq!("application/json", parts.headers.get("content-type").unwrap());
        assert_eq!("{\"name\":\"test_pizza\",\"price\":100}", String::from_utf8(body.to_ascii_lowercase()).unwrap());
    }

    #[tokio::test]
    async fn build_failure_response_test() {
        let result: Response<Body> = build_failure_response("test error message").await;
        let (parts, body) = result.into_parts();
        assert_eq!(400, parts.status);
        assert_eq!("application/json", parts.headers.get("content-type").unwrap());
        assert_eq!("{\"error\":\"test error message\"}", String::from_utf8(body.to_ascii_lowercase()).unwrap());
    }

    #[test]
    fn process_event_valid_pizza_test() {
        let pizza_list: PizzaList = PizzaList::new();
    }
}