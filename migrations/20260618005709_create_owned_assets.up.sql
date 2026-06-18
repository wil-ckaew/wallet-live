-- Add up migration script here
CREATE TABLE IF NOT EXISTS owned_assets (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    asset_id BIGINT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    quantity_owned DOUBLE PRECISION NOT NULL,
    bought_for DOUBLE PRECISION NOT NULL,
    bought_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_owned_assets_user_id ON owned_assets(user_id);
CREATE INDEX idx_owned_assets_asset_id ON owned_assets(asset_id);