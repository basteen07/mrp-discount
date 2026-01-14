use shopify_function::prelude::*;

#[typegen("schema.graphql")]
pub mod schema {
    #[query("src/cart_lines_discounts_generate_run.graphql")]
    pub mod cart_lines_discounts_generate_run;

    #[query("src/cart_delivery_options_discounts_generate_run.graphql")]
    pub mod cart_delivery_options_discounts_generate_run;
}

mod cart_lines_discounts_generate_run;
mod cart_delivery_options_discounts_generate_run;
