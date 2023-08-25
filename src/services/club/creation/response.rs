use crate::{
    application::domain::models::club::domain_club_model::DomainClubModel,
    services::proto::club::CreatedClubResponseData,
};

impl From<DomainClubModel> for CreatedClubResponseData {
    fn from(val: DomainClubModel) -> Self {
        CreatedClubResponseData {
            id: val.id,
            name: val.name,
            legal_name: val.legal_name,
            created_at: val.created_at.to_string(),
        }
    }
}
