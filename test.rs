#[cfg(test)]
mod tests {
    use super::*;
    use ic_cdk::export::Principal;

    #[test]
    fn test_send_tokens() {
        let owner = Principal::anonymous();
        init_wallet(owner);

        unsafe {
            let wallet = WALLET.as_mut().unwrap();
            wallet.balances.insert(owner, 100);
        }

        let to = Principal::management_canister();
        let result = send_tokens(to, 50);
        assert!(result.is_ok());

        unsafe {
            let wallet = WALLET.as_ref().unwrap();
            assert_eq!(*wallet.balances.get(&owner).unwrap(), 50);
            assert_eq!(*wallet.balances.get(&to).unwrap(), 50);
        }
    }

    #[test]
    fn test_receive_tokens() {
        let from = Principal::management_canister();
        let result = receive_tokens(from, 100);
        assert!(result.is_ok());

        unsafe {
            let wallet = WALLET.as_ref().unwrap();
            assert_eq!(*wallet.balances.get(&from).unwrap(), 100);
        }
    }

    #[test]
    fn test_get_balance() {
        let caller = Principal::anonymous();
        init_wallet(caller);

        unsafe {
            let wallet = WALLET.as_mut().unwrap();
            wallet.balances.insert(caller, 75);
        }

        let balance = get_balance();
        assert_eq!(balance, 75);
    }
}
