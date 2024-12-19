use goose::prelude::*;
use goose_eggs::{validate_and_load_static_assets, Validate};

async fn loadtest_index(user: &mut GooseUser) -> TransactionResult {
    let goose = user.get("/goose/").await?;

    let validate = &Validate::builder()
        .status(200)
        .text("Gander")
        .build();

    validate_and_load_static_assets(user, goose, &validate).await?;
    Ok(())
}
async fn test_update_food(user: &mut GooseUser)->TransactionResult{
    let params = [("type", "Non-Veg"), ("foodName", "Chicken+Biryani"), ("foodCost", "9")];
    let _goose = user.post_form("/UpdateFood1", &params).await?;

    Ok(())
}



#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(scenario!("LoadtestTransactions")
            .register_transaction(transaction!(test_update_food))
        )
        .execute()
        .await?;

    Ok(())
}

