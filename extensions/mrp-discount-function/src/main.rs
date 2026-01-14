use shopify_function::prelude::*;
use shopify_function::Result;
use std::str::FromStr;

#[shopify_function]
fn run(input: input::ResponseData) -> Result<output::FunctionRunResult> {
    let mut discounts: Vec<output::Discount> = Vec::new();

    for line in input.cart.lines.iter() {
        // Parse selling price safely
        let selling_price = match Decimal::from_str(&line.cost.total_amount.amount) {
            Ok(value) => value,
            Err(_) => continue,
        };

        // Extract compare_at_price (MRP) only for product variants
        let mrp = match &line.merchandise {
            input::CartLineMerchandise::ProductVariant(variant) => {
                match &variant.compare_at_price {
                    Some(price) => Decimal::from_str(&price.amount).ok(),
                    None => None,
                }
            }
            _ => None,
        };

        let mrp = match mrp {
            Some(value) => value,
            None => continue,
        };

        // Calculate 25% discount on MRP
        let discount_multiplier = Decimal::from_str("0.75").unwrap();
        let discounted_mrp = mrp * discount_multiplier;

        // Apply discount only if selling price is higher
        if selling_price <= discounted_mrp {
            continue;
        }

        let discount_amount = selling_price - discounted_mrp;

        // Defensive check
        if discount_amount <= Decimal::ZERO {
            continue;
        }

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
