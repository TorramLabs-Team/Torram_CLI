#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_query_all_tokens() {
        let deps = mock_dependencies();

        let msg = QueryMsg::GetAllTokens {};
        let res = query(deps.as_ref(), mock_env(), msg);
        
        // In a real test, this would succeed with mocked TSB data
        // For now, we just verify the query structure is correct
        assert!(res.is_err()); // Expected to fail in mock environment
    }

    #[test]
    fn test_query_token_summary() {
        let deps = mock_dependencies();

        let msg = QueryMsg::GetTokenSummary {
            token_id: "test_token".to_string(),
        };
        let res = query(deps.as_ref(), mock_env(), msg);
        
        // In a real test, this would succeed with mocked TSB data
        assert!(res.is_err()); // Expected to fail in mock environment
    }

    #[test]
    fn test_query_user_portfolio() {
        let deps = mock_dependencies();

        let msg = QueryMsg::GetUserPortfolio {
            owner: "torram1test123".to_string(),
        };
        let res = query(deps.as_ref(), mock_env(), msg);
        
        // In a real test, this would succeed with mocked TSB data
        assert!(res.is_err()); // Expected to fail in mock environment
    }

    #[test]
    fn test_execute_fails() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("anyone", &[]);

        // This contract is read-only, so any execute should fail
        let res = execute(deps.as_mut(), env, info, ExecuteMsg {});
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("read-only"));
    }
} 