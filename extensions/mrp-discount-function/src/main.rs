use shopify_function::prelude::*;
use shopify_function::Result;
use std::str::FromStr;

#[shopify_function]
fn run(input: input::ResponseData) -> Result<output::FunctionRunResult> {
    let mut discounts = Vec::new();

    for line in input.cart.lines {
        let selling_price = match Decimal::from_str(&line.cost.total_amount.amount) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let mrp = match line.merchandise {
            input::CartLineMerchandise::ProductVariant(variant) => {
                variant
                    .compare_at_price
                    .and_then(|p| Decimal::from_str(&p.amount).ok())
            }
            _ => None,
        };

        let mrp = match mrp {
            Some(v) => v,
            None => continue,
        };

        let discounted_mrp = mrp * Decimal::from_str("0.75").unwrap();

        if selling_price <= discounted_mrp {
            continue;
        }

        let discount_amount = selling_price - discounted_mrp;

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
                    amount: output::Decimal(discount_amount),
                    applies_to_each_item: false,
                },
            ),
        });
    }

    Ok(output::FunctionRunResult {
        discounts,
        discount_application_strategy: output::DiscountApplicationStrategy::All,
    })
}
