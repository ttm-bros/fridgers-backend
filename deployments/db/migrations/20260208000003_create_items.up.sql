CREATE TABLE items (
    id UUID PRIMARY KEY,
    compartment_id UUID NOT NULL REFERENCES compartments(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    quantity DOUBLE PRECISION NOT NULL,
    unit VARCHAR(255) NOT NULL,
    expires_at DATE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
