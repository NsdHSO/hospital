#[cfg(test)]
mod vehicle {
    use crate::tests::vehicle::Vehicle;

    #[tokio::test]
    pub async fn checkName() {
        let vehicle = Vehicle::new(
            None,
            None,
            Some("Term".to_string()),
            Some("TErmin".to_string()),
            Some("UPGRADE".to_string()),
        );

        assert_eq!(vehicle.productName(), Some("TErmin".to_string()));
    }

    #[tokio::test]
    pub async fn checkName_rec() {
        let vehicle = Vehicle::new(
            Some("Term".to_string()),
            Some("TErmin".to_string()),
            None,
            None,
            Some("MIGRATED".to_string()),
        );

        assert_eq!(vehicle.productName(), Some("TErmin".to_string()));
    }

    #[tokio::test]
    pub async fn checkName_simple() {
        let vehicle = Vehicle::new(
            Some("product_code".to_string()),
            Some("product_description".to_string()),
            Some("category_code".to_string()),
            Some("category_description".to_string()),
            Some("NONE".to_string()),
        );

        assert_eq!(
            vehicle.productName(),
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
        productCode: Option<String>,
        productDescription: Option<String>,
        categoryCode: Option<String>,
        categoryDescription: Option<String>,
        tierEvolution: Option<String>,
    ) -> Self {
        Self {
            product_code: Some(productCode).unwrap(),
            product_description: Some(productDescription).unwrap(),
            category_code: Some(categoryCode).unwrap(),
            category_description: Some(categoryDescription).unwrap(),
            tier_evolution: Some(tierEvolution).unwrap(),
        }
    }

    pub fn productName(&self) -> Option<String> {
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
