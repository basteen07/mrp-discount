use crate::schema::CartLineTarget;
use crate::schema::CartLinesDiscountsGenerateRunResult;
use crate::schema::CartOperation;
use crate::schema::DiscountClass;
use crate::schema::OrderDiscountCandidate;
use crate::schema::OrderDiscountCandidateTarget;
use crate::schema::OrderDiscountCandidateValue;
use crate::schema::OrderDiscountSelectionStrategy;
use crate::schema::OrderDiscountsAddOperation;
use crate::schema::OrderSubtotalTarget;
use crate::schema::Percentage;
use crate::schema::ProductDiscountCandidate;
use crate::schema::ProductDiscountCandidateTarget;
use crate::schema::ProductDiscountCandidateValue;
use crate::schema::ProductDiscountSelectionStrategy;
use crate::schema::ProductDiscountsAddOperation;

use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

#[shopify_function]
fn cart_lines_discounts_generate_run(
    input: schema::cart_lines_discounts_generate_run::Input,
) -> Result<CartLinesDiscountsGenerateRunResult> {
    let max_cart_line = input
        .cart()
        .lines()
        .iter()
        .max_by(|a, b| {
            a.cost()
                .subtotal_amount()
                .amount()
                .partial_cmp(b.cost().subtotal_amount().amount())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .ok_or("No cart lines found")?;

    let has_order_discount_class = input
        .discount()
        .discount_classes()
        .contains(&DiscountClass::Order);
    let has_product_discount_class = input
        .discount()
        .discount_classes()
        .contains(&DiscountClass::Product);

    if !has_order_discount_class && !has_product_discount_class {
        return Ok(CartLinesDiscountsGenerateRunResult { operations: vec![] });
    }

    let mut operations = vec![];

    for line in input.cart().lines() {
        let selling_price = Decimal::from(&line.cost().total_amount().amount());

        let mrp = match line.merchandise() {
            schema::cart_lines_discounts_generate_run::CartLineMerchandise::ProductVariant(variant) => {
                variant
                    .compare_at_price()
                    .map(|p| Decimal::from(&p.amount()))
            }
            _ => None,
        };

        let mrp = match mrp {
            Some(v) => v,
            None => continue,
        };

        let discounted_mrp = mrp * Decimal::from(0.75);

        if selling_price <= discounted_mrp {
            continue;
        }

        let discount_amount = selling_price - discounted_mrp;

        operations.push(CartOperation::ProductDiscountsAddOperation(
            ProductDiscountsAddOperation {
                discount: ProductDiscountCandidate {
                    message: Some("25% off MRP".to_string()),
                    targets: vec![ProductDiscountCandidateTarget::ProductVariant(
                        ProductVariantTarget {
                            id: line.merchandise().id().to_string(),
                            quantity: None,
                        },
                    )],
                    value: ProductDiscountCandidateValue::FixedAmount(
                        FixedAmountValue {
                            amount: discount_amount,
                            applies_to_each_item: false,
                        },
                    ),
                    once_per_order: false,
                    selection_strategy: ProductDiscountSelectionStrategy::All,
                },
            },
        ));
    }

    Ok(CartLinesDiscountsGenerateRunResult { operations })
}
