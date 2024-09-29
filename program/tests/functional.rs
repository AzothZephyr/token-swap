#[tokio::test]
async fn test_admin_functions() {
    let (mut banks_client, payer, recent_blockhash) = program_test().start().await;
    let token_swap = TestSwap::init(&mut banks_client, &payer, &recent_blockhash).await.unwrap();

    // Test pausing the swap
    let pause_accounts = vec![
        AccountMeta::new(token_swap.swap_account, false),
        AccountMeta::new_readonly(token_swap.admin_pubkey, true),
    ];
    let pause_instruction = Instruction {
        program_id: token_swap_program_id(),
        accounts: pause_accounts,
        data: SwapInstruction::Pause.pack(),
    };
    let mut transaction = Transaction::new_with_payer(&[pause_instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Try to swap when paused (should fail)
    let swap_result = token_swap.swap(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &token_swap.token_a,
        &token_swap.token_b,
        100,
        50,
    ).await;
    assert!(swap_result.is_err());

    // Unpause the swap
    let unpause_accounts = vec![
        AccountMeta::new(token_swap.swap_account, false),
        AccountMeta::new_readonly(token_swap.admin_pubkey, true),
    ];
    let unpause_instruction = Instruction {
        program_id: token_swap_program_id(),
        accounts: unpause_accounts,
        data: SwapInstruction::Unpause.pack(),
    };
    let mut transaction = Transaction::new_with_payer(&[unpause_instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Try to swap when unpaused (should succeed)
    let swap_result = token_swap.swap(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &token_swap.token_a,
        &token_swap.token_b,
        100,
        50,
    ).await;
    assert!(swap_result.is_ok());

    // Change the price
    let new_price = 200;
    let change_price_accounts = vec![
        AccountMeta::new(token_swap.swap_account, false),
        AccountMeta::new_readonly(token_swap.admin_pubkey, true),
    ];
    let change_price_instruction = Instruction {
        program_id: token_swap_program_id(),
        accounts: change_price_accounts,
        data: SwapInstruction::ChangePrice { new_price }.pack(),
    };
    let mut transaction = Transaction::new_with_payer(&[change_price_instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify the new price by performing a swap
    let swap_result = token_swap.swap(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &token_swap.token_a,
        &token_swap.token_b,
        100,
        50,
    ).await.unwrap();
    // Assert that the swap result reflects the new price
    // This will depend on how your swap calculation works with the new price
}