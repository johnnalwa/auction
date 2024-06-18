#[cfg(test)]
mod tests {
    use super::*;
    use ic_cdk::export::Principal;

    #[test]
    fn test_list_item() {
        // Setup
        let title = "Test Item".to_string();
        let description = "A test item.".to_string();

        // Act
        let item_id = list_item(title.clone(), description.clone());

        // Assert
        let item = get_item(item_id).unwrap();
        assert_eq!(item.title, title);
        assert_eq!(item.description, description);
    }

    #[test]
    fn test_bid() {
        // Setup
        let title = "Test Item".to_string();
        let description = "A test item.".to_string();
        let item_id = list_item(title, description);

        // Act
        bid(item_id, 100);

        // Assert
        let highest_bid = get_highest_bid(item_id).unwrap();
        assert_eq!(highest_bid.amount, 100);
    }

    #[test]
    fn test_update_item() {
        // Setup
        let title = "Original Title".to_string();
        let description = "Original Description.".to_string();
        let item_id = list_item(title, description);

        // Act
        update_item(item_id, "Updated Title".to_string(), "Updated Description".to_string());

        // Assert
        let item = get_item(item_id).unwrap();
        assert_eq!(item.title, "Updated Title");
        assert_eq!(item.description, "Updated Description");
    }

    #[test]
    fn test_stop_listing() {
        // Setup
        let title = "Test Item".to_string();
        let description = "A test item.".to_string();
        let item_id = list_item(title, description);

        // Act
        stop_listing(item_id);

        // Assert
        let item = get_item(item_id).unwrap();
        assert!(!item.active);
    }
}
