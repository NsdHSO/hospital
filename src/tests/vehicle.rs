#[cfg(test)]
mod vehicle {
    use crate::tests::vehicle::Vehicle;

    #[tokio::test]
    pub async fn check_name() {
        let vehicle = Vehicle::new(
            None,
            None,
            Some("Term".to_string()),
            Some("TErmin".to_string()),
            Some("UPGRADE".to_string()),
        );

        assert_eq!(vehicle.product_name(), Some("TErmin".to_string()));
    }

    #[tokio::test]
    pub async fn check_name_rec() {
        let vehicle = Vehicle::new(
            Some("Term".to_string()),
            Some("TErmin".to_string()),
            None,
            None,
            Some("MIGRATED".to_string()),
        );

        assert_eq!(vehicle.product_name(), Some("TErmin".to_string()));
    }

    #[tokio::test]
    pub async fn check_name_simple() {
        let vehicle = Vehicle::new(
            Some("product_code".to_string()),
            Some("product_description".to_string()),
            Some("category_code".to_string()),
            Some("category_description".to_string()),
            Some("NONE".to_string()),
        );

        assert_eq!(
            vehicle.product_name(),
            Some("category_description".to_string())
        );
    }
}
struct Vehicle {
    product_code: Option<String>,
    product_description: Option<String>,
    category_code: Option<String>,
    category_description: Option<String>,
    tier_evolution: Option<String>,
}

impl Vehicle {
    pub fn new(
        product_code: Option<String>,
        product_description: Option<String>,
        category_code: Option<String>,
        category_description: Option<String>,
        tier_evolution: Option<String>,
    ) -> Self {
        Self {
            product_code,
            product_description,
            category_code,
            category_description,
            tier_evolution,
        }
    }

    pub fn product_name(&self) -> Option<String> {
        match self.tier_evolution.clone().unwrap().as_str() {
            "NONE" => {
                if self.category_code.clone().is_some() {
                    return self.category_description.clone();
                };
                self.product_description.clone()
            }
            _ => {
                if self.product_code.clone().is_some() {
                    return self.product_description.clone();
                };
                self.category_description.clone()
            }
        }
    }
}
