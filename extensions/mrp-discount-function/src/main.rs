use shopify_function::prelude::*;
use shopify_function::Result;
 
#[shopify_function]
fn run(input: input::ResponseData) -> Result<output::FunctionRunResult> {
    let mut discounts = vec![];
 
    for line in input.cart.lines {
        let price = line.cost.total_amount.amount.parse::<f64>().unwrap_or(0.0);
 
        let compare_at_price = match line.merchandise {
            input::CartLineMerchandise::ProductVariant(variant) => {
                variant.compare_at_price
                    .map(|p| p.amount.parse::<f64>().unwrap_or(0.0))
            }
            _ => None,
        };
 
        if let Some(mrp) = compare_at_price {
            let target_price = mrp * 0.75; // 25% off MRP
 
            if price > target_price {
                let discount_amount = price - target_price;
 
                discounts.push(output::Discount {
                    message: Some("25% off MRP".to_string()),
                    targets: vec![output::Target::ProductVariant(
                        output::ProductVariantTarget {
                            id: line.merchandise.id().to_string(),
                            quantity: None,
                        },
                    )],
                    value: output::Value::FixedAmount(
                        output::FixedAmountValue {
                            amount: output::Decimal(discount_amount.to_string()),
                            applies_to_each_item: false,
                        },
                    ),
                });
            }
        }
    }
 
    Ok(output::FunctionRunResult {
        discounts,
        discount_application_strategy: output::DiscountApplicationStrategy::First,
    })
}