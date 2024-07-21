use hyoubkp::{executor::Executor, tokmap::TokenMapperKind};

#[test]
fn test_expr_bank_transfer() {
    let mut executor = Executor::new(TokenMapperKind::Example);
    assert_eq!(
        executor.parse_expr("工行农行 20").unwrap().to_string(),
        "Transaction desc: \n\
            资产:银行:ABC 农业银行 debit 20.00\n\
            资产:银行:ICBC 工商银行 credit 20.00\n"
    );
}

#[test]
fn test_expr_unmaped_account() {
    let mut executor = Executor::new(TokenMapperKind::Example);
    assert_eq!(
        executor.parse_expr("工行邮储 20").unwrap().to_string(),
        "Transaction desc:  FIXME:[工行邮储 20]\n\
            不平衡的-CNY debit 20.00\n\
            资产:银行:ICBC 工商银行 credit 20.00\n"
    );
}

#[test]
fn test_expr_reward() {
    let mut executor = Executor::new(TokenMapperKind::Example);
    assert_eq!(
        executor.parse_expr("工行农行 20-5").unwrap().to_string(),
        "Transaction desc: \n\
            资产:银行:ABC 农业银行 debit 20.00\n\
            收入:优惠券变现 credit 5.00\n\
            资产:银行:ICBC 工商银行 credit 15.00\n"
    );

    assert_eq!(
        executor.parse_expr("工行农行 20-5-5").unwrap().to_string(),
        "Transaction desc: \n\
            资产:银行:ABC 农业银行 debit 20.00\n\
            收入:优惠券变现 credit 5.00\n\
            收入:优惠券变现 credit 5.00\n\
            资产:银行:ICBC 工商银行 credit 10.00\n"
    );
}

#[test]
fn test_expense() {
    let mut executor = Executor::new(TokenMapperKind::Example);
    assert_eq!(
        executor.parse_expr("中行用餐 20-5").unwrap().to_string(),
        "Transaction desc: \n\
            支出:用餐 debit 20.00\n\
            收入:优惠或礼遇 credit 5.00\n\
            负债:信用卡:中行 1234 credit 15.00\n"
    );
    assert_eq!(
        executor
            .parse_expr("中行用餐 20-5 10@9@8；农行10")
            .unwrap()
            .to_string(),
        "Transaction desc: \n\
        支出:用餐 debit 20.00\n\
        支出:用餐 debit 10.00\n\
        支出:用餐 debit 10.00\n\
        收入:优惠或礼遇 credit 5.00\n\
        负债:信用卡:中行 1234 credit 15.00\n\
        收入:优惠或礼遇 credit 1.00\n\
        收入:优惠或礼遇 credit 1.00\n\
        负债:信用卡:中行 1234 credit 8.00\n\
        负债:信用卡:农行 6666 credit 10.00\n"
    );
}

#[test]
fn test_expr_compound() {
    let mut executor = Executor::new(TokenMapperKind::Example);
    assert_eq!(
        executor
            .parse_expr("工行农行 20-1 30 50@45-10@1 中行 10，建行 5-1 邮储 6.2")
            .unwrap()
            .to_string(),
        "Transaction desc:  FIXME:[农行 20-1 30 50@45-10@1 中行 10，建行 5-1 邮储 6.2]\n\
            资产:银行:ABC 农业银行 debit 20.00\n\
            资产:银行:ABC 农业银行 debit 30.00\n\
            资产:银行:ABC 农业银行 debit 50.00\n\
            资产:银行:BOC 中国银行 debit 10.00\n\
            资产:银行:CCB 建设银行 debit 5.00\n\
            不平衡的-CNY debit 6.20\n\
            收入:优惠券变现 credit 1.00\n\
            资产:银行:ICBC 工商银行 credit 19.00\n\
            资产:银行:ICBC 工商银行 credit 30.00\n\
            收入:优惠券变现 credit 5.00\n\
            收入:优惠券变现 credit 10.00\n\
            收入:优惠券变现 credit 34.00\n\
            资产:银行:ICBC 工商银行 credit 1.00\n\
            资产:银行:ICBC 工商银行 credit 10.00\n\
            收入:优惠券变现 credit 1.00\n\
            资产:银行:BOC 中国银行 credit 4.00\n\
            资产:银行:BOC 中国银行 credit 6.20\n"
    );
}
